// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_char, c_int};

pub const PATOMIC_VERSION_MAJOR: i32 = 1;
pub const PATOMIC_VERSION_MINOR: i32 = 1;
pub const PATOMIC_VERSION_PATCH: i32 = 0;
pub const PATOMIC_VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

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

#[cfg(test)]
use core::ffi::CStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_string_is_not_null() {
        let version_ptr = unsafe { patomic_version_string() };
        assert!(!version_ptr.is_null());
    }

    #[test]
    fn version_constants_match_runtime() {
        unsafe {
            assert_eq!(PATOMIC_VERSION_MAJOR, patomic_version_major());
            assert_eq!(PATOMIC_VERSION_MINOR, patomic_version_minor());
            assert_eq!(PATOMIC_VERSION_PATCH, patomic_version_patch());

            let version_ptr = patomic_version_string();
            assert!(!version_ptr.is_null(), "patomic_version_string() returned NULL");

            let version_cstr = CStr::from_ptr(version_ptr).to_str().unwrap();
            assert_eq!(PATOMIC_VERSION_STRING, version_cstr);
        }
    }

    #[test]
    fn version_not_compatible_major_ne() {
        let major = PATOMIC_VERSION_MAJOR;
        for bad_major in [major - 1, major + 1] {
            assert!(!PATOMIC_VERSION_COMPATIBLE_WITH(bad_major, 0, 0));
        }
    }

    #[test]
    fn version_not_compatible_major_eq_minor_gt() {
        let major = PATOMIC_VERSION_MAJOR;
        let minor_gt = PATOMIC_VERSION_MINOR + 1;
        assert!(!PATOMIC_VERSION_COMPATIBLE_WITH(major, minor_gt, 0));
    }

    #[test]
    fn version_not_compatible_major_eq_minor_eq_patch_gt() {
        let major = PATOMIC_VERSION_MAJOR;
        let minor = PATOMIC_VERSION_MINOR;
        let patch_gt = PATOMIC_VERSION_PATCH + 1;
        assert!(!PATOMIC_VERSION_COMPATIBLE_WITH(major, minor, patch_gt));
    }

    #[test]
    fn version_compatible_major_eq_minor_lt_patch_any() {
        let major = PATOMIC_VERSION_MAJOR;
        let minor_lt = PATOMIC_VERSION_MINOR - 1;
        let patch = PATOMIC_VERSION_PATCH;
        for patch_any in [-1, 0, patch - 1, patch, patch + 1] {
            assert!(PATOMIC_VERSION_COMPATIBLE_WITH(
                major, minor_lt, patch_any
            ));
        }
    }

    #[test]
    fn version_compatible_major_eq_minor_eq_patch_le() {
        let major = PATOMIC_VERSION_MAJOR;
        let minor = PATOMIC_VERSION_MINOR;
        let patch = PATOMIC_VERSION_PATCH;
        for patch_le in [patch - 1, patch] {
            assert!(PATOMIC_VERSION_COMPATIBLE_WITH(major, minor, patch_le));
        }
    }
}
