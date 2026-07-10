// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use bitflags::bitflags;

use patomic_sys::*;

use crate::ops::{FfiOpsImplicit, FfiOpsTransaction};

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct LdstOpKind: u16 {
        const LOAD = patomic_opkind_LOAD as u16;
        const STORE = patomic_opkind_STORE as u16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct XchgOpKind: u16 {
        const EXCHANGE = patomic_opkind_EXCHANGE as u16;
        const CMPXCHG_WEAK = patomic_opkind_CMPXCHG_WEAK as u16;
        const CMPXCHG_STRONG = patomic_opkind_CMPXCHG_STRONG as u16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BitOpKind: u16 {
        const TEST = patomic_opkind_TEST as u16;
        const TEST_SET = patomic_opkind_TEST_SET as u16;
        const TEST_RESET = patomic_opkind_TEST_RESET as u16;
        const TEST_COMPL = patomic_opkind_TEST_COMPL as u16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct BinOpKind: u16 {
        const OR = patomic_opkind_OR as u16;
        const XOR = patomic_opkind_XOR as u16;
        const AND = patomic_opkind_AND as u16;
        const NOT = patomic_opkind_NOT as u16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct AriOpKind: u16 {
        const ADD = patomic_opkind_ADD as u16;
        const SUB = patomic_opkind_SUB as u16;
        const INC  = patomic_opkind_INC as u16;
        const DEC = patomic_opkind_DEC as u16;
        const NEG = patomic_opkind_NEG as u16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TSpecOpKind: u16 {
        const MULTI_CMPXCHG = patomic_opkind_MULTI_CMPXCHG as u16;
        const GENERIC = patomic_opkind_GENERIC as u16;
        const GENERIC_WFB = patomic_opkind_GENERIC_WFB as u16;
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct TFlagOpKind: u16 {
        const TEST = patomic_opkind_TEST as u16;
        const TEST_SET = patomic_opkind_TEST_SET as u16;
        const CLEAR = patomic_opkind_CLEAR as u16;
    }
}

pub enum AtomicOpKind {
    Ldst(LdstOpKind),
    Xchg(XchgOpKind),
    Bit(BitOpKind),
    BinV(BinOpKind),
    BinF(BinOpKind),
    AriV(AriOpKind),
    AriF(AriOpKind),
}

pub enum TransactionOpKind {
    Ldst(LdstOpKind),
    Xchg(XchgOpKind),
    Bit(BitOpKind),
    BinV(BinOpKind),
    BinF(BinOpKind),
    AriV(AriOpKind),
    AriF(AriOpKind),
    TSpec(TSpecOpKind),
    TFlag(TFlagOpKind),
}
