// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    patomic_t,
    patomic_explicit_t,
};

unsafe extern "C" {
    pub fn patomic_combine(
        priority: *mut patomic_t,
        other: *const patomic_t,
    );

    pub fn patomic_combine_explicit(
        priority: *mut patomic_explicit_t,
        other: *const patomic_explicit_t,
    );
}
