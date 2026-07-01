// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{AtomicError, AtomicResult, SharedFlagRef};

use crate::ops::UncheckedTransactionOps;

pub trait TransactionOps: UncheckedTransactionOps {
    fn flag_test(flag: SharedFlagRef) -> AtomicResult<bool> {
        if Self::ffi_ops().flag_ops.fp_test.is_none() {
            return Err(AtomicError::UnsupportedOperation)
        };
        Ok(unsafe { Self::unchecked_flag_test(flag) })
    }

    fn flag_test_set(flag: SharedFlagRef) -> AtomicResult<bool> {
        if Self::ffi_ops().flag_ops.fp_test_set.is_none() {
            return Err(AtomicError::UnsupportedOperation)
        };
        Ok(unsafe { Self::unchecked_flag_test_set(flag) })
    }

    fn flag_clear(flag: SharedFlagRef) -> AtomicResult<()> {
        if Self::ffi_ops().flag_ops.fp_clear.is_none() {
            return Err(AtomicError::UnsupportedOperation)
        };
        Ok(unsafe { Self::unchecked_flag_clear(flag) })
    }
}
