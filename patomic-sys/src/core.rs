// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    patomic_align_t,
    patomic_ops_t,
    patomic_ops_explicit_t,
    patomic_ops_transaction_t,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_t {
    pub ops: patomic_ops_t,
    pub align: patomic_align_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_explicit_t {
    pub ops: patomic_ops_explicit_t,
    pub align: patomic_align_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_t {
    pub ops: patomic_ops_transaction_t,
}
