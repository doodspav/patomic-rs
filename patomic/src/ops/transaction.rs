// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::SharedFlagRef;

use crate::error::{
    TransactionError, TransactionResult
};

use crate::ops::UncheckedTransactionOps;

pub trait TransactionOps: UncheckedTransactionOps {
    fn flag_test(flag: SharedFlagRef) -> TransactionResult<bool> {
        if Self::ffi_ops().flag_ops.fp_test.is_none() {
            return Err(TransactionError::UnsupportedOperation)
        };
        Ok(unsafe { Self::unchecked_flag_test(flag) })
    }

    fn flag_test_set(flag: SharedFlagRef) -> TransactionResult<bool> {
        if Self::ffi_ops().flag_ops.fp_test_set.is_none() {
            return Err(TransactionError::UnsupportedOperation)
        };
        Ok(unsafe { Self::unchecked_flag_test_set(flag) })
    }

    fn flag_clear(flag: SharedFlagRef) -> TransactionResult<()> {
        if Self::ffi_ops().flag_ops.fp_clear.is_none() {
            return Err(TransactionError::UnsupportedOperation)
        };
        Ok(unsafe { Self::unchecked_flag_clear(flag) })
    }
}
