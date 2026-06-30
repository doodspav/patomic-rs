// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidAlignment,
    InvalidSize,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidAlignment => write!(f, "invalid alignment"),
            Self::InvalidSize => write!(f, "invalid size"),
        }
    }
}

impl core::error::Error for Error {}
