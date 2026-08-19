// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Fixed-width bit-packing for 256-value blocks with primitive-size lanes.
//!
//! Backs the 256-doc [`block256`](super) PACKED path. The layout targets
//! **decode op-count**: values
//! are packed at the smallest *primitive* that fits the bit width — 8-bit lanes
//! for width ≤ 8, 16-bit for ≤ 16, 32-bit above — so a narrow column (tf is
//! usually 1–3 bits, dense doc-deltas 1–8) unpacks with a quarter / half the
//! shift-mask rounds. Four values share one 32-bit word as byte lanes
//! (`collapse8` / `expand8`), two as half lanes (`collapse16` / `expand16`); a
//! lane-replicated mask keeps the lanes from bleeding into each other.
//!
//! [`pack`] (scalar, build-time) and [`unpack`] are exact inverses. Decode is a
//! shift-level extraction + a per-width **branchless stitch** (only for widths
//! that don't divide the primitive, whose values span two packed words) +
//! `expand`. On `aarch64` the shift levels and `expand` run on NEON and the stitch
//! kernels are generated straight-line const-shift code (see `bitpack_stitch.rs`);
//! a scalar path mirrors them exactly as the reference. The on-disk bytes are
//! this codec's own layout — a self-contained 256-block encoding.

// Scalar reference helpers (`unpack_scalar`, `expand*`/`collapse*`) are unused on
// targets that take the NEON path; they are the correctness oracle and the
// fallback decoder, so keep them regardless of the current target.
#![allow(dead_code)]
// The generated stitch kernels (`bitpack_stitch.rs`, `include!`d below) transcribe
// a `<< 0` verbatim from the width formula and drive their straddle loops with an
// explicit index counter; both read cleaner as generated than clippy-idiomatic.
#![allow(clippy::identity_op, clippy::explicit_counter_loop)]

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    uint32x4x2_t, vaddq_u32, vandq_u32, vdupq_n_s32, vdupq_n_u32, vextq_u32, vgetq_lane_u32,
    vld1q_u32, vld3q_u32, vorrq_u32, vshlq_n_u32, vshlq_u32, vshrq_n_u32, vst1q_u32, vst2q_u32,
};
#[cfg(target_arch = "aarch64")]
use core::ptr::copy_nonoverlapping;

/// Values per block — one 256-doc posting block's worth.
pub(super) const BLOCK: usize = 256;

// Per-width branchless stitch kernels (generated). Reassemble the straddle
// values that span two packed words; the whole cost at non-dividing widths.
include!("bitpack_stitch.rs");

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
    let mut tmp_buf = [0u32; BLOCK]; // n_out <= 32*8 = 256; stack, no alloc
    let tmp = &mut tmp_buf[..n_out];
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
            rem_val += bits - rem_int;
            tmp[tmp_idx] |= (ints[idx] >> rem_val) & mask2;
            tmp_idx += 1;
        }
    }

    out.reserve(n_out * 4);
    for &w in tmp.iter() {
        out.extend_from_slice(&w.to_le_bytes());
    }
}

/// Inverse of [`pack`]: decode `bits * 32` bytes into `dest[..256]`. `bits == 0`
/// fills zeros. `bytes` must be at least `packed_len(bits)` long. Dispatches to a
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

/// Read packed word `i` (little-endian) directly from the byte buffer.
#[inline]
fn read_word(bytes: &[u8], i: usize) -> u32 {
    let o = i * 4;
    u32::from_le_bytes([bytes[o], bytes[o + 1], bytes[o + 2], bytes[o + 3]])
}

/// Fill `tmp[..n_out]` with each packed word masked to its leftover (`cmask`)
/// bits — the low straddle bits the shift levels did not take. This is the input
/// the generated [`stitch_dispatch`] kernels read. Called only at non-dividing
/// widths (dividing widths have no straddle tail).
#[inline]
fn fill_leftover(bytes: &[u8], n_out: usize, cmask: u32, tmp: &mut [u32; BLOCK]) {
    for (i, t) in tmp[..n_out].iter_mut().enumerate() {
        *t = read_word(bytes, i) & cmask;
    }
}

/// SIMD stitch for the four non-dividing widths whose straddle period is exactly
/// three words yielding one value (6/12/24) — a `vld3q` deinterleaves four
/// periods, one shift-combine yields four contiguous values. Byte-identical to the
/// scalar `stitch_{6,12,24}`; `S0`/`S1` are those kernels' constant shifts.
///
/// # Safety
/// `neon`; reads `12 * iters <= n_out` words within `tmp`, writes `4 * iters`
/// words into `dest[o..]` (`o + 4*iters <= 256`).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn stitch3_m1<const S0: i32, const S1: i32>(
    tmp: &[u32],
    dest: &mut [u32; BLOCK],
    o: usize,
    iters: usize,
) {
    // SAFETY: `neon`. Caller passes (o, iters) matching the width so reads stay in
    // `tmp[..iters*12]` and writes in `dest[o..o+iters*4] ⊆ dest[..256]`.
    unsafe {
        let tp = tmp.as_ptr();
        let dp = dest.as_mut_ptr().add(o);
        let mut w = 0usize;
        let mut d = 0usize;
        for _ in 0..iters {
            let r = vld3q_u32(tp.add(w));
            let v = vorrq_u32(
                vshlq_n_u32::<S0>(r.0),
                vorrq_u32(vshlq_n_u32::<S1>(r.1), r.2),
            );
            vst1q_u32(dp.add(d), v);
            w += 12;
            d += 4;
        }
    }
}

/// SIMD stitch for width 3 (three-word period yielding two values). Byte-identical
/// to the scalar `stitch_3`; a `vst2q` interleaves the two value streams.
///
/// # Safety
/// `neon`; reads `24` words within `tmp[..24]`, writes `16` words into `dest[48..64]`.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn stitch3_w3(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    // SAFETY: `neon`. Reads `tmp[..24]`, writes `dest[48..64] ⊆ dest[..256]`.
    unsafe {
        let tp = tmp.as_ptr();
        let dp = dest.as_mut_ptr().add(48);
        let m = vdupq_n_u32(0x0101_0101);
        let mut w = 0usize;
        let mut d = 0usize;
        for _ in 0..2 {
            let r = vld3q_u32(tp.add(w));
            let l0 = vorrq_u32(vshlq_n_u32::<1>(r.0), vandq_u32(vshrq_n_u32::<1>(r.1), m));
            let l1 = vorrq_u32(vshlq_n_u32::<2>(vandq_u32(r.1, m)), r.2);
            vst2q_u32(dp.add(d), uint32x4x2_t(l0, l1));
            w += 12;
            d += 8;
        }
    }
}

/// Dispatch the straddle stitch to a NEON kernel where the period fits `vld3q`
/// (widths 3/6/12/24), else the scalar branchless kernel.
///
/// # Safety
/// `neon`; the four specialized widths satisfy their kernels' bounds, and the
/// scalar fallback is safe.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn stitch_dispatch_neon(tmp: &[u32], bits: usize, dest: &mut [u32; BLOCK]) {
    // SAFETY: `neon`. Each specialized width passes (o, iters) satisfying its
    // kernel's bounds; the scalar fallback is safe.
    unsafe {
        match bits {
            3 => stitch3_w3(tmp, dest),
            6 => stitch3_m1::<4, 2>(tmp, dest, 48, 4),
            12 => stitch3_m1::<8, 4>(tmp, dest, 96, 8),
            24 => stitch3_m1::<16, 8>(tmp, dest, 192, 16),
            _ => stitch_dispatch(tmp, bits, dest),
        }
    }
}

/// Scalar reference decoder — the correctness oracle for the SIMD paths, and the
/// decoder on architectures without a hand-written kernel.
fn unpack_scalar(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    if bits == 0 {
        dest.fill(0);
        return;
    }
    let bits = bits as usize;
    let p = primitive(bits);
    let num_ints = BLOCK * p / 32;
    let n_out = bits * 8;
    debug_assert!(bytes.len() >= n_out * 4);
    let mask_full = lane_mask(p, bits);

    // Shift levels: extract the cleanly-aligned values at each descending shift,
    // reading packed words straight from `bytes` (no intermediate buffer).
    let mut idx = 0usize;
    let mut shift = p as i32 - bits as i32;
    loop {
        for i in 0..n_out {
            dest[idx] = (read_word(bytes, i) >> shift) & mask_full;
            idx += 1;
        }
        shift -= bits as i32;
        if shift < 0 {
            break;
        }
    }
    // Non-dividing widths: reassemble the straddle values via the generated
    // per-width kernel, reading each word's leftover (low `rem_int`) bits.
    if idx < num_ints {
        let cmask = lane_mask(p, (shift + bits as i32) as usize);
        let mut tmp = [0u32; BLOCK];
        fill_leftover(bytes, n_out, cmask, &mut tmp);
        stitch_dispatch(&tmp, bits, dest);
    }
    match p {
        8 => expand8(dest),
        16 => expand16(dest),
        _ => {}
    }
}

/// NEON decoder: the shift-level extraction and `expand` run four lanes at a
/// time; the straddle tail stays scalar. Byte-identical to [`unpack_scalar`]
/// (proptested across all widths).
///
/// # Safety
/// Requires the `neon` target feature (baseline on `aarch64`). Every load reads
/// 16 bytes within `bytes[..n_out * 4]` (`i + 8 <= n_out`), and every store
/// stays inside `dest[..256]`: the shift-level loop writes
/// `num_levels * n_out <= num_ints <= 256` words in 8-lane steps, and `expand`'s
/// indices are bounded by 64 / 128.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn unpack_neon(bytes: &[u8], bits: u8, dest: &mut [u32; BLOCK]) {
    // SAFETY: `neon`. See the function-level bounds argument above; every load
    // reads 16 bytes within `bytes[..n_out*4]` and every store stays in
    // `dest[..256]`.
    unsafe {
        if bits == 0 {
            dest.fill(0);
            return;
        }
        // Fused fast paths for widths equal to their primitive (no shift levels, no
        // straddle): decode straight into the 256 expanded positions in one pass.
        match bits {
            8 => return expand8_from_bytes(bytes, dest),
            16 => return expand16_from_bytes(bytes, dest),
            32 => {
                return copy_nonoverlapping(
                    bytes.as_ptr(),
                    dest.as_mut_ptr() as *mut u8,
                    BLOCK * 4,
                );
            }
            _ => {}
        }
        let bits = bits as usize;
        let p = primitive(bits);
        let num_ints = BLOCK * p / 32;
        let n_out = bits * 8;
        debug_assert!(bytes.len() >= n_out * 4);
        let mask_full = lane_mask(p, bits);

        // Shift levels, 8 lanes/iteration (two NEON registers), reading packed words
        // straight from `bytes` via unaligned `vld1q` (no intermediate buffer).
        // `vshlq_u32` with a negative (uniform) count is a per-lane logical right
        // shift. `n_out` is always a multiple of 8 (`bits * 8`), so the step is exact.
        let maskv = vdupq_n_u32(mask_full);
        let bp = bytes.as_ptr();
        let dp = dest.as_mut_ptr();
        let mut idx = 0usize;
        let mut shift = p as i32 - bits as i32;
        loop {
            let negv = vdupq_n_s32(-shift);
            let mut i = 0usize;
            while i < n_out {
                let w0 = vld1q_u32(bp.add(i * 4).cast::<u32>());
                let w1 = vld1q_u32(bp.add((i + 4) * 4).cast::<u32>());
                vst1q_u32(dp.add(idx), vandq_u32(vshlq_u32(w0, negv), maskv));
                vst1q_u32(dp.add(idx + 4), vandq_u32(vshlq_u32(w1, negv), maskv));
                i += 8;
                idx += 8;
            }
            shift -= bits as i32;
            if shift < 0 {
                break;
            }
        }
        // Non-dividing widths: mask each word to its leftover bits (8-lane), then run
        // the generated branchless per-width stitch to reassemble the straddle values.
        if idx < num_ints {
            let cmaskv = vdupq_n_u32(lane_mask(p, (shift + bits as i32) as usize));
            let mut tmp = [0u32; BLOCK];
            let tp = tmp.as_mut_ptr();
            let mut i = 0usize;
            while i < n_out {
                let w0 = vld1q_u32(bp.add(i * 4).cast::<u32>());
                let w1 = vld1q_u32(bp.add((i + 4) * 4).cast::<u32>());
                vst1q_u32(tp.add(i), vandq_u32(w0, cmaskv));
                vst1q_u32(tp.add(i + 4), vandq_u32(w1, cmaskv));
                i += 8;
            }
            stitch_dispatch_neon(&tmp, bits, dest);
        }
        match p {
            8 => expand8_neon(dest),
            16 => expand16_neon(dest),
            _ => {}
        }
    }
}

/// NEON [`expand8`]. Each word's four byte lanes fan out to slots `i`, `64+i`,
/// `128+i`, `192+i`; the low-slot store follows the load, and higher slots hold
/// no live input, so the in-place fan-out is safe.
///
/// # Safety
/// Requires `neon`; all loads/stores stay within `a[..256]` (`i < 64`).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn expand8_neon(a: &mut [u32; BLOCK]) {
    // SAFETY: `neon`; all loads/stores stay within `a[..256]` (`i < 64`).
    unsafe {
        let m = vdupq_n_u32(0xFF);
        let p = a.as_mut_ptr();
        let mut i = 0usize;
        while i < 64 {
            let l0 = vld1q_u32(p.add(i));
            let l1 = vld1q_u32(p.add(i + 4));
            vst1q_u32(p.add(i), vandq_u32(vshrq_n_u32::<24>(l0), m));
            vst1q_u32(p.add(i + 4), vandq_u32(vshrq_n_u32::<24>(l1), m));
            vst1q_u32(p.add(64 + i), vandq_u32(vshrq_n_u32::<16>(l0), m));
            vst1q_u32(p.add(64 + i + 4), vandq_u32(vshrq_n_u32::<16>(l1), m));
            vst1q_u32(p.add(128 + i), vandq_u32(vshrq_n_u32::<8>(l0), m));
            vst1q_u32(p.add(128 + i + 4), vandq_u32(vshrq_n_u32::<8>(l1), m));
            vst1q_u32(p.add(192 + i), vandq_u32(l0, m));
            vst1q_u32(p.add(192 + i + 4), vandq_u32(l1, m));
            i += 8;
        }
    }
}

/// NEON [`expand16`].
///
/// # Safety
/// Requires `neon`; all loads/stores stay within `a[..256]` (`i < 128`).
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn expand16_neon(a: &mut [u32; BLOCK]) {
    // SAFETY: `neon`; all loads/stores stay within `a[..256]` (`i < 128`).
    unsafe {
        let m = vdupq_n_u32(0xFFFF);
        let p = a.as_mut_ptr();
        let mut i = 0usize;
        while i < 128 {
            let l0 = vld1q_u32(p.add(i));
            let l1 = vld1q_u32(p.add(i + 4));
            vst1q_u32(p.add(i), vandq_u32(vshrq_n_u32::<16>(l0), m));
            vst1q_u32(p.add(i + 4), vandq_u32(vshrq_n_u32::<16>(l1), m));
            vst1q_u32(p.add(128 + i), vandq_u32(l0, m));
            vst1q_u32(p.add(128 + i + 4), vandq_u32(l1, m));
            i += 8;
        }
    }
}

/// Fused width-8 decode: read each packed word from `bytes` and fan its four byte
/// lanes straight to the 256 expanded slots — one pass, no intermediate copy.
///
/// # Safety
/// Requires `neon`; each load reads 16 bytes within `bytes[..256]` (`i + 8 <= 64`
/// ⇒ byte offset `< 256`), and stores stay within `dest[..256]`.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn expand8_from_bytes(bytes: &[u8], dest: &mut [u32; BLOCK]) {
    // SAFETY: `neon`; each load reads 16 bytes within `bytes[..256]` and stores
    // stay within `dest[..256]` (`i < 64`).
    unsafe {
        debug_assert!(bytes.len() >= 256);
        let m = vdupq_n_u32(0xFF);
        let bp = bytes.as_ptr();
        let dp = dest.as_mut_ptr();
        let mut i = 0usize;
        while i < 64 {
            let l0 = vld1q_u32(bp.add(i * 4).cast::<u32>());
            let l1 = vld1q_u32(bp.add((i + 4) * 4).cast::<u32>());
            vst1q_u32(dp.add(i), vandq_u32(vshrq_n_u32::<24>(l0), m));
            vst1q_u32(dp.add(i + 4), vandq_u32(vshrq_n_u32::<24>(l1), m));
            vst1q_u32(dp.add(64 + i), vandq_u32(vshrq_n_u32::<16>(l0), m));
            vst1q_u32(dp.add(64 + i + 4), vandq_u32(vshrq_n_u32::<16>(l1), m));
            vst1q_u32(dp.add(128 + i), vandq_u32(vshrq_n_u32::<8>(l0), m));
            vst1q_u32(dp.add(128 + i + 4), vandq_u32(vshrq_n_u32::<8>(l1), m));
            vst1q_u32(dp.add(192 + i), vandq_u32(l0, m));
            vst1q_u32(dp.add(192 + i + 4), vandq_u32(l1, m));
            i += 8;
        }
    }
}

/// Fused width-16 decode: read each packed word from `bytes` and fan its two
/// 16-bit lanes straight to the 256 expanded slots — one pass.
///
/// # Safety
/// Requires `neon`; each load reads 16 bytes within `bytes[..512]` (`i + 8 <= 128`
/// ⇒ byte offset `< 512`), and stores stay within `dest[..256]`.
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
unsafe fn expand16_from_bytes(bytes: &[u8], dest: &mut [u32; BLOCK]) {
    // SAFETY: `neon`; each load reads 16 bytes within `bytes[..512]` and stores
    // stay within `dest[..256]` (`i < 128`).
    unsafe {
        debug_assert!(bytes.len() >= 512);
        let m = vdupq_n_u32(0xFFFF);
        let bp = bytes.as_ptr();
        let dp = dest.as_mut_ptr();
        let mut i = 0usize;
        while i < 128 {
            let l0 = vld1q_u32(bp.add(i * 4).cast::<u32>());
            let l1 = vld1q_u32(bp.add((i + 4) * 4).cast::<u32>());
            vst1q_u32(dp.add(i), vandq_u32(vshrq_n_u32::<16>(l0), m));
            vst1q_u32(dp.add(i + 4), vandq_u32(vshrq_n_u32::<16>(l1), m));
            vst1q_u32(dp.add(128 + i), vandq_u32(l0, m));
            vst1q_u32(dp.add(128 + i + 4), vandq_u32(l1, m));
            i += 8;
        }
    }
}

/// Decode-throughput A/B vs the crate's `BitPacker8x` (`--ignored --nocapture`,
/// release). Isolates the bit-unpack primitive across widths: on `aarch64` the
/// crate side is hand-NEON, so a competitive autovec `unpack` here says the
/// scalar layout vectorizes well enough to need no hand intrinsics.
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
    /// vs `BitPacker8x::decompress_sorted` (the fused SIMD unpack+integrate we
    /// replaced). This is the doc-id decode path COUNT/ranked hammer — the metric
    /// the delta-integrate work targets.
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

        /// The per-arch dispatched decoder is byte-identical to the scalar
        /// reference at every width — the correctness gate for the SIMD kernels.
        #[test]
        fn dispatched_matches_scalar(
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
