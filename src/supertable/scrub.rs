// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Explicit, operator-driven verification of a table's superfiles.
//!
//! The default checksum policy verifies a superfile's bytes once, as they
//! land in the local disk cache, and not again on each open. What that trades
//! away is corruption arriving *after* a copy has landed: a bit flipped on the
//! cache disk, in a file nothing re-reads from storage. A scrub is how an
//! operator closes that window on their own schedule, in the same way
//! Lucene's index checker is run deliberately rather than on every search.

use std::{fs::read, sync::Arc};

use futures::{StreamExt, stream};
use tokio::task::spawn_blocking;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    config::CrcVerification,
    runtime_bridge::bridge_on_runtime,
    storage::StorageProvider,
    superfile::reader::{OpenOptions, SuperfileReader},
    supertable::{
        Supertable, error::ScrubError, handle::SupertableInner, manifest::SuperfileEntry,
        reader_cache::DiskCacheStore,
    },
};

/// Superfiles one scrub verifies at a time. Each one in flight holds its whole
/// bytes in memory while its checksums are computed, so this is a memory bound
/// before it is a concurrency one: a scrub is a background chore and must not
/// crowd out the queries running beside it.
const SCRUB_CONCURRENCY: usize = 8;

/// Where a scrub read the copy it checked. Which copy was checked decides what
/// a clean result means, so a report that did not say would be ambiguous.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckedCopy {
    /// The copy in the local disk cache, which is the one nothing else
    /// validates and the one a scrub exists to check.
    LocalCache,
    /// The authoritative copy in the object store, read because no complete
    /// local copy was present.
    ObjectStore,
}

/// One superfile a scrub could not verify, and why.
///
/// "Damaged" covers both a checksum that did not match and a copy that could
/// not be read at all, because an operator needs to act on either: a superfile
/// the manifest names and a reader cannot open is an integrity problem whether
/// its bytes are wrong or absent.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct DamagedSuperfile {
    /// The superfile's identifier, as the manifest records it.
    pub superfile_id: Uuid,
    /// Its object-storage key, so the finding can be acted on directly.
    pub storage_path: String,
    /// Which copy failed.
    pub copy: CheckedCopy,
    /// What went wrong, as text. The underlying error types are internal.
    pub cause: String,
}

/// What a [`crate::Supertable::scrub`] pass found.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ScrubReport {
    /// Superfiles whose checksums were verified and matched.
    pub superfiles_verified: u64,
    /// Bytes read and checksummed across those superfiles.
    pub bytes_verified: u64,
    /// Superfiles verified from a copy on the local cache disk. The remainder
    /// were read from object storage because no complete local copy existed.
    pub from_local_cache: u64,
    /// Everything that failed, in no particular order.
    pub damaged: Vec<DamagedSuperfile>,
}

impl ScrubReport {
    /// Whether every superfile the table references verified.
    pub fn is_clean(&self) -> bool {
        self.damaged.is_empty()
    }
}

impl Supertable {
    /// Verify the checksums of every superfile this table references, and
    /// report the ones that fail.
    ///
    /// Reads each superfile in full: the copy on the local cache disk when a
    /// complete one is there, and otherwise the authoritative copy from object
    /// storage. A table that does not fit in the cache will therefore pull the
    /// remainder back from storage, which costs what a full table scan costs.
    /// Nothing is modified, no writer slot is taken, and queries keep running
    /// throughout.
    ///
    /// Covers what the superfile's own checksums cover: the embedded full-text
    /// and vector index blobs, including their per-subsection checksums. The
    /// surrounding Parquet body carries no checksum of ours and is not checked
    /// here, so a clean report means the index structures are intact, not that
    /// every byte of the file is.
    ///
    /// A damaged superfile is reported, not repaired. When the bad copy is the
    /// local one, deleting the cache file is enough: the next read refetches it
    /// from storage, which is always current.
    pub fn scrub(&self) -> Result<ScrubReport, ScrubError> {
        bridge_on_runtime(self.scrub_async(), &self.inner().query_runtime())
    }

    #[cfg_attr(
        feature = "detailed-tracing",
        tracing::instrument(name = "scrub", skip_all, fields(role = self.role().as_str()))
    )]
    pub(crate) async fn scrub_async(&self) -> Result<ScrubReport, ScrubError> {
        let mut report = scrub_inner(self.inner()).await?;

        // The hidden vector index is a table of its own, and a corrupt
        // superfile in it breaks vector search exactly as a corrupt user
        // superfile breaks everything else. An operator asking whether their
        // data is intact is asking about both.
        if let Some(hidden) = self.inner().vector_index_table.as_ref() {
            let hidden_report = scrub_inner(hidden.inner()).await?;
            report.superfiles_verified += hidden_report.superfiles_verified;
            report.bytes_verified += hidden_report.bytes_verified;
            report.from_local_cache += hidden_report.from_local_cache;
            report.damaged.extend(hidden_report.damaged);
        }

        if report.is_clean() {
            info!(
                superfiles = report.superfiles_verified,
                bytes = report.bytes_verified,
                "scrub: every superfile verified"
            );
        } else {
            warn!(
                damaged = report.damaged.len(),
                superfiles = report.superfiles_verified,
                "scrub: superfiles failed verification"
            );
        }
        Ok(report)
    }
}

/// Scrub one table's superfiles. Called once for the user table and once for
/// the hidden vector index, which is a `Supertable` in its own right.
async fn scrub_inner(inner: &Arc<SupertableInner>) -> Result<ScrubReport, ScrubError> {
    let storage = inner.options.storage.clone().ok_or(ScrubError::NoStorage)?;

    // Advance to the committed manifest first, so the scrub describes the
    // table rather than one handle's memory of it. A superfile another handle
    // committed since this handle's snapshot would otherwise go unchecked, and
    // an operator would read the clean report as covering it.
    inner
        .refresh()
        .await
        .map_err(|error| ScrubError::Enumerate(error.to_string()))?;

    let entries = inner
        .manifest
        .load_full()
        .get_all_superfiles_loaded()
        .await
        .map_err(|error| ScrubError::Enumerate(error.to_string()))?;

    let disk_cache = inner.options.disk_cache.clone();
    let outcomes = stream::iter(entries.into_iter().map(|entry| {
        let storage = Arc::clone(&storage);
        let disk_cache = disk_cache.clone();
        async move { verify_one(&storage, disk_cache.as_ref(), &entry).await }
    }))
    .buffer_unordered(SCRUB_CONCURRENCY)
    .collect::<Vec<_>>()
    .await;

    let mut report = ScrubReport::default();
    for outcome in outcomes {
        match outcome {
            Ok(verified) => {
                report.superfiles_verified += 1;
                report.bytes_verified += verified.bytes;
                if verified.copy == CheckedCopy::LocalCache {
                    report.from_local_cache += 1;
                }
            }
            Err(damaged) => report.damaged.push(damaged),
        }
    }
    Ok(report)
}

/// One superfile that verified, and what it cost to say so.
struct Verified {
    bytes: u64,
    copy: CheckedCopy,
}

/// Read one superfile in full and check its checksums.
///
/// Prefers the local cache file, because that is the copy the default policy
/// stops re-checking and therefore the copy a scrub is for. Only a promoted
/// whole-file copy qualifies: a partially filled one lives under a different
/// name, and a copy promoted without its vector blob under another, so neither
/// is mistaken for a complete superfile here.
async fn verify_one(
    storage: &Arc<dyn StorageProvider>,
    disk_cache: Option<&Arc<DiskCacheStore>>,
    entry: &SuperfileEntry,
) -> Result<Verified, DamagedSuperfile> {
    let storage_path = entry.storage_path();
    let local = disk_cache.map(|cache| cache.cache_path(&entry.uri));

    let (bytes, copy) = match local {
        Some(path) if path.is_file() => {
            let read_local = spawn_blocking(move || read(&path)).await;
            match read_local {
                Ok(Ok(bytes)) => (bytes.into(), CheckedCopy::LocalCache),
                Ok(Err(error)) => {
                    return Err(damaged(entry, storage_path, CheckedCopy::LocalCache, error));
                }
                Err(error) => {
                    return Err(damaged(entry, storage_path, CheckedCopy::LocalCache, error));
                }
            }
        }
        _ => match storage.get(&storage_path).await {
            Ok((bytes, _meta)) => (bytes, CheckedCopy::ObjectStore),
            Err(error) => {
                return Err(damaged(
                    entry,
                    storage_path,
                    CheckedCopy::ObjectStore,
                    error,
                ));
            }
        },
    };

    let size = bytes.len() as u64;
    // Checksumming the whole file is the work this call exists to do, and it
    // is pure compute over bytes already in hand. Off the async workers, so a
    // scrub of a large table does not stall the I/O the queries beside it need.
    let opened = spawn_blocking(move || {
        SuperfileReader::open_with(
            bytes,
            OpenOptions {
                verify_crc: CrcVerification::Always.verifies_on_open(),
            },
        )
        .map(|_reader| ())
    })
    .await;

    match opened {
        Ok(Ok(())) => Ok(Verified { bytes: size, copy }),
        Ok(Err(error)) => Err(damaged(entry, storage_path, copy, error)),
        Err(error) => Err(damaged(entry, storage_path, copy, error)),
    }
}

/// Build a finding. The cause is carried as text because the error types
/// underneath are internal, and a scrub report is something an operator reads.
fn damaged(
    entry: &SuperfileEntry,
    storage_path: String,
    copy: CheckedCopy,
    cause: impl ToString,
) -> DamagedSuperfile {
    DamagedSuperfile {
        superfile_id: entry.superfile_id,
        storage_path,
        copy,
        cause: cause.to_string(),
    }
}
