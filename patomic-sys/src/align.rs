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
