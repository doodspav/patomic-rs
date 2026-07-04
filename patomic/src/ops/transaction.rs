// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{SharedBytesRef, SharedFlagRef, transaction::*};

use crate::error::{
    TransactionOpResult,
    TransactionBitwiseOpResult,
    TransactionUnsupportedOpError, TransactionUnsupportedOpResult,
};

use crate::ops::UncheckedTransactionOps;
use crate::ops::macros::{
    do_transaction_checks,
    do_transaction_checks_bit_test,
};

pub trait TransactionOps: UncheckedTransactionOps {
    fn store_transaction(
        &self, obj: SharedBytesRef, desired: &[u8], config: TransactionConfig
    ) -> TransactionOpResult<TransactionOutcome> {
        do_transaction_checks!(
            self.ffi_ops(), fp_store, config,
            obj, desired,
        );
        Ok(unsafe { self.unchecked_store_transaction(obj, desired, config) })
    }

    fn load_transaction(
        &self, obj: SharedBytesRef, ret: &mut [u8], config: TransactionConfig
    ) -> TransactionOpResult<TransactionOutcome> {
        do_transaction_checks!(
            self.ffi_ops(), fp_load, config,
            obj, ret,
        );
        Ok(unsafe { self.unchecked_load_transaction(obj, ret, config) })
    }

    fn exchange_transaction(
        &self, obj: SharedBytesRef, desired: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOpResult<TransactionOutcome> {
        do_transaction_checks!(
            self.ffi_ops().xchg_ops, fp_exchange, config,
            obj, desired, ret,
        );
        Ok(unsafe {
            self.unchecked_exchange_transaction(obj, desired, ret, config)
        })
    }

    fn compare_exchange_weak_transaction(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        config: TransactionConfigWfb
    ) -> TransactionOpResult<(bool, TransactionOutcomeWfb)> {
        do_transaction_checks!(
            self.ffi_ops().xchg_ops, fp_cmpxchg_weak, config,
            obj, expected, desired,
        );
        Ok(unsafe {
            self.unchecked_compare_exchange_weak_transaction(
                obj, expected, desired, config
            )
        })
    }

    fn flag_test(
        &self, flag: SharedFlagRef
    ) -> TransactionUnsupportedOpResult<bool> {
        if self.ffi_ops().flag_ops.fp_test.is_none() {
            return Err(TransactionUnsupportedOpError)
        };
        Ok(unsafe { self.unchecked_flag_test(flag) })
    }

    fn flag_test_set(
        &self, flag: SharedFlagRef
    ) -> TransactionUnsupportedOpResult<bool> {
        if self.ffi_ops().flag_ops.fp_test_set.is_none() {
            return Err(TransactionUnsupportedOpError)
        };
        Ok(unsafe { self.unchecked_flag_test_set(flag) })
    }

    fn flag_clear(
        &self, flag: SharedFlagRef
    ) -> TransactionUnsupportedOpResult<()> {
        if self.ffi_ops().flag_ops.fp_clear.is_none() {
            return Err(TransactionUnsupportedOpError)
        };
        Ok(unsafe { self.unchecked_flag_clear(flag) })
    }
}
