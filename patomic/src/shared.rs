// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::cell::UnsafeCell;

/// A shared reference to a byte buffer.
///
/// Objects of this type must only be accessed through lock-free atomic
/// operations, such as those provided by [`AtomicBackend`] and
/// [`TransactionBackend`] (transactions are also lock-free and atomic).
///
/// This type is the buffer that atomic operations in this crate are performed
/// on. It may be freely copied and sent across threads.
///
/// Multiple [`SharedBytesRef`]s may refer to the same buffer at the same time;
/// the atomic operations performed on them are responsible for synchronizing
/// access.
///
/// [`AtomicBackend`]: crate::backend::AtomicBackend
/// [`TransactionBackend`]: crate::backend::TransactionBackend;
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SharedBytesRef<'a> {
    bytes: &'a [UnsafeCell<u8>],
}

// SAFETY: the buffer is only ever accessed through atomic operations, which
// synchronize access from multiple threads.
unsafe impl Send for SharedBytesRef<'_> {}
unsafe impl Sync for SharedBytesRef<'_> {}

impl<'a> SharedBytesRef<'a> {
    /// Creates a [`SharedBytesRef`] from a shared slice of cells.
    ///
    /// Use this constructor when the buffer needs to be accessible from
    /// multiple places at once.
    pub fn from_cells(bytes: &'a [UnsafeCell<u8>]) -> Self {
        Self { bytes }
    }

    /// Creates a [`SharedBytesRef`] from an exclusive slice of bytes.
    ///
    /// The slice is exclusively borrowed for the lifetime of the returned
    /// value (which may itself be copied).
    pub fn from_mut(bytes: &'a mut [u8]) -> Self {
        let ptr = bytes as *mut [u8] as *mut [UnsafeCell<u8>];
        let bytes = unsafe { &*ptr };
        Self { bytes }
    }

    /// Returns the length of the buffer in bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns `true` if the buffer has a length of zero.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns a raw const pointer to the start of the buffer.
    pub fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr() as *const u8
    }

    /// Returns a raw mut pointer to the start of the buffer.
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.bytes.as_ptr() as *mut u8
    }
}

impl<'a> From<&'a mut [u8]> for SharedBytesRef<'a> {
    /// Equivalent to [`SharedBytesRef::from_mut`].
    fn from(bytes: &'a mut [u8]) -> Self {
        Self::from_mut(bytes)
    }
}

impl<'a, const N: usize> From<&'a mut [u8; N]> for SharedBytesRef<'a> {
    /// Equivalent to [`SharedBytesRef::from_mut`].
    fn from(bytes: &'a mut [u8; N]) -> Self {
        Self::from_mut(bytes.as_mut_slice())
    }
}

impl<'a> From<&'a [UnsafeCell<u8>]> for SharedBytesRef<'a> {
    /// Equivalent to [`SharedBytesRef::from_cells`].
    fn from(bytes: &'a [UnsafeCell<u8>]) -> Self {
        Self::from_cells(bytes)
    }
}

impl<'a, const N: usize> From<&'a [UnsafeCell<u8>; N]> for SharedBytesRef<'a> {
    /// Equivalent to [`SharedBytesRef::from_cells`].
    fn from(bytes: &'a [UnsafeCell<u8>; N]) -> Self {
        Self::from_cells(bytes.as_slice())
    }
}

/// A shared reference to a byte flag.
///
/// Objects of this type must only be accessed through lock-free atomic
/// operations, such as the atomic flag operations provided by
/// [`TransactionBackend`].
///
/// This type is the flag which is read from at the start of each transaction
/// attempt. It may be freely copied and sent across threads.
///
/// Multiple [`SharedFlagRef`]s may refer to the same flag at the same time;
/// the atomic flag operations performed on them are responsible for
/// synchronizing access.
///
/// # Note
///
/// Writing to this flag while it is being used in a live transaction operation
/// will cause the transaction to abort. This is because the transaction will
/// read the flag at the start, and modifying a cache line read by a transaction
/// from outside the transaction will cause the transaction to abort.
///
/// This is intentional.
///
/// [`TransactionBackend`]: crate::backend::TransactionBackend
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct SharedFlagRef<'a> {
    flag: &'a UnsafeCell<u8>,
}

// SAFETY: the flag is only ever accessed through atomic operations, which
// synchronize access from multiple threads.
unsafe impl Send for SharedFlagRef<'_> {}
unsafe impl Sync for SharedFlagRef<'_> {}

impl<'a> SharedFlagRef<'a> {
    /// Creates a [`SharedFlagRef`] from a shared cell.
    ///
    /// Use this constructor when the flag needs to be accessible from
    /// multiple places at once.
    pub fn from_cell(flag: &'a UnsafeCell<u8>) -> Self {
        Self { flag }
    }

    /// Creates a [`SharedFlagRef`] from an exclusive reference to a byte.
    ///
    /// The byte is exclusively borrowed for the lifetime of the returned
    /// value (which may itself be copied).
    pub fn from_mut(flag: &'a mut u8) -> Self {
        Self { flag: UnsafeCell::from_mut(flag) }
    }

    /// Returns a raw const pointer to the flag.
    pub fn as_ptr(&self) -> *const u8 {
        self.flag.get()
    }

    /// Returns a raw mut pointer to the flag.
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.flag.get()
    }
}

impl<'a> From<&'a mut u8> for SharedFlagRef<'a> {
    /// Equivalent to [`SharedFlagRef::from_mut`].
    fn from(flag: &'a mut u8) -> Self {
        Self::from_mut(flag)
    }
}

impl<'a> From<&'a UnsafeCell<u8>> for SharedFlagRef<'a> {
    /// Equivalent to [`SharedFlagRef::from_cell`].
    fn from(flag: &'a UnsafeCell<u8>) -> Self {
        Self::from_cell(flag)
    }
}
