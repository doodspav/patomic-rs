// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidAlignment,
    InvalidSize,
}

pub type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidAlignment => write!(f, "invalid alignment"),
            Self::InvalidSize => write!(f, "invalid size"),
        }
    }
}

impl core::error::Error for Error {}
