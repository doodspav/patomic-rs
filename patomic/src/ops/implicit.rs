// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{AtomicLayout, SharedBytesRef};

use crate::error::{AtomicOpResult, AtomicBitwiseOpResult};

use crate::ops::UncheckedImplicitOps;
use crate::ops::macros::{
    do_atomic_checks,
    do_atomic_checks_bit_test,
};

pub trait ImplicitOps: AtomicLayout + UncheckedImplicitOps {
    fn store(&self, obj: SharedBytesRef, desired: &[u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops(), fp_store,
            obj, desired,
        );
        Ok(unsafe { self.unchecked_store(obj, desired) })
    }

    fn load(&self, obj: SharedBytesRef, ret: &mut [u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops(), fp_load,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_load(obj, ret) })
    }

    fn exchange(
        &self, obj: SharedBytesRef, desired: &[u8], ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().xchg_ops, fp_exchange,
            obj, desired, ret,
        );
        Ok(unsafe { self.unchecked_exchange(obj, desired, ret) })
    }

    fn compare_exchange_weak(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8]
    ) -> AtomicOpResult<bool> {
        do_atomic_checks!(
            self, self.ffi_ops().xchg_ops, fp_cmpxchg_weak,
            obj, desired, expected,
        );
        Ok(unsafe {
            self.unchecked_compare_exchange_weak(obj, expected, desired)
        })
    }

    fn compare_exchange_strong(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8]
    ) -> AtomicOpResult<bool> {
        do_atomic_checks!(
            self, self.ffi_ops().xchg_ops, fp_cmpxchg_strong,
            obj, desired, expected,
        );
        Ok(unsafe {
            self.unchecked_compare_exchange_strong(obj, expected, desired)
        })
    }

    fn bit_test(&self, obj: SharedBytesRef, offset: usize
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test,
            obj; offset
        );
        Ok(unsafe { self.unchecked_bit_test(obj, offset) })
    }

    fn bit_test_compl(&self, obj: SharedBytesRef, offset: usize
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test_compl,
            obj; offset
        );
        Ok(unsafe { self.unchecked_bit_test_compl(obj, offset) })
    }

    fn bit_test_set(&self, obj: SharedBytesRef, offset: usize
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test_set,
            obj; offset
        );
        Ok(unsafe { self.unchecked_bit_test_set(obj, offset) })
    }

    fn bit_test_reset(&self, obj: SharedBytesRef, offset: usize
    ) -> AtomicBitwiseOpResult<bool> {
        do_atomic_checks_bit_test!(
            self, self.ffi_ops().bitwise_ops, fp_test_reset,
            obj; offset
        );
        Ok(unsafe { self.unchecked_bit_test_reset(obj, offset) })
    }

    fn or(&self, obj: SharedBytesRef, arg: &[u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_or,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_or(obj, arg) })
    }

    fn xor(&self, obj: SharedBytesRef, arg: &[u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_xor,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_xor(obj, arg) })
    }

    fn and(&self, obj: SharedBytesRef, arg: &[u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_and,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_and(obj, arg) })
    }

    fn not(&self, obj: SharedBytesRef) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_not,
            obj,
        );
        Ok(unsafe { self.unchecked_not(obj) })
    }

    fn fetch_or(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_or,
            obj, arg, ret,
        );
        Ok(unsafe { self.unchecked_fetch_or(obj, arg, ret) })
    }

    fn fetch_xor(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_xor,
            obj, arg, ret,
        );
        Ok(unsafe { self.unchecked_fetch_xor(obj, arg, ret) })
    }

    fn fetch_and(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_and,
            obj, arg, ret,
        );
        Ok(unsafe { self.unchecked_fetch_and(obj, arg, ret) })
    }

    fn fetch_not(
        &self, obj: SharedBytesRef, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().binary_ops, fp_fetch_not,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_not(obj, ret) })
    }

    fn add(&self, obj: SharedBytesRef, arg: &[u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_add,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_add(obj, arg) })
    }

    fn sub(&self, obj: SharedBytesRef, arg: &[u8]) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_sub,
            obj, arg,
        );
        Ok(unsafe { self.unchecked_sub(obj, arg) })
    }

    fn inc(&self, obj: SharedBytesRef) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_inc,
            obj,
        );
        Ok(unsafe { self.unchecked_inc(obj) })
    }

    fn dec(&self, obj: SharedBytesRef) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_dec,
            obj,
        );
        Ok(unsafe { self.unchecked_dec(obj) })
    }

    fn neg(&self, obj: SharedBytesRef) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_neg,
            obj,
        );
        Ok(unsafe { self.unchecked_neg(obj) })
    }

    fn fetch_add(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_add,
            obj, arg, ret,
        );
        Ok(unsafe { self.unchecked_fetch_add(obj, arg, ret) })
    }

    fn fetch_sub(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_sub,
            obj, arg, ret,
        );
        Ok(unsafe { self.unchecked_fetch_sub(obj, arg, ret) })
    }

    fn fetch_inc(
        &self, obj: SharedBytesRef, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_inc,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_inc(obj, ret) })
    }

    fn fetch_dec(
        &self, obj: SharedBytesRef, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_dec,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_dec(obj, ret) })
    }

    fn fetch_neg(
        &self, obj: SharedBytesRef, ret: &mut [u8]
    ) -> AtomicOpResult<()> {
        do_atomic_checks!(
            self, self.ffi_ops().arithmetic_ops, fp_fetch_neg,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_fetch_neg(obj, ret) })
    }
}
