// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_uint, c_ulong};

pub type patomic_id_t = c_ulong;

pub const patomic_ids_ALL: patomic_id_t = !0;
pub const patomic_id_NULL: patomic_id_t = 0;
pub const patomic_id_STDC: patomic_id_t = 1 << 0;
pub const patomic_id_MSVC: patomic_id_t = 1 << 1;

pub type patomic_kind_t = c_int;

pub const patomic_kind_UNKN: patomic_kind_t = 0x0;
pub const patomic_kind_DYN: patomic_kind_t = 0x1;
pub const patomic_kind_OS: patomic_kind_t = 0x2;
pub const patomic_kind_LIB: patomic_kind_t = 0x4;
pub const patomic_kind_BLTN: patomic_kind_t = 0x8;
pub const patomic_kind_ASM: patomic_kind_t = 0x10;

pub const patomic_kinds_ALL: patomic_kind_t =
    patomic_kind_DYN
        | patomic_kind_OS
        | patomic_kind_LIB
        | patomic_kind_BLTN
        | patomic_kind_ASM;

unsafe extern "C" {
    pub fn patomic_get_ids(kinds: c_uint) -> c_ulong;

    pub fn patomic_get_kind(id: c_ulong) -> c_uint;
}
