// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_char, c_int};

pub const PATOMIC_VERSION_MAJOR: i32 = 1;
pub const PATOMIC_VERSION_MINOR: i32 = 1;
pub const PATOMIC_VERSION_PATCH: i32 = 0;
pub const PATOMIC_VERSION_STRING: &str = "1.1.0";

#[inline]
pub const fn PATOMIC_VERSION_COMPATIBLE_WITH(
    major: i32,
    minor: i32,
    patch: i32,
) -> bool {
    major == PATOMIC_VERSION_MAJOR
        && minor <= PATOMIC_VERSION_MINOR
        && (minor < PATOMIC_VERSION_MINOR || patch <= PATOMIC_VERSION_PATCH)
}

unsafe extern "C" {
    pub fn patomic_version_string() -> *const c_char;

    pub fn patomic_version_major() -> c_int;

    pub fn patomic_version_minor() -> c_int;

    pub fn patomic_version_patch() -> c_int;

    pub fn patomic_version_compatible_with(
        major: c_int,
        minor: c_int,
        patch: c_int,
    ) -> c_int;
}
