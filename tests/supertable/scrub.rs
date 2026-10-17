// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! The explicit verification pass, over a real table.
//!
//! Two halves have to hold together, and either alone is worthless. A scrub
//! that reports a healthy table as damaged will be switched off, so a clean
//! table must come back clean; a scrub that misses a corrupted superfile is
//! worse than none at all, so a damaged one must come back named. The tests
//! here assert both over the same fixture, and separately for the two copies
//! a scrub can read: the one in object storage and the one on the cache disk,
//! which is the copy the default checksum policy stops re-checking.

use std::{
    fs,
    path::{Path, PathBuf},
    thread::sleep,
    time::{Duration, Instant},
};

use infino::{
    Bm25SearchOptions, CheckedCopy, ColdFetchMode, ConnectOptions, ScrubError, Supertable, connect,
    connect_with,
    superfile::format::{footer::read_kv_metadata, kv},
};
use tempfile::TempDir;

use crate::reindex_fixture::{N_DOCS, TABLE, superfile_paths, write_table};

/// Bytes back from the vector blob's end to flip. Near the end rather than
/// near the start so the flip lands in data a checksum covers rather than in
/// a header or an offset table, which a structural check would reject on its
/// own and tell us nothing about verification.
const CORRUPT_AT_VEC_TAIL: usize = 16;

/// Longest a promoted cache copy is waited for. The fill that writes it runs
/// in the background, so there is nothing to synchronise on but its result.
const PROMOTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Gap between checks while waiting for that copy.
const PROMOTION_POLL: Duration = Duration::from_millis(25);

/// Flip one byte inside a superfile's vector blob, which is a region the
/// superfile's own checksums cover.
///
/// The region matters: most of the file is Parquet, which these checksums do
/// not cover, so a flip at an arbitrary offset would go unnoticed and say
/// nothing about whether verification ran.
fn flip_a_checksummed_byte(path: &Path) {
    let mut bytes = fs::read(path).expect("read the superfile");
    let footer = read_kv_metadata(&bytes).expect("read superfile key-value metadata");
    let at = |key: &str| -> usize {
        footer
            .get(key)
            .expect("every fixture superfile carries a vector blob")
            .parse()
            .expect("the footer records it as a number")
    };
    let end = at(kv::VEC_OFFSET) + at(kv::VEC_LENGTH);
    bytes[end - CORRUPT_AT_VEC_TAIL] ^= 0xFF;
    fs::write(path, &bytes).expect("write the corrupted superfile back");
}

/// Whole superfile copies the disk cache has promoted under `cache_root`.
///
/// A partially filled copy and one promoted without its vector blob live
/// under their own suffixes, as does a download still in flight; none of them
/// is a complete superfile, so none is counted here.
fn promoted_copies(cache_root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![cache_root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "parquet") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

/// Wait for the background fill to promote at least one whole copy.
fn await_a_promoted_copy(cache_root: &Path) -> Vec<PathBuf> {
    let deadline = Instant::now() + PROMOTION_TIMEOUT;
    loop {
        let copies = promoted_copies(cache_root);
        if !copies.is_empty() {
            return copies;
        }
        assert!(
            Instant::now() < deadline,
            "the disk cache promoted no whole copy within {PROMOTION_TIMEOUT:?}"
        );
        sleep(PROMOTION_POLL);
    }
}

/// Every superfile of the fixture table opens and matches its checksums, so a
/// scrub finds nothing.
///
/// The half that matters most. Verification that rejected healthy data would
/// turn a readable table into an unreadable one, and it would show up here.
#[test]
fn a_healthy_table_reports_nothing() {
    let root = TempDir::new().expect("tempdir");
    write_table(root.path());
    let user_superfiles = superfile_paths(root.path()).len();

    let db = connect(root.path().to_str().expect("utf-8 path")).expect("connect");
    let table = db.open_table(TABLE).expect("open the fixture table");
    let report = table.scrub().expect("scrub a healthy table");

    assert!(
        report.is_clean(),
        "a healthy table must report nothing, got {:?}",
        report.damaged
    );
    assert!(
        report.superfiles_verified >= user_superfiles as u64,
        "every one of the {user_superfiles} superfiles must be verified, \
         got {}",
        report.superfiles_verified
    );
    assert!(
        report.bytes_verified > 0,
        "a verified superfile has bytes behind it"
    );
}

/// One corrupted superfile is named, and it is the only one named.
///
/// Both halves are the gate. A scrub that reported its neighbours too would
/// be turned off as noise, and one that reported nothing would leave the
/// corruption in place.
#[test]
fn one_corrupt_superfile_is_the_only_one_reported() {
    let root = TempDir::new().expect("tempdir");
    write_table(root.path());
    let superfiles = superfile_paths(root.path());
    assert!(
        superfiles.len() > 1,
        "the fixture needs neighbours for 'and no other' to mean anything"
    );
    // Not the first, so a scrub that stopped at the head would not pass by
    // accident.
    let victim = superfiles[1].clone();
    flip_a_checksummed_byte(&victim);

    let db = connect(root.path().to_str().expect("utf-8 path")).expect("connect");
    let table = db.open_table(TABLE).expect("open the corrupted table");
    let report = table
        .scrub()
        .expect("a corrupt superfile is a finding, not a failed scrub");

    assert_eq!(
        report.damaged.len(),
        1,
        "exactly the corrupted superfile, got {:?}",
        report.damaged
    );
    let found = &report.damaged[0];
    let name = victim
        .file_name()
        .and_then(|name| name.to_str())
        .expect("utf-8 filename");
    assert!(
        found.storage_path.ends_with(name),
        "the finding must name the superfile that was corrupted: {found:?}"
    );
    assert_eq!(
        found.copy,
        CheckedCopy::ObjectStore,
        "with no cache configured there is only one copy to read"
    );
    assert!(
        !found.cause.is_empty(),
        "an operator acting on the finding needs to be told what went wrong"
    );
    assert!(
        report.superfiles_verified >= (superfiles.len() - 1) as u64,
        "the untouched superfiles still verify"
    );
}

/// A cached copy corrupted after it landed is caught, while the copy in
/// object storage behind it is reported clean.
///
/// This is the window the default policy leaves open: it checks a superfile's
/// bytes as they land on the cache disk and not again on each open, so a bit
/// that flips on that disk afterwards goes unseen until something looks. This
/// is that something.
#[test]
fn a_corrupt_cached_copy_is_reported_while_storage_stays_clean() {
    let root = TempDir::new().expect("tempdir");
    let cache = TempDir::new().expect("cache tempdir");
    write_table(root.path());

    let options = ConnectOptions::new()
        .with_cache_dir(cache.path())
        .with_cold_fetch_mode(ColdFetchMode::HybridWithPrefetch);
    let db = connect_with(root.path().to_str().expect("utf-8 path"), options).expect("connect");
    let table: Supertable = db.open_table(TABLE).expect("open the fixture table");
    table
        .bm25_search(
            "body",
            "common",
            N_DOCS as usize,
            Bm25SearchOptions::new(),
            None,
        )
        .expect("a search fills the cache");

    let victim = await_a_promoted_copy(cache.path())
        .first()
        .cloned()
        .expect("the wait returns a non-empty list");
    flip_a_checksummed_byte(&victim);

    let report = table.scrub().expect("scrub the cached copies");
    assert_eq!(
        report.damaged.len(),
        1,
        "exactly the corrupted cached copy, got {:?}",
        report.damaged
    );
    assert_eq!(
        report.damaged[0].copy,
        CheckedCopy::LocalCache,
        "the damage is on the cache disk, and the report has to say so"
    );
    assert!(
        report.from_local_cache > 0,
        "the cached copies are the ones a scrub prefers to read"
    );

    // And the authoritative copy is untouched: a second table reading only
    // storage finds nothing, which is what makes deleting the cache file the
    // whole of the repair.
    let fresh = connect(root.path().to_str().expect("utf-8 path")).expect("connect");
    let from_storage = fresh.open_table(TABLE).expect("open against storage");
    assert!(
        from_storage
            .scrub()
            .expect("scrub against storage")
            .is_clean(),
        "only the cached copy was corrupted"
    );
}

/// A table with no durable storage has no copies to read, and says so rather
/// than reporting a table it did not check as clean.
#[test]
fn a_table_without_storage_refuses_to_scrub() {
    let db = connect("memory://").expect("connect to an in-memory catalog");
    let table = crate::reindex_fixture::create_fixture_table(&db);
    assert!(
        matches!(table.scrub(), Err(ScrubError::NoStorage)),
        "a scrub with nothing to read is an error, not a clean report"
    );
}
