// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use patomic_sys::*;

use crate::SharedFlagRef;

pub unsafe trait FfiOpsTransaction {
    fn ffi_ops(&self) -> patomic_ops_transaction_t;
}

pub trait UncheckedTransactionOps: FfiOpsTransaction {
    unsafe fn unchecked_flag_test(&self, flag: SharedFlagRef) -> bool {
        let fp_test = self.ffi_ops().flag_ops.fp_test.unwrap_unchecked();
        fp_test(flag.as_ptr() as *const patomic_transaction_flag_t) != 0
    }
    
    unsafe fn unchecked_flag_test_set(&self, flag: SharedFlagRef) -> bool {
        let fp_test_set = 
            self.ffi_ops().flag_ops.fp_test_set.unwrap_unchecked();
        fp_test_set(flag.as_mut_ptr() as *mut patomic_transaction_flag_t) != 0
    }
    
    unsafe fn unchecked_flag_clear(&self, flag: SharedFlagRef) {
        let fp_clear = self.ffi_ops().flag_ops.fp_clear.unwrap_unchecked();
        fp_clear(flag.as_mut_ptr() as *mut patomic_transaction_flag_t)
    }
}
