// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod align;
mod ops;

use align::*;

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
