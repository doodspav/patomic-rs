// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidSize,
    InvalidAlignment,
    InvalidMemoryOrder,
    InvalidFailOrder,
    UnsupportedOperation,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSize => write!(f, "invalid size"),
            Self::InvalidAlignment => write!(f, "invalid alignment"),
            Self::InvalidMemoryOrder => write!(f, "invalid memory order"),
            Self::InvalidFailOrder => write!(f, "invalid fail order"),
            Self::UnsupportedOperation => write!(f, "unsupported operation"),
        }
    }
}

impl core::error::Error for Error {}
