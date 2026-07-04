// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::num::NonZeroUsize;

use patomic_sys::*;

use crate::align::{Alignment, AtomicLayout};
use crate::ops::*;

pub struct AtomicBackend {
    width: NonZeroUsize,
    alignment: Alignment,
    ops: patomic_ops_t,
    ops_explicit: patomic_ops_explicit_t,
}

impl AtomicLayout for AtomicBackend {
    fn width(&self) -> NonZeroUsize {
        self.width
    }

    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

unsafe impl FfiOpsImplicit for AtomicBackend {
    fn ffi_ops(&self) -> &patomic_ops_t {
        &self.ops
    }
}

unsafe impl FfiOpsExplicit for AtomicBackend {
    fn ffi_ops(&self) -> &patomic_ops_explicit_t {
        &self.ops_explicit
    }
}

impl UncheckedImplicitOps for AtomicBackend {}
impl UncheckedExplicitOps for AtomicBackend {}

impl ImplicitOps for AtomicBackend {}
impl ExplicitOps for AtomicBackend {}

pub struct TransactionBackend {
    ops_transaction: patomic_ops_transaction_t,
}

unsafe impl FfiOpsTransaction for TransactionBackend {
    fn ffi_ops(&self) -> &patomic_ops_transaction_t {
        &self.ops_transaction
    }
}

impl UncheckedTransactionOps for TransactionBackend {}

impl TransactionOps for TransactionBackend {}
