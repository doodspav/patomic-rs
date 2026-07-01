// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicError {
    InvalidSize,
    InvalidAlignment,
    InvalidOffset,
    InvalidOrdering,
    UnsupportedOperation,
}

pub type AtomicResult<T> = Result<T, AtomicError>;

impl core::fmt::Display for AtomicError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidSize => write!(f, "invalid size"),
            Self::InvalidAlignment => write!(f, "invalid alignment"),
            Self::InvalidOffset => write!(f, "invalid offset"),
            Self::InvalidOrdering => write!(f, "invalid ordering"),
            Self::UnsupportedOperation => write!(f, "unsupported operation"),
        }
    }
}

impl core::error::Error for AtomicError {}
