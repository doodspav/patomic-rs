// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Low-level bindings to the [`patomic`] C library.
//! 
//! No documentation is provided, since it would be an unnecessary duplicate of
//! existing documentation for the underlying library.
//! 
//! The only tests provided for this crate are those which test Rust const
//! functions which replace C macros.
//! 
//! This crate's version will be kept in sync with the version of the underlying
//! library.
//! 
//! [`patomic`]: https://github.com/doodspav/patomic

#![no_std]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use ::core::ffi::{c_uint, c_ulong};

mod align;
mod combine;
mod core;
mod feature_check;
mod ids;
mod memory_order;
mod ops;
mod options;
mod transaction;
mod version;

pub use align::*;
pub use combine::*;
pub use core::*;
pub use feature_check::*;
pub use ids::*;
pub use memory_order::*;
pub use ops::*;
pub use options::*;
pub use transaction::*;
pub use version::*;

unsafe extern "C" {
    pub fn patomic_create(
        byte_width: usize,
        order: patomic_memory_order_t,
        options: c_uint,
        kinds: c_uint,
        ids: c_ulong,
    ) -> patomic_t;

    pub fn patomic_create_explicit(
        byte_width: usize,
        options: c_uint,
        kinds: c_uint,
        ids: c_ulong,
    ) -> patomic_explicit_t;

    pub fn patomic_create_transaction(
        options: c_uint,
        kinds: c_uint,
        ids: c_ulong,
    ) -> patomic_transaction_t;
}
