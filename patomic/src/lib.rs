// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Portable access to lock-free atomic operations at runtime, provided by the
//! [`patomic`] C library.
//!
//! Atomic operations are obtained at runtime from a backend, which is
//! constructed from one or more underlying C implementations, including the
//! `NULL` implementation.
//!
//! Operations that are not supported by any selected implementation on the
//! target platform are reported as unsupported at runtime rather than failing
//! to compile.
//!
//! Transactional operations are also provided in a similar manner, with the
//! caveat that there is no guarantee that a transaction ever succeeds. It
//! should be noted that platform support for lock-free hardware transactional
//! operations is rare in server hardware, and almost non-existent in consumer
//! hardware.
//!
//! Usage is split into stages:
//! - create a backend (see [`backend`])
//! - check that the desired functionality is supported (see [`capabilities`])
//! - _(atomic only)_ check that alignment requirements are met
//!   (see [`Alignment`])
//! - perform the operation (see [`ops`])
//! - _(transaction only)_ check whether the transaction succeeded
//!   (see [`transaction`])
//!
//! This library does not provide standalone atomic or transactional types.
//! Instead, the backends operate on user-provided buffers referenced by
//! [`SharedBytesRef`]s.
//!
//! This crate is `#![no_std]`.
//!
//! [`patomic`]: https://github.com/doodspav/patomic

#![no_std]

mod align;
mod ordering;
mod shared;

pub mod backend;
pub mod capabilities;
pub mod error;
pub mod ops;
pub mod transaction;

pub use align::{Alignment, AtomicLayout};
pub use ordering::Ordering;
pub use shared::{SharedBytesRef, SharedFlagRef};

/// The version of this crate.
///
/// This always matches the version of the underlying [`patomic`] C library
/// exposed as `PATOMIC_VERSION_STRING`, which this crate follows for
/// [Semantic Versioning] purposes.
///
/// [`patomic`]: https://github.com/doodspav.patomic
/// [Semantic Versioning]: https://semver.org
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use patomic_sys as sys;

    #[test]
    fn version_matches_sys_version() {
        assert_eq!(VERSION, sys::PATOMIC_VERSION_STRING);
    }
}
