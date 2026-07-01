// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};

use patomic_sys::*;

use crate::Ordering;
use crate::SharedBytesRef;
use crate::align::AtomicLayout;
use crate::error::{Error, Result};

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
}
