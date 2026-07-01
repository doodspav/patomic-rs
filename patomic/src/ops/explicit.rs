// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};

use patomic_sys::*;

use crate::{AtomicLayout, Error, Ordering, Result, SharedBytesRef};

use crate::ops::macros::{
    do_atomic_checks,
    do_atomic_checks_bit_test,
};

pub trait ExplicitOps: AtomicLayout {
    fn ffi_ops() -> patomic_ops_explicit_t;

    fn store_explicit(
        obj: SharedBytesRef, ordering: Ordering, desired: &[u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops(), fp_store,
            obj, desired,
        );
        if !ordering.is_valid_store_ordering() {
            return Err(Error::InvalidOrdering)
        };
        Ok(unsafe {
            fp_store(
                obj.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
                ordering as c_int,
            )
        })
    }

    fn load_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops(), fp_load,
            obj, ret,
        );
        if !ordering.is_valid_load_ordering() {
            return Err(Error::InvalidOrdering)
        };
        Ok(unsafe {
            fp_load(
                obj.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn exchange_explicit(
        obj: SharedBytesRef, desired: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_exchange,
            obj, desired, ret,
        );
        Ok(unsafe {
            fp_exchange(
                obj.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn compare_exchange_weak_explicit(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> Result<bool> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_weak,
            obj, desired, expected,
        );
        if !fail.is_valid_fail_ordering_for(succ) {
            return Err(Error::InvalidFailOrdering)
        };
        Ok(unsafe {
            fp_cmpxchg_weak(
                obj.as_mut_ptr() as *mut c_void,
                expected.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
                succ as c_int,
                fail as c_int,
            ) != 0
        })
    }

    fn compare_exchange_strong_explicit(
        obj: SharedBytesRef, expected: &mut [u8], desired: &[u8],
        succ: Ordering, fail: Ordering
    ) -> Result<bool> {
        do_atomic_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_strong,
            obj, desired, expected,
        );
        if !fail.is_valid_fail_ordering_for(succ) {
            return Err(Error::InvalidFailOrdering)
        };
        Ok(unsafe {
            fp_cmpxchg_strong(
                obj.as_mut_ptr() as *mut c_void,
                expected.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
                succ as c_int,
                fail as c_int,
            ) != 0
        })
    }

    fn bit_test_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> Result<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test,
            obj; offset
        );
        if !ordering.is_valid_load_ordering() {
            return Err(Error::InvalidOrdering)
        };
        Ok(unsafe {
            fp_test(
                obj.as_ptr() as *const c_void,
                offset as c_int,
                ordering as c_int,
            ) != 0
        })
    }

    fn bit_test_compl_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> Result<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_compl,
            obj; offset
        );
        Ok(unsafe {
            fp_test_compl(
                obj.as_mut_ptr() as *mut c_void,
                offset as c_int,
                ordering as c_int,
            ) != 0
        })
    }

    fn bit_test_set_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> Result<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_set,
            obj; offset
        );
        Ok(unsafe {
            fp_test_set(
                obj.as_mut_ptr() as *mut c_void,
                offset as c_int,
                ordering as c_int,
            ) != 0
        })
    }

    fn bit_test_reset_explicit(
        obj: SharedBytesRef, offset: usize, ordering: Ordering
    ) -> Result<bool> {
        do_atomic_checks_bit_test!(
            Self::ffi_ops().bitwise_ops, fp_test_reset,
            obj; offset
        );
        Ok(unsafe {
            fp_test_reset(
                obj.as_mut_ptr() as *mut c_void,
                offset as c_int,
                ordering as c_int,
            ) != 0
        })
    }

    fn or_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_or,
            obj, arg,
        );
        Ok(unsafe {
            fp_or(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
            )
        })
    }

    fn xor_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_xor,
            obj, arg,
        );
        Ok(unsafe {
            fp_xor(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
            )
        })
    }

    fn and_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_and,
            obj, arg,
        );
        Ok(unsafe {
            fp_and(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
            )
        })
    }

    fn not_explicit(obj: SharedBytesRef, ordering: Ordering) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_not,
            obj,
        );
        Ok(unsafe {
            fp_not(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
            )
        })
    }

    fn fetch_or_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_or,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_or(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_xor_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_xor,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_xor(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_and_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_and,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_and(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_not_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_not,
            obj, ret,
        );
        Ok(unsafe {
            fp_fetch_not(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn add_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_add,
            obj, arg,
        );
        Ok(unsafe {
            fp_add(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
            )
        })
    }

    fn sub_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_sub,
            obj, arg,
        );
        Ok(unsafe {
            fp_sub(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
            )
        })
    }

    fn inc_explicit(obj: SharedBytesRef, ordering: Ordering) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_inc,
            obj,
        );
        Ok(unsafe {
            fp_inc(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
            )
        })
    }

    fn dec_explicit(obj: SharedBytesRef, ordering: Ordering) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_dec,
            obj,
        );
        Ok(unsafe {
            fp_dec(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
            )
        })
    }

    fn neg_explicit(obj: SharedBytesRef, ordering: Ordering) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_neg,
            obj,
        );
        Ok(unsafe {
            fp_neg(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
            )
        })
    }

    fn fetch_add_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_add,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_add(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_sub_explicit(
        obj: SharedBytesRef, arg: &[u8], ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_sub,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_sub(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_inc_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_inc,
            obj, ret,
        );
        Ok(unsafe {
            fp_fetch_inc(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_dec_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_dec,
            obj, ret,
        );
        Ok(unsafe {
            fp_fetch_dec(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_neg_explicit(
        obj: SharedBytesRef, ordering: Ordering, ret: &mut [u8]
    ) -> Result<()> {
        do_atomic_checks!(
            Self::ffi_ops().arithmetic_ops, fp_fetch_neg,
            obj, ret,
        );
        Ok(unsafe {
            fp_fetch_neg(
                obj.as_mut_ptr() as *mut c_void,
                ordering as c_int,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }
}
