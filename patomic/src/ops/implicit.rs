// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};

use patomic_sys::*;

use crate::align::AtomicLayout;
use crate::error::{Error, Result};

macro_rules! do_implicit_checks {
    (
        $ops:expr,
        $fp:ident,
        $obj:ident
        $(, $bytes:ident)* $(,)?
    ) => {
        // check that operation is supported
        let Some($fp) = $ops.$fp else {
            return Err(Error::UnsupportedOperation);
        };

        // check that atomic object is suitably aligned
        if !Self::alignment().is_met_by($obj) {
            return Err(Error::InvalidAlignment);
        }

        // check that all objects have the expected width
        {
            let width = Self::width().get();
            if $obj.len() != width {
                return Err(Error::InvalidSize);
            }
            $(
                if $bytes.len() != width {
                    return Err(Error::InvalidSize);
                }
            )*
        }
    };
}

pub trait ImplicitOps: AtomicLayout {
    fn ffi_ops() -> patomic_ops_t;

    fn store(obj: &mut [u8], desired: &[u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops(), fp_store,
            obj, desired,
        );
        Ok(unsafe {
            fp_store(
                obj.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
            )
        })
    }

    fn load(obj: &[u8], ret: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops(), fp_load,
            obj, ret,
        );
        Ok(unsafe {
            fp_load(
                obj.as_ptr() as *const c_void,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn exchange(obj: &mut [u8], desired: &[u8], ret: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().xchg_ops, fp_exchange,
            obj, desired, ret,
        );
        Ok(unsafe {
            fp_exchange(
                obj.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn compare_exchange_weak(
        obj: &mut [u8], expected: &mut [u8], desired: &[u8]
    ) -> Result<bool> {
        do_implicit_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_weak,
            obj, desired, expected,
        );
        Ok(unsafe {
            fp_cmpxchg_weak(
                obj.as_mut_ptr() as *mut c_void,
                expected.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
            ) != 0
        })
    }

    fn compare_exchange_strong(
        obj: &mut [u8], expected: &mut [u8], desired: &[u8]
    ) -> Result<bool> {
        do_implicit_checks!(
            Self::ffi_ops().xchg_ops, fp_cmpxchg_strong,
            obj, desired, expected,
        );
        Ok(unsafe {
            fp_cmpxchg_strong(
                obj.as_mut_ptr() as *mut c_void,
                expected.as_mut_ptr() as *mut c_void,
                desired.as_ptr() as *const c_void,
            ) != 0
        })
    }

    fn bit_test(obj: &[u8], offset: usize) -> Result<bool> {
        todo!()
    }

    fn bit_test_compl(obj: &mut [u8], offset: usize) -> Result<bool> {
        todo!()
    }

    fn bit_test_set(obj: &mut [u8], offset: usize) -> Result<bool> {
        todo!()
    }

    fn bit_test_reset(obj: &mut [u8], offset: usize) -> Result<bool> {
        todo!()
    }

    fn or(obj: &mut [u8], arg: &[u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_or,
            obj, arg,
        );
        Ok(unsafe {
            fp_or(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
            )
        })
    }

    fn xor(obj: &mut [u8], arg: &[u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_xor,
            obj, arg,
        );
        Ok(unsafe {
            fp_xor(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
            )
        })
    }

    fn and(obj: &mut [u8], arg: &[u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_and,
            obj, arg,
        );
        Ok(unsafe {
            fp_and(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
            )
        })
    }

    fn not(obj: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_not,
            obj,
        );
        Ok(unsafe {
            fp_not(obj.as_mut_ptr() as *mut c_void)
        })
    }

    fn fetch_or(obj: &mut [u8], arg: &[u8], ret: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_or,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_or(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_xor(obj: &mut [u8], arg: &[u8], ret: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_xor,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_xor(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_and(obj: &mut [u8], arg: &[u8], ret: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_and,
            obj, arg, ret,
        );
        Ok(unsafe {
            fp_fetch_and(
                obj.as_mut_ptr() as *mut c_void,
                arg.as_ptr() as *const c_void,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }

    fn fetch_not(obj: &mut [u8], ret: &mut [u8]) -> Result<()> {
        do_implicit_checks!(
            Self::ffi_ops().binary_ops, fp_fetch_not,
            obj, ret,
        );
        Ok(unsafe {
            fp_fetch_not(
                obj.as_mut_ptr() as *mut c_void,
                ret.as_mut_ptr() as *mut c_void,
            )
        })
    }
}
