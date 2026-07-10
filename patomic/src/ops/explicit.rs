// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{AtomicLayout, Ordering, SharedBytesRef};

use crate::error::{
    AtomicOpResult,
    AtomicBitwiseOpResult,
    AtomicExplicitAccessOpResult, AtomicExplicitAccessOpError,
    AtomicExplicitBitTestOpResult, AtomicExplicitBitTestOpError,
};

use crate::ops::UncheckedExplicitOps;
use crate::ops::macros::{
    do_atomic_checks,
    do_atomic_checks_bit_test,
};

pub trait ExplicitOps: AtomicLayout + UncheckedExplicitOps {
    fn store_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, desired: &[u8]
    ) -> AtomicExplicitAccessOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops(), fp_store,
            obj, desired,
        );
        if !ordering.is_valid_store_ordering() {
            return Err(AtomicExplicitAccessOpError::InvalidOrdering)
        };
        Ok(unsafe { self.unchecked_store_explicit(obj, ordering, &desired) })
    }

    fn load_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicExplicitAccessOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops(), fp_load,
            obj, ret,
        );
        if !ordering.is_valid_load_ordering() {
            return Err(AtomicExplicitAccessOpError::InvalidOrdering)
        };
        Ok(unsafe { self.unchecked_load_explicit(obj, ordering, ret) })
    }

    fn exchange_explicit(
        &self, obj: SharedBytesRef, desired: &[u8], ordering: Ordering, 
        ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().xchg_ops, fp_exchange,
            obj, desired, ret,
        );
        Ok(unsafe {
            self.unchecked_exchange_explicit(obj, desired, ordering, ret)
        })
    }

    fn compare_exchange_weak_explicit(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> AtomicExplicitAccessOpResult<bool> {
        do_atomic_checks!(
            self, self.ffi_ops().xchg_ops, fp_cmpxchg_weak,
            obj, desired, expected,
        );
        if !fail.is_valid_fail_ordering_for(succ) {
            return Err(AtomicExplicitAccessOpError::InvalidOrdering)
        };
        Ok(unsafe {
            self.unchecked_compare_exchange_weak_explicit(
                obj, expected, desired, succ, fail
            )
        })
    }

    fn compare_exchange_strong_explicit(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> AtomicExplicitAccessOpResult<bool> {
        do_atomic_checks!(
            self, self.ffi_ops().xchg_ops, fp_cmpxchg_strong,
            obj, desired, expected,
        );
        if !fail.is_valid_fail_ordering_for(succ) {
            return Err(AtomicExplicitAccessOpError::InvalidOrdering)
        };
        Ok(unsafe {
            self.unchecked_compare_exchange_strong_explicit(
                obj, expected, desired, succ, fail
            )
        })
    }

    fn bit_test_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicExplicitBitTestOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test,
            obj; offset
        );
        if !ordering.is_valid_load_ordering() {
            return Err(AtomicExplicitBitTestOpError::InvalidOrdering)
        };
        Ok(unsafe { self.unchecked_bit_test_explicit(obj, offset, ordering) })
    }

    fn bit_test_compl_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test_compl,
            obj; offset
        );
        Ok(unsafe {
            self.unchecked_bit_test_compl_explicit(obj, offset, ordering)
        })
    }

    fn bit_test_set_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test_set,
            obj; offset
        );
        Ok(unsafe {
            self.unchecked_bit_test_set_explicit(obj, offset, ordering)
        })
    }

    fn bit_test_reset_explicit(
        &self, obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test_reset,
            obj; offset
        );
        Ok(unsafe {
            self.unchecked_bit_test_reset_explicit(obj, offset, ordering)
        })
    }

    fn or_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_or,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_or_explicit(obj, arg, ordering) })
    }

    fn xor_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_xor,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_xor_explicit(obj, arg, ordering) })
    }

    fn and_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_and,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_and_explicit(obj, arg, ordering) })
    }

    fn not_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_not,
            obj,
        );
        Ok(unsafe { self.unchecked_not_explicit(obj, ordering) })
    }

    fn fetch_or_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering, 
        ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_or,
            obj, arg, ret,
        );
        Ok(unsafe {
            self.unchecked_fetch_or_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_xor_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering, 
        ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_xor,
            obj, arg, ret,
        );
        Ok(unsafe {
            self.unchecked_fetch_xor_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_and_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering, 
        ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_and,
            obj, arg, ret,
        );
        Ok(unsafe {
            self.unchecked_fetch_and_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_not_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_not,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_not_explicit(obj, ordering, ret) })
    }

    fn add_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_add,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_add_explicit(obj, arg, ordering) })
    }

    fn sub_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_sub,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_sub_explicit(obj, arg, ordering) })
    }

    fn inc_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_inc,
            obj,
        );
        Ok(unsafe { self.unchecked_inc_explicit(obj, ordering) })
    }

    fn dec_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_dec,
            obj,
        );
        Ok(unsafe { self.unchecked_dec_explicit(obj, ordering) })
    }

    fn neg_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_neg,
            obj,
        );
        Ok(unsafe { self.unchecked_neg_explicit(obj, ordering) })
    }

    fn fetch_add_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering, 
        ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_add,
            obj, arg, ret,
        );
        Ok(unsafe {
            self.unchecked_fetch_add_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_sub_explicit(
        &self, obj: SharedBytesRef, arg: &[u8], ordering: Ordering, 
        ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_sub,
            obj, arg, ret,
        );
        Ok(unsafe {
            self.unchecked_fetch_sub_explicit(obj, arg, ordering, ret)
        })
    }

    fn fetch_inc_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_inc,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_inc_explicit(obj, ordering, ret) })
    }

    fn fetch_dec_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_dec,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_dec_explicit(obj, ordering, ret) })
    }

    fn fetch_neg_explicit(
        &self, obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_neg,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_neg_explicit(obj, ordering, ret) })
    }
}
