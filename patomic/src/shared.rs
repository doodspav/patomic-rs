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
    pub fn from_cells(bytes: &'a [UnsafeCell<u8>]) -> Self {
        Self { bytes }
    }

    pub fn from_mut(bytes: &'a mut [u8]) -> Self {
        let ptr = bytes as *mut [u8] as *mut [UnsafeCell<u8>];
        let bytes = unsafe { &*ptr };
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

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SharedFlagRef<'a> {
    flag: &'a UnsafeCell<u8>,
}

unsafe impl Send for SharedFlagRef<'_> {}
unsafe impl Sync for SharedFlagRef<'_> {}

impl<'a> SharedFlagRef<'a> {
    pub fn from_cell(flag: &'a UnsafeCell<u8>) -> Self {
        Self { flag }
    }

    pub fn from_mut(flag: &'a mut u8) -> Self {
        let ptr = flag as *mut u8 as *mut UnsafeCell<u8>;
        let flag = unsafe { &*ptr };
        Self { flag }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.flag.get()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.flag.get()
    }
}
