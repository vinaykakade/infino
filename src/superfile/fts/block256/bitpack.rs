// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Portable, auto-vectorizable bit-packing for 256-value blocks.
//!
//! One on-disk layout that the compiler vectorizes on both targets — AVX2 on
//! `x86_64`, NEON on `aarch64` — under `-C target-cpu=native`, with a scalar
//! fallback everywhere else. There are **no architecture intrinsics, no
//! `std::simd`, and no `unsafe`**: the decode is plain integer shift/mask that
//! the optimizer lowers to wide vector ops, so a single source stays correct
//! and fast across architectures (the AVX2-only `bitpacking::BitPacker8x` would
//! run scalar — and slow — on ARM).
//!
//! ## Layout
//!
//! The 256 values are striped across `LANES = 8` parallel lanes: value `i` goes
//! to lane `i % LANES`, row `i / LANES` (so lane `l` holds
//! `values[l], values[l+8], … values[l+248]`). Each lane bit-packs its 32
//! values LSB-first at a shared `bits` width into a `32 * bits`-bit stream =
//! `bits` `u32` words. The 8 lanes' word `w` are stored **adjacently** —
//! `packed[w * LANES + lane]` — so decoding row-by-row touches all 8 lanes'
//! word `w` in one contiguous 8-`u32` span, which is what lets the row loop's
//! inner 8-lane loop vectorize.
//!
//! `bits` is the same across all 8 lanes of a block (the block's max bit width),
//! so every lane crosses `u32` word boundaries at the same rows — the branch on
//! whether row `r` spans two words is loop-invariant across lanes.

/// Parallel lanes per block. 8 matches an AVX2 register (8×`u32`); NEON runs it
/// as 2×4. Changing this changes the on-disk layout.
pub const LANES: usize = 8;

/// Values packed per lane. `LANES * ROWS == BLOCK_LEN`.
pub const ROWS: usize = 32;

/// Values per block. Double `posting::BLOCK_LEN` (128).
pub const BLOCK_LEN: usize = LANES * ROWS;

/// Number of `u32` words the packed form of one block occupies at `bits` width.
/// `bits == 0` packs to nothing (every value is zero and recovered from width).
#[inline]
pub fn packed_len_u32(bits: u32) -> usize {
    bits as usize * LANES
}

/// Low-`bits` mask (`bits` in `1..=32`).
#[inline]
fn low_mask(bits: u32) -> u64 {
    (1u64 << bits) - 1
}

/// Pack `values` at `bits` width into `out`, which must be exactly
/// [`packed_len_u32(bits)`] long. `bits == 0` is a no-op (all values are zero).
///
/// # Panics
/// - `out.len() != packed_len_u32(bits)`.
/// - `bits > 32`.
pub fn pack(bits: u32, values: &[u32; BLOCK_LEN], out: &mut [u32]) {
    assert!(bits <= 32, "pack: bits {bits} > 32");
    assert_eq!(out.len(), packed_len_u32(bits), "pack: out length mismatch");
    if bits == 0 {
        return;
    }
    for w in out.iter_mut() {
        *w = 0;
    }
    let mask = low_mask(bits);
    for row in 0..ROWS {
        let bitpos = row as u32 * bits;
        let word = (bitpos / 32) as usize;
        let off = bitpos % 32;
        let spans = off + bits > 32;
        for lane in 0..LANES {
            let v = (u64::from(values[row * LANES + lane])) & mask;
            let shifted = v << off;
            out[word * LANES + lane] |= shifted as u32;
            if spans {
                out[(word + 1) * LANES + lane] |= (shifted >> 32) as u32;
            }
        }
    }
}

/// Decode a block packed by [`pack`] at `bits` width into `out`. `packed` must be
/// at least [`packed_len_u32(bits)`] long.
///
/// The inner loop over the 8 lanes is a straight shift/mask/store with a
/// loop-invariant (per-row) word index, offset, and span flag — the shape the
/// optimizer turns into one wide vector step per row.
///
/// # Panics
/// - `bits > 32`.
/// - `packed.len() < packed_len_u32(bits)`.
pub fn unpack(bits: u32, packed: &[u32], out: &mut [u32]) {
    assert!(bits <= 32, "unpack: bits {bits} > 32");
    assert!(out.len() >= BLOCK_LEN, "unpack: out < BLOCK_LEN");
    if bits == 0 {
        out[..BLOCK_LEN].fill(0);
        return;
    }
    assert!(
        packed.len() >= packed_len_u32(bits),
        "unpack: packed too short ({}) for bits {bits}",
        packed.len()
    );
    // Dispatch to a width-specialized decoder so the shift/mask/word offsets are
    // compile-time constants — the ROWS loop unrolls and the 8-lane inner loop
    // lowers to one wide vector op per row (AVX2 on x86-64, NEON on aarch64).
    // A single runtime-`bits` decoder cannot: variable shift amounts and word
    // indices block both unrolling and vectorization. This mirrors a
    // per-bit-width family of decoders (decode1..decode32) rather than one
    // generic loop.
    macro_rules! dispatch {
        ($($b:literal)+) => {
            match bits {
                $($b => unpack_const::<$b>(packed, out),)+
                _ => unreachable!("bits in 1..=32 checked above"),
            }
        };
    }
    dispatch!(1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
}

/// Width-specialized decode: `B` is a const so every `word`/`off`/`spans`/`mask`
/// below folds to a constant, the `ROWS` loop unrolls, and the 8-lane body
/// vectorizes. `packed` is pre-checked to hold `B * LANES` words.
#[inline]
fn unpack_const<const B: u32>(packed: &[u32], out: &mut [u32]) {
    let mask = low_mask(B);
    for row in 0..ROWS {
        let bitpos = row as u32 * B;
        let word = (bitpos / 32) as usize;
        let off = bitpos % 32;
        let spans = off + B > 32;
        for lane in 0..LANES {
            let mut v = u64::from(packed[word * LANES + lane]) >> off;
            if spans {
                v |= u64::from(packed[(word + 1) * LANES + lane]) << (32 - off);
            }
            out[row * LANES + lane] = (v & mask) as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::{BLOCK_LEN, pack, packed_len_u32, unpack};

    /// Minimum bit width that can represent `v`.
    fn bits_for(v: u32) -> u32 {
        32 - v.leading_zeros()
    }

    /// Round-trip identity at a fixed width: pack then unpack must reproduce the
    /// input for every value width the block requires.
    fn assert_roundtrip(values: &[u32; BLOCK_LEN], bits: u32) {
        let mut packed = vec![0u32; packed_len_u32(bits)];
        pack(bits, values, &mut packed);
        let mut out = [0u32; BLOCK_LEN];
        unpack(bits, &packed, &mut out);
        assert_eq!(&out, values, "round-trip mismatch at bits={bits}");
    }

    #[test]
    fn roundtrip_all_widths_boundary_values() {
        // For each width b, pack values that exercise the full b-bit range,
        // including the max (all ones) which stresses word-boundary spans.
        for bits in 0..=32u32 {
            let maxv = if bits == 0 {
                0
            } else if bits == 32 {
                u32::MAX
            } else {
                (1u32 << bits) - 1
            };
            let mut values = [0u32; BLOCK_LEN];
            for (i, slot) in values.iter_mut().enumerate() {
                // A mix: alternating 0 / max / a middle pattern keyed by index,
                // all within `bits`.
                *slot = match i % 3 {
                    0 => maxv,
                    1 => 0,
                    _ => (i as u32) & maxv,
                };
            }
            assert_roundtrip(&values, bits);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(400))]

        /// Random values round-trip at their natural (minimum) width and at any
        /// wider width up to 32.
        #[test]
        fn roundtrip_random(
            values in prop::array::uniform32(any::<u32>())
                .prop_flat_map(|seed| {
                    // Build a 256-long block by tiling the 32-seed, then vary.
                    Just(seed)
                }),
            extra_bits in 0u32..=8,
        ) {
            let mut block = [0u32; BLOCK_LEN];
            for (i, slot) in block.iter_mut().enumerate() {
                *slot = values[i % 32] ^ (i as u32).wrapping_mul(2654435761);
            }
            let needed = block.iter().copied().map(bits_for).max().unwrap_or(0);
            let bits = (needed + extra_bits).min(32);
            assert_roundtrip(&block, bits);
        }
    }
}
