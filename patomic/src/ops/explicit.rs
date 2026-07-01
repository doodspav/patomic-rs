// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;

use crate::{AtomicError, AtomicLayout, AtomicResult, Ordering, SharedBytesRef};

use crate::ops::UncheckedExplicitOps;
use crate::ops::macros::{
    do_atomic_checks,
    do_atomic_checks_bit_test,
};

pub trait ExplicitOps: AtomicLayout + UncheckedExplicitOps {
    fn store_explicit(
        obj: SharedBytesRef, ordering: Ordering, desired: &[u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops(), fp_store,
            obj, desired,
        );
        if !ordering.is_valid_store_ordering() {
            return Err(AtomicError::InvalidOrdering)
        };
        Ok(unsafe { Self::unchecked_store_explicit(obj, ordering, &desired) })
    }

    fn load_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops(), fp_load,
            obj, ret,
        );
        if !ordering.is_valid_load_ordering() {
            return Err(AtomicError::InvalidOrdering)
        };
        Ok(unsafe { Self::unchecked_load_explicit(obj, ordering, ret) })
    }

    fn exchange_explicit(
        obj: SharedBytesRef, desired: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_exchange,
            obj, desired, ret,
        );
        Ok(unsafe {
            Self::unchecked_exchange_explicit(obj, desired, ordering, ret)
        })
    }

    fn compare_exchange_weak_explicit(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> AtomicResult<bool> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_weak,
            obj, desired, expected,
        );
        if !fail.is_valid_fail_ordering_for(succ) {
            return Err(AtomicError::InvalidOrdering)
        };
        Ok(unsafe {
            Self::unchecked_compare_exchange_weak_explicit(
                obj, expected, desired, succ, fail
            )
        })
    }

    fn compare_exchange_strong_explicit(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> AtomicResult<bool> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_strong,
            obj, desired, expected,
        );
        if !fail.is_valid_fail_ordering_for(succ) {
            return Err(AtomicError::InvalidOrdering)
        };
        Ok(unsafe {
            Self::unchecked_compare_exchange_strong_explicit(
                obj, expected, desired, succ, fail
            )
        })
    }

    fn bit_test_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test,
            obj; offset
        );
        if !ordering.is_valid_load_ordering() {
            return Err(AtomicError::InvalidOrdering)
        };
        Ok(unsafe { Self::unchecked_bit_test_explicit(obj, offset, ordering) })
    }

    fn bit_test_compl_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_compl,
            obj; offset
        );
        Ok(unsafe {
            Self::unchecked_bit_test_compl_explicit(obj, offset, ordering)
        })
    }

    fn bit_test_set_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_set,
            obj; offset
        );
        Ok(unsafe {
            Self::unchecked_bit_test_set_explicit(obj, offset, ordering)
        })
    }

    fn bit_test_reset_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_reset,
            obj; offset
        );
        Ok(unsafe {
            Self::unchecked_bit_test_reset_explicit(obj, offset, ordering)
        })
    }

    fn or_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_or,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_or_explicit(obj, arg, ordering) })
    }

    fn xor_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_xor,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_xor_explicit(obj, arg, ordering) })
    }

    fn and_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_and,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_and_explicit(obj, arg, ordering) })
    }

    fn not_explicit(
        obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_not,
            obj,
        );
        Ok(unsafe { Self::unchecked_not_explicit(obj, ordering) })
    }

    fn fetch_or_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_or,
            obj, arg, ret,
        );
        Ok(unsafe {
            Self::unchecked_fetch_or_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_xor_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_xor,
            obj, arg, ret,
        );
        Ok(unsafe {
            Self::unchecked_fetch_xor_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_and_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_and,
            obj, arg, ret,
        );
        Ok(unsafe {
            Self::unchecked_fetch_and_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_not_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_not,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_not_explicit(obj, ordering, ret) })
    }

    fn add_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_add,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_add_explicit(obj, arg, ordering) })
    }

    fn sub_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_sub,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_sub_explicit(obj, arg, ordering) })
    }

    fn inc_explicit(
        obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_inc,
            obj,
        );
        Ok(unsafe { Self::unchecked_inc_explicit(obj, ordering) })
    }

    fn dec_explicit(
        obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_dec,
            obj,
        );
        Ok(unsafe { Self::unchecked_dec_explicit(obj, ordering) })
    }

    fn neg_explicit(
        obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_neg,
            obj,
        );
        Ok(unsafe { Self::unchecked_neg_explicit(obj, ordering) })
    }

    fn fetch_add_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_add,
            obj, arg, ret,
        );
        Ok(unsafe {
            Self::unchecked_fetch_add_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_sub_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_sub,
            obj, arg, ret,
        );
        Ok(unsafe {
            Self::unchecked_fetch_sub_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_inc_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_inc,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_inc_explicit(obj, ordering, ret) })
    }

    fn fetch_dec_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_dec,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_dec_explicit(obj, ordering, ret) })
    }

    fn fetch_neg_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_neg,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_neg_explicit(obj, ordering, ret) })
    }
}
