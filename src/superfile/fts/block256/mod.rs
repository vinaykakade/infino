// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! 256-doc posting block codec.
//!
//! A self-contained block codec that doubles the block size to 256 docs —
//! halving the number of decode calls and skip-table entries per posting list
//! and widening each decode. The doc-id deltas and term frequencies of a block
//! are fixed-width bit-packed by the in-tree [`bitpack`] codec, whose 256-wide
//! kernels decode with a SIMD unpack plus a SIMD delta-integrate for the sorted
//! doc ids (NEON on `aarch64`), with a scalar fallback elsewhere.
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

use super::positions::{push_varint, read_varint};

// The hand-written fixed-width bit-packing codec backing the 256-doc PACKED path.
mod bitpack;

/// Docs per block — double the 128-doc
/// [`posting::BLOCK_LEN`](crate::superfile::fts::posting::BLOCK_LEN).
pub const BLOCK_LEN: usize = bitpack::BLOCK;

/// Minimum bit width to represent `v` (`num_bits(0) == 0`).
#[inline]
fn num_bits(v: u32) -> u8 {
    (u32::BITS - v.leading_zeros()) as u8
}

/// Bytes a bit-packed region of `bits`-wide values occupies over a full block:
/// `bits * BLOCK_LEN / 8`.
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

/// Doc ids stored as fixed-width bit-packed deltas.
pub const ENCODING_PACKED: u8 = 0;
/// Doc ids stored as a presence bitset over `[base_doc_id, last_doc_id]`, with
/// `base_doc_id` aligned down to a 64-bit word. Chosen only when it does not
/// grow the block (dense blocks).
pub const ENCODING_BITSET: u8 = 1;
/// Doc ids stored as LEB128 varint deltas — no padding. Used only for a
/// **partial** block (`count < BLOCK_LEN`, i.e. a term's short tail block) when
/// it is smaller than the padded-`PACKED` and `BITSET` alternatives. A full
/// block never uses it, so the hot decode path stays on the SIMD kernels; the
/// varint tail avoids paying for ~`BLOCK_LEN - count` padded deltas on the many
/// rare terms whose whole posting list is one short block.
pub const ENCODING_VINT: u8 = 2;

/// LEB128 byte length of `v` (`varint_len(0) == 1`).
#[inline]
fn varint_len(v: u32) -> usize {
    let bits = (u32::BITS - v.leading_zeros()).max(1);
    (bits as usize).div_ceil(7)
}

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

    // Doc-id deltas in a full BLOCK_LEN buffer. `base == doc_ids[0]`, so
    // `delta[0] == 0`; `delta[i] == doc_ids[i] - doc_ids[i-1]`. Padding slots
    // (`count..`) stay 0, so they cost no bits.
    let mut deltas = [0u32; BLOCK_LEN];
    let mut prev = base;
    let mut max_delta = 0u32;
    for (slot, &d) in deltas.iter_mut().zip(b.doc_ids.iter()) {
        let delta = d - prev;
        *slot = delta;
        max_delta = max_delta.max(delta);
        prev = d;
    }
    let delta_bits = num_bits(max_delta);

    // Term frequencies in a full BLOCK_LEN buffer, padded with 0.
    let mut tfs = [0u32; BLOCK_LEN];
    tfs[..count].copy_from_slice(&b.tfs);
    let tf_bits = num_bits(max_tf);

    let deltas_size = packed_bytes(delta_bits);
    let tfs_size = packed_bytes(tf_bits);

    // Doc-id encoding: pick the smallest of three options.
    // - PACKED (fixed-width bit-packed, SIMD-decoded): always available; a
    //   partial block is padded to BLOCK_LEN.
    // - BITSET (presence bits, word-aligned origin): always available; wins on
    //   dense near-consecutive docs and lets a union OR it in without decoding.
    // - VINT (LEB128 deltas, no padding): partial blocks only; wins on the short
    //   tail block of a rare, widely-spread term where padded PACKED is mostly
    //   waste. A full block never uses it, so the hot path stays on the kernels.
    let aligned_base = bitset_block_base(base);
    let bitset_words = ((last_doc_id - aligned_base) as usize) / 64 + 1;
    let bitset_size = bitset_words * 8;

    let vint_size = if count < BLOCK_LEN {
        let mut prev = base;
        let mut s = 0usize;
        for &d in &b.doc_ids {
            s += varint_len(d - prev);
            prev = d;
        }
        Some(s)
    } else {
        None
    };

    // PACKED is the default; BITSET wins on a tie (cheaper decode); VINT only
    // when strictly smaller than both.
    let mut encoding = ENCODING_PACKED;
    let mut doc_ids_size = deltas_size;
    if bitset_size <= doc_ids_size {
        encoding = ENCODING_BITSET;
        doc_ids_size = bitset_size;
    }
    if let Some(vs) = vint_size
        && vs < doc_ids_size
    {
        encoding = ENCODING_VINT;
        doc_ids_size = vs;
    }

    let mut bytes = Vec::with_capacity(HEADER_SIZE + doc_ids_size + tfs_size);

    // delta_bits is meaningful only for PACKED; 0 otherwise.
    bytes.push(if encoding == ENCODING_PACKED {
        delta_bits
    } else {
        0
    });
    bytes.push(tf_bits);
    bytes.push(encoding);
    bytes.push((count - 1) as u8);
    let stored_base = if encoding == ENCODING_BITSET {
        aligned_base
    } else {
        base
    };
    bytes.extend_from_slice(&stored_base.to_le_bytes());

    match encoding {
        ENCODING_BITSET => {
            let mut words = vec![0u64; bitset_words];
            for &d in &b.doc_ids {
                let bit = (d - aligned_base) as usize;
                words[bit / 64] |= 1u64 << (bit % 64);
            }
            for w in words {
                bytes.extend_from_slice(&w.to_le_bytes());
            }
        }
        ENCODING_VINT => {
            let start = bytes.len();
            let mut prev = base;
            for &d in &b.doc_ids {
                push_varint(&mut bytes, d - prev);
                prev = d;
            }
            debug_assert_eq!(bytes.len() - start, doc_ids_size, "vint doc-id size");
        }
        _ => {
            // PACKED: bit-pack the doc-id deltas.
            let before = bytes.len();
            bitpack::pack(&deltas, delta_bits, &mut bytes);
            debug_assert_eq!(bytes.len() - before, deltas_size, "packed delta size");
        }
    }

    // tfs bit-packed, trailing the doc-id region.
    let before = bytes.len();
    bitpack::pack(&tfs, tf_bits, &mut bytes);
    debug_assert_eq!(bytes.len() - before, tfs_size, "packed tf size");

    EncodedBlock {
        bytes,
        last_doc_id,
        max_tf,
    }
}

/// Decode a block's doc ids into `dest` (must be `>= BLOCK_LEN`), skipping the tf
/// half. Returns the real doc count. PACKED blocks fill all `BLOCK_LEN` slots
/// (padding repeats the last doc id); BITSET and VINT blocks fill the first
/// `count`.
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

    if encoding == ENCODING_VINT {
        // Scalar LEB128 tail (a short partial block): read exactly `count`
        // deltas and prefix-sum them onto `base`. delta[0] == 0 ⇒ dest[0] == base.
        let mut at = HEADER_SIZE;
        let mut acc = base;
        for slot in dest.iter_mut().take(count) {
            let delta = read_varint(bytes, &mut at).expect("block256: truncated vint doc-id run");
            acc = acc.wrapping_add(delta);
            *slot = acc;
        }
        return count;
    }

    // PACKED: unpack the doc-id deltas straight into `dest`, then integrate them
    // in place (prefix-sum onto `base`). Padded deltas are 0, so padded doc ids
    // repeat the last real value.
    let deltas_size = packed_bytes(delta_bits);
    let doc_dest: &mut [u32; BLOCK_LEN] = (&mut dest[..BLOCK_LEN])
        .try_into()
        .expect("decode: BLOCK_LEN doc-id slice");
    bitpack::unpack(
        &bytes[HEADER_SIZE..HEADER_SIZE + deltas_size],
        delta_bits,
        doc_dest,
    );
    bitpack::integrate(doc_dest, base);
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
    let tfs_dest: &mut [u32; BLOCK_LEN] = (&mut dest[..BLOCK_LEN])
        .try_into()
        .expect("decode_tfs: BLOCK_LEN slice");
    bitpack::unpack(&bytes[tfs_start..], tf_bits, tfs_dest);
}

/// Decode both doc ids and tfs. Returns the doc count.
pub fn decode_block(bytes: &[u8], dest_doc_ids: &mut [u32], dest_tfs: &mut [u32]) -> usize {
    let count = decode_block_doc_ids(bytes, dest_doc_ids);
    decode_block_tfs(bytes, dest_tfs);
    count
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

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

    #[test]
    fn sparse_partial_block_forces_vint() {
        // A rare term's whole posting list: a few docs spread across a large
        // corpus. Padded PACKED would pay ~BLOCK_LEN wide deltas; the bitset
        // spans millions of bits — vint deltas are far smaller, so VINT is
        // chosen. This is the tail case that regressed index size before.
        let doc_ids = vec![10, 5_000_000, 9_000_003, 12_400_101, 30_000_777];
        let tfs = vec![1, 2, 1, 3, 1];
        let enc = encode_block(&Block {
            doc_ids: doc_ids.clone(),
            tfs: tfs.clone(),
        });
        assert_eq!(enc.bytes[ENCODING_OFF], ENCODING_VINT);
        roundtrip(doc_ids, tfs);
    }

    #[test]
    fn full_block_never_vint() {
        // Even a sparse *full* block stays on the SIMD PACKED path — VINT is a
        // partial-block-only encoding.
        let doc_ids: Vec<u32> = (0..BLOCK_LEN as u32).map(|i| i * 100_003).collect();
        let tfs: Vec<u32> = (0..BLOCK_LEN as u32).map(|i| (i % 3) + 1).collect();
        let enc = encode_block(&Block {
            doc_ids: doc_ids.clone(),
            tfs: tfs.clone(),
        });
        assert_ne!(enc.bytes[ENCODING_OFF], ENCODING_VINT);
        roundtrip(doc_ids, tfs);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]

        /// Round-trip any block: random counts and gap magnitudes drive the
        /// encoder across all three doc-id encodings (PACKED/BITSET/VINT) and
        /// every bit width, and decode must reproduce the input exactly.
        #[test]
        fn roundtrip_random(
            base in 0u32..2_000_000,
            gaps in prop::collection::vec(1u32..300_000, 1..=BLOCK_LEN),
            tf_seed in prop::collection::vec(1u32..8_000, BLOCK_LEN),
        ) {
            let count = gaps.len();
            let mut doc_ids = Vec::with_capacity(count);
            let mut acc = base;
            for (i, &g) in gaps.iter().enumerate() {
                if i > 0 {
                    acc = acc.saturating_add(g);
                }
                doc_ids.push(acc);
            }
            // saturating_add can create a duplicate only at the u32 ceiling;
            // skip those rare non-strictly-ascending draws.
            prop_assume!(doc_ids.windows(2).all(|w| w[0] < w[1]));
            let tfs: Vec<u32> = tf_seed[..count].to_vec();

            let enc = encode_block(&Block {
                doc_ids: doc_ids.clone(),
                tfs: tfs.clone(),
            });
            let mut d = [0u32; BLOCK_LEN];
            let mut t = [0u32; BLOCK_LEN];
            let n = decode_block(&enc.bytes, &mut d, &mut t);
            prop_assert_eq!(n, count);
            prop_assert_eq!(&d[..count], &doc_ids[..]);
            prop_assert_eq!(&t[..count], &tfs[..]);
            prop_assert_eq!(enc.last_doc_id, doc_ids[count - 1]);
        }
    }
}
