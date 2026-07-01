// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;

use crate::{AtomicError, AtomicLayout, AtomicResult, SharedBytesRef};

use crate::ops::UncheckedImplicitOps;
use crate::ops::macros::{
    do_atomic_checks,
    do_atomic_checks_bit_test,
};

pub trait ImplicitOps: AtomicLayout + UncheckedImplicitOps {
    fn store(obj: SharedBytesRef, desired: &[u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops(), fp_store,
            obj, desired,
        );
        Ok(unsafe { Self::unchecked_store(obj, desired) })
    }

    fn load(obj: SharedBytesRef, ret: &mut [u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops(), fp_load,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_load(obj, ret) })
    }

    fn exchange(
        obj: SharedBytesRef, desired: &[u8], ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_exchange,
            obj, desired, ret,
        );
        Ok(unsafe { Self::unchecked_exchange(obj, desired, ret) })
    }

    fn compare_exchange_weak(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8]
    ) -> AtomicResult<bool> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_weak,
            obj, desired, expected,
        );
        Ok(unsafe {
            Self::unchecked_compare_exchange_weak(obj, expected, desired)
        })
    }

    fn compare_exchange_strong(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8]
    ) -> AtomicResult<bool> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_strong,
            obj, desired, expected,
        );
        Ok(unsafe {
            Self::unchecked_compare_exchange_strong(obj, expected, desired)
        })
    }

    fn bit_test(obj: SharedBytesRef, offset: usize) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test,
            obj; offset
        );
        Ok(unsafe { Self::unchecked_bit_test(obj, offset) })
    }

    fn bit_test_compl(obj: SharedBytesRef, offset: usize) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_compl,
            obj; offset
        );
        Ok(unsafe { Self::unchecked_bit_test_compl(obj, offset) })
    }

    fn bit_test_set(obj: SharedBytesRef, offset: usize) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_set,
            obj; offset
        );
        Ok(unsafe { Self::unchecked_bit_test_set(obj, offset) })
    }

    fn bit_test_reset(obj: SharedBytesRef, offset: usize) -> AtomicResult<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_reset,
            obj; offset
        );
        Ok(unsafe { Self::unchecked_bit_test_reset(obj, offset) })
    }

    fn or(obj: SharedBytesRef, arg: &[u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_or,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_or(obj, arg) })
    }

    fn xor(obj: SharedBytesRef, arg: &[u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_xor,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_xor(obj, arg) })
    }

    fn and(obj: SharedBytesRef, arg: &[u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_and,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_and(obj, arg) })
    }

    fn not(obj: SharedBytesRef) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_not,
            obj,
        );
        Ok(unsafe { Self::unchecked_not(obj) })
    }

    fn fetch_or(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_or,
            obj, arg, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_or(obj, arg, ret) })
    }

    fn fetch_xor(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_xor,
            obj, arg, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_xor(obj, arg, ret) })
    }

    fn fetch_and(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_and,
            obj, arg, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_and(obj, arg, ret) })
    }

    fn fetch_not(obj: SharedBytesRef, ret: &mut [u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_not,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_not(obj, ret) })
    }

    fn add(obj: SharedBytesRef, arg: &[u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_add,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_add(obj, arg) })
    }

    fn sub(obj: SharedBytesRef, arg: &[u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_sub,
            obj, arg,
        );
        Ok(unsafe { Self::unchecked_sub(obj, arg) })
    }

    fn inc(obj: SharedBytesRef) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_inc,
            obj,
        );
        Ok(unsafe { Self::unchecked_inc(obj) })
    }

    fn dec(obj: SharedBytesRef) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_dec,
            obj,
        );
        Ok(unsafe { Self::unchecked_dec(obj) })
    }

    fn neg(obj: SharedBytesRef) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_neg,
            obj,
        );
        Ok(unsafe { Self::unchecked_neg(obj) })
    }

    fn fetch_add(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_add,
            obj, arg, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_add(obj, arg, ret) })
    }

    fn fetch_sub(
        obj: SharedBytesRef, arg: &[u8], ret: &mut [u8]
    ) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_sub,
            obj, arg, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_sub(obj, arg, ret) })
    }

    fn fetch_inc(obj: SharedBytesRef, ret: &mut [u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_inc,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_inc(obj, ret) })
    }

    fn fetch_dec(obj: SharedBytesRef, ret: &mut [u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_dec,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_dec(obj, ret) })
    }

    fn fetch_neg(obj: SharedBytesRef, ret: &mut [u8]) -> AtomicResult<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_neg,
            obj, ret,
        );
        Ok(unsafe { Self::unchecked_fetch_neg(obj, ret) })
    }
}
