// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;

pub type patomic_option_t = c_int;

pub const patomic_option_NONE: patomic_option_t = 0x0;
