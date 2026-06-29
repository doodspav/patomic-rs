// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::cmp::Ordering;
use core::ffi::c_void;

use patomic_sys::*;

#[derive(Debug, Copy, Clone)]
pub struct Alignment {
    pub recommended: usize,
    pub minimum: usize,
    pub size_within: usize,
}

impl From<patomic_align_t> for Alignment {
    fn from(value: patomic_align_t) -> Self {
        Self {
            recommended: value.recommended,
            minimum: value.minimum,
            size_within: value.size_within,
        }
    }
}

impl From<Alignment> for patomic_align_t {
    fn from(value: Alignment) -> Self {
        Self {
            recommended: value.recommended,
            minimum: value.minimum,
            size_within: value.size_within,
        }
    }
}

impl PartialEq for Alignment {
    fn eq(&self, other: &Self) -> bool {
        self.recommended == other.recommended
            && self.minimum == other.minimum
            && self.size_within == other.size_within
    }
}

impl Eq for Alignment {}

impl Ord for Alignment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.recommended
            .cmp(&other.recommended)
            .then_with(|| self.minimum.cmp(&other.minimum))
            .then_with(|| match (self.size_within, other.size_within) {
                (a, b) if a == b => Ordering::Equal,
                (0, _) => Ordering::Less,
                (_, 0) => Ordering::Greater,
                (a, b) => b.cmp(&a),
            })
    }
}

impl PartialOrd for Alignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait AlignmentRequirements {
    fn alignment(&self) -> Alignment;

    fn meets_alignment(&self, bytes: &[u8]) -> bool {
        let ptr = bytes.as_ptr() as *const c_void;
        let align = self.alignment().into();
        unsafe {
            patomic_align_meets_recommended(ptr, align) != 0
                || patomic_align_meets_minimum(ptr, align, bytes.len()) != 0
        }
    }
}
