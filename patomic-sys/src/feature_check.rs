// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_uint};

use crate::{
    patomic_ops_t,
    patomic_ops_explicit_t,
    patomic_ops_transaction_t,
};

pub type patomic_opcat_t = c_int;

pub const patomic_opcat_NONE: patomic_opcat_t = 0x0;
pub const patomic_opcat_LDST: patomic_opcat_t = 0x1;
pub const patomic_opcat_XCHG: patomic_opcat_t = 0x2;
pub const patomic_opcat_BIT: patomic_opcat_t = 0x4;
pub const patomic_opcat_BIN_V: patomic_opcat_t = 0x8;
pub const patomic_opcat_BIN_F: patomic_opcat_t = 0x10;
pub const patomic_opcat_ARI_V: patomic_opcat_t = 0x20;
pub const patomic_opcat_ARI_F: patomic_opcat_t = 0x40;
pub const patomic_opcat_TSPEC: patomic_opcat_t = 0x80;
pub const patomic_opcat_TFLAG: patomic_opcat_t = 0x100;
pub const patomic_opcat_TRAW: patomic_opcat_t = 0x200;

pub const patomic_opcats_BIN: patomic_opcat_t =
    patomic_opcat_BIN_V | patomic_opcat_BIN_F;

pub const patomic_opcats_ARI: patomic_opcat_t =
    patomic_opcat_ARI_V | patomic_opcat_ARI_F;

pub const patomic_opcats_IMPLICIT: patomic_opcat_t =
    patomic_opcat_LDST
        | patomic_opcat_XCHG
        | patomic_opcat_BIT
        | patomic_opcats_BIN
        | patomic_opcats_ARI;

pub const patomic_opcats_EXPLICIT: patomic_opcat_t =
    patomic_opcats_IMPLICIT;

pub const patomic_opcats_TRANSACTION: patomic_opcat_t =
    patomic_opcats_EXPLICIT
        | patomic_opcat_TSPEC
        | patomic_opcat_TFLAG
        | patomic_opcat_TRAW;

pub type patomic_opkind_t = c_int;

pub const patomic_opkind_NONE: patomic_opkind_t = 0x0;

pub const patomic_opkind_LOAD: patomic_opkind_t = 0x1;
pub const patomic_opkind_STORE: patomic_opkind_t = 0x2;
pub const patomic_opkinds_LDST: patomic_opkind_t =
    patomic_opkind_LOAD | patomic_opkind_STORE;

pub const patomic_opkind_EXCHANGE: patomic_opkind_t = 0x1;
pub const patomic_opkind_CMPXCHG_WEAK: patomic_opkind_t = 0x2;
pub const patomic_opkind_CMPXCHG_STRONG: patomic_opkind_t = 0x4;
pub const patomic_opkinds_XCHG: patomic_opkind_t =
    patomic_opkind_EXCHANGE
        | patomic_opkind_CMPXCHG_WEAK
        | patomic_opkind_CMPXCHG_STRONG;

pub const patomic_opkind_TEST: patomic_opkind_t = 0x1;
pub const patomic_opkind_TEST_SET: patomic_opkind_t = 0x2;
pub const patomic_opkind_TEST_RESET: patomic_opkind_t = 0x4;
pub const patomic_opkind_TEST_COMPL: patomic_opkind_t = 0x8;
pub const patomic_opkind_CLEAR: patomic_opkind_t = 0x10;

pub const patomic_opkinds_BIT: patomic_opkind_t =
    patomic_opkind_TEST
        | patomic_opkind_TEST_SET
        | patomic_opkind_TEST_RESET
        | patomic_opkind_TEST_COMPL;

pub const patomic_opkinds_TFLAG: patomic_opkind_t =
    patomic_opkind_TEST | patomic_opkind_TEST_SET | patomic_opkind_CLEAR;

pub const patomic_opkind_OR: patomic_opkind_t = 0x1;
pub const patomic_opkind_XOR: patomic_opkind_t = 0x2;
pub const patomic_opkind_AND: patomic_opkind_t = 0x4;
pub const patomic_opkind_NOT: patomic_opkind_t = 0x8;
pub const patomic_opkinds_BIN: patomic_opkind_t =
    patomic_opkind_OR
        | patomic_opkind_XOR
        | patomic_opkind_AND
        | patomic_opkind_NOT;

pub const patomic_opkind_ADD: patomic_opkind_t = 0x1;
pub const patomic_opkind_SUB: patomic_opkind_t = 0x2;
pub const patomic_opkind_INC: patomic_opkind_t = 0x4;
pub const patomic_opkind_DEC: patomic_opkind_t = 0x8;
pub const patomic_opkind_NEG: patomic_opkind_t = 0x10;
pub const patomic_opkinds_ARI: patomic_opkind_t =
    patomic_opkind_ADD
        | patomic_opkind_SUB
        | patomic_opkind_INC
        | patomic_opkind_DEC
        | patomic_opkind_NEG;

pub const patomic_opkind_DOUBLE_CMPXCHG: patomic_opkind_t = 0x1;
pub const patomic_opkind_MULTI_CMPXCHG: patomic_opkind_t = 0x2;
pub const patomic_opkind_GENERIC: patomic_opkind_t = 0x4;
pub const patomic_opkind_GENERIC_WFB: patomic_opkind_t = 0x8;
pub const patomic_opkinds_TSPEC: patomic_opkind_t =
    patomic_opkind_DOUBLE_CMPXCHG
        | patomic_opkind_MULTI_CMPXCHG
        | patomic_opkind_GENERIC
        | patomic_opkind_GENERIC_WFB;

pub const patomic_opkind_TBEGIN: patomic_opkind_t = 0x1;
pub const patomic_opkind_TCOMMIT: patomic_opkind_t = 0x2;
pub const patomic_opkind_TABORT_ALL: patomic_opkind_t = 0x4;
pub const patomic_opkind_TABORT_SINGLE: patomic_opkind_t = 0x8;
pub const patomic_opkind_TTEST: patomic_opkind_t = 0x10;
pub const patomic_opkind_TDEPTH: patomic_opkind_t = 0x20;
pub const patomic_opkinds_TRAW: patomic_opkind_t =
    patomic_opkind_TBEGIN
        | patomic_opkind_TCOMMIT
        | patomic_opkind_TABORT_ALL
        | patomic_opkind_TABORT_SINGLE
        | patomic_opkind_TTEST
        | patomic_opkind_TDEPTH;

unsafe extern "C" {
    pub fn patomic_feature_check_all(
        ops: *const patomic_ops_t,
        opcats: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_all_explicit(
        ops: *const patomic_ops_explicit_t,
        opcats: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_all_transaction(
        ops: *const patomic_ops_transaction_t,
        opcats: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_any(
        ops: *const patomic_ops_t,
        opcats: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_any_explicit(
        ops: *const patomic_ops_explicit_t,
        opcats: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_any_transaction(
        ops: *const patomic_ops_transaction_t,
        opcats: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_leaf(
        ops: *const patomic_ops_t,
        opcat: patomic_opcat_t,
        opkinds: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_leaf_explicit(
        ops: *const patomic_ops_explicit_t,
        opcat: patomic_opcat_t,
        opkinds: c_uint,
    ) -> c_uint;

    pub fn patomic_feature_check_leaf_transaction(
        ops: *const patomic_ops_transaction_t,
        opcat: patomic_opcat_t,
        opkinds: c_uint,
    ) -> c_uint;
}
