// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors
//
// GENERATED CODE - do not edit by hand. Branchless fixed-shift stitch kernels,
// one per non-dividing width, reassembling the straddle values spanning two packed
// words. Included into the parent module (BLOCK, stitch fns share its scope); the
// `tmp` argument holds the leftover-masked packed words. A `<< 0` is left as-is to
// keep the kernels a verbatim, regenerable transcription of the width formula.

#[inline]
fn stitch_3(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 48usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 1;
        l0 |= (tmp[ti + 1] >> 1) & 0x01010101;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x01010101) << 2;
        l1 |= tmp[ti + 2] << 0;
        dest[ii + 1] = l1;
        ti += 3;
        ii += 2;
    }
}

#[inline]
fn stitch_5(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 40usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 2;
        l0 |= (tmp[ti + 1] >> 1) & 0x03030303;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x01010101) << 4;
        l1 |= tmp[ti + 2] << 1;
        l1 |= (tmp[ti + 3] >> 2) & 0x01010101;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 3] & 0x03030303) << 3;
        l2 |= tmp[ti + 4] << 0;
        dest[ii + 2] = l2;
        ti += 5;
        ii += 3;
    }
}

#[inline]
fn stitch_6(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 48usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 4;
        l0 |= tmp[ti + 1] << 2;
        l0 |= tmp[ti + 2] << 0;
        dest[ii + 0] = l0;
        ti += 3;
        ii += 1;
    }
}

#[inline]
fn stitch_7(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 56usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 6;
        l0 |= tmp[ti + 1] << 5;
        l0 |= tmp[ti + 2] << 4;
        l0 |= tmp[ti + 3] << 3;
        l0 |= tmp[ti + 4] << 2;
        l0 |= tmp[ti + 5] << 1;
        l0 |= tmp[ti + 6] << 0;
        dest[ii + 0] = l0;
        ti += 7;
        ii += 1;
    }
}

#[inline]
fn stitch_9(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 72usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 2;
        l0 |= (tmp[ti + 1] >> 5) & 0x00030003;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x001f001f) << 4;
        l1 |= (tmp[ti + 2] >> 3) & 0x000f000f;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 2] & 0x00070007) << 6;
        l2 |= (tmp[ti + 3] >> 1) & 0x003f003f;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 3] & 0x00010001) << 8;
        l3 |= tmp[ti + 4] << 1;
        l3 |= (tmp[ti + 5] >> 6) & 0x00010001;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 5] & 0x003f003f) << 3;
        l4 |= (tmp[ti + 6] >> 4) & 0x00070007;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 6] & 0x000f000f) << 5;
        l5 |= (tmp[ti + 7] >> 2) & 0x001f001f;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 7] & 0x00030003) << 7;
        l6 |= tmp[ti + 8] << 0;
        dest[ii + 6] = l6;
        ti += 9;
        ii += 7;
    }
}

#[inline]
fn stitch_10(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 80usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 4;
        l0 |= (tmp[ti + 1] >> 2) & 0x000f000f;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x00030003) << 8;
        l1 |= tmp[ti + 2] << 2;
        l1 |= (tmp[ti + 3] >> 4) & 0x00030003;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 3] & 0x000f000f) << 6;
        l2 |= tmp[ti + 4] << 0;
        dest[ii + 2] = l2;
        ti += 5;
        ii += 3;
    }
}

#[inline]
fn stitch_11(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 88usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 6;
        l0 |= tmp[ti + 1] << 1;
        l0 |= (tmp[ti + 2] >> 4) & 0x00010001;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 2] & 0x000f000f) << 7;
        l1 |= tmp[ti + 3] << 2;
        l1 |= (tmp[ti + 4] >> 3) & 0x00030003;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 4] & 0x00070007) << 8;
        l2 |= tmp[ti + 5] << 3;
        l2 |= (tmp[ti + 6] >> 2) & 0x00070007;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 6] & 0x00030003) << 9;
        l3 |= tmp[ti + 7] << 4;
        l3 |= (tmp[ti + 8] >> 1) & 0x000f000f;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 8] & 0x00010001) << 10;
        l4 |= tmp[ti + 9] << 5;
        l4 |= tmp[ti + 10] << 0;
        dest[ii + 4] = l4;
        ti += 11;
        ii += 5;
    }
}

#[inline]
fn stitch_12(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 96usize;
    for _ in 0..32 {
        let mut l0 = tmp[ti + 0] << 8;
        l0 |= tmp[ti + 1] << 4;
        l0 |= tmp[ti + 2] << 0;
        dest[ii + 0] = l0;
        ti += 3;
        ii += 1;
    }
}

#[inline]
fn stitch_13(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 104usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 10;
        l0 |= tmp[ti + 1] << 7;
        l0 |= tmp[ti + 2] << 4;
        l0 |= tmp[ti + 3] << 1;
        l0 |= (tmp[ti + 4] >> 2) & 0x00010001;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 4] & 0x00030003) << 11;
        l1 |= tmp[ti + 5] << 8;
        l1 |= tmp[ti + 6] << 5;
        l1 |= tmp[ti + 7] << 2;
        l1 |= (tmp[ti + 8] >> 1) & 0x00030003;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 8] & 0x00010001) << 12;
        l2 |= tmp[ti + 9] << 9;
        l2 |= tmp[ti + 10] << 6;
        l2 |= tmp[ti + 11] << 3;
        l2 |= tmp[ti + 12] << 0;
        dest[ii + 2] = l2;
        ti += 13;
        ii += 3;
    }
}

#[inline]
fn stitch_14(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 112usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 12;
        l0 |= tmp[ti + 1] << 10;
        l0 |= tmp[ti + 2] << 8;
        l0 |= tmp[ti + 3] << 6;
        l0 |= tmp[ti + 4] << 4;
        l0 |= tmp[ti + 5] << 2;
        l0 |= tmp[ti + 6] << 0;
        dest[ii + 0] = l0;
        ti += 7;
        ii += 1;
    }
}

#[inline]
fn stitch_15(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 120usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 14;
        l0 |= tmp[ti + 1] << 13;
        l0 |= tmp[ti + 2] << 12;
        l0 |= tmp[ti + 3] << 11;
        l0 |= tmp[ti + 4] << 10;
        l0 |= tmp[ti + 5] << 9;
        l0 |= tmp[ti + 6] << 8;
        l0 |= tmp[ti + 7] << 7;
        l0 |= tmp[ti + 8] << 6;
        l0 |= tmp[ti + 9] << 5;
        l0 |= tmp[ti + 10] << 4;
        l0 |= tmp[ti + 11] << 3;
        l0 |= tmp[ti + 12] << 2;
        l0 |= tmp[ti + 13] << 1;
        l0 |= tmp[ti + 14] << 0;
        dest[ii + 0] = l0;
        ti += 15;
        ii += 1;
    }
}

#[inline]
fn stitch_17(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 136usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 2;
        l0 |= (tmp[ti + 1] >> 13) & 0x00000003;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x00001fff) << 4;
        l1 |= (tmp[ti + 2] >> 11) & 0x0000000f;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 2] & 0x000007ff) << 6;
        l2 |= (tmp[ti + 3] >> 9) & 0x0000003f;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 3] & 0x000001ff) << 8;
        l3 |= (tmp[ti + 4] >> 7) & 0x000000ff;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 4] & 0x0000007f) << 10;
        l4 |= (tmp[ti + 5] >> 5) & 0x000003ff;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 5] & 0x0000001f) << 12;
        l5 |= (tmp[ti + 6] >> 3) & 0x00000fff;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 6] & 0x00000007) << 14;
        l6 |= (tmp[ti + 7] >> 1) & 0x00003fff;
        dest[ii + 6] = l6;
        let mut l7 = (tmp[ti + 7] & 0x00000001) << 16;
        l7 |= tmp[ti + 8] << 1;
        l7 |= (tmp[ti + 9] >> 14) & 0x00000001;
        dest[ii + 7] = l7;
        let mut l8 = (tmp[ti + 9] & 0x00003fff) << 3;
        l8 |= (tmp[ti + 10] >> 12) & 0x00000007;
        dest[ii + 8] = l8;
        let mut l9 = (tmp[ti + 10] & 0x00000fff) << 5;
        l9 |= (tmp[ti + 11] >> 10) & 0x0000001f;
        dest[ii + 9] = l9;
        let mut l10 = (tmp[ti + 11] & 0x000003ff) << 7;
        l10 |= (tmp[ti + 12] >> 8) & 0x0000007f;
        dest[ii + 10] = l10;
        let mut l11 = (tmp[ti + 12] & 0x000000ff) << 9;
        l11 |= (tmp[ti + 13] >> 6) & 0x000001ff;
        dest[ii + 11] = l11;
        let mut l12 = (tmp[ti + 13] & 0x0000003f) << 11;
        l12 |= (tmp[ti + 14] >> 4) & 0x000007ff;
        dest[ii + 12] = l12;
        let mut l13 = (tmp[ti + 14] & 0x0000000f) << 13;
        l13 |= (tmp[ti + 15] >> 2) & 0x00001fff;
        dest[ii + 13] = l13;
        let mut l14 = (tmp[ti + 15] & 0x00000003) << 15;
        l14 |= tmp[ti + 16] << 0;
        dest[ii + 14] = l14;
        ti += 17;
        ii += 15;
    }
}

#[inline]
fn stitch_18(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 144usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 4;
        l0 |= (tmp[ti + 1] >> 10) & 0x0000000f;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x000003ff) << 8;
        l1 |= (tmp[ti + 2] >> 6) & 0x000000ff;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 2] & 0x0000003f) << 12;
        l2 |= (tmp[ti + 3] >> 2) & 0x00000fff;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 3] & 0x00000003) << 16;
        l3 |= tmp[ti + 4] << 2;
        l3 |= (tmp[ti + 5] >> 12) & 0x00000003;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 5] & 0x00000fff) << 6;
        l4 |= (tmp[ti + 6] >> 8) & 0x0000003f;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 6] & 0x000000ff) << 10;
        l5 |= (tmp[ti + 7] >> 4) & 0x000003ff;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 7] & 0x0000000f) << 14;
        l6 |= tmp[ti + 8] << 0;
        dest[ii + 6] = l6;
        ti += 9;
        ii += 7;
    }
}

#[inline]
fn stitch_19(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 152usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 6;
        l0 |= (tmp[ti + 1] >> 7) & 0x0000003f;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x0000007f) << 12;
        l1 |= (tmp[ti + 2] >> 1) & 0x00000fff;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 2] & 0x00000001) << 18;
        l2 |= tmp[ti + 3] << 5;
        l2 |= (tmp[ti + 4] >> 8) & 0x0000001f;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 4] & 0x000000ff) << 11;
        l3 |= (tmp[ti + 5] >> 2) & 0x000007ff;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 5] & 0x00000003) << 17;
        l4 |= tmp[ti + 6] << 4;
        l4 |= (tmp[ti + 7] >> 9) & 0x0000000f;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 7] & 0x000001ff) << 10;
        l5 |= (tmp[ti + 8] >> 3) & 0x000003ff;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 8] & 0x00000007) << 16;
        l6 |= tmp[ti + 9] << 3;
        l6 |= (tmp[ti + 10] >> 10) & 0x00000007;
        dest[ii + 6] = l6;
        let mut l7 = (tmp[ti + 10] & 0x000003ff) << 9;
        l7 |= (tmp[ti + 11] >> 4) & 0x000001ff;
        dest[ii + 7] = l7;
        let mut l8 = (tmp[ti + 11] & 0x0000000f) << 15;
        l8 |= tmp[ti + 12] << 2;
        l8 |= (tmp[ti + 13] >> 11) & 0x00000003;
        dest[ii + 8] = l8;
        let mut l9 = (tmp[ti + 13] & 0x000007ff) << 8;
        l9 |= (tmp[ti + 14] >> 5) & 0x000000ff;
        dest[ii + 9] = l9;
        let mut l10 = (tmp[ti + 14] & 0x0000001f) << 14;
        l10 |= tmp[ti + 15] << 1;
        l10 |= (tmp[ti + 16] >> 12) & 0x00000001;
        dest[ii + 10] = l10;
        let mut l11 = (tmp[ti + 16] & 0x00000fff) << 7;
        l11 |= (tmp[ti + 17] >> 6) & 0x0000007f;
        dest[ii + 11] = l11;
        let mut l12 = (tmp[ti + 17] & 0x0000003f) << 13;
        l12 |= tmp[ti + 18] << 0;
        dest[ii + 12] = l12;
        ti += 19;
        ii += 13;
    }
}

#[inline]
fn stitch_20(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 160usize;
    for _ in 0..32 {
        let mut l0 = tmp[ti + 0] << 8;
        l0 |= (tmp[ti + 1] >> 4) & 0x000000ff;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x0000000f) << 16;
        l1 |= tmp[ti + 2] << 4;
        l1 |= (tmp[ti + 3] >> 8) & 0x0000000f;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 3] & 0x000000ff) << 12;
        l2 |= tmp[ti + 4] << 0;
        dest[ii + 2] = l2;
        ti += 5;
        ii += 3;
    }
}

#[inline]
fn stitch_21(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 168usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 10;
        l0 |= (tmp[ti + 1] >> 1) & 0x000003ff;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 1] & 0x00000001) << 20;
        l1 |= tmp[ti + 2] << 9;
        l1 |= (tmp[ti + 3] >> 2) & 0x000001ff;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 3] & 0x00000003) << 19;
        l2 |= tmp[ti + 4] << 8;
        l2 |= (tmp[ti + 5] >> 3) & 0x000000ff;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 5] & 0x00000007) << 18;
        l3 |= tmp[ti + 6] << 7;
        l3 |= (tmp[ti + 7] >> 4) & 0x0000007f;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 7] & 0x0000000f) << 17;
        l4 |= tmp[ti + 8] << 6;
        l4 |= (tmp[ti + 9] >> 5) & 0x0000003f;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 9] & 0x0000001f) << 16;
        l5 |= tmp[ti + 10] << 5;
        l5 |= (tmp[ti + 11] >> 6) & 0x0000001f;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 11] & 0x0000003f) << 15;
        l6 |= tmp[ti + 12] << 4;
        l6 |= (tmp[ti + 13] >> 7) & 0x0000000f;
        dest[ii + 6] = l6;
        let mut l7 = (tmp[ti + 13] & 0x0000007f) << 14;
        l7 |= tmp[ti + 14] << 3;
        l7 |= (tmp[ti + 15] >> 8) & 0x00000007;
        dest[ii + 7] = l7;
        let mut l8 = (tmp[ti + 15] & 0x000000ff) << 13;
        l8 |= tmp[ti + 16] << 2;
        l8 |= (tmp[ti + 17] >> 9) & 0x00000003;
        dest[ii + 8] = l8;
        let mut l9 = (tmp[ti + 17] & 0x000001ff) << 12;
        l9 |= tmp[ti + 18] << 1;
        l9 |= (tmp[ti + 19] >> 10) & 0x00000001;
        dest[ii + 9] = l9;
        let mut l10 = (tmp[ti + 19] & 0x000003ff) << 11;
        l10 |= tmp[ti + 20] << 0;
        dest[ii + 10] = l10;
        ti += 21;
        ii += 11;
    }
}

#[inline]
fn stitch_22(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 176usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 12;
        l0 |= tmp[ti + 1] << 2;
        l0 |= (tmp[ti + 2] >> 8) & 0x00000003;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 2] & 0x000000ff) << 14;
        l1 |= tmp[ti + 3] << 4;
        l1 |= (tmp[ti + 4] >> 6) & 0x0000000f;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 4] & 0x0000003f) << 16;
        l2 |= tmp[ti + 5] << 6;
        l2 |= (tmp[ti + 6] >> 4) & 0x0000003f;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 6] & 0x0000000f) << 18;
        l3 |= tmp[ti + 7] << 8;
        l3 |= (tmp[ti + 8] >> 2) & 0x000000ff;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 8] & 0x00000003) << 20;
        l4 |= tmp[ti + 9] << 10;
        l4 |= tmp[ti + 10] << 0;
        dest[ii + 4] = l4;
        ti += 11;
        ii += 5;
    }
}

#[inline]
fn stitch_23(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 184usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 14;
        l0 |= tmp[ti + 1] << 5;
        l0 |= (tmp[ti + 2] >> 4) & 0x0000001f;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 2] & 0x0000000f) << 19;
        l1 |= tmp[ti + 3] << 10;
        l1 |= tmp[ti + 4] << 1;
        l1 |= (tmp[ti + 5] >> 8) & 0x00000001;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 5] & 0x000000ff) << 15;
        l2 |= tmp[ti + 6] << 6;
        l2 |= (tmp[ti + 7] >> 3) & 0x0000003f;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 7] & 0x00000007) << 20;
        l3 |= tmp[ti + 8] << 11;
        l3 |= tmp[ti + 9] << 2;
        l3 |= (tmp[ti + 10] >> 7) & 0x00000003;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 10] & 0x0000007f) << 16;
        l4 |= tmp[ti + 11] << 7;
        l4 |= (tmp[ti + 12] >> 2) & 0x0000007f;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 12] & 0x00000003) << 21;
        l5 |= tmp[ti + 13] << 12;
        l5 |= tmp[ti + 14] << 3;
        l5 |= (tmp[ti + 15] >> 6) & 0x00000007;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 15] & 0x0000003f) << 17;
        l6 |= tmp[ti + 16] << 8;
        l6 |= (tmp[ti + 17] >> 1) & 0x000000ff;
        dest[ii + 6] = l6;
        let mut l7 = (tmp[ti + 17] & 0x00000001) << 22;
        l7 |= tmp[ti + 18] << 13;
        l7 |= tmp[ti + 19] << 4;
        l7 |= (tmp[ti + 20] >> 5) & 0x0000000f;
        dest[ii + 7] = l7;
        let mut l8 = (tmp[ti + 20] & 0x0000001f) << 18;
        l8 |= tmp[ti + 21] << 9;
        l8 |= tmp[ti + 22] << 0;
        dest[ii + 8] = l8;
        ti += 23;
        ii += 9;
    }
}

#[inline]
fn stitch_24(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 192usize;
    for _ in 0..64 {
        let mut l0 = tmp[ti + 0] << 16;
        l0 |= tmp[ti + 1] << 8;
        l0 |= tmp[ti + 2] << 0;
        dest[ii + 0] = l0;
        ti += 3;
        ii += 1;
    }
}

#[inline]
fn stitch_25(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 200usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 18;
        l0 |= tmp[ti + 1] << 11;
        l0 |= tmp[ti + 2] << 4;
        l0 |= (tmp[ti + 3] >> 3) & 0x0000000f;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 3] & 0x00000007) << 22;
        l1 |= tmp[ti + 4] << 15;
        l1 |= tmp[ti + 5] << 8;
        l1 |= tmp[ti + 6] << 1;
        l1 |= (tmp[ti + 7] >> 6) & 0x00000001;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 7] & 0x0000003f) << 19;
        l2 |= tmp[ti + 8] << 12;
        l2 |= tmp[ti + 9] << 5;
        l2 |= (tmp[ti + 10] >> 2) & 0x0000001f;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 10] & 0x00000003) << 23;
        l3 |= tmp[ti + 11] << 16;
        l3 |= tmp[ti + 12] << 9;
        l3 |= tmp[ti + 13] << 2;
        l3 |= (tmp[ti + 14] >> 5) & 0x00000003;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 14] & 0x0000001f) << 20;
        l4 |= tmp[ti + 15] << 13;
        l4 |= tmp[ti + 16] << 6;
        l4 |= (tmp[ti + 17] >> 1) & 0x0000003f;
        dest[ii + 4] = l4;
        let mut l5 = (tmp[ti + 17] & 0x00000001) << 24;
        l5 |= tmp[ti + 18] << 17;
        l5 |= tmp[ti + 19] << 10;
        l5 |= tmp[ti + 20] << 3;
        l5 |= (tmp[ti + 21] >> 4) & 0x00000007;
        dest[ii + 5] = l5;
        let mut l6 = (tmp[ti + 21] & 0x0000000f) << 21;
        l6 |= tmp[ti + 22] << 14;
        l6 |= tmp[ti + 23] << 7;
        l6 |= tmp[ti + 24] << 0;
        dest[ii + 6] = l6;
        ti += 25;
        ii += 7;
    }
}

#[inline]
fn stitch_26(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 208usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 20;
        l0 |= tmp[ti + 1] << 14;
        l0 |= tmp[ti + 2] << 8;
        l0 |= tmp[ti + 3] << 2;
        l0 |= (tmp[ti + 4] >> 4) & 0x00000003;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 4] & 0x0000000f) << 22;
        l1 |= tmp[ti + 5] << 16;
        l1 |= tmp[ti + 6] << 10;
        l1 |= tmp[ti + 7] << 4;
        l1 |= (tmp[ti + 8] >> 2) & 0x0000000f;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 8] & 0x00000003) << 24;
        l2 |= tmp[ti + 9] << 18;
        l2 |= tmp[ti + 10] << 12;
        l2 |= tmp[ti + 11] << 6;
        l2 |= tmp[ti + 12] << 0;
        dest[ii + 2] = l2;
        ti += 13;
        ii += 3;
    }
}

#[inline]
fn stitch_27(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 216usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 22;
        l0 |= tmp[ti + 1] << 17;
        l0 |= tmp[ti + 2] << 12;
        l0 |= tmp[ti + 3] << 7;
        l0 |= tmp[ti + 4] << 2;
        l0 |= (tmp[ti + 5] >> 3) & 0x00000003;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 5] & 0x00000007) << 24;
        l1 |= tmp[ti + 6] << 19;
        l1 |= tmp[ti + 7] << 14;
        l1 |= tmp[ti + 8] << 9;
        l1 |= tmp[ti + 9] << 4;
        l1 |= (tmp[ti + 10] >> 1) & 0x0000000f;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 10] & 0x00000001) << 26;
        l2 |= tmp[ti + 11] << 21;
        l2 |= tmp[ti + 12] << 16;
        l2 |= tmp[ti + 13] << 11;
        l2 |= tmp[ti + 14] << 6;
        l2 |= tmp[ti + 15] << 1;
        l2 |= (tmp[ti + 16] >> 4) & 0x00000001;
        dest[ii + 2] = l2;
        let mut l3 = (tmp[ti + 16] & 0x0000000f) << 23;
        l3 |= tmp[ti + 17] << 18;
        l3 |= tmp[ti + 18] << 13;
        l3 |= tmp[ti + 19] << 8;
        l3 |= tmp[ti + 20] << 3;
        l3 |= (tmp[ti + 21] >> 2) & 0x00000007;
        dest[ii + 3] = l3;
        let mut l4 = (tmp[ti + 21] & 0x00000003) << 25;
        l4 |= tmp[ti + 22] << 20;
        l4 |= tmp[ti + 23] << 15;
        l4 |= tmp[ti + 24] << 10;
        l4 |= tmp[ti + 25] << 5;
        l4 |= tmp[ti + 26] << 0;
        dest[ii + 4] = l4;
        ti += 27;
        ii += 5;
    }
}

#[inline]
fn stitch_28(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 224usize;
    for _ in 0..32 {
        let mut l0 = tmp[ti + 0] << 24;
        l0 |= tmp[ti + 1] << 20;
        l0 |= tmp[ti + 2] << 16;
        l0 |= tmp[ti + 3] << 12;
        l0 |= tmp[ti + 4] << 8;
        l0 |= tmp[ti + 5] << 4;
        l0 |= tmp[ti + 6] << 0;
        dest[ii + 0] = l0;
        ti += 7;
        ii += 1;
    }
}

#[inline]
fn stitch_29(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 232usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 26;
        l0 |= tmp[ti + 1] << 23;
        l0 |= tmp[ti + 2] << 20;
        l0 |= tmp[ti + 3] << 17;
        l0 |= tmp[ti + 4] << 14;
        l0 |= tmp[ti + 5] << 11;
        l0 |= tmp[ti + 6] << 8;
        l0 |= tmp[ti + 7] << 5;
        l0 |= tmp[ti + 8] << 2;
        l0 |= (tmp[ti + 9] >> 1) & 0x00000003;
        dest[ii + 0] = l0;
        let mut l1 = (tmp[ti + 9] & 0x00000001) << 28;
        l1 |= tmp[ti + 10] << 25;
        l1 |= tmp[ti + 11] << 22;
        l1 |= tmp[ti + 12] << 19;
        l1 |= tmp[ti + 13] << 16;
        l1 |= tmp[ti + 14] << 13;
        l1 |= tmp[ti + 15] << 10;
        l1 |= tmp[ti + 16] << 7;
        l1 |= tmp[ti + 17] << 4;
        l1 |= tmp[ti + 18] << 1;
        l1 |= (tmp[ti + 19] >> 2) & 0x00000001;
        dest[ii + 1] = l1;
        let mut l2 = (tmp[ti + 19] & 0x00000003) << 27;
        l2 |= tmp[ti + 20] << 24;
        l2 |= tmp[ti + 21] << 21;
        l2 |= tmp[ti + 22] << 18;
        l2 |= tmp[ti + 23] << 15;
        l2 |= tmp[ti + 24] << 12;
        l2 |= tmp[ti + 25] << 9;
        l2 |= tmp[ti + 26] << 6;
        l2 |= tmp[ti + 27] << 3;
        l2 |= tmp[ti + 28] << 0;
        dest[ii + 2] = l2;
        ti += 29;
        ii += 3;
    }
}

#[inline]
fn stitch_30(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 240usize;
    for _ in 0..16 {
        let mut l0 = tmp[ti + 0] << 28;
        l0 |= tmp[ti + 1] << 26;
        l0 |= tmp[ti + 2] << 24;
        l0 |= tmp[ti + 3] << 22;
        l0 |= tmp[ti + 4] << 20;
        l0 |= tmp[ti + 5] << 18;
        l0 |= tmp[ti + 6] << 16;
        l0 |= tmp[ti + 7] << 14;
        l0 |= tmp[ti + 8] << 12;
        l0 |= tmp[ti + 9] << 10;
        l0 |= tmp[ti + 10] << 8;
        l0 |= tmp[ti + 11] << 6;
        l0 |= tmp[ti + 12] << 4;
        l0 |= tmp[ti + 13] << 2;
        l0 |= tmp[ti + 14] << 0;
        dest[ii + 0] = l0;
        ti += 15;
        ii += 1;
    }
}

#[inline]
fn stitch_31(tmp: &[u32], dest: &mut [u32; BLOCK]) {
    let mut ti = 0usize;
    let mut ii = 248usize;
    for _ in 0..8 {
        let mut l0 = tmp[ti + 0] << 30;
        l0 |= tmp[ti + 1] << 29;
        l0 |= tmp[ti + 2] << 28;
        l0 |= tmp[ti + 3] << 27;
        l0 |= tmp[ti + 4] << 26;
        l0 |= tmp[ti + 5] << 25;
        l0 |= tmp[ti + 6] << 24;
        l0 |= tmp[ti + 7] << 23;
        l0 |= tmp[ti + 8] << 22;
        l0 |= tmp[ti + 9] << 21;
        l0 |= tmp[ti + 10] << 20;
        l0 |= tmp[ti + 11] << 19;
        l0 |= tmp[ti + 12] << 18;
        l0 |= tmp[ti + 13] << 17;
        l0 |= tmp[ti + 14] << 16;
        l0 |= tmp[ti + 15] << 15;
        l0 |= tmp[ti + 16] << 14;
        l0 |= tmp[ti + 17] << 13;
        l0 |= tmp[ti + 18] << 12;
        l0 |= tmp[ti + 19] << 11;
        l0 |= tmp[ti + 20] << 10;
        l0 |= tmp[ti + 21] << 9;
        l0 |= tmp[ti + 22] << 8;
        l0 |= tmp[ti + 23] << 7;
        l0 |= tmp[ti + 24] << 6;
        l0 |= tmp[ti + 25] << 5;
        l0 |= tmp[ti + 26] << 4;
        l0 |= tmp[ti + 27] << 3;
        l0 |= tmp[ti + 28] << 2;
        l0 |= tmp[ti + 29] << 1;
        l0 |= tmp[ti + 30] << 0;
        dest[ii + 0] = l0;
        ti += 31;
        ii += 1;
    }
}

/// Dispatch to the width-specialized stitch. Dividing widths have none.
#[inline]
fn stitch_dispatch(tmp: &[u32], bits: usize, dest: &mut [u32; BLOCK]) {
    match bits {
        3 => stitch_3(tmp, dest),
        5 => stitch_5(tmp, dest),
        6 => stitch_6(tmp, dest),
        7 => stitch_7(tmp, dest),
        9 => stitch_9(tmp, dest),
        10 => stitch_10(tmp, dest),
        11 => stitch_11(tmp, dest),
        12 => stitch_12(tmp, dest),
        13 => stitch_13(tmp, dest),
        14 => stitch_14(tmp, dest),
        15 => stitch_15(tmp, dest),
        17 => stitch_17(tmp, dest),
        18 => stitch_18(tmp, dest),
        19 => stitch_19(tmp, dest),
        20 => stitch_20(tmp, dest),
        21 => stitch_21(tmp, dest),
        22 => stitch_22(tmp, dest),
        23 => stitch_23(tmp, dest),
        24 => stitch_24(tmp, dest),
        25 => stitch_25(tmp, dest),
        26 => stitch_26(tmp, dest),
        27 => stitch_27(tmp, dest),
        28 => stitch_28(tmp, dest),
        29 => stitch_29(tmp, dest),
        30 => stitch_30(tmp, dest),
        31 => stitch_31(tmp, dest),
        _ => {}
    }
}
