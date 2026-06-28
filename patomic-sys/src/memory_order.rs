// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;

pub type patomic_memory_order_t = c_int;

pub const patomic_RELAXED: patomic_memory_order_t = 0;
pub const patomic_CONSUME: patomic_memory_order_t = 1;
pub const patomic_ACQUIRE: patomic_memory_order_t = 2;
pub const patomic_RELEASE: patomic_memory_order_t = 3;
pub const patomic_ACQ_REL: patomic_memory_order_t = 4;
pub const patomic_SEQ_CST: patomic_memory_order_t = 5;

#[inline]
pub const fn PATOMIC_IS_VALID_ORDER(order: c_int) -> bool {
    matches!(
        order,
        patomic_RELAXED
            | patomic_CONSUME
            | patomic_ACQUIRE
            | patomic_RELEASE
            | patomic_ACQ_REL
            | patomic_SEQ_CST
    )
}

#[inline]
pub const fn PATOMIC_IS_VALID_STORE_ORDER(order: c_int) -> bool {
    matches!(
        order,
        patomic_RELAXED
            | patomic_RELEASE
            | patomic_SEQ_CST
    )
}

#[inline]
pub const fn PATOMIC_IS_VALID_LOAD_ORDER(order: c_int) -> bool {
    matches!(
        order,
        patomic_RELAXED
            | patomic_CONSUME
            | patomic_ACQUIRE
            | patomic_SEQ_CST
    )
}

#[inline]
pub const fn PATOMIC_IS_VALID_FAIL_ORDER(
    succ: c_int,
    fail: c_int,
) -> bool {
    succ >= fail
        && PATOMIC_IS_VALID_ORDER(succ)
        && PATOMIC_IS_VALID_LOAD_ORDER(fail)
}

#[inline]
pub const fn PATOMIC_CMPXCHG_FAIL_ORDER(
    succ: c_int,
) -> c_int {
    if succ == patomic_ACQ_REL || succ == patomic_RELEASE {
        patomic_ACQUIRE
    } else {
        succ
    }
}

unsafe extern "C" {
    pub fn patomic_is_valid_order(order: c_int) -> c_int;

    pub fn patomic_is_valid_store_order(order: c_int) -> c_int;

    pub fn patomic_is_valid_load_order(order: c_int) -> c_int;

    pub fn patomic_is_valid_fail_order(
        succ: c_int,
        fail: c_int,
    ) -> c_int;

    pub fn patomic_cmpxchg_fail_order(
        succ: c_int,
    ) -> c_int;
}
