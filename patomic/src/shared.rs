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

impl<'a> From<&'a mut [u8]> for SharedBytesRef<'a> {
    fn from(bytes: &'a mut [u8]) -> Self {
        Self::from_mut(bytes)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for SharedBytesRef<'a> {
    fn from(bytes: &'a mut [u8; N]) -> Self {
        Self::from_mut(bytes.as_mut_slice())
    }
}

impl<'a> From<&'a [UnsafeCell<u8>]> for SharedBytesRef<'a> {
    fn from(bytes: &'a [UnsafeCell<u8>]) -> Self {
        Self::from_cells(bytes)
    }
}

impl<'a, const N: usize> From<&'a [UnsafeCell<u8>; N]> for SharedBytesRef<'a> {
    fn from(bytes: &'a [UnsafeCell<u8>; N]) -> Self {
        Self::from_cells(bytes.as_slice())
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
        Self { flag: UnsafeCell::from_mut(flag) }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.flag.get()
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.flag.get()
    }
}

impl<'a> From<&'a mut u8> for SharedFlagRef<'a> {
    fn from(flag: &'a mut u8) -> Self {
        Self::from_mut(flag)
    }
}

impl<'a> From<&'a UnsafeCell<u8>> for SharedFlagRef<'a> {
    fn from(flag: &'a UnsafeCell<u8>) -> Self {
        Self::from_cell(flag)
    }
}
