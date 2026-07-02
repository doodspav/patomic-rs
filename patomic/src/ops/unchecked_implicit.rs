// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};

use patomic_sys::*;

use crate::SharedBytesRef;

pub unsafe trait FfiOpsImplicit {
    fn ffi_ops() -> patomic_ops_t;
}

pub trait UncheckedImplicitOps: FfiOpsImplicit {
    unsafe fn unchecked_store(obj: SharedBytesRef, desired: &[u8]) {
        let fp_store = Self::ffi_ops().fp_store.unwrap_unchecked();
        fp_store(
            obj.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
        )
    }

    unsafe fn unchecked_load(obj: SharedBytesRef, ret: &mut [u8]) {
        let fp_load = Self::ffi_ops().fp_load.unwrap_unchecked();
        fp_load(
            obj.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_exchange(
        obj: SharedBytesRef, desired: &[u8], ret: &mut [u8]
    ) {
        let fp_exchange =
            Self::ffi_ops().xchg_ops.fp_exchange.unwrap_unchecked();
        fp_exchange(
            obj.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_compare_exchange_weak(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8]
    ) -> bool {
        let fp_cmpxchg_weak =
            Self::ffi_ops().xchg_ops.fp_cmpxchg_weak.unwrap_unchecked();
        fp_cmpxchg_weak(
            obj.as_mut_ptr() as *mut c_void,
            expected.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
        ) != 0
    }

    unsafe fn unchecked_compare_exchange_strong(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8]
    ) -> bool {
        let fp_cmpxchg_strong =
            Self::ffi_ops().xchg_ops.fp_cmpxchg_strong.unwrap_unchecked();
        fp_cmpxchg_strong(
            obj.as_mut_ptr() as *mut c_void,
            expected.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
        ) != 0
    }

    unsafe fn unchecked_bit_test(obj: SharedBytesRef, offset: usize) -> bool {
        let fp_test = Self::ffi_ops().bitwise_ops.fp_test.unwrap_unchecked();
        fp_test(
            obj.as_ptr() as *const c_void,
            offset as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_compl(
        obj: SharedBytesRef, offset: usize
    ) -> bool {
        let fp_test_compl =
            Self::ffi_ops().bitwise_ops.fp_test_compl.unwrap_unchecked();
        fp_test_compl(
            obj.as_ptr() as *mut c_void,
            offset as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_set(
        obj: SharedBytesRef, offset: usize
    ) -> bool {
        let fp_test_set =
            Self::ffi_ops().bitwise_ops.fp_test_set.unwrap_unchecked();
        fp_test_set(
            obj.as_ptr() as *mut c_void,
            offset as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_reset(
        obj: SharedBytesRef, offset: usize
    ) -> bool {
        let fp_test_reset =
            Self::ffi_ops().bitwise_ops.fp_test_reset.unwrap_unchecked();
        fp_test_reset(
            obj.as_ptr() as *mut c_void,
            offset as c_int,
        ) != 0
    }

    unsafe fn unchecked_or(obj: SharedBytesRef, arg: &[u8]) {
        let fp_or = Self::ffi_ops().binary_ops.fp_or.unwrap_unchecked();
        fp_or(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
        )
    }

    unsafe fn unchecked_xor(obj: SharedBytesRef, arg: &[u8]) {
        let fp_xor = Self::ffi_ops().binary_ops.fp_xor.unwrap_unchecked();
        fp_xor(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
        )
    }

    unsafe fn unchecked_and(obj: SharedBytesRef, arg: &[u8]) {
        let fp_and = Self::ffi_ops().binary_ops.fp_and.unwrap_unchecked();
        fp_and(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
        )
    }

    unsafe fn unchecked_not(obj: SharedBytesRef) {
        let fp_not = Self::ffi_ops().binary_ops.fp_not.unwrap_unchecked();
        fp_not(obj.as_mut_ptr() as *mut c_void)
    }

    unsafe fn unchecked_fetch_or(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) {
        let fp_fetch_or =
            Self::ffi_ops().binary_ops.fp_fetch_or.unwrap_unchecked();
        fp_fetch_or(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_xor(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) {
        let fp_fetch_xor =
            Self::ffi_ops().binary_ops.fp_fetch_xor.unwrap_unchecked();
        fp_fetch_xor(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_and(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) {
        let fp_fetch_and =
            Self::ffi_ops().binary_ops.fp_fetch_and.unwrap_unchecked();
        fp_fetch_and(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_not(obj: SharedBytesRef, ret: &mut [u8]) {
        let fp_fetch_not =
            Self::ffi_ops().binary_ops.fp_fetch_not.unwrap_unchecked();
        fp_fetch_not(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_add(obj: SharedBytesRef, arg: &[u8]) {
        let fp_add = Self::ffi_ops().arithmetic_ops.fp_add.unwrap_unchecked();
        fp_add(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
        )
    }

    unsafe fn unchecked_sub(obj: SharedBytesRef, arg: &[u8]) {
        let fp_sub = Self::ffi_ops().arithmetic_ops.fp_sub.unwrap_unchecked();
        fp_sub(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
        )
    }

    unsafe fn unchecked_inc(obj: SharedBytesRef) {
        let fp_inc = Self::ffi_ops().arithmetic_ops.fp_inc.unwrap_unchecked();
        fp_inc(obj.as_mut_ptr() as *mut c_void)
    }

    unsafe fn unchecked_dec(obj: SharedBytesRef) {
        let fp_dec = Self::ffi_ops().arithmetic_ops.fp_dec.unwrap_unchecked();
        fp_dec(obj.as_mut_ptr() as *mut c_void)
    }

    unsafe fn unchecked_neg(obj: SharedBytesRef) {
        let fp_neg = Self::ffi_ops().arithmetic_ops.fp_neg.unwrap_unchecked();
        fp_neg(obj.as_mut_ptr() as *mut c_void)
    }

    unsafe fn unchecked_fetch_add(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) {
        let fp_fetch_add =
            Self::ffi_ops().arithmetic_ops.fp_fetch_add.unwrap_unchecked();
        fp_fetch_add(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_sub(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) {
        let fp_fetch_sub =
            Self::ffi_ops().arithmetic_ops.fp_fetch_sub.unwrap_unchecked();
        fp_fetch_sub(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_inc(obj: SharedBytesRef, ret: &mut [u8]) {
        let fp_fetch_inc =
            Self::ffi_ops().arithmetic_ops.fp_fetch_inc.unwrap_unchecked();
        fp_fetch_inc(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_dec(obj: SharedBytesRef, ret: &mut [u8]) {
        let fp_fetch_dec =
            Self::ffi_ops().arithmetic_ops.fp_fetch_dec.unwrap_unchecked();
        fp_fetch_dec(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_neg(obj: SharedBytesRef, ret: &mut [u8]) {
        let fp_fetch_neg =
            Self::ffi_ops().arithmetic_ops.fp_fetch_neg.unwrap_unchecked();
        fp_fetch_neg(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
        )
    }
}
