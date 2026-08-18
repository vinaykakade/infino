// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Portable 256-doc posting block codec.
//!
//! A self-contained block codec that doubles the block size to 256 docs and
//! decodes through one auto-vectorizable layout (see [`bitpack`]) — halving the
//! number of decode calls and skip-table entries per posting list and widening
//! each decode, while staying correct and fast on both `x86_64` and `aarch64`
//! with no architecture intrinsics and no `unsafe`.
//!
//! This lives beside the current 128-doc codec in
//! [`posting`](crate::superfile::fts::posting) rather than replacing it in
//! place, so the new format and its kernels can be reviewed and measured in
//! isolation before the reader/builder are switched over to it.
//!
//! Landing in stages: the bit-pack core ([`bitpack`]) and its round-trip
//! proptest come first; the block encode/decode (delta doc-ids + prefix-sum,
//! tfs, and the dense presence-bitset path) and the reader/builder integration
//! follow. Until then the module is unused by the query path.
#![allow(dead_code)] // prototype: codec precedes its reader/builder integration

pub(crate) mod bitpack;

/// Docs per block — double the 128-doc [`posting::BLOCK_LEN`](crate::superfile::fts::posting::BLOCK_LEN).
pub const BLOCK_LEN: usize = bitpack::BLOCK_LEN;
