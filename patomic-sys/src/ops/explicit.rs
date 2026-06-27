// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_void};

pub type patomic_opsig_explicit_store_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        desired: *const c_void,
        order: c_int,
    ),
>;

pub type patomic_opsig_explicit_load_t = Option<
    unsafe extern "C" fn(
        obj: *const c_void,
        order: c_int,
        ret: *mut c_void,
    ),
>;

pub type patomic_opsig_explicit_exchange_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        desired: *const c_void,
        order: c_int,
        ret: *mut c_void,
    ),
>;

pub type patomic_opsig_explicit_cmpxchg_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        expected: *mut c_void,
        desired: *const c_void,
        succ: c_int,
        fail: c_int,
    ) -> c_int,
>;

pub type patomic_opsig_explicit_test_t = Option<
    unsafe extern "C" fn(
        obj: *const c_void,
        offset: c_int,
        order: c_int,
    ) -> c_int,
>;

pub type patomic_opsig_explicit_test_modify_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        offset: c_int,
        order: c_int,
    ) -> c_int,
>;

pub type patomic_opsig_explicit_fetch_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        arg: *const c_void,
        order: c_int,
        ret: *mut c_void,
    ),
>;

pub type patomic_opsig_explicit_fetch_noarg_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        order: c_int,
        ret: *mut c_void,
    ),
>;

pub type patomic_opsig_explicit_void_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        arg: *const c_void,
        order: c_int,
    ),
>;

pub type patomic_opsig_explicit_void_noarg_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        order: c_int,
    ),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_explicit_arithmetic_t {
    pub fp_add: patomic_opsig_explicit_void_t,
    pub fp_sub: patomic_opsig_explicit_void_t,
    pub fp_inc: patomic_opsig_explicit_void_noarg_t,
    pub fp_dec: patomic_opsig_explicit_void_noarg_t,
    pub fp_neg: patomic_opsig_explicit_void_noarg_t,
    pub fp_fetch_add: patomic_opsig_explicit_fetch_t,
    pub fp_fetch_sub: patomic_opsig_explicit_fetch_t,
    pub fp_fetch_inc: patomic_opsig_explicit_fetch_noarg_t,
    pub fp_fetch_dec: patomic_opsig_explicit_fetch_noarg_t,
    pub fp_fetch_neg: patomic_opsig_explicit_fetch_noarg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_explicit_binary_t {
    pub fp_or: patomic_opsig_explicit_void_t,
    pub fp_xor: patomic_opsig_explicit_void_t,
    pub fp_and: patomic_opsig_explicit_void_t,
    pub fp_not: patomic_opsig_explicit_void_noarg_t,
    pub fp_fetch_or: patomic_opsig_explicit_fetch_t,
    pub fp_fetch_xor: patomic_opsig_explicit_fetch_t,
    pub fp_fetch_and: patomic_opsig_explicit_fetch_t,
    pub fp_fetch_not: patomic_opsig_explicit_fetch_noarg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_explicit_bitwise_t {
    pub fp_test: patomic_opsig_explicit_test_t,
    pub fp_test_compl: patomic_opsig_explicit_test_modify_t,
    pub fp_test_set: patomic_opsig_explicit_test_modify_t,
    pub fp_test_reset: patomic_opsig_explicit_test_modify_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_explicit_xchg_t {
    pub fp_exchange: patomic_opsig_explicit_exchange_t,
    pub fp_cmpxchg_weak: patomic_opsig_explicit_cmpxchg_t,
    pub fp_cmpxchg_strong: patomic_opsig_explicit_cmpxchg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_explicit_t {
    pub fp_store: patomic_opsig_explicit_store_t,
    pub fp_load: patomic_opsig_explicit_load_t,
    pub xchg_ops: patomic_ops_explicit_xchg_t,
    pub bitwise_ops: patomic_ops_explicit_bitwise_t,
    pub binary_ops: patomic_ops_explicit_binary_t,
    pub arithmetic_ops: patomic_ops_explicit_arithmetic_t,
}
