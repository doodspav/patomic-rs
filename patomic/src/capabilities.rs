// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use bitflags::bitflags;

use patomic_sys::*;

use crate::ops::{FfiOpsImplicit, FfiOpsTransaction};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OpCat {
    Ldst,
    Xchg,
    Bit,
    BinV,
    BinF,
    AriV,
    AriF,
    TSpec,
    TFlag,
    TRaw,
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
        const TRAW = patomic_opcat_TRAW as u16;
    }
}
