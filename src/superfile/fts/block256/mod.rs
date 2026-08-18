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
//! ## Block byte layout
//!
//! ```text
//!   offset  bytes   field
//!   ─────────────────────────────────────────────────────────────────
//!   0       1       delta_bits   (u8; 0..=32 — 0 on a BITSET block)
//!   1       1       tf_bits      (u8; 0..=32)
//!   2       1       encoding     (u8; PACKED=0, BITSET=1)
//!   3       1       count - 1    (u8; doc_count is 1..=256, stored biased so
//!                                 256 fits a byte)
//!   4       4       base_doc_id  (LE u32; PACKED: doc_ids[0]. BITSET: doc_ids[0]
//!                                 aligned down to a 64-bit word)
//!   8       …       doc ids      (PACKED: 256 deltas at delta_bits, via bitpack;
//!                                 BITSET: presence bitset over [base, last])
//!   …       …       tfs          (always 256 tfs at tf_bits, via bitpack)
//! ```
//!
//! The tf half is always the trailing `bitpack::packed_len_u32(tf_bits) * 4`
//! bytes, regardless of the doc-id encoding — so a reader that only needs doc
//! ids ([`decode_block_doc_ids`]) never touches it, and the tf decode can be
//! deferred.
#![allow(dead_code)] // prototype: codec precedes its reader/builder integration

pub(crate) mod bitpack;

use bitpack::{BLOCK_LEN as PACK_BLOCK_LEN, pack, packed_len_u32, unpack};

/// Docs per block — double the 128-doc [`posting::BLOCK_LEN`](crate::superfile::fts::posting::BLOCK_LEN).
pub const BLOCK_LEN: usize = PACK_BLOCK_LEN;

/// Fixed block header size in bytes.
pub const HEADER_SIZE: usize = 8;

const DELTA_BITS_OFF: usize = 0;
const TF_BITS_OFF: usize = 1;
const ENCODING_OFF: usize = 2;
const COUNT_M1_OFF: usize = 3;
const BASE_OFF: usize = 4;

/// Doc ids stored as PFOR deltas (bit-packed via [`bitpack`]).
pub const ENCODING_PACKED: u8 = 0;
/// Doc ids stored as a presence bitset over `[base_doc_id, last_doc_id]`, with
/// `base_doc_id` aligned down to a 64-bit word. Chosen only when it does not
/// grow the block (dense blocks).
pub const ENCODING_BITSET: u8 = 1;

/// Align a doc id down to the 64-bit word that contains it — the origin of a
/// bitset block's presence bitmap.
#[inline]
pub fn bitset_block_base(doc_id: u32) -> u32 {
    doc_id & !63
}

/// Minimum bit width to represent `v` (`bits_for(0) == 0`).
#[inline]
fn bits_for(v: u32) -> u32 {
    32 - v.leading_zeros()
}

/// One block of postings — sorted-ascending `doc_ids` plus per-doc `tfs`, both
/// `1..=BLOCK_LEN` long and equal length.
pub struct Block {
    pub doc_ids: Vec<u32>,
    pub tfs: Vec<u32>,
}

/// Encoded block bytes plus the `last_doc_id` / `max_tf` skip-table/BMW fields
/// lifted out so callers need not re-decode to read them.
pub struct EncodedBlock {
    pub bytes: Vec<u8>,
    pub last_doc_id: u32,
    pub max_tf: u32,
}

/// Read the `w`-th packed `u32` word for `lane` out of `body` (LE), where the
/// packed words are stored `packed[w * LANES + lane]`.
#[inline]
fn read_word(body: &[u8], idx: usize) -> u32 {
    let o = idx * 4;
    u32::from_le_bytes([body[o], body[o + 1], body[o + 2], body[o + 3]])
}

/// Copy `n` LE `u32` words starting at `body[0]` into `dst[..n]`.
#[inline]
fn read_words(body: &[u8], n: usize, dst: &mut [u32]) {
    for (i, slot) in dst.iter_mut().enumerate().take(n) {
        *slot = read_word(body, i);
    }
}

/// Encode one block.
///
/// # Panics
/// - empty block, `doc_ids.len() != tfs.len()`, or `len > BLOCK_LEN`.
/// - `doc_ids` not strictly ascending (debug-only).
pub fn encode_block(b: &Block) -> EncodedBlock {
    let count = b.doc_ids.len();
    assert!(count > 0, "encode_block: empty block");
    assert_eq!(
        count,
        b.tfs.len(),
        "encode_block: doc_ids/tfs length mismatch"
    );
    assert!(
        count <= BLOCK_LEN,
        "encode_block: doc_count {count} > BLOCK_LEN {BLOCK_LEN}"
    );
    debug_assert!(
        b.doc_ids.windows(2).all(|w| w[0] < w[1]),
        "encode_block: doc_ids must be strictly ascending"
    );

    let base = b.doc_ids[0];
    let last_doc_id = b.doc_ids[count - 1];
    let max_tf = b.tfs.iter().copied().max().unwrap_or(0);

    // Deltas in doc order, padded to BLOCK_LEN with 0 (so padded doc ids repeat
    // the last real value). delta[0] = 0 since doc_ids[0] is the stored base.
    let mut deltas = [0u32; BLOCK_LEN];
    let mut max_delta = 0u32;
    for (slot, w) in deltas[1..count].iter_mut().zip(b.doc_ids.windows(2)) {
        let d = w[1] - w[0];
        *slot = d;
        max_delta = max_delta.max(d);
    }
    let delta_bits = bits_for(max_delta);

    let mut tfs = [0u32; BLOCK_LEN];
    tfs[..count].copy_from_slice(&b.tfs);
    let tf_bits = bits_for(max_tf);

    let deltas_size = packed_len_u32(delta_bits) * 4;
    let tfs_size = packed_len_u32(tf_bits) * 4;

    // Prefer a presence bitset when it is no larger than the packed deltas
    // (dense blocks — a common term's near-consecutive docs). The bitset origin
    // is word-aligned so a union count can OR it in without a per-word shift.
    let aligned_base = bitset_block_base(base);
    let bitset_words = ((last_doc_id - aligned_base) as usize) / 64 + 1;
    let bitset_size = bitset_words * 8;
    let use_bitset = bitset_size <= deltas_size;

    let doc_ids_size = if use_bitset { bitset_size } else { deltas_size };
    let mut bytes = Vec::with_capacity(HEADER_SIZE + doc_ids_size + tfs_size);

    bytes.push(if use_bitset { 0 } else { delta_bits as u8 });
    bytes.push(tf_bits as u8);
    bytes.push(if use_bitset {
        ENCODING_BITSET
    } else {
        ENCODING_PACKED
    });
    bytes.push((count - 1) as u8);
    bytes.extend_from_slice(&if use_bitset { aligned_base } else { base }.to_le_bytes());

    if use_bitset {
        let mut words = vec![0u64; bitset_words];
        for &d in &b.doc_ids {
            let bit = (d - aligned_base) as usize;
            words[bit / 64] |= 1u64 << (bit % 64);
        }
        for w in words {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
    } else {
        let mut packed = [0u32; BLOCK_LEN]; // max packed_len_u32(32) = 256
        let n = packed_len_u32(delta_bits);
        pack(delta_bits, &deltas, &mut packed[..n]);
        for &w in &packed[..n] {
            bytes.extend_from_slice(&w.to_le_bytes());
        }
    }

    let mut packed_tfs = [0u32; BLOCK_LEN];
    let ntf = packed_len_u32(tf_bits);
    pack(tf_bits, &tfs, &mut packed_tfs[..ntf]);
    for &w in &packed_tfs[..ntf] {
        bytes.extend_from_slice(&w.to_le_bytes());
    }

    EncodedBlock {
        bytes,
        last_doc_id,
        max_tf,
    }
}

/// Decode a block's doc ids into `dest` (must be `>= BLOCK_LEN`), skipping the tf
/// half. Returns the real doc count. PACKED blocks fill all `BLOCK_LEN` slots
/// (padding repeats the last doc id); BITSET blocks fill the first `count`.
///
/// # Panics
/// - `dest.len() < BLOCK_LEN`, or `bytes` shorter than the header/body claims.
pub fn decode_block_doc_ids(bytes: &[u8], dest: &mut [u32]) -> usize {
    assert!(dest.len() >= BLOCK_LEN, "decode: dest < BLOCK_LEN");
    assert!(bytes.len() >= HEADER_SIZE, "decode: bytes < header");
    let delta_bits = u32::from(bytes[DELTA_BITS_OFF]);
    let tf_bits = u32::from(bytes[TF_BITS_OFF]);
    let encoding = bytes[ENCODING_OFF];
    let count = bytes[COUNT_M1_OFF] as usize + 1;
    let base = u32::from_le_bytes([bytes[BASE_OFF], bytes[5], bytes[6], bytes[7]]);
    let tfs_size = packed_len_u32(tf_bits) * 4;

    if encoding == ENCODING_BITSET {
        let words = &bytes[HEADER_SIZE..bytes.len() - tfs_size];
        let mut j = 0usize;
        for (wi, chunk) in words.chunks_exact(8).enumerate() {
            let mut word = u64::from_le_bytes(chunk.try_into().expect("8 bytes"));
            while word != 0 {
                dest[j] = base + (wi as u32 * 64 + word.trailing_zeros());
                j += 1;
                word &= word - 1;
            }
        }
        debug_assert_eq!(j, count, "bitset set-bit count must equal doc_count");
        return count;
    }

    let n = packed_len_u32(delta_bits);
    let mut packed = [0u32; BLOCK_LEN];
    read_words(&bytes[HEADER_SIZE..], n, &mut packed);
    let mut deltas = [0u32; BLOCK_LEN];
    unpack(delta_bits, &packed[..n.max(1)], &mut deltas);
    // Prefix-sum: doc[0] = base (delta[0] == 0); doc[i] = doc[i-1] + delta[i].
    let mut acc = base;
    for (i, d) in deltas.iter().enumerate() {
        acc = acc.wrapping_add(*d);
        dest[i] = acc;
    }
    count
}

/// Decode a block's term frequencies into `dest` (must be `>= BLOCK_LEN`). The
/// tf half is the trailing packed region regardless of the doc-id encoding.
///
/// # Panics
/// - `dest.len() < BLOCK_LEN`, or `bytes` shorter than header + tfs.
pub fn decode_block_tfs(bytes: &[u8], dest: &mut [u32]) {
    assert!(dest.len() >= BLOCK_LEN, "decode_tfs: dest < BLOCK_LEN");
    let tf_bits = u32::from(bytes[TF_BITS_OFF]);
    let tfs_words = packed_len_u32(tf_bits);
    let tfs_size = tfs_words * 4;
    assert!(
        bytes.len() >= HEADER_SIZE + tfs_size,
        "decode_tfs: bytes short"
    );
    let tfs_start = bytes.len() - tfs_size;
    let mut packed = [0u32; BLOCK_LEN];
    read_words(&bytes[tfs_start..], tfs_words, &mut packed);
    unpack(tf_bits, &packed[..tfs_words.max(1)], dest);
}

/// Decode both doc ids and tfs. Returns the doc count.
pub fn decode_block(bytes: &[u8], dest_doc_ids: &mut [u32], dest_tfs: &mut [u32]) -> usize {
    let count = decode_block_doc_ids(bytes, dest_doc_ids);
    decode_block_tfs(bytes, dest_tfs);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(doc_ids: Vec<u32>, tfs: Vec<u32>) {
        let count = doc_ids.len();
        let enc = encode_block(&Block {
            doc_ids: doc_ids.clone(),
            tfs: tfs.clone(),
        });
        assert_eq!(enc.last_doc_id, doc_ids[count - 1]);
        assert_eq!(
            enc.max_tf,
            tfs.iter().copied().max().expect("non-empty tfs")
        );

        let mut d = [0u32; BLOCK_LEN];
        let mut t = [0u32; BLOCK_LEN];
        let n = decode_block(&enc.bytes, &mut d, &mut t);
        assert_eq!(n, count, "count");
        assert_eq!(&d[..count], &doc_ids[..], "doc_ids");
        assert_eq!(&t[..count], &tfs[..], "tfs");

        // doc-ids-only path agrees.
        let mut d2 = [0u32; BLOCK_LEN];
        assert_eq!(decode_block_doc_ids(&enc.bytes, &mut d2), count);
        assert_eq!(&d2[..count], &doc_ids[..]);
    }

    #[test]
    fn single_doc_block() {
        roundtrip(vec![42], vec![7]);
    }

    #[test]
    fn sparse_full_block_forces_packed() {
        // Wide gaps ⇒ deltas large ⇒ packed cheaper than a bitset.
        let doc_ids: Vec<u32> = (0..BLOCK_LEN as u32).map(|i| i * 10_000).collect();
        let tfs: Vec<u32> = (0..BLOCK_LEN as u32).map(|i| (i % 13) + 1).collect();
        let enc = encode_block(&Block {
            doc_ids: doc_ids.clone(),
            tfs: tfs.clone(),
        });
        assert_eq!(enc.bytes[ENCODING_OFF], ENCODING_PACKED);
        roundtrip(doc_ids, tfs);
    }

    #[test]
    fn dense_block_forces_bitset() {
        // Dense docs over a compact, word-aligned range: the presence bitset
        // (~one bit per doc in range) is smaller than PFOR deltas here, so it is
        // chosen. (Perfectly-consecutive full blocks instead pack to 1 bit/delta,
        // which can beat the bitset — that case correctly stays PACKED.)
        let doc_ids: Vec<u32> = (0..128u32).collect();
        let tfs: Vec<u32> = (0..128u32).map(|i| (i % 5) + 1).collect();
        let enc = encode_block(&Block {
            doc_ids: doc_ids.clone(),
            tfs: tfs.clone(),
        });
        assert_eq!(enc.bytes[ENCODING_OFF], ENCODING_BITSET);
        roundtrip(doc_ids, tfs);
    }

    #[test]
    fn partial_block_various_counts() {
        for count in [1usize, 2, 63, 64, 65, 127, 128, 129, 200, 255, 256] {
            let doc_ids: Vec<u32> = (0..count as u32).map(|i| 5 + i * 3).collect();
            let tfs: Vec<u32> = (0..count as u32).map(|i| (i % 7) + 1).collect();
            roundtrip(doc_ids, tfs);
        }
    }

    #[test]
    fn base_zero_and_high_tf() {
        roundtrip(vec![0, 1, 5, 9], vec![1, 1000, 3, 1]);
    }
}
