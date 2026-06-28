// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::transaction::*;

use core::ffi::{c_int, c_uchar, c_ulong, c_void};

pub type patomic_opsig_transaction_store_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        desired: *const c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_load_t = Option<
    unsafe extern "C" fn(
        obj: *const c_void,
        ret: *mut c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_exchange_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        desired: *const c_void,
        ret: *mut c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_cmpxchg_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        expected: *mut c_void,
        desired: *const c_void,
        config: patomic_transaction_config_wfb_t,
        result: *mut patomic_transaction_result_wfb_t,
    ) -> c_int,
>;

pub type patomic_opsig_transaction_test_t = Option<
    unsafe extern "C" fn(
        obj: *const c_void,
        offset: c_int,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ) -> c_int,
>;

pub type patomic_opsig_transaction_test_modify_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        offset: c_int,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ) -> c_int,
>;

pub type patomic_opsig_transaction_fetch_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        arg: *const c_void,
        ret: *mut c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_fetch_noarg_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        ret: *mut c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_void_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        arg: *const c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_void_noarg_t = Option<
    unsafe extern "C" fn(
        obj: *mut c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_double_cmpxchg_t = Option<
    unsafe extern "C" fn(
        cxa: patomic_transaction_cmpxchg_t,
        cxb: patomic_transaction_cmpxchg_t,
        config: patomic_transaction_config_wfb_t,
        result: *mut patomic_transaction_result_wfb_t,
    ) -> c_int,
>;

pub type patomic_opsig_transaction_multi_cmpxchg_t = Option<
    unsafe extern "C" fn(
        cxs_buf: *const patomic_transaction_cmpxchg_t,
        cxs_len: usize,
        config: patomic_transaction_config_wfb_t,
        result: *mut patomic_transaction_result_wfb_t,
    ) -> c_int,
>;

pub type patomic_opsig_transaction_generic_t = Option<
    unsafe extern "C" fn(
        function: Option<unsafe extern "C" fn(ctx: *mut c_void)>,
        ctx: *mut c_void,
        config: patomic_transaction_config_t,
        result: *mut patomic_transaction_result_t,
    ),
>;

pub type patomic_opsig_transaction_generic_wfb_t = Option<
    unsafe extern "C" fn(
        function: Option<unsafe extern "C" fn(ctx: *mut c_void)>,
        ctx: *mut c_void,
        fallback_function: Option<unsafe extern "C" fn(ctx: *mut c_void)>,
        fallback_ctx: *mut c_void,
        config: patomic_transaction_config_wfb_t,
        result: *mut patomic_transaction_result_wfb_t,
    ) -> c_int,
>;

pub type patomic_opsig_transaction_flag_test_t = Option<
    unsafe extern "C" fn(flag: *const patomic_transaction_flag_t) -> c_int,
>;

pub type patomic_opsig_transaction_flag_test_set_t = Option<
    unsafe extern "C" fn(flag: *mut patomic_transaction_flag_t) -> c_int,
>;

pub type patomic_opsig_transaction_flag_clear_t = Option<
    unsafe extern "C" fn(flag: *mut patomic_transaction_flag_t),
>;

pub type patomic_opsig_transaction_tbegin_t = Option<
    unsafe extern "C" fn() -> c_ulong,
>;

pub type patomic_opsig_transaction_tcommit_t = Option<
    unsafe extern "C" fn(),
>;

pub type patomic_opsig_transaction_tabort_all_t = Option<
    unsafe extern "C" fn(reason: c_uchar),
>;

pub type patomic_opsig_transaction_tabort_single_t = Option<
    unsafe extern "C" fn(reason: c_uchar, unused_tag_type_parameter: c_int),
>;

pub type patomic_opsig_transaction_ttest_t = Option<
    unsafe extern "C" fn() -> c_int,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_arithmetic_t {
    pub fp_add: patomic_opsig_transaction_void_t,
    pub fp_sub: patomic_opsig_transaction_void_t,
    pub fp_inc: patomic_opsig_transaction_void_noarg_t,
    pub fp_dec: patomic_opsig_transaction_void_noarg_t,
    pub fp_neg: patomic_opsig_transaction_void_noarg_t,
    pub fp_fetch_add: patomic_opsig_transaction_fetch_t,
    pub fp_fetch_sub: patomic_opsig_transaction_fetch_t,
    pub fp_fetch_inc: patomic_opsig_transaction_fetch_noarg_t,
    pub fp_fetch_dec: patomic_opsig_transaction_fetch_noarg_t,
    pub fp_fetch_neg: patomic_opsig_transaction_fetch_noarg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_binary_t {
    pub fp_or: patomic_opsig_transaction_void_t,
    pub fp_xor: patomic_opsig_transaction_void_t,
    pub fp_and: patomic_opsig_transaction_void_t,
    pub fp_not: patomic_opsig_transaction_void_noarg_t,
    pub fp_fetch_or: patomic_opsig_transaction_fetch_t,
    pub fp_fetch_xor: patomic_opsig_transaction_fetch_t,
    pub fp_fetch_and: patomic_opsig_transaction_fetch_t,
    pub fp_fetch_not: patomic_opsig_transaction_fetch_noarg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_bitwise_t {
    pub fp_test: patomic_opsig_transaction_test_t,
    pub fp_test_compl: patomic_opsig_transaction_test_modify_t,
    pub fp_test_set: patomic_opsig_transaction_test_modify_t,
    pub fp_test_reset: patomic_opsig_transaction_test_modify_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_xchg_t {
    pub fp_exchange: patomic_opsig_transaction_exchange_t,
    pub fp_cmpxchg_weak: patomic_opsig_transaction_cmpxchg_t,
    pub fp_cmpxchg_strong: patomic_opsig_transaction_cmpxchg_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_special_t {
    pub fp_double_cmpxchg: patomic_opsig_transaction_double_cmpxchg_t,
    pub fp_multi_cmpxchg: patomic_opsig_transaction_multi_cmpxchg_t,
    pub fp_generic: patomic_opsig_transaction_generic_t,
    pub fp_generic_wfb: patomic_opsig_transaction_generic_wfb_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_flag_t {
    pub fp_test: patomic_opsig_transaction_flag_test_t,
    pub fp_test_set: patomic_opsig_transaction_flag_test_set_t,
    pub fp_clear: patomic_opsig_transaction_flag_clear_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_raw_t {
    pub fp_tbegin: patomic_opsig_transaction_tbegin_t,
    pub fp_tcommit: patomic_opsig_transaction_tcommit_t,
    pub fp_tabort_all: patomic_opsig_transaction_tabort_all_t,
    pub fp_tabort_single: patomic_opsig_transaction_tabort_single_t,
    pub fp_ttest: patomic_opsig_transaction_ttest_t,
    pub fp_tdepth: patomic_opsig_transaction_ttest_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_ops_transaction_t {
    pub fp_store: patomic_opsig_transaction_store_t,
    pub fp_load: patomic_opsig_transaction_load_t,
    pub xchg_ops: patomic_ops_transaction_xchg_t,
    pub bitwise_ops: patomic_ops_transaction_bitwise_t,
    pub binary_ops: patomic_ops_transaction_binary_t,
    pub arithmetic_ops: patomic_ops_transaction_arithmetic_t,
    pub special_ops: patomic_ops_transaction_special_t,
    pub flag_ops: patomic_ops_transaction_flag_t,
    pub raw_ops: patomic_ops_transaction_raw_t,
}
