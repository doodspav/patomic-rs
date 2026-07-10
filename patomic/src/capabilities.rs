// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_uint;

use bitflags::bitflags;

use patomic_sys::*;

use crate::ops::{FfiOpsImplicit, FfiOpsTransaction};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AtomicOpCat {
    Ldst,
    Xchg,
    Bit,
    BinV,
    BinF,
    AriV,
    AriF,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TransactionOpCat {
    Ldst,
    Xchg,
    Bit,
    BinV,
    BinF,
    AriV,
    AriF,
    TSpec,
    TFlag,
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct OpCatSet: u16 {
        const LDST = patomic_opcat_LDST as u16;
        const XCHG = patomic_opcat_XCHG as u16;
        const BIT = patomic_opcat_BIT as u16;
        const BIN_V = patomic_opcat_BIN_V as u16;
        const BIN_F = patomic_opcat_BIN_F as u16;
        const ARI_V = patomic_opcat_ARI_V as u16;
        const ARI_F = patomic_opcat_ARI_F as u16;
        const TSPEC = patomic_opcat_TSPEC as u16;
        const TFLAG = patomic_opcat_TFLAG as u16;

        const ALL_ATOMIC =
            Self::LDST.bits()  |
            Self::XCHG.bits()  |
            Self::BIT.bits()   |
            Self::BIN_V.bits() |
            Self::BIN_F.bits() |
            Self::ARI_V.bits() |
            Self::ARI_F.bits();

        const ALL_TRANSACTION =
            Self::ALL_ATOMIC.bits() |
            Self::TSPEC.bits()      |
            Self::TFLAG.bits();
    }
}

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
        const DOUBLE_CMPXCHG = patomic_opkind_DOUBLE_CMPXCHG as u16;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AtomicOpKind {
    Ldst(LdstOpKind),
    Xchg(XchgOpKind),
    Bit(BitOpKind),
    Bin(BinOpKind),
    Ari(AriOpKind),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TransactionOpKind {
    Ldst(LdstOpKind),
    Xchg(XchgOpKind),
    Bit(BitOpKind),
    Bin(BinOpKind),
    Ari(AriOpKind),
    TSpec(TSpecOpKind),
    TFlag(TFlagOpKind),
}

pub trait AtomicCapabilities: FfiOpsImplicit {
    fn capabilities_all(&self) -> OpCatSet {
        let ops = self.ffi_ops();
        let bits = unsafe { patomic_feature_check_all(
            ops, OpCatSet::ALL_ATOMIC.bits() as c_uint
        ) } as u16;
        let unsupported = OpCatSet::from_bits_retain(bits);
        OpCatSet::ALL_ATOMIC.difference(unsupported)
    }

    fn capabilities_any(&self) -> OpCatSet {
        let ops = self.ffi_ops();
        let bits = unsafe { patomic_feature_check_any(
            ops, OpCatSet::ALL_ATOMIC.bits() as c_uint
        ) } as u16;
        let unsupported = OpCatSet::from_bits_retain(bits);
        OpCatSet::ALL_ATOMIC.difference(unsupported)
    }

    fn capabilities_leaf(&self, cat: AtomicOpCat) -> AtomicOpKind {
        let ops = self.ffi_ops();
        let (cat_bit, kind_bits) = match cat {
            AtomicOpCat::Ldst => (patomic_opcat_LDST, LdstOpKind::all().bits()),
            AtomicOpCat::Xchg => (patomic_opcat_XCHG, XchgOpKind::all().bits()),
            AtomicOpCat::Bit  => (patomic_opcat_BIT,   BitOpKind::all().bits()),
            AtomicOpCat::BinV => (patomic_opcat_BIN_V, BinOpKind::all().bits()),
            AtomicOpCat::BinF => (patomic_opcat_BIN_F, BinOpKind::all().bits()),
            AtomicOpCat::AriV => (patomic_opcat_ARI_V, AriOpKind::all().bits()),
            AtomicOpCat::AriF => (patomic_opcat_ARI_F, AriOpKind::all().bits()),
        };
        let bits = unsafe { patomic_feature_check_leaf(
            ops, cat_bit, kind_bits as c_uint
        ) } as u16;

        macro_rules! leaf {
            ($bits:ident, $Kind:ty, $Variant:ident) => {{
                let unsupported = <$Kind>::from_bits_retain($bits);
                AtomicOpKind::$Variant(<$Kind>::all().difference(unsupported))
            }};
        }

        match cat {
            AtomicOpCat::Ldst => leaf!(bits, LdstOpKind, Ldst),
            AtomicOpCat::Xchg => leaf!(bits, XchgOpKind, Xchg),
            AtomicOpCat::Bit  => leaf!(bits, BitOpKind, Bit),
            AtomicOpCat::BinV | AtomicOpCat::BinF => leaf!(bits, BinOpKind, Bin),
            AtomicOpCat::AriV | AtomicOpCat::AriF => leaf!(bits, AriOpKind, Ari),
        }
    }
}

pub trait TransactionCapabilities: FfiOpsTransaction {
    fn capabilities_all(&self) -> OpCatSet {
        let ops = self.ffi_ops();
        let bits = unsafe { patomic_feature_check_all_transaction(
            ops, OpCatSet::ALL_TRANSACTION.bits() as c_uint
        ) } as u16;
        let unsupported = OpCatSet::from_bits_retain(bits);
        OpCatSet::ALL_TRANSACTION.difference(unsupported)
    }

    fn capabilities_any(&self) -> OpCatSet {
        let ops = self.ffi_ops();
        let bits = unsafe { patomic_feature_check_any_transaction(
            ops, OpCatSet::ALL_TRANSACTION.bits() as c_uint
        ) } as u16;
        let unsupported = OpCatSet::from_bits_retain(bits);
        OpCatSet::ALL_TRANSACTION.difference(unsupported)
    }

    fn capabilities_leaf(&self, cat: TransactionOpCat) -> TransactionOpKind {
        let ops = self.ffi_ops();
        let (cat_bit, kind_bits) = match cat {
            TransactionOpCat::Ldst => (patomic_opcat_LDST, LdstOpKind::all().bits()),
            TransactionOpCat::Xchg => (patomic_opcat_XCHG, XchgOpKind::all().bits()),
            TransactionOpCat::Bit  => (patomic_opcat_BIT,   BitOpKind::all().bits()),
            TransactionOpCat::BinV => (patomic_opcat_BIN_V, BinOpKind::all().bits()),
            TransactionOpCat::BinF => (patomic_opcat_BIN_F, BinOpKind::all().bits()),
            TransactionOpCat::AriV => (patomic_opcat_ARI_V, AriOpKind::all().bits()),
            TransactionOpCat::AriF => (patomic_opcat_ARI_F, AriOpKind::all().bits()),
            TransactionOpCat::TSpec => (patomic_opcat_TSPEC, TSpecOpKind::all().bits()),
            TransactionOpCat::TFlag => (patomic_opcat_TFLAG, TFlagOpKind::all().bits()),
        };
        let bits = unsafe { patomic_feature_check_leaf_transaction(
            ops, cat_bit, kind_bits as c_uint
        ) } as u16;

        macro_rules! leaf {
            ($bits:ident, $Kind:ty, $Variant:ident) => {{
                let unsupported = <$Kind>::from_bits_retain($bits);
                TransactionOpKind::$Variant(
                    <$Kind>::all().difference(unsupported)
                )
            }};
        }

        match cat {
            TransactionOpCat::Ldst => leaf!(bits, LdstOpKind, Ldst),
            TransactionOpCat::Xchg => leaf!(bits, XchgOpKind, Xchg),
            TransactionOpCat::Bit  => leaf!(bits, BitOpKind, Bit),
            TransactionOpCat::BinV | TransactionOpCat::BinF =>
                leaf!(bits, BinOpKind, Bin),
            TransactionOpCat::AriV | TransactionOpCat::AriF =>
                leaf!(bits, AriOpKind, Ari),
            TransactionOpCat::TSpec => leaf!(bits, TSpecOpKind, TSpec),
            TransactionOpCat::TFlag => leaf!(bits, TFlagOpKind, TFlag),
        }
    }
}
