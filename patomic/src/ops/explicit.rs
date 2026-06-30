// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use patomic_sys::*;
use crate::align::AtomicLayout;

pub trait ExplicitOps: AtomicLayout {

    fn ffi_ops() -> patomic_ops_explicit_t;
}
