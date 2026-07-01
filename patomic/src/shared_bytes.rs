// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::cell::UnsafeCell;

#[derive(Clone, Copy)]
pub struct SharedBytesRef<'a> {
    bytes: &'a [UnsafeCell<u8>],
}

unsafe impl Send for SharedBytesRef<'_> {}
unsafe impl Sync for SharedBytesRef<'_> {}

impl<'a> SharedBytesRef<'a> {
    pub fn from_slice(bytes: &'a [UnsafeCell<u8>]) -> Self {
        Self { bytes }
    }

    pub unsafe fn from_raw(bytes: &'a [u8]) -> Self {
        let ptr = bytes as *const [u8] as *const [UnsafeCell<u8>];
        Self {
            bytes: unsafe { &*ptr },
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr().cast()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.bytes.as_ptr() as *mut u8
    }
}
