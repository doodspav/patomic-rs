// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};
use core::mem::{ManuallyDrop, MaybeUninit};
use patomic_sys::*;

use crate::{SharedBytesRef, SharedFlagRef, transaction::*};

extern "C" fn call_closure<F: FnOnce()>(ctx: *mut c_void) {
    unsafe { ctx.cast::<F>().read()() }
}

pub unsafe trait FfiOpsTransaction {
    fn ffi_ops(&self) -> patomic_ops_transaction_t;
}

pub trait UncheckedTransactionOps: FfiOpsTransaction {
    unsafe fn unchecked_store_transaction(
        &self, obj: SharedBytesRef, desired: &[u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_store = self.ffi_ops().fp_store.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_store(
            obj.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_load_transaction(
        &self, obj: SharedBytesRef, ret: &mut [u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_load = self.ffi_ops().fp_load.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_load(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_exchange_transaction(
        &self, obj: SharedBytesRef, desired: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_exchange =
            self.ffi_ops().xchg_ops.fp_exchange.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_exchange(
            obj.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_compare_exchange_weak_transaction(
        &self, obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        config: TransactionConfigWfb
    ) -> (bool, TransactionOutcomeWfb) {
        let fp_cmpxchg_weak =
            self.ffi_ops().xchg_ops.fp_cmpxchg_weak.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let ok = fp_cmpxchg_weak(
            obj.as_mut_ptr() as *mut c_void,
            expected.as_mut_ptr() as *mut c_void,
            desired.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        ) != 0;
        (ok, outcome.assume_init().into())
    }

    unsafe fn unchecked_bit_test_transaction(
        &self, obj: SharedBytesRef, offset: usize, config: TransactionConfig
    ) -> (bool, TransactionOutcome) {
        let fp_test = self.ffi_ops().bitwise_ops.fp_test.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let bit = fp_test(
            obj.as_mut_ptr() as *const c_void,
            offset as c_int,
            config.into(),
            outcome.as_mut_ptr(),
        ) != 0;
        (bit, outcome.assume_init().into())
    }

    unsafe fn unchecked_bit_test_compl_transaction(
        &self, obj: SharedBytesRef, offset: usize, config: TransactionConfig
    ) -> (bool, TransactionOutcome) {
        let fp_test_compl =
            self.ffi_ops().bitwise_ops.fp_test_compl.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let bit = fp_test_compl(
            obj.as_mut_ptr() as *mut c_void,
            offset as c_int,
            config.into(),
            outcome.as_mut_ptr(),
        ) != 0;
        (bit, outcome.assume_init().into())
    }

    unsafe fn unchecked_bit_test_set_transaction(
        &self, obj: SharedBytesRef, offset: usize, config: TransactionConfig
    ) -> (bool, TransactionOutcome) {
        let fp_test_set =
            self.ffi_ops().bitwise_ops.fp_test_set.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let bit = fp_test_set(
            obj.as_mut_ptr() as *mut c_void,
            offset as c_int,
            config.into(),
            outcome.as_mut_ptr(),
        ) != 0;
        (bit, outcome.assume_init().into())
    }

    unsafe fn unchecked_bit_test_reset_transaction(
        &self, obj: SharedBytesRef, offset: usize, config: TransactionConfig
    ) -> (bool, TransactionOutcome) {
        let fp_test_reset =
            self.ffi_ops().bitwise_ops.fp_test_reset.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let bit = fp_test_reset(
            obj.as_mut_ptr() as *mut c_void,
            offset as c_int,
            config.into(),
            outcome.as_mut_ptr(),
        ) != 0;
        (bit, outcome.assume_init().into())
    }

    unsafe fn unchecked_or_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_or = self.ffi_ops().binary_ops.fp_or.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_or(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_xor_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_xor = self.ffi_ops().binary_ops.fp_xor.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_xor(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_and_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_and = self.ffi_ops().binary_ops.fp_and.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_and(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_not_transaction(
        &self, obj: SharedBytesRef, config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_not = self.ffi_ops().binary_ops.fp_not.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_not(
            obj.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_or_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_or =
            self.ffi_ops().binary_ops.fp_fetch_or.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_or(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_xor_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_xor =
            self.ffi_ops().binary_ops.fp_fetch_xor.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_xor(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_and_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_and =
            self.ffi_ops().binary_ops.fp_fetch_and.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_and(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_not_transaction(
        &self, obj: SharedBytesRef, ret: &mut [u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_not =
            self.ffi_ops().binary_ops.fp_fetch_not.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_not(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_add_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_add = self.ffi_ops().arithmetic_ops.fp_add.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_add(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_sub_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_sub = self.ffi_ops().arithmetic_ops.fp_sub.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_sub(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_inc_transaction(
        &self, obj: SharedBytesRef, config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_inc = self.ffi_ops().arithmetic_ops.fp_inc.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_inc(
            obj.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_dec_transaction(
        &self, obj: SharedBytesRef, config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_dec = self.ffi_ops().arithmetic_ops.fp_dec.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_dec(
            obj.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_neg_transaction(
        &self, obj: SharedBytesRef, config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_neg = self.ffi_ops().arithmetic_ops.fp_neg.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_neg(
            obj.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_add_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_add =
            self.ffi_ops().arithmetic_ops.fp_fetch_add.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_add(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_sub_transaction(
        &self, obj: SharedBytesRef, arg: &[u8], ret: &mut [u8],
        config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_sub =
            self.ffi_ops().arithmetic_ops.fp_fetch_sub.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_sub(
            obj.as_mut_ptr() as *mut c_void,
            arg.as_ptr() as *const c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_inc_transaction(
        &self, obj: SharedBytesRef, ret: &mut [u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_inc =
            self.ffi_ops().arithmetic_ops.fp_fetch_inc.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_inc(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_dec_transaction(
        &self, obj: SharedBytesRef, ret: &mut [u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_dec =
            self.ffi_ops().arithmetic_ops.fp_fetch_dec.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_dec(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_fetch_neg_transaction(
        &self, obj: SharedBytesRef, ret: &mut [u8], config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_fetch_neg =
            self.ffi_ops().arithmetic_ops.fp_fetch_neg.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_fetch_neg(
            obj.as_mut_ptr() as *mut c_void,
            ret.as_mut_ptr() as *mut c_void,
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_double_cmpxchg_transaction(
        &self, cxa: CmpxchgOperands, cxb: CmpxchgOperands,
        config: TransactionConfigWfb
    ) -> TransactionOutcomeWfb {
        let fp_double_cmpxchg =
            self.ffi_ops().special_ops.fp_double_cmpxchg.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_double_cmpxchg(
            cxa.into(),
            cxb.into(),
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_multi_cmpxchg_transaction(
        &self, cxs: &[CmpxchgOperands], config: TransactionConfigWfb
    ) -> TransactionOutcomeWfb {
        let fp_multi_cmpxchg =
            self.ffi_ops().special_ops.fp_multi_cmpxchg.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        fp_multi_cmpxchg(
            cxs.as_ptr() as *const patomic_transaction_cmpxchg_t,
            cxs.len(),
            config.into(),
            outcome.as_mut_ptr(),
        );
        outcome.assume_init().into()
    }

    unsafe fn unchecked_generic_transaction<F: FnOnce()>(
        &self, closure: F, config: TransactionConfig
    ) -> TransactionOutcome {
        let fp_generic =
            self.ffi_ops().special_ops.fp_generic.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let mut closure = ManuallyDrop::new(closure);
        let width = config.width;

        fp_generic(
            Some(call_closure::<F>),
            (&raw mut closure).cast(),
            config.into(),
            outcome.as_mut_ptr(),
        );

        let outcome: TransactionOutcome = outcome.assume_init().into();
        let is_closure_called = width != 0
            && outcome.status.exit_code == TransactionExitCode::Success;
        if !is_closure_called {
            ManuallyDrop::drop(&mut closure);
        }
        outcome
    }

    unsafe fn unchecked_generic_wfb_transaction<F: FnOnce(), G: FnOnce()>(
        &self, closure: F, fallback_closure: G, config: TransactionConfigWfb
    ) -> TransactionOutcomeWfb {
        let fp_generic_wfb =
            self.ffi_ops().special_ops.fp_generic_wfb.unwrap_unchecked();
        let mut outcome = MaybeUninit::uninit();
        let mut closure = ManuallyDrop::new(closure);
        let mut fallback_closure = ManuallyDrop::new(fallback_closure);
        let width = config.width;

        fp_generic_wfb(
            Some(call_closure::<F>),
            (&raw mut closure).cast(),
            Some(call_closure::<G>),
            (&raw mut fallback_closure).cast(),
            config.into(),
            outcome.as_mut_ptr(),
        );

        let outcome: TransactionOutcomeWfb = outcome.assume_init().into();
        let is_closure_called = width != 0
            && outcome.status.exit_code == TransactionExitCode::Success;
        let is_fallback_closure_called = width != 0
            && outcome.status.exit_code != TransactionExitCode::Success
            && outcome.fallback_status.exit_code == TransactionExitCode::Success;
        if !is_closure_called {
            ManuallyDrop::drop(&mut closure);
        }
        if !is_fallback_closure_called {
            ManuallyDrop::drop(&mut fallback_closure);
        }
        outcome
    }

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
