// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod align;
mod combine;
mod core;
mod feature_check;
mod ids;
mod memory_order;
mod ops;
mod options;
mod transaction;

pub use align::*;
pub use combine::*;
pub use core::*;
pub use feature_check::*;
pub use ids::*;
pub use memory_order::*;
pub use ops::*;
pub use options::*;
pub use transaction::*;

pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = sub(6, 2);
        assert_eq!(result, 4);
    }
}
