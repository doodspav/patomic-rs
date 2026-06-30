// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use patomic_sys::*;

pub trait TransactionOps {

    fn ffi_ops() -> patomic_ops_transaction_t;
}
