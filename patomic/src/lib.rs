// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod align;
mod backend;
mod ordering;
mod shared;
mod transaction;

pub mod error;
pub mod ops;

pub use align::{Alignment, AtomicLayout};
pub use backend::{AtomicBackend, TransactionBackend};
pub use error::{
    AtomicError, AtomicResult,
    TransactionError, TransactionResult,
};
pub use ordering::Ordering;
pub use shared::{SharedBytesRef, SharedFlagRef};

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
