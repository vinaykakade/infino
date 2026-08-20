// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Fixed-width bit-packing for 256-value blocks with a transposed lane layout.
//!
//! Backs the 256-doc [`block256`](super) PACKED path. The 256 values are stored
//! as **8 interleaved lane-streams**: value `i` lives in lane `i % 8`, at stream
//! position `i / 8`. Each lane is an independent 32-value bit-stream packed at
//! `bits` bits, and all 8 lanes share the same bit-alignment. That alignment is
//! the whole point: a value that straddles two 32-bit packed words is decoded
//! **inline** — right-shift the current word, OR in the low bits of the next word
//! (left-shifted) — in one uniform loop, with no width-dependent branch and no
//! separate reassembly pass. Decode is therefore flat across every bit width, and
//! the 4-lane stores land the values already in order, so no de-interleave step
//! is needed before the delta-integrate.
//!
//! [`pack`] (scalar, build-time) and [`unpack`] are exact inverses. On `aarch64`
//! [`unpack`] runs a NEON kernel (two `uint32x4_t` = 8 lanes per step); a scalar
//! path mirrors it exactly as the reference. The on-disk bytes are this codec's
//! own layout — a self-contained 256-block encoding.

// `unpack_scalar` is the reference / non-aarch64 decoder and correctness oracle;
// on aarch64 it is reached only from tests, so keep it regardless of target.
#![allow(dead_code)]

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    vaddq_u32, vandq_u32, vdupq_n_s32, vdupq_n_u32, vextq_u32, vgetq_lane_u32, vld1q_u32,
    vorrq_u32, vshlq_u32, vst1q_u32,
};
#[cfg(target_arch = "aarch64")]
use core::ptr::copy_nonoverlapping;

/// Values per block — one 256-doc posting block's worth.
pub(super) const BLOCK: usize = 256;

/// Number of interleaved lane-streams. A value-register is 8 lanes wide; the block
/// is `BLOCK / LANES = 32` value-registers, one per stream position.
const LANES: usize = 8;

/// Value-registers per block (`BLOCK / LANES`).
const REGS: usize = BLOCK / LANES;

/// `b`-bit low mask (`b` in `1..=32`).
#[inline]
fn mask32(b: usize) -> u32 {
    if b >= 32 { u32::MAX } else { (1u32 << b) - 1 }
}

/// Bytes a `bits`-wide 256-value block occupies: `bits * 32`.
#[inline]
pub(super) fn packed_len(bits: u8) -> usize {
    bits as usize * BLOCK / 8
}

/// Append the packing of `vals` at `bits` bits to `out` (`bits * 32` bytes).
/// `bits == 0` writes nothing. Every value must fit in `bits` bits.
///
/// Scalar and build-time only — off the hot path, so it favours clarity. Value
/// register `vr` (its 8 lane values) is written at bit position `vr * bits` into
/// each lane's stream, spilling the high bits into the next packed word when the
/// value straddles a 32-bit boundary.
pub(super) fn pack(vals: &[u32; BLOCK], bits: u8, out: &mut Vec<u8>) {
    if bits == 0 {
        return;
    }
    let bits = bits as usize;
    debug_assert!(bits <= 32);
    let n_words = LANES * bits; // output u32 count (= bits * 32 bytes)
    let mut w = [0u32; LANES * 32]; // max LANES*32; use w[..n_words]
    for vr in 0..REGS {
        let bit_pos = vr * bits;
        let reg = bit_pos / 32;
        let off = bit_pos % 32;
        for l in 0..LANES {
            let v = vals[vr * LANES + l];
            w[reg * LANES + l] |= v << off;
            if off + bits > 32 {
                // Straddle: the high `off + bits - 32` bits go to the next word.
                w[(reg + 1) * LANES + l] |= v >> (32 - off);
            }
        }
    }
    out.reserve(n_words * 4);
    for &word in &w[..n_words] {
        out.extend_from_slice(&word.to_le_bytes());
    }
}

/// Inverse of [`pack`]: decode `bits * 32` bytes into `dest[..256]`. `bits == 0`
/// fills zeros. `bytes` must be at least [`packed_len`] long. Dispatches to a
/// hand-vectorized decoder per architecture; [`unpack_scalar`] is the reference.
#[inline]
pub(super) fn unpack(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: NEON is baseline on aarch64, so the target-feature precondition
    // always holds. `unpack_neon` is the vectorized twin of `unpack_scalar`,
    // proptested byte-identical across every width.
    unsafe {
        unpack_neon(bytes, bits, dest);
    }
    #[cfg(not(target_arch = "aarch64"))]
    unpack_scalar(bytes, bits, dest);
}

/// Read packed word `k` (little-endian) directly from the byte buffer.
#[inline]
fn read_word(bytes: &[u8], k: usize) -> u32 {
    let o = k * 4;
    u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]])
}

/// Scalar reference decoder — the correctness oracle for [`unpack_neon`] and the
/// decoder on architectures without a hand-written kernel. Reads each value from
/// its lane-stream position by the direct bit-address formula, independent of the
/// NEON kernel's incremental word-advance, so the two cross-check.
fn unpack_scalar(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    if bits == 0 {
        dest.fill(0);
        return;
    }
    let bits = bits as usize;
    let mask = mask32(bits);
    for vr in 0..REGS {
        let bit_pos = vr * bits;
        let reg = bit_pos / 32;
        let off = bit_pos % 32;
        for l in 0..LANES {
            let mut v = read_word(bytes, reg * LANES + l) >> off;
            if off + bits > 32 {
                v |= read_word(bytes, (reg + 1) * LANES + l) << (32 - off);
            }
            dest[vr * LANES + l] = v & mask;
        }
    }
}

/// NEON decoder: 8 lanes (two `uint32x4_t`) per value-register, 32 registers per
/// block. Each register is right-shifted to its in-word offset and masked; when
/// it straddles into the next packed word, the next word is loaded and its low
/// bits OR-ed in — the same uniform step at every width, so there is no
/// width-dependent branch and no separate straddle pass. The 4-lane stores land
/// the values in order.
///
/// # Safety
/// Requires the `neon` target feature (baseline on `aarch64`). `bytes` must hold
/// at least `packed_len(bits)` bytes (`= bits * 8` u32). Every load reads one
/// value-register (8 u32) at word offset `reg * 8`, with `reg <= bits - 1`, so it
/// stays within `bytes`; every store writes `dest[vr*8 .. vr*8+8]` with `vr < 32`.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn unpack_neon(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    // SAFETY: see the function-level bounds argument above.
    unsafe {
        if bits == 0 {
            dest.fill(0);
            return;
        }
        let dp = dest.as_mut_ptr();
        if bits == 32 {
            // Each value-register is a full packed word — a straight copy.
            copy_nonoverlapping(bytes.as_ptr(), dp as *mut u8, BLOCK * 4);
            return;
        }
        let bits = bits as usize;
        let ip = bytes.as_ptr().cast::<u32>();
        let mask = vdupq_n_u32(mask32(bits));

        // `reg` is the packed word currently in (wa, wb). Value-register 0 sits at
        // offset 0, so it is just masked.
        let mut reg = 0usize;
        let mut wa = vld1q_u32(ip);
        let mut wb = vld1q_u32(ip.add(4));
        vst1q_u32(dp, vandq_u32(wa, mask));
        vst1q_u32(dp.add(4), vandq_u32(wb, mask));

        for vr in 1..REGS {
            let inner_cursor = (vr * bits) % 32;
            let inner_capacity = 32 - inner_cursor;
            let (sa, sb) = if inner_cursor != 0 {
                // Negative-count shift == per-lane logical right shift.
                let neg = vdupq_n_s32(-(inner_cursor as i32));
                (vshlq_u32(wa, neg), vshlq_u32(wb, neg))
            } else {
                (wa, wb)
            };
            let mut oa = vandq_u32(sa, mask);
            let mut ob = vandq_u32(sb, mask);
            // If this register is now fully consumed, advance to the next packed
            // word; if the value straddled it, OR in the straddled high bits.
            if inner_capacity <= bits && vr != REGS - 1 {
                reg += 1;
                wa = vld1q_u32(ip.add(reg * LANES));
                wb = vld1q_u32(ip.add(reg * LANES + 4));
                if inner_capacity < bits {
                    let pos = vdupq_n_s32(inner_capacity as i32);
                    oa = vorrq_u32(oa, vandq_u32(vshlq_u32(wa, pos), mask));
                    ob = vorrq_u32(ob, vandq_u32(vshlq_u32(wb, pos), mask));
                }
            }
            vst1q_u32(dp.add(vr * LANES), oa);
            vst1q_u32(dp.add(vr * LANES + 4), ob);
        }
    }
}

/// In-place inclusive prefix-sum of doc-id deltas, offset by `base`:
/// `a[i] = base + sum(a[0..=i])`. With `a[0]` a zero delta (the block base is
/// stored separately), `a[0]` becomes `base`. This is the delta-integrate step
/// that turns unpacked deltas into ascending doc ids; keeping it out of [`unpack`]
/// lets the same unpack serve tfs (no integrate) and doc ids (integrate) without a
/// branch in the hot bit-extraction. NEON on aarch64, [`integrate_scalar`]
/// elsewhere and as the proptest reference.
#[inline]
pub(super) fn integrate(a: &mut [u32; BLOCK], base: u32) {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: NEON is baseline on aarch64; `integrate_neon` is the vectorized twin
    // of `integrate_scalar`, proptested byte-identical.
    unsafe {
        integrate_neon(a, base);
    }
    #[cfg(not(target_arch = "aarch64"))]
    integrate_scalar(a, base);
}

/// Scalar reference prefix-sum — a serial 256-add chain. The correctness oracle
/// for [`integrate_neon`] and the decoder on non-aarch64 targets.
fn integrate_scalar(a: &mut [u32; BLOCK], base: u32) {
    let mut acc = base;
    for slot in a.iter_mut() {
        acc = acc.wrapping_add(*slot);
        *slot = acc;
    }
}

/// NEON prefix-sum. Each 4-lane group gets its inclusive within-vector prefix via
/// two `vext`-shift-and-adds, then the running `carry` (all values before this
/// group, plus `base`) is splat-added. The loop-carried dependency is only the
/// **scalar** `carry += group_total`, where `group_total` is the *pre-carry*
/// prefix's top lane (independent of `carry`), so the serial chain is 64 one-cycle
/// scalar adds — shorter than a vector-add carry chain (measured: a `vaddq` carry
/// is ~2–3× the latency and lost across widths) and far shorter than the 256-long
/// chain of [`integrate_scalar`]. The `vgetq_lane`/`vdupq_n` GPR round-trips are
/// cheap on this core and pipeline off the critical path; the within-group prefix
/// runs throughput-bound alongside.
///
/// # Safety
/// Requires `neon` (baseline on aarch64). Every load/store touches `a[i..i+4]`
/// with `i < 256` in steps of 4, so all accesses stay within `a[..256]`.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn integrate_neon(a: &mut [u32; BLOCK], base: u32) {
    // SAFETY: see the function-level bounds argument above.
    unsafe {
        let zero = vdupq_n_u32(0);
        let p = a.as_mut_ptr();
        let mut carry = base;
        let mut i = 0usize;
        while i < BLOCK {
            let x = vld1q_u32(p.add(i));
            // Inclusive within-vector prefix: [d0, d0+d1, d0+d1+d2, d0+d1+d2+d3].
            let s1 = vaddq_u32(x, vextq_u32::<3>(zero, x));
            let pfx = vaddq_u32(s1, vextq_u32::<2>(zero, s1));
            // group_total is the top lane of the pre-carry prefix — independent of
            // `carry`, so the carry chain stays a plain scalar add.
            let group_total = vgetq_lane_u32::<3>(pfx);
            vst1q_u32(p.add(i), vaddq_u32(pfx, vdupq_n_u32(carry)));
            carry = carry.wrapping_add(group_total);
            i += 4;
        }
    }
}

/// Decode-throughput A/B vs the crate's `BitPacker8x` (`--ignored --nocapture`,
/// release). Isolates the bit-unpack primitive across widths; on `aarch64` the
/// crate side is hand-NEON when the fork patch is active, so the ratios read
/// against a real SIMD decoder.
#[cfg(test)]
mod bench {
    use std::{hint::black_box, time::Instant};

    use bitpacking::{BitPacker, BitPacker8x};

    use super::*;

    #[test]
    #[ignore = "manual decode A/B: cargo test --release ...bitpack::bench -- --ignored --nocapture"]
    fn decode_vs_bitpacker8x() {
        let bp = BitPacker8x::new();
        let iters = 300_000u32;
        println!("\nwidth   unpack(ns)   BitPacker8x(ns)   ratio (unpack/bp; <1 = ours faster)");
        for &bits in &[1u8, 2, 3, 4, 5, 6, 7, 8, 10, 12, 16, 20, 24, 32] {
            let mask = if bits == 32 {
                u32::MAX
            } else {
                (1u32 << bits) - 1
            };
            let mut vals = [0u32; BLOCK];
            for (i, v) in vals.iter_mut().enumerate() {
                *v = (i as u32).wrapping_mul(2_654_435_761) & mask;
            }
            let mut mine = Vec::new();
            pack(&vals, bits, &mut mine);
            let mut theirs = vec![0u8; BLOCK * 4];
            let n = bp.compress(&vals, &mut theirs, bits);
            theirs.truncate(n);

            let mut dest = [0u32; BLOCK];
            // Each op returns a checksum of spread output slots; the loop folds
            // it into a black-boxed sink so the decode cannot be elided.
            let time = |f: &mut dyn FnMut() -> u32| {
                let mut sink = 0u32;
                for _ in 0..iters / 8 {
                    sink = sink.wrapping_add(f());
                }
                let t = Instant::now();
                for _ in 0..iters {
                    sink = sink.wrapping_add(f());
                }
                let ns = t.elapsed().as_nanos() as f64 / iters as f64;
                black_box(sink);
                ns
            };
            let mine_ns = time(&mut || {
                unpack(black_box(&mine), bits, &mut dest);
                dest[0] ^ dest[100] ^ dest[200] ^ dest[255]
            });
            let bp_ns = time(&mut || {
                bp.decompress(black_box(&theirs), &mut dest, bits);
                dest[0] ^ dest[100] ^ dest[200] ^ dest[255]
            });
            println!(
                "{bits:5}   {mine_ns:9.1}   {bp_ns:13.1}   {:.2}",
                mine_ns / bp_ns
            );
        }
    }

    /// Sorted-decode A/B: our `unpack` + `integrate` (delta-decode + prefix-sum)
    /// vs `BitPacker8x::decompress_sorted` (fused SIMD unpack+integrate). This is
    /// the doc-id decode path COUNT/ranked hammer — the metric the codec targets.
    #[test]
    #[ignore = "manual sorted-decode A/B: cargo test --release ...bitpack::bench::sorted -- --ignored --nocapture"]
    fn sorted_decode_vs_bitpacker8x() {
        let bp = BitPacker8x::new();
        let iters = 300_000u32;
        let base = 1_000_000u32;
        println!(
            "\nwidth   ours u+i(ns)   decompress_sorted(ns)   ratio (ours/bp; <1 = ours faster)"
        );
        for &bits in &[1u8, 2, 3, 4, 5, 6, 7, 8, 10, 12, 16, 20, 24] {
            let dmask = if bits == 32 {
                u32::MAX
            } else {
                (1u32 << bits) - 1
            };
            // Ascending doc ids whose deltas fit in `bits` (delta[0] = 0).
            let mut docs = [0u32; BLOCK];
            let mut deltas = [0u32; BLOCK];
            let mut acc = base;
            for i in 0..BLOCK {
                let d = if i == 0 {
                    0
                } else {
                    ((i as u32).wrapping_mul(2_654_435_761) & dmask).max(1)
                };
                acc = acc.wrapping_add(d);
                docs[i] = acc;
                deltas[i] = d;
            }
            let mut mine = Vec::new();
            pack(&deltas, bits, &mut mine);
            let mut theirs = vec![0u8; BLOCK * 4];
            let n = bp.compress_sorted(base, &docs, &mut theirs, bits);
            theirs.truncate(n);

            let mut dest = [0u32; BLOCK];
            let time = |f: &mut dyn FnMut() -> u32| {
                let mut sink = 0u32;
                for _ in 0..iters / 8 {
                    sink = sink.wrapping_add(f());
                }
                let t = Instant::now();
                for _ in 0..iters {
                    sink = sink.wrapping_add(f());
                }
                let ns = t.elapsed().as_nanos() as f64 / iters as f64;
                black_box(sink);
                ns
            };
            let mine_ns = time(&mut || {
                unpack(black_box(&mine), bits, &mut dest);
                integrate(&mut dest, base);
                dest[0] ^ dest[100] ^ dest[200] ^ dest[255]
            });
            let bp_ns = time(&mut || {
                bp.decompress_sorted(base, black_box(&theirs), &mut dest, bits);
                dest[0] ^ dest[100] ^ dest[200] ^ dest[255]
            });
            println!(
                "{bits:5}   {mine_ns:11.1}   {bp_ns:21.1}   {:.2}",
                mine_ns / bp_ns
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    /// Byte length matches `bits * 32` and an all-ones block round-trips at every
    /// width (exercises the max value and the straddle at non-dividing widths).
    #[test]
    fn packed_len_and_roundtrip_all_ones() {
        for bits in 1..=32usize {
            let vals = [mask32(bits); BLOCK];
            let mut out = Vec::new();
            pack(&vals, bits as u8, &mut out);
            assert_eq!(out.len(), packed_len(bits as u8), "bits={bits}");
            let mut dec = [0u32; BLOCK];
            unpack(&out, bits as u8, &mut dec);
            assert_eq!(dec, vals, "all-ones round-trip, bits={bits}");
        }
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
        /// input exactly. `bits` spans 1..=32, values masked to the width.
        #[test]
        fn roundtrip_all_widths(
            bits in 1u8..=32,
            raw in prop::array::uniform32(any::<u32>()),
        ) {
            let mask = mask32(bits as usize);
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

        /// Same, but with genuinely random per-slot values.
        #[test]
        fn roundtrip_random_dense(
            bits in 1u8..=32,
            vals_seed in prop::collection::vec(any::<u32>(), BLOCK),
        ) {
            let mask = mask32(bits as usize);
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

        /// The per-arch dispatched decoder is byte-identical to the scalar
        /// reference at every width — the correctness gate for the SIMD kernel.
        #[test]
        fn dispatched_matches_scalar(
            bits in 1u8..=32,
            vals_seed in prop::collection::vec(any::<u32>(), BLOCK),
        ) {
            let mask = mask32(bits as usize);
            let mut vals = [0u32; BLOCK];
            for (v, &s) in vals.iter_mut().zip(vals_seed.iter()) {
                *v = s & mask;
            }
            let mut out = Vec::new();
            pack(&vals, bits, &mut out);
            let mut a = [0u32; BLOCK];
            let mut b = [0u32; BLOCK];
            unpack(&out, bits, &mut a);
            unpack_scalar(&out, bits, &mut b);
            prop_assert_eq!(a, b);
        }

        /// The dispatched `integrate` (SIMD prefix-sum) is byte-identical to the
        /// scalar reference for any deltas and base — the correctness gate for the
        /// NEON delta-integrate. Full-range u32 deltas exercise wrapping too.
        #[test]
        fn integrate_matches_scalar(
            base in any::<u32>(),
            deltas in prop::collection::vec(any::<u32>(), BLOCK),
        ) {
            let mut a = [0u32; BLOCK];
            let mut b = [0u32; BLOCK];
            for (i, &d) in deltas.iter().enumerate() {
                a[i] = d;
                b[i] = d;
            }
            integrate(&mut a, base);
            integrate_scalar(&mut b, base);
            prop_assert_eq!(a, b);
        }
    }
}
