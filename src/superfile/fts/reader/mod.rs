// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! FTS read path. The `FtsReader` type and its query kernels are split
//! across the submodules below; this file only wires them together and
//! re-exports the surface callers reach as `fts::reader::*`.

mod core;
mod count;
mod cursor;
mod filter;
mod metadata;
mod options;
mod phrase;
mod scorers;
mod search;
mod sink;
#[cfg(test)]
mod test_util;
mod work;

pub use core::*;
// Non-essential block-decode amortization counters (measurement branch only; not for merge).
pub use cursor::noness_amort_dump;

pub use metadata::OpenOptions;
pub use options::{Bm25SearchOptions, Bm25Stats, BoolMode};
pub use work::MatchWork;
