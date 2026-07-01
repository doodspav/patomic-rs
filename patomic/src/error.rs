// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidSize,
    InvalidAlignment,
    InvalidOffset,
    InvalidOrdering,
    InvalidFailOrdering,
    UnsupportedOperation,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSize => write!(f, "invalid size"),
            Self::InvalidAlignment => write!(f, "invalid alignment"),
            Self::InvalidOffset => write!(f, "invalid offset"),
            Self::InvalidOrdering => write!(f, "invalid ordering"),
            Self::InvalidFailOrdering => write!(f, "invalid fail ordering"),
            Self::UnsupportedOperation => write!(f, "unsupported operation"),
        }
    }
}

impl core::error::Error for Error {}
