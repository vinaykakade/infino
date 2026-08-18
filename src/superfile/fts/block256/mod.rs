// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! 256-doc posting block codec.
//!
//! A self-contained block codec that doubles the block size to 256 docs —
//! halving the number of decode calls and skip-table entries per posting list
//! and widening each decode. The doc-id deltas and term frequencies of a block
//! are bit-packed with [`BitPacker8x`], whose 256-wide kernels decode with a
//! single SIMD unpack (plus a SIMD delta-integrate for the sorted doc ids):
//! AVX2 on `x86_64`, NEON on `aarch64`, and a scalar fallback elsewhere.
//!
//! This lives beside the 128-doc codec in
//! [`posting`](crate::superfile::fts::posting) rather than replacing it: the
//! block width is a per-superfile format property, so a reader dispatches to
//! this codec or the 128-doc one by version and a single binary reads both.
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
//!   8       …       doc ids      (PACKED: 256 deltas at delta_bits, bit-packed;
//!                                 BITSET: presence bitset over [base, last])
//!   …       …       tfs          (always 256 tfs at tf_bits, bit-packed)
//! ```
//!
//! The tf half is always the trailing `tf_bits * BLOCK_LEN / 8` bytes,
//! regardless of the doc-id encoding — so a reader that only needs doc ids
//! ([`decode_block_doc_ids`]) never touches it, and the tf decode can be
//! deferred.
#![allow(dead_code)] // prototype: codec precedes its reader/builder integration

use bitpacking::{BitPacker, BitPacker8x};

/// Docs per block — double the 128-doc
/// [`posting::BLOCK_LEN`](crate::superfile::fts::posting::BLOCK_LEN).
pub const BLOCK_LEN: usize = BitPacker8x::BLOCK_LEN;

/// Bytes a bit-packed region of `bits`-wide values occupies over a full block:
/// `bits * BLOCK_LEN / 8`. Equals [`BitPacker8x`]'s compressed block size.
#[inline]
fn packed_bytes(bits: u8) -> usize {
    bits as usize * BLOCK_LEN / 8
}

/// Fixed block header size in bytes.
pub const HEADER_SIZE: usize = 8;

/// Header byte offset of `delta_bits`.
pub const DELTA_BITS_OFF: usize = 0;
/// Header byte offset of `tf_bits`.
pub const TF_BITS_OFF: usize = 1;
/// Header byte offset of the `encoding` discriminant.
pub const ENCODING_OFF: usize = 2;
/// Header byte offset of `count - 1`.
pub const COUNT_M1_OFF: usize = 3;
/// Header byte offset of the base doc id (LE u32, 4 bytes).
pub const BASE_OFF: usize = 4;

/// Doc ids stored as PFOR deltas (bit-packed via [`BitPacker8x`]).
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

    let bp = BitPacker8x::new();

    // Doc ids in a full BLOCK_LEN buffer, padding by repeating the last real
    // value so the padded deltas are 0 and cost no extra bits. base ==
    // doc_ids[0] is the sorted "initial", so delta[0] == 0.
    let mut docs = [last_doc_id; BLOCK_LEN];
    docs[..count].copy_from_slice(&b.doc_ids);
    let delta_bits = bp.num_bits_sorted(base, &docs);

    // Term frequencies in a full BLOCK_LEN buffer, padded with 0.
    let mut tfs = [0u32; BLOCK_LEN];
    tfs[..count].copy_from_slice(&b.tfs);
    let tf_bits = bp.num_bits(&tfs);

    let deltas_size = packed_bytes(delta_bits);
    let tfs_size = packed_bytes(tf_bits);

    // Prefer a presence bitset when it is no larger than the packed deltas
    // (dense blocks — a common term's near-consecutive docs). The bitset origin
    // is word-aligned so a union count can OR it in without a per-word shift.
    let aligned_base = bitset_block_base(base);
    let bitset_words = ((last_doc_id - aligned_base) as usize) / 64 + 1;
    let bitset_size = bitset_words * 8;
    let use_bitset = bitset_size <= deltas_size;

    let doc_ids_size = if use_bitset { bitset_size } else { deltas_size };
    let mut bytes = Vec::with_capacity(HEADER_SIZE + doc_ids_size + tfs_size);

    bytes.push(if use_bitset { 0 } else { delta_bits });
    bytes.push(tf_bits);
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
        // Max compressed size is 32 bits * BLOCK_LEN / 8 == BLOCK_LEN * 4 bytes.
        let mut packed = [0u8; BLOCK_LEN * 4];
        let n = bp.compress_sorted(base, &docs, &mut packed, delta_bits);
        debug_assert_eq!(n, deltas_size, "compressed delta size");
        bytes.extend_from_slice(&packed[..n]);
    }

    let mut packed_tfs = [0u8; BLOCK_LEN * 4];
    let ntf = bp.compress(&tfs, &mut packed_tfs, tf_bits);
    debug_assert_eq!(ntf, tfs_size, "compressed tf size");
    bytes.extend_from_slice(&packed_tfs[..ntf]);

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
    let delta_bits = bytes[DELTA_BITS_OFF];
    let tf_bits = bytes[TF_BITS_OFF];
    let encoding = bytes[ENCODING_OFF];
    let count = bytes[COUNT_M1_OFF] as usize + 1;
    let base = u32::from_le_bytes([bytes[BASE_OFF], bytes[5], bytes[6], bytes[7]]);
    let tfs_size = packed_bytes(tf_bits);

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

    // BitPacker8x decompress_sorted does the SIMD unpack *and* the SIMD
    // delta-integrate, so `dest` holds doc ids directly (no scalar prefix-sum).
    let deltas_size = packed_bytes(delta_bits);
    let bp = BitPacker8x::new();
    bp.decompress_sorted(
        base,
        &bytes[HEADER_SIZE..HEADER_SIZE + deltas_size],
        &mut dest[..BLOCK_LEN],
        delta_bits,
    );
    count
}

/// Decode a block's term frequencies into `dest` (must be `>= BLOCK_LEN`). The
/// tf half is the trailing packed region regardless of the doc-id encoding.
///
/// # Panics
/// - `dest.len() < BLOCK_LEN`, or `bytes` shorter than header + tfs.
pub fn decode_block_tfs(bytes: &[u8], dest: &mut [u32]) {
    assert!(dest.len() >= BLOCK_LEN, "decode_tfs: dest < BLOCK_LEN");
    let tf_bits = bytes[TF_BITS_OFF];
    let tfs_size = packed_bytes(tf_bits);
    assert!(
        bytes.len() >= HEADER_SIZE + tfs_size,
        "decode_tfs: bytes short"
    );
    let tfs_start = bytes.len() - tfs_size;
    let bp = BitPacker8x::new();
    bp.decompress(&bytes[tfs_start..], &mut dest[..BLOCK_LEN], tf_bits);
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
