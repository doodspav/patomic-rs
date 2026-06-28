// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

mod align;
mod ops;

pub use align::*;
pub use ops::*;

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
