// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use patomic_sys::*;

use crate::SharedFlagRef;

pub trait UncheckedTransactionOps {
    fn ffi_ops() -> patomic_ops_transaction_t;
    
    unsafe fn unchecked_flag_test(flag: SharedFlagRef) -> bool {
        let fp_test = Self::ffi_ops().flag_ops.fp_test.unwrap_unchecked();
        fp_test(flag.as_ptr() as *const patomic_transaction_flag_t) != 0
    }
    
    unsafe fn unchecked_flag_test_set(flag: SharedFlagRef) -> bool {
        let fp_test_set = 
            Self::ffi_ops().flag_ops.fp_test_set.unwrap_unchecked();
        fp_test_set(flag.as_mut_ptr() as *mut patomic_transaction_flag_t) != 0
    }
    
    unsafe fn unchecked_flag_clear(flag: SharedFlagRef) {
        let fp_clear = Self::ffi_ops().flag_ops.fp_clear.unwrap_unchecked();
        fp_clear(flag.as_mut_ptr() as *mut patomic_transaction_flag_t)
    }
}
