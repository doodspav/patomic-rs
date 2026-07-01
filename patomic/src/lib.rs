// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod align;
mod error;
mod ordering;
pub mod ops;
mod shared_bytes;

pub use align::*;
pub use error::*;
pub use ordering::*;
pub use shared_bytes::*;

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
