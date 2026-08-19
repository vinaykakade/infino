// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Fixed-width bit-packing for 256-value blocks with primitive-size lanes.
//!
//! Backs the 256-doc [`block256`](super) PACKED path in place of the
//! `BitPacker8x`-based packing. The layout targets **decode op-count**: values
//! are packed at the smallest *primitive* that fits the bit width — 8-bit lanes
//! for width ≤ 8, 16-bit for ≤ 16, 32-bit above — so a narrow column (tf is
//! usually 1–3 bits, dense doc-deltas 1–8) unpacks with a quarter / half the
//! shift-mask rounds. Four values share one 32-bit word as byte lanes
//! (`collapse8` / `expand8`), two as half lanes (`collapse16` / `expand16`); a
//! lane-replicated mask keeps the lanes from bleeding into each other.
//!
//! Currently scalar (correctness-first, no SIMD). [`pack`] and [`unpack`] are
//! exact inverses. The decode is deliberately shaped as a shift-level extraction
//! + straddle stitch + `expand`: the shift-level loop is a contiguous shift-mask
//! over the packed words — the shape a later SIMD pass vectorizes. The on-disk
//! bytes are their own layout (not `BitPacker8x`-compatible), so this is a new
//! 256-block encoding.

#![allow(dead_code)] // codec + proptest land before it is wired into block256.

/// Values per block — one 256-doc posting block's worth.
pub(super) const BLOCK: usize = 256;

/// `b`-bit low mask replicated into every 32-bit lane (`b` in `1..=32`).
#[inline]
fn mask32(b: usize) -> u32 {
    if b >= 32 { u32::MAX } else { (1u32 << b) - 1 }
}

/// `b`-bit low mask replicated into each of the two 16-bit lanes (`b` in `1..=16`).
#[inline]
fn mask16(b: usize) -> u32 {
    let m = (1u32 << b) - 1;
    m | (m << 16)
}

/// `b`-bit low mask replicated into each of the four 8-bit lanes (`b` in `1..=8`).
#[inline]
fn mask8(b: usize) -> u32 {
    let m = (1u32 << b) - 1;
    let m = m | (m << 8);
    m | (m << 16)
}

/// Lane-replicated `b`-bit mask for primitive `p` (8, 16, or 32).
#[inline]
fn lane_mask(p: usize, b: usize) -> u32 {
    match p {
        8 => mask8(b),
        16 => mask16(b),
        _ => mask32(b),
    }
}

/// Pack four quarter-stride values into one 32-bit word's four byte lanes.
#[inline]
fn collapse8(a: &mut [u32; BLOCK]) {
    for i in 0..64 {
        a[i] = (a[i] << 24) | (a[64 + i] << 16) | (a[128 + i] << 8) | a[192 + i];
    }
}

/// Inverse of [`collapse8`]: spread each word's four byte lanes back to 256 slots.
#[inline]
fn expand8(a: &mut [u32; BLOCK]) {
    for i in 0..64 {
        let l = a[i];
        a[i] = (l >> 24) & 0xFF;
        a[64 + i] = (l >> 16) & 0xFF;
        a[128 + i] = (l >> 8) & 0xFF;
        a[192 + i] = l & 0xFF;
    }
}

/// Pack two half-stride values into one 32-bit word's two 16-bit lanes.
#[inline]
fn collapse16(a: &mut [u32; BLOCK]) {
    for i in 0..128 {
        a[i] = (a[i] << 16) | a[128 + i];
    }
}

/// Inverse of [`collapse16`].
#[inline]
fn expand16(a: &mut [u32; BLOCK]) {
    for i in 0..128 {
        let l = a[i];
        a[i] = (l >> 16) & 0xFFFF;
        a[128 + i] = l & 0xFFFF;
    }
}

/// Smallest primitive (8/16/32) that holds a `bits`-wide value.
#[inline]
fn primitive(bits: usize) -> usize {
    if bits <= 8 {
        8
    } else if bits <= 16 {
        16
    } else {
        32
    }
}

/// Bytes a `bits`-wide 256-value block occupies: `bits * 32`.
#[inline]
pub(super) fn packed_len(bits: u8) -> usize {
    bits as usize * BLOCK / 8
}

/// Append the packing of `vals` at `bits` bits to `out` (`bits * 32` bytes).
/// `bits == 0` writes nothing. Every value must fit in `bits` bits.
pub(super) fn pack(vals: &[u32; BLOCK], bits: u8, out: &mut Vec<u8>) {
    if bits == 0 {
        return;
    }
    let bits = bits as usize;
    debug_assert!(bits <= 32);
    let p = primitive(bits);

    // Collapse into the primitive's lanes (identity for 32-bit).
    let mut ints = *vals;
    match p {
        8 => collapse8(&mut ints),
        16 => collapse16(&mut ints),
        _ => {}
    }

    let num_ints = BLOCK * p / 32; // collapsed word count: 64 / 128 / 256
    let n_out = bits * 8; // output words (= bits * 32 bytes)
    let mut tmp = vec![0u32; n_out];
    let mut idx = 0usize;

    // Shift levels: each output word accumulates several collapsed words at
    // descending in-lane shifts. This is the part a SIMD pass vectorizes.
    let mut shift = p as i32 - bits as i32;
    for word in tmp.iter_mut() {
        *word = ints[idx] << shift;
        idx += 1;
    }
    shift -= bits as i32;
    while shift >= 0 {
        for word in tmp.iter_mut() {
            *word |= ints[idx] << shift;
            idx += 1;
        }
        shift -= bits as i32;
    }

    // Straddle tail: the collapsed words whose bits span two output words.
    let rem_int = (shift + bits as i32) as usize; // leftover lane bits per output word
    let mask_rem = lane_mask(p, rem_int.max(1));
    let mut tmp_idx = 0usize;
    let mut rem_val = bits;
    while idx < num_ints {
        if rem_val >= rem_int {
            rem_val -= rem_int;
            tmp[tmp_idx] |= (ints[idx] >> rem_val) & mask_rem;
            tmp_idx += 1;
            if rem_val == 0 {
                idx += 1;
                rem_val = bits;
            }
        } else {
            let mask1 = lane_mask(p, rem_val);
            let mask2 = lane_mask(p, rem_int - rem_val);
            tmp[tmp_idx] |= (ints[idx] & mask1) << (rem_int - rem_val);
            idx += 1;
            rem_val = bits - rem_int + rem_val;
            tmp[tmp_idx] |= (ints[idx] >> rem_val) & mask2;
            tmp_idx += 1;
        }
    }

    out.reserve(n_out * 4);
    for w in tmp {
        out.extend_from_slice(&w.to_le_bytes());
    }
}

/// Inverse of [`pack`]: decode `bits * 32` bytes into `dest[..256]`. `bits == 0`
/// fills zeros. `bytes` must be at least `packed_len(bits)` long.
pub(super) fn unpack(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    dest.fill(0);
    if bits == 0 {
        return;
    }
    let bits = bits as usize;
    let p = primitive(bits);
    let num_ints = BLOCK * p / 32;
    let n_out = bits * 8;
    debug_assert!(bytes.len() >= n_out * 4);

    // Load the packed words.
    let mut tmp = vec![0u32; n_out];
    for (i, w) in tmp.iter_mut().enumerate() {
        let o = i * 4;
        *w = u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]]);
    }

    let mask_full = lane_mask(p, bits);

    // Reverse the shift levels (the vectorizable extraction).
    let mut idx = 0usize;
    let mut shift = p as i32 - bits as i32;
    for &w in &tmp {
        dest[idx] = (w >> shift) & mask_full;
        idx += 1;
    }
    shift -= bits as i32;
    while shift >= 0 {
        for &w in &tmp {
            dest[idx] = (w >> shift) & mask_full;
            idx += 1;
        }
        shift -= bits as i32;
    }

    // Reverse the straddle tail (mirrors `pack`'s straddle, reading instead of
    // writing). `dest` was zeroed, so the `|=` reassembles split values.
    let rem_int = (shift + bits as i32) as usize;
    let mask_rem = lane_mask(p, rem_int.max(1));
    let mut tmp_idx = 0usize;
    let mut rem_val = bits;
    while idx < num_ints {
        if rem_val >= rem_int {
            rem_val -= rem_int;
            dest[idx] |= (tmp[tmp_idx] & mask_rem) << rem_val;
            tmp_idx += 1;
            if rem_val == 0 {
                idx += 1;
                rem_val = bits;
            }
        } else {
            let mask1 = lane_mask(p, rem_val);
            let mask2 = lane_mask(p, rem_int - rem_val);
            dest[idx] |= (tmp[tmp_idx] >> (rem_int - rem_val)) & mask1;
            idx += 1;
            rem_val = bits - rem_int + rem_val;
            dest[idx] |= (tmp[tmp_idx] & mask2) << rem_val;
            tmp_idx += 1;
        }
    }

    // Spread the primitive lanes back to 256 positions (identity for 32-bit).
    match p {
        8 => expand8(dest),
        16 => expand16(dest),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Byte length matches `bits * 32`, and every primitive boundary is exercised.
    #[test]
    fn packed_len_and_primitive_boundaries() {
        for bits in 1..=32usize {
            let vals = [((1u64 << bits) - 1) as u32; BLOCK];
            let mut out = Vec::new();
            pack(&vals, bits as u8, &mut out);
            assert_eq!(out.len(), packed_len(bits as u8), "bits={bits}");
            let mut dec = [0u32; BLOCK];
            unpack(&out, bits as u8, &mut dec);
            assert_eq!(dec, vals, "all-ones round-trip, bits={bits}");
        }
        // primitive() picks 8/16/32 at the boundaries.
        assert_eq!(primitive(8), 8);
        assert_eq!(primitive(9), 16);
        assert_eq!(primitive(16), 16);
        assert_eq!(primitive(17), 32);
    }

    #[test]
    fn zero_bits_is_empty_and_zeros() {
        let mut out = vec![0xAAu8; 3];
        pack(&[0u32; BLOCK], 0, &mut out);
        assert_eq!(out.len(), 3, "0 bits writes no bytes");
        let mut dec = [7u32; BLOCK];
        unpack(&[], 0, &mut dec);
        assert_eq!(dec, [0u32; BLOCK]);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(2048))]

        /// Round-trip any block at any width: pack then unpack reproduces the
        /// input exactly. `bits` spans 1..=32 (both primitive boundaries), and
        /// values are masked to the width the caller guarantees.
        #[test]
        fn roundtrip_all_widths(
            bits in 1u8..=32,
            raw in prop::array::uniform32(any::<u32>()),
        ) {
            // Expand 32 random seeds into 256 values, masked to `bits`.
            let mask = if bits == 32 { u32::MAX } else { (1u32 << bits) - 1 };
            let mut vals = [0u32; BLOCK];
            for (i, v) in vals.iter_mut().enumerate() {
                *v = raw[i % 32].wrapping_mul(2_654_435_761).wrapping_add(i as u32) & mask;
            }
            let mut out = Vec::new();
            pack(&vals, bits, &mut out);
            prop_assert_eq!(out.len(), packed_len(bits));
            let mut dec = [0u32; BLOCK];
            unpack(&out, bits, &mut dec);
            prop_assert_eq!(dec, vals);
        }

        /// Same, but with genuinely random per-slot values (via a shuffle of the
        /// seed array position) to avoid any structure the multiply-hash imposes.
        #[test]
        fn roundtrip_random_dense(
            bits in 1u8..=32,
            vals_seed in prop::collection::vec(any::<u32>(), BLOCK),
        ) {
            let mask = if bits == 32 { u32::MAX } else { (1u32 << bits) - 1 };
            let mut vals = [0u32; BLOCK];
            for (v, &s) in vals.iter_mut().zip(vals_seed.iter()) {
                *v = s & mask;
            }
            let mut out = Vec::new();
            pack(&vals, bits, &mut out);
            let mut dec = [0u32; BLOCK];
            unpack(&out, bits, &mut dec);
            prop_assert_eq!(dec, vals);
        }
    }
}
