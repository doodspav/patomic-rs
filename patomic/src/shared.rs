// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::cell::UnsafeCell;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SharedBytesRef<'a> {
    bytes: &'a [UnsafeCell<u8>],
}

unsafe impl Send for SharedBytesRef<'_> {}
unsafe impl Sync for SharedBytesRef<'_> {}

impl<'a> SharedBytesRef<'a> {
    pub fn from_slice(bytes: &'a [UnsafeCell<u8>]) -> Self {
        Self { bytes }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr() as *const u8
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.bytes.as_ptr() as *mut u8
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct SharedFlag {
    flag: UnsafeCell<u8>,
}

unsafe impl Send for SharedFlag {}
unsafe impl Sync for SharedFlag {}

impl SharedFlag {
    pub fn as_ptr(&self) -> *const u8 {
        self.flag.get()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.flag.get()
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SharedFlagRef<'a> {
    flag: &'a SharedFlag,
}

unsafe impl Send for SharedFlagRef<'_> {}
unsafe impl Sync for SharedFlagRef<'_> {}

impl<'a> SharedFlagRef<'a> {
    pub fn as_ptr(&self) -> *const u8 {
        self.flag.as_ptr()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.flag.as_mut_ptr()
    }
}
