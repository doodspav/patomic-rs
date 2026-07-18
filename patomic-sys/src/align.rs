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

        fn as_ptr(&self) -> *const u8 {
            self.data.as_ptr()
        }

        fn as_slice(&self) -> &[u8] {
            &self.data
        }
    }

    fn make_align(
        recommended: NonZeroUsize, minimum: NonZeroUsize, size_within: usize
    ) -> patomic_align_t {
        patomic_align_t { recommended, minimum, size_within }
    }

    fn make_aligned_pointer(
        buf: &[u8], align: NonZeroUsize, size: usize
    ) -> *const u8 {
        // calculate the offset to the next aligned address
        let raw_addr = buf.as_ptr().addr();
        let remainder = raw_addr % align.get();
        let offset = (align.get() - remainder) % align.get();

        // check that there is enough room in the buffer
        assert!(buf.len() >= offset + size);

        // return aligned pointer
        unsafe { buf.as_ptr().add(offset) }
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
    fn max_cache_line_size_abi_unstable_is_pow2() {
        let val = PATOMIC_MAX_CACHE_LINE_SIZE_ABI_UNSTABLE;
        assert!(val.is_power_of_two());
    }

    #[test]
    fn max_cache_line_size_fn_cmp_le_unstable() {
        let fn_val = unsafe { patomic_cache_line_size() };
        let unstable_val = PATOMIC_MAX_CACHE_LINE_SIZE_ABI_UNSTABLE;
        assert!(fn_val <= unstable_val);
    }

    #[test]
    fn meets_recommended_fails_recommended_non_pow2() {
        let buf = OverAlignedBuffer::new();
        let align = make_align(nz(3), nz(1), 0);
        let ptr = make_aligned_pointer(buf.as_slice(), align.recommended, 1);

        assert!(!align.recommended.is_power_of_two());

        // pointer is aligned (don't use runtime_alignof because non-pow2)
        assert_eq!(0, ptr as usize % align.recommended.get());

        assert!(!PATOMIC_ALIGN_MEETS_RECOMMENDED(ptr.cast(), align));
    }

    #[test]
    fn meets_recommended_fails_cmp_gt_pointer_align() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(runtime_alignof(ptr) * 2), nz(1), 0);

        assert!(align.recommended.is_power_of_two());
        assert!(align.recommended.get() > runtime_alignof(ptr));

        assert!(!PATOMIC_ALIGN_MEETS_RECOMMENDED(ptr.cast(), align));
    }

    #[test]
    fn meets_recommended_succeeds_cmp_eq_pointer_align() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(runtime_alignof(ptr)), nz(1), 0);

        assert!(align.recommended.is_power_of_two());
        assert_eq!(align.recommended.get(), runtime_alignof(ptr));

        assert!(PATOMIC_ALIGN_MEETS_RECOMMENDED(ptr.cast(), align));
    }

    #[test]
    fn meets_recommended_succeeds_cmp_lt_pointer_align() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(runtime_alignof(ptr) / 2), nz(1), 0);

        assert!(align.recommended.is_power_of_two());
        assert!(align.recommended.get() < runtime_alignof(ptr));

        assert!(PATOMIC_ALIGN_MEETS_RECOMMENDED(ptr.cast(), align));
    }

    #[test]
    fn meets_recommended_succeeds_pointer_is_null() {
        let align = make_align(nz(32768), nz(1), 0);
        let ptr: *const u8 = core::ptr::null();

        assert!(align.recommended.is_power_of_two());

        assert!(PATOMIC_ALIGN_MEETS_RECOMMENDED(ptr.cast(), align));
    }

    #[test]
    fn meets_minimum_fails_minimum_non_pow2() {
        let buf = OverAlignedBuffer::new();
        let align = make_align(nz(1), nz(3), 0);
        let ptr = make_aligned_pointer(buf.as_slice(), align.minimum, 1);

        assert_eq!(0, align.size_within);
        assert!(!align.minimum.is_power_of_two());

        // pointer is aligned (don't use runtime_alignof because non-pow2)
        assert_eq!(0, ptr as usize % align.minimum.get());

        assert!(!PATOMIC_ALIGN_MEETS_MINIMUM(ptr.cast(), align, nz(1)));
    }

    #[test]
    fn meets_minimum_fails_cmp_gt_pointer_align() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(1), nz(runtime_alignof(ptr) * 2), 0);

        assert_eq!(0, align.size_within);
        assert!(align.minimum.is_power_of_two());
        assert!(align.minimum.get() > runtime_alignof(ptr));

        assert!(!PATOMIC_ALIGN_MEETS_MINIMUM(ptr.cast(), align, nz(1)));
    }

    #[test]
    fn meets_minimum_succeeds_cmp_eq_pointer_align() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(1), nz(runtime_alignof(ptr)), 0);

        assert_eq!(0, align.size_within);
        assert!(align.minimum.is_power_of_two());
        assert_eq!(align.minimum.get(), runtime_alignof(ptr));

        assert!(PATOMIC_ALIGN_MEETS_MINIMUM(ptr.cast(), align, nz(1)));
    }

    #[test]
    fn meets_minimum_succeeds_cmp_lt_pointer_align() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(1), nz(runtime_alignof(ptr) / 2), 0);

        assert_eq!(0, align.size_within);
        assert!(align.minimum.is_power_of_two());
        assert!(align.minimum.get() < runtime_alignof(ptr));

        assert!(PATOMIC_ALIGN_MEETS_MINIMUM(ptr.cast(), align, nz(1)));
    }

    #[test]
    fn meets_minimum_succeeds_pointer_is_null() {
        let align = make_align(nz(1), nz(32768), 0);
        let ptr: *const u8 = core::ptr::null();

        assert_eq!(0, align.size_within);
        assert!(align.minimum.is_power_of_two());

        assert!(PATOMIC_ALIGN_MEETS_MINIMUM(ptr.cast(), align, nz(1)));
    }

    #[test]
    fn meets_minimum_succeeds_buffer_smaller_fits_in_size_within() {
        let buf = OverAlignedBuffer::new();
        let ptr = unsafe { buf.as_ptr().add(1) };
        let align = make_align(nz(1), nz(1), 8);

        assert!(align.size_within > 2);
        assert!(align.minimum.is_power_of_two());

        assert!(runtime_alignof(buf.as_ptr()) >= align.minimum.get());
        assert!(runtime_alignof(buf.as_ptr()) >= align.size_within);
        assert!(runtime_alignof(ptr) >= align.minimum.get());

        assert!(PATOMIC_ALIGN_MEETS_MINIMUM(
            ptr.cast(), align, nz(align.size_within - 2)
        ));
    }

    #[test]
    fn meets_minimum_succeeds_buffer_exactly_fits_in_size_within() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(1), nz(1), 8);

        assert_ne!(0, align.size_within);
        assert!(align.minimum.is_power_of_two());

        assert!(runtime_alignof(ptr) >= align.minimum.get());
        assert!(runtime_alignof(ptr) >= align.size_within);

        assert!(PATOMIC_ALIGN_MEETS_MINIMUM(
            ptr.cast(), align, nz(align.size_within)
        ));
    }

    #[test]
    fn meets_minimum_fails_buffer_larger_than_size_within() {
        let buf = OverAlignedBuffer::new();
        let ptr = buf.as_ptr();
        let align = make_align(nz(1), nz(1), 8);

        assert_ne!(0, align.size_within);
        assert!(align.minimum.is_power_of_two());

        assert!(runtime_alignof(ptr) >= align.minimum.get());
        assert!(runtime_alignof(ptr) >= align.size_within);

        assert!(!PATOMIC_ALIGN_MEETS_MINIMUM(
            ptr.cast(), align, nz(align.size_within + 1)
        ));
    }

    #[test]
    fn meets_minimum_fails_buffer_fits_but_misaligned_for_size_within() {
        // we need a pointer that is 16 bytes offset from a 64 byte aligned addr
        // the buffer is 32 bytes, crossing the 64 byte alignment boundary
        #[repr(align(64))]
        struct Buffer([u8; 80]);

        let buf = Buffer([0u8; 80]);
        let ptr = unsafe { buf.0.as_ptr().add(64 - 16) };
        let align = make_align(nz(1), nz(16), 64);
        let size = nz(32);

        assert!(align.minimum.is_power_of_two());

        // size_within is 64 bytes but pointer is only aligned to 16 bytes
        assert_eq!(64, align.size_within);
        assert_eq!(16, align.minimum.get());
        assert_eq!(16, runtime_alignof(ptr));

        // pointer is 16 bytes from a 64 byte alignment boundary and crosses it
        assert!(64 <= runtime_alignof(unsafe { ptr.add(16) }));
        assert!(size.get() > 16);

        assert!(!PATOMIC_ALIGN_MEETS_MINIMUM(ptr.cast(), align, size));
    }
}
