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
    uint32x4_t, vaddq_u32, vandq_u32, vdupq_n_u32, vextq_u32, vgetq_lane_u32, vld1q_u32, vorrq_u32,
    vshlq_n_u32, vshrq_n_u32, vst1q_u32,
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128i, __m256i, _mm_add_epi32, _mm_cvtsi128_si32, _mm_loadu_si128, _mm_set1_epi32,
    _mm_shuffle_epi32, _mm_slli_si128, _mm_storeu_si128, _mm256_and_si256, _mm256_loadu_si256,
    _mm256_or_si256, _mm256_set1_epi32, _mm256_slli_epi32, _mm256_srli_epi32, _mm256_storeu_si256,
};
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
use core::ptr::copy_nonoverlapping;

// Const-unrolled per-width NEON unpack kernels (`unpack_w1..unpack_w31` +
// `unpack_neon_unrolled` dispatch). Generated: every shift is a compile-time
// constant, so there is no per-iteration shift-vector setup or branch.
#[cfg(target_arch = "aarch64")]
include!("bitpack_unpack_neon.rs");

// Const-unrolled per-width AVX2 unpack kernels (`unpack_avx2_w1..w31` +
// `unpack_avx2_unrolled` dispatch). Generated; a `__m256i` is one 8-lane
// value-register, so decode is one load/shift/mask/store per value-register.
#[cfg(target_arch = "x86_64")]
include!("bitpack_unpack_avx2.rs");

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
    #[cfg(target_arch = "x86_64")]
    {
        // Runtime-detect AVX2 (present on the deployment target); the SIMD kernel
        // is proptested byte-identical to the scalar reference.
        if is_x86_feature_detected!("avx2") {
            // SAFETY: guarded by the AVX2 detection just above.
            unsafe { unpack_avx2(bytes, bits, dest) }
        } else {
            unpack_scalar(bytes, bits, dest);
        }
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    unpack_scalar(bytes, bits, dest);
}

/// AVX2 decoder: dispatches to the const-unrolled per-width kernel (one `__m256i`
/// per value-register). `bits == 0` fills zeros; `bits == 32` is a straight copy.
///
/// # Safety
/// Requires the `avx2` target feature (checked by the caller). `bytes` must hold
/// at least `packed_len(bits)` bytes; `dest` is `[u32; 256]`.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn unpack_avx2(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    // SAFETY: see the function-level contract; every load/store is in range.
    unsafe {
        if bits == 0 {
            dest.fill(0);
            return;
        }
        let dp = dest.as_mut_ptr();
        if bits == 32 {
            copy_nonoverlapping(bytes.as_ptr(), dp as *mut u8, BLOCK * 4);
            return;
        }
        let ip = bytes.as_ptr() as *const __m256i;
        let mask = _mm256_set1_epi32(mask32(bits as usize) as i32);
        unpack_avx2_unrolled(ip, dp as *mut __m256i, bits, mask);
    }
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
        // Widths 1..=31 dispatch to a const-unrolled per-width kernel: every shift
        // amount is a compile-time constant, so decode is straight-line with no
        // per-value-register shift-vector setup or branch.
        let ip = bytes.as_ptr().cast::<u32>();
        let mask = vdupq_n_u32(mask32(bits as usize));
        unpack_neon_unrolled(ip, dp, bits, mask);
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
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx2") {
            // SAFETY: guarded by the AVX2 detection just above.
            unsafe { integrate_avx2(a, base) }
        } else {
            integrate_scalar(a, base);
        }
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    integrate_scalar(a, base);
}

/// SSE2 prefix-sum (`__m128i`, 4 lanes) — the x86 twin of [`integrate_neon`], same
/// scalar-carry scheme: within-vector inclusive prefix via two byte-shift adds,
/// then a scalar `carry += group_total` where `group_total` is the pre-carry
/// prefix's top lane (independent of `carry`). Gated on `avx2` for one feature
/// gate with [`unpack_avx2`]; only SSE2 ops are used.
///
/// # Safety
/// Requires `avx2` (checked by the caller). Reads/writes `a[..256]` in 4-lane
/// steps.
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn integrate_avx2(a: &mut [u32; BLOCK], base: u32) {
    // SAFETY: see the function-level contract.
    unsafe {
        let p = a.as_mut_ptr().cast::<__m128i>();
        let mut carry = base;
        for i in 0..BLOCK / 4 {
            let x = _mm_loadu_si128(p.add(i));
            // Inclusive within-vector prefix (`slli_si128` shifts by whole bytes):
            // + [0,x0,x1,x2] then + [0,0,s0,s1].
            let s1 = _mm_add_epi32(x, _mm_slli_si128::<4>(x));
            let pfx = _mm_add_epi32(s1, _mm_slli_si128::<8>(s1));
            // group_total = pre-carry prefix lane 3 (broadcast to lane 0, read out).
            let group_total = _mm_cvtsi128_si32(_mm_shuffle_epi32::<0b11_11_11_11>(pfx)) as u32;
            _mm_storeu_si128(p.add(i), _mm_add_epi32(pfx, _mm_set1_epi32(carry as i32)));
            carry = carry.wrapping_add(group_total);
        }
    }
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

// AVX2 correctness gate. `is_x86_feature_detected!("avx2")` is false under Rosetta,
// so these call the AVX2 kernels *directly* (bypassing the runtime dispatch) to
// verify them against the scalar reference; build with `-C target-feature=+avx2`
// for `x86_64-apple-darwin` and run under Rosetta, or natively on an AVX2 host.
#[cfg(all(test, target_arch = "x86_64"))]
mod avx2_tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(2048))]

        /// AVX2 unpack is byte-identical to the scalar reference at every width.
        #[test]
        fn avx2_unpack_matches_scalar(
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
            // SAFETY: the test binary is built with `+avx2`; calling directly
            // bypasses the runtime detection that Rosetta reports as false.
            unsafe { unpack_avx2(&out, bits, &mut a) };
            unpack_scalar(&out, bits, &mut b);
            prop_assert_eq!(a, b);
        }

        /// AVX2 integrate is byte-identical to the scalar reference.
        #[test]
        fn avx2_integrate_matches_scalar(
            base in any::<u32>(),
            deltas in prop::collection::vec(any::<u32>(), BLOCK),
        ) {
            let mut a = [0u32; BLOCK];
            let mut b = [0u32; BLOCK];
            for (i, &d) in deltas.iter().enumerate() {
                a[i] = d;
                b[i] = d;
            }
            // SAFETY: built with `+avx2`; direct call bypasses runtime detection.
            unsafe { integrate_avx2(&mut a, base) };
            integrate_scalar(&mut b, base);
            prop_assert_eq!(a, b);
        }
    }
}
