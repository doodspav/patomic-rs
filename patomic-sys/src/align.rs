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

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(align(64))]
    struct OverAlignedBuffer {
        data: [u8; Self::SIZE],
    }

    impl OverAlignedBuffer {
        const SIZE: usize = 64;

        const fn new() -> Self {
            Self { data: [0u8; Self::SIZE] }
        }

        const fn len(&self) -> usize {
            self.data.len()
        }

        fn ptr(&self) -> *const u8 {
            self.data.as_ptr()
        }
    }

    fn make_align(
        recommended: usize, minimum: usize, size_within: usize
    ) -> patomic_align_t {
        patomic_align_t {
            recommended: NonZeroUsize::new(recommended).unwrap(),
            minimum: NonZeroUsize::new(minimum).unwrap(),
            size_within
        }
    }

    fn make_aligned_pointer(
        buf_ptr: *const u8, buf_size: usize, align: NonZeroUsize, size: usize
    ) -> *const u8 {
        // calculate the offset to the next aligned address
        let raw_addr = buf_ptr.addr();
        let remainder = raw_addr % align.get();
        let offset = (align.get() - remainder) % align.get();

        // check that there is enough room in the buffer
        if buf_size < (offset + size) {
            return core::ptr::null();
        }

        // return aligned pointer
        unsafe { buf_ptr.add(offset) }
    }

    fn runtime_alignof(ptr: *const u8) -> usize {
        let addr = ptr as usize;
        assert_ne!(addr, 0, "runtime_alignof is undefined for nullptr");
        1usize << addr.trailing_zeros()
    }

    fn nz(non_zero_value: usize) -> NonZeroUsize {
        NonZeroUsize::new(non_zero_value).unwrap()
    }

    #[test]
    fn max_cache_line_size_abi_unstable_is_pow2() {}

    #[test]
    fn max_cache_line_size_fn_cmp_le_unstable() {}

    #[test]
    fn meets_recommended_fails_recommended_non_pow2() {}

    #[test]
    fn meets_recommended_fails_cmp_gt_pointer_align() {}

    #[test]
    fn meets_recommended_succeeds_cmp_eq_pointer_align() {}

    #[test]
    fn meets_recommended_succeeds_cmp_lt_pointer_align() {}

    #[test]
    fn meets_recommended_succeeds_pointer_is_null() {}

    #[test]
    fn meets_minimum_fails_minimum_non_pow2() {}

    #[test]
    fn meets_minimum_fails_cmp_gt_pointer_align() {}

    #[test]
    fn meets_minimum_succeeds_cmp_eq_pointer_align() {}

    #[test]
    fn meets_minimum_succeeds_cmp_lt_pointer_align() {}

    #[test]
    fn meets_minimum_succeeds_pointer_is_null() {}

    #[test]
    fn meets_minimum_succeeds_zero_size_buffer_any_size_within() {}

    #[test]
    fn meets_minimum_succeeds_buffer_smaller_fits_in_size_within() {}

    #[test]
    fn meets_minimum_succeeds_buffer_exactly_fits_in_size_within() {}

    #[test]
    fn meets_minimum_fails_buffer_larger_than_size_within() {}

    #[test]
    fn meets_minimum_fails_buffer_fits_but_misaligned_for_size_within() {}
}
