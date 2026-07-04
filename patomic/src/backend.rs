// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_uint, c_ulong};
use core::num::NonZeroUsize;

use bitflags::bitflags;

use patomic_sys::*;

use crate::align::{Alignment, AtomicLayout};
use crate::ops::*;

// todo: document that new ids may be added as a minor change
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Id: u32 {
        const NULL = patomic_id_NULL as u32;
        const STDC = patomic_id_STDC as u32;
        const MSVC = patomic_id_MSVC as u32;
    }
}

impl Id {
    pub const ALL: Self = Self::from_bits_retain(u32::MAX);
}

pub struct SingleId(Id);

impl SingleId {
    pub const fn new(id: Id) -> Option<Self> {
        if id.bits().count_ones() <= 1 {
            Some(Self(id))
        } else {
            None
        }
    }

    pub const unsafe fn new_unchecked(id: Id) -> Self {
        Self(id)
    }

    pub const fn get(self) -> Id {
        self.0
    }

    pub fn kind(&self) -> Kind {
        let id = self.0.bits() as c_ulong;
        Kind::from_bits_retain(unsafe {
            patomic_get_kind(id) as u16
        })
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Kind: u16 {
        const UNKN = patomic_kind_UNKN as u16;
        const DYN = patomic_kind_DYN as u16;
        const OS = patomic_kind_OS as u16;
        const LIB = patomic_kind_LIB as u16;
        const BLTN = patomic_kind_BLTN as u16;
        const ASM = patomic_kind_ASM as u16;
    }
}

impl Kind {
    pub fn ids(&self) -> Id {
        let kind = self.bits() as c_uint;
        Id::from_bits_retain(unsafe {
            patomic_get_ids(kind) as u32
        })
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
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
