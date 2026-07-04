// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};

use patomic_sys::*;

use crate::{Ordering, SharedBytesRef};

pub unsafe trait FfiOpsExplicit {
    fn ffi_ops(&self) -> &patomic_ops_explicit_t;
}

pub trait UncheckedExplicitOps: FfiOpsExplicit {
    unsafe fn unchecked_store_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, desired: &[u8]
    ) {
        let fp_store = self.ffi_ops().fp_store.unwrap_unchecked();
        fp_store(
            obj.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_load_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) {
        let fp_load = self.ffi_ops().fp_load.unwrap_unchecked();
        fp_load(
            obj.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_exchange_explicit(
        &self, obj: SharedBytesRef, desired: &[u8], ordering: Ordering,
        ret: &mut [u8]
    ) {
        let fp_exchange =
            self.ffi_ops().xchg_ops.fp_exchange.unwrap_unchecked();
        fp_exchange(
            obj.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_compare_exchange_weak_explicit(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> bool {
        let fp_cmpxchg_weak =
            self.ffi_ops().xchg_ops.fp_cmpxchg_weak.unwrap_unchecked();
        fp_cmpxchg_weak(
            obj.as_mut_ptr() as *mut c_void,
            expected.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            succ as c_int,
            fail as c_int,
        ) != 0
    }

    unsafe fn unchecked_compare_exchange_strong_explicit(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> bool {
        let fp_cmpxchg_strong =
            self.ffi_ops().xchg_ops.fp_cmpxchg_strong.unwrap_unchecked();
        fp_cmpxchg_strong(
            obj.as_mut_ptr() as *mut c_void,
            expected.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            succ as c_int,
            fail as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> bool {
        let fp_test = self.ffi_ops().bitwise_ops.fp_test.unwrap_unchecked();
        fp_test(
            obj.as_ptr() as *const c_void,
            offset as c_int,
            ordering as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_compl_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> bool {
        let fp_test_compl =
            self.ffi_ops().bitwise_ops.fp_test_compl.unwrap_unchecked();
        fp_test_compl(
            obj.as_mut_ptr() as *mut c_void,
            offset as c_int,
            ordering as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_set_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> bool {
        let fp_test_set =
            self.ffi_ops().bitwise_ops.fp_test_set.unwrap_unchecked();
        fp_test_set(
            obj.as_mut_ptr() as *mut c_void,
            offset as c_int,
            ordering as c_int,
        ) != 0
    }

    unsafe fn unchecked_bit_test_reset_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> bool {
        let fp_test_reset =
            self.ffi_ops().bitwise_ops.fp_test_reset.unwrap_unchecked();
        fp_test_reset(
            obj.as_mut_ptr() as *mut c_void,
            offset as c_int,
            ordering as c_int,
        ) != 0
    }

    unsafe fn unchecked_or_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) {
        let fp_or = self.ffi_ops().binary_ops.fp_or.unwrap_unchecked();
        fp_or(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_xor_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) {
        let fp_xor = self.ffi_ops().binary_ops.fp_xor.unwrap_unchecked();
        fp_xor(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_and_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) {
        let fp_and = self.ffi_ops().binary_ops.fp_and.unwrap_unchecked();
        fp_and(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_not_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) {
        let fp_not = self.ffi_ops().binary_ops.fp_not.unwrap_unchecked();
        fp_not(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_fetch_or_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering,
        ret: &mut [u8]
    ) {
        let fp_fetch_or =
            self.ffi_ops().binary_ops.fp_fetch_or.unwrap_unchecked();
        fp_fetch_or(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_xor_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering,
        ret: &mut [u8]
    ) {
        let fp_fetch_xor =
            self.ffi_ops().binary_ops.fp_fetch_xor.unwrap_unchecked();
        fp_fetch_xor(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_and_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering,
        ret: &mut [u8]
    ) {
        let fp_fetch_and =
            self.ffi_ops().binary_ops.fp_fetch_and.unwrap_unchecked();
        fp_fetch_and(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_not_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) {
        let fp_fetch_not =
            self.ffi_ops().binary_ops.fp_fetch_not.unwrap_unchecked();
        fp_fetch_not(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_add_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) {
        let fp_add = self.ffi_ops().arithmetic_ops.fp_add.unwrap_unchecked();
        fp_add(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_sub_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) {
        let fp_sub = self.ffi_ops().arithmetic_ops.fp_sub.unwrap_unchecked();
        fp_sub(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_inc_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) {
        let fp_inc = self.ffi_ops().arithmetic_ops.fp_inc.unwrap_unchecked();
        fp_inc(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_dec_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) {
        let fp_dec = self.ffi_ops().arithmetic_ops.fp_dec.unwrap_unchecked();
        fp_dec(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_neg_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) {
        let fp_neg = self.ffi_ops().arithmetic_ops.fp_neg.unwrap_unchecked();
        fp_neg(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
        )
    }

    unsafe fn unchecked_fetch_add_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering,
        ret: &mut [u8]
    ) {
        let fp_fetch_add =
            self.ffi_ops().arithmetic_ops.fp_fetch_add.unwrap_unchecked();
        fp_fetch_add(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_sub_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering,
        ret: &mut [u8]
    ) {
        let fp_fetch_sub =
            self.ffi_ops().arithmetic_ops.fp_fetch_sub.unwrap_unchecked();
        fp_fetch_sub(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_inc_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) {
        let fp_fetch_inc =
            self.ffi_ops().arithmetic_ops.fp_fetch_inc.unwrap_unchecked();
        fp_fetch_inc(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_dec_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) {
        let fp_fetch_dec =
            self.ffi_ops().arithmetic_ops.fp_fetch_dec.unwrap_unchecked();
        fp_fetch_dec(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }

    unsafe fn unchecked_fetch_neg_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) {
        let fp_fetch_neg =
            self.ffi_ops().arithmetic_ops.fp_fetch_neg.unwrap_unchecked();
        fp_fetch_neg(
            obj.as_mut_ptr() as *mut c_void,
            ordering as c_int,
            ret.as_mut_ptr() as *mut c_void,
        )
    }
}
