// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

// GENERATED CODE — do not edit by hand. See tools that emit this file.
//
// Const-unrolled AVX2 unpack, one kernel per bit width. A `__m256i` is exactly one
// 8-lane value-register, so each kernel loads one packed word, shifts to the
// in-word offset, masks, and OR-s in the straddle from the next word — every shift
// a compile-time constant. Mirrors the NEON kernel and the scalar reference.


#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w1(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w2(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w3(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w4(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w5(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w6(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w7(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w8(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w9(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w10(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w11(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w12(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w13(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w14(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w15(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w16(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    _mm256_storeu_si256(dp.add(29), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w17(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w18(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w19(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w20(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w21(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<19>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w22(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w23(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<21>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<19>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w24(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    _mm256_storeu_si256(dp.add(19), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    _mm256_storeu_si256(dp.add(27), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w25(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<21>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<23>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<19>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w26(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(25));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w27(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<25>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<23>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<21>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<26>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<19>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(25));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(26));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w28(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    _mm256_storeu_si256(dp.add(7), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    _mm256_storeu_si256(dp.add(23), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(25));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(26));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(27));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w29(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<21>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<27>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<19>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<25>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<28>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(25));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(26));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    w = _mm256_loadu_si256(ip.add(27));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<23>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    w = _mm256_loadu_si256(ip.add(28));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<26>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w30(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<26>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<28>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    _mm256_storeu_si256(dp.add(15), o);
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(25));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(26));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(27));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    w = _mm256_loadu_si256(ip.add(28));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<26>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(29));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<28>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_w31(ip: *const __m256i, dp: *mut __m256i, mask: __m256i) {
    // SAFETY: caller guarantees `ip` holds packed_len bytes and `dp`
    // addresses dest[..256]; AVX2 is enabled by the caller.
    unsafe {
    let mut w = _mm256_loadu_si256(ip.add(0));
    let o = _mm256_and_si256(w, mask);
    _mm256_storeu_si256(dp.add(0), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<31>(w), mask);
    w = _mm256_loadu_si256(ip.add(1));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<1>(w), mask));
    _mm256_storeu_si256(dp.add(1), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<30>(w), mask);
    w = _mm256_loadu_si256(ip.add(2));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<2>(w), mask));
    _mm256_storeu_si256(dp.add(2), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<29>(w), mask);
    w = _mm256_loadu_si256(ip.add(3));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<3>(w), mask));
    _mm256_storeu_si256(dp.add(3), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<28>(w), mask);
    w = _mm256_loadu_si256(ip.add(4));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<4>(w), mask));
    _mm256_storeu_si256(dp.add(4), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<27>(w), mask);
    w = _mm256_loadu_si256(ip.add(5));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<5>(w), mask));
    _mm256_storeu_si256(dp.add(5), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<26>(w), mask);
    w = _mm256_loadu_si256(ip.add(6));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<6>(w), mask));
    _mm256_storeu_si256(dp.add(6), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<25>(w), mask);
    w = _mm256_loadu_si256(ip.add(7));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<7>(w), mask));
    _mm256_storeu_si256(dp.add(7), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<24>(w), mask);
    w = _mm256_loadu_si256(ip.add(8));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<8>(w), mask));
    _mm256_storeu_si256(dp.add(8), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<23>(w), mask);
    w = _mm256_loadu_si256(ip.add(9));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<9>(w), mask));
    _mm256_storeu_si256(dp.add(9), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<22>(w), mask);
    w = _mm256_loadu_si256(ip.add(10));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<10>(w), mask));
    _mm256_storeu_si256(dp.add(10), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<21>(w), mask);
    w = _mm256_loadu_si256(ip.add(11));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<11>(w), mask));
    _mm256_storeu_si256(dp.add(11), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<20>(w), mask);
    w = _mm256_loadu_si256(ip.add(12));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<12>(w), mask));
    _mm256_storeu_si256(dp.add(12), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<19>(w), mask);
    w = _mm256_loadu_si256(ip.add(13));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<13>(w), mask));
    _mm256_storeu_si256(dp.add(13), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<18>(w), mask);
    w = _mm256_loadu_si256(ip.add(14));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<14>(w), mask));
    _mm256_storeu_si256(dp.add(14), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<17>(w), mask);
    w = _mm256_loadu_si256(ip.add(15));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<15>(w), mask));
    _mm256_storeu_si256(dp.add(15), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<16>(w), mask);
    w = _mm256_loadu_si256(ip.add(16));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<16>(w), mask));
    _mm256_storeu_si256(dp.add(16), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<15>(w), mask);
    w = _mm256_loadu_si256(ip.add(17));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<17>(w), mask));
    _mm256_storeu_si256(dp.add(17), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<14>(w), mask);
    w = _mm256_loadu_si256(ip.add(18));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<18>(w), mask));
    _mm256_storeu_si256(dp.add(18), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<13>(w), mask);
    w = _mm256_loadu_si256(ip.add(19));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<19>(w), mask));
    _mm256_storeu_si256(dp.add(19), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<12>(w), mask);
    w = _mm256_loadu_si256(ip.add(20));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<20>(w), mask));
    _mm256_storeu_si256(dp.add(20), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<11>(w), mask);
    w = _mm256_loadu_si256(ip.add(21));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<21>(w), mask));
    _mm256_storeu_si256(dp.add(21), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<10>(w), mask);
    w = _mm256_loadu_si256(ip.add(22));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<22>(w), mask));
    _mm256_storeu_si256(dp.add(22), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<9>(w), mask);
    w = _mm256_loadu_si256(ip.add(23));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<23>(w), mask));
    _mm256_storeu_si256(dp.add(23), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<8>(w), mask);
    w = _mm256_loadu_si256(ip.add(24));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<24>(w), mask));
    _mm256_storeu_si256(dp.add(24), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<7>(w), mask);
    w = _mm256_loadu_si256(ip.add(25));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<25>(w), mask));
    _mm256_storeu_si256(dp.add(25), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<6>(w), mask);
    w = _mm256_loadu_si256(ip.add(26));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<26>(w), mask));
    _mm256_storeu_si256(dp.add(26), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<5>(w), mask);
    w = _mm256_loadu_si256(ip.add(27));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<27>(w), mask));
    _mm256_storeu_si256(dp.add(27), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<4>(w), mask);
    w = _mm256_loadu_si256(ip.add(28));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<28>(w), mask));
    _mm256_storeu_si256(dp.add(28), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<3>(w), mask);
    w = _mm256_loadu_si256(ip.add(29));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<29>(w), mask));
    _mm256_storeu_si256(dp.add(29), o);
    let mut o = _mm256_and_si256(_mm256_srli_epi32::<2>(w), mask);
    w = _mm256_loadu_si256(ip.add(30));
    o = _mm256_or_si256(o, _mm256_and_si256(_mm256_slli_epi32::<30>(w), mask));
    _mm256_storeu_si256(dp.add(30), o);
    let o = _mm256_and_si256(_mm256_srli_epi32::<1>(w), mask);
    _mm256_storeu_si256(dp.add(31), o);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn unpack_avx2_unrolled(ip: *const __m256i, dp: *mut __m256i, bits: u8, mask: __m256i) {
    // SAFETY: forwarded to the per-width kernel; see `unpack`.
    unsafe {
        match bits {
            1 => unpack_avx2_w1(ip, dp, mask),
            2 => unpack_avx2_w2(ip, dp, mask),
            3 => unpack_avx2_w3(ip, dp, mask),
            4 => unpack_avx2_w4(ip, dp, mask),
            5 => unpack_avx2_w5(ip, dp, mask),
            6 => unpack_avx2_w6(ip, dp, mask),
            7 => unpack_avx2_w7(ip, dp, mask),
            8 => unpack_avx2_w8(ip, dp, mask),
            9 => unpack_avx2_w9(ip, dp, mask),
            10 => unpack_avx2_w10(ip, dp, mask),
            11 => unpack_avx2_w11(ip, dp, mask),
            12 => unpack_avx2_w12(ip, dp, mask),
            13 => unpack_avx2_w13(ip, dp, mask),
            14 => unpack_avx2_w14(ip, dp, mask),
            15 => unpack_avx2_w15(ip, dp, mask),
            16 => unpack_avx2_w16(ip, dp, mask),
            17 => unpack_avx2_w17(ip, dp, mask),
            18 => unpack_avx2_w18(ip, dp, mask),
            19 => unpack_avx2_w19(ip, dp, mask),
            20 => unpack_avx2_w20(ip, dp, mask),
            21 => unpack_avx2_w21(ip, dp, mask),
            22 => unpack_avx2_w22(ip, dp, mask),
            23 => unpack_avx2_w23(ip, dp, mask),
            24 => unpack_avx2_w24(ip, dp, mask),
            25 => unpack_avx2_w25(ip, dp, mask),
            26 => unpack_avx2_w26(ip, dp, mask),
            27 => unpack_avx2_w27(ip, dp, mask),
            28 => unpack_avx2_w28(ip, dp, mask),
            29 => unpack_avx2_w29(ip, dp, mask),
            30 => unpack_avx2_w30(ip, dp, mask),
            31 => unpack_avx2_w31(ip, dp, mask),
            _ => unreachable!("bits 1..=31"),
        }
    }
}
