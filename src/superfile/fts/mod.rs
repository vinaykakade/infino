// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Full-text search subsystem — the BM25 + posting list + term dictionary
//! stack lives here.

pub mod analysis;
pub mod bm25;
pub mod builder;
pub(crate) mod positions;
pub mod posting;
pub mod reader;
// `pub` in a `test-helpers` build so the clustering measurement under
// `reorder::spread` is reachable from a diagnostic without widening the
// shipped surface; `pub(crate)` otherwise. Mirrors the layer visibility
// pattern in `lib.rs`, and the `cargo-public-api` snapshot is taken without
// `test-helpers`, so this stays off the public contract.
#[cfg(feature = "test-helpers")]
pub mod reorder;
#[cfg(not(feature = "test-helpers"))]
pub(crate) mod reorder;
pub(crate) mod short;
pub(crate) mod sorted_merge;
pub mod tokenize;
