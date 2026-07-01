// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use patomic_sys::*;

use crate::{AtomicError, AtomicResult, SharedFlagRef};

pub trait TransactionOps {
    fn ffi_ops() -> patomic_ops_transaction_t;

    fn flag_test(flag: SharedFlagRef) -> AtomicResult<bool> {
        let Some(fp_test) = Self::ffi_ops().flag_ops.fp_test else {
            return Err(AtomicError::UnsupportedOperation)
        };
        Ok(unsafe {
            fp_test(flag.as_ptr() as *const patomic_transaction_flag_t) != 0
        })
    }

    fn flag_test_set(flag: SharedFlagRef) -> AtomicResult<bool> {
        let Some(fp_test_set) = Self::ffi_ops().flag_ops.fp_test_set else {
            return Err(AtomicError::UnsupportedOperation)
        };
        Ok(unsafe {
            fp_test_set(flag.as_mut_ptr() as *mut patomic_transaction_flag_t) != 0
        })
    }

    fn flag_clear(flag: SharedFlagRef) -> AtomicResult<()> {
        let Some(fp_clear) = Self::ffi_ops().flag_ops.fp_clear else {
            return Err(AtomicError::UnsupportedOperation)
        };
        Ok(unsafe {
            fp_clear(flag.as_mut_ptr() as *mut patomic_transaction_flag_t)
        })
    }
}
