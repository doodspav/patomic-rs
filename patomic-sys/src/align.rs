// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};
use core::num::NonZeroUsize;

pub const PATOMIC_MAX_CACHE_LINE_SIZE_ABI_UNSTABLE: usize = 128;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_align_t {
    pub recommended: NonZeroUsize,
    pub minimum: NonZeroUsize,
    pub size_within: usize,
}

#[inline]
pub fn PATOMIC_ALIGN_MEETS_RECOMMENDED(
    ptr: *const c_void,
    align: patomic_align_t,
) -> bool {
    if !align.recommended.is_power_of_two() {
        return false;
    }
    let addr = ptr.addr();
    let recommended = align.recommended.get();
    addr & (recommended - 1) == 0
}

#[inline]
pub fn PATOMIC_ALIGN_MEETS_MINIMUM(
    ptr: *const c_void,
    align: patomic_align_t,
    width: NonZeroUsize,
) -> bool {
    if !align.minimum.is_power_of_two() {
        return false;
    }
    let mut addr = ptr.addr();
    if (addr & (align.minimum.get() - 1)) != 0 {
        return false;
    }

    if align.size_within == 0 {
        return true;
    }
    addr %= align.size_within;
    width.get() <= align.size_within - addr
}

unsafe extern "C" {
    pub fn patomic_cache_line_size() -> usize;

    pub fn patomic_align_meets_recommended(
        ptr: *const c_void,
        align: patomic_align_t,
    ) -> c_int;

    pub fn patomic_align_meets_minimum(
        ptr: *const c_void,
        align: patomic_align_t,
        width: usize,
    ) -> c_int;
}
