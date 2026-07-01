// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

macro_rules! do_atomic_checks {
    (
        $ops:expr,
        $fp:ident,
        $obj:ident
        $(, $bytes:ident)* $(,)?
    ) => {
        // check that operation is supported
        let Some($fp) = $ops.$fp else {
            return Err(AtomicError::UnsupportedOperation);
        };

        // check that atomic object is suitably aligned
        if !Self::alignment().is_met_by($obj) {
            return Err(AtomicError::InvalidAlignment);
        }

        // check that all objects have the expected width
        {
            let width = Self::width().get();
            if $obj.len() != width {
                return Err(AtomicError::InvalidSize);
            }
            $(
                if $bytes.len() != width {
                    return Err(AtomicError::InvalidSize);
                }
            )*
        }
    };
}

macro_rules! do_atomic_checks_bit_test {
    (
        $ops:expr,
        $fp:ident,
        $obj:ident
        $(, $bytes:ident)* ;
        $offset:ident $(,)?
    ) => {
        // do initial checks
        do_atomic_checks!($ops, $fp, $obj $(, $bytes)*);

        // check that offset does not go out of bounds
        let bit_width = $obj.len() * (u8::BITS as usize);
        if $offset >= bit_width || $offset > c_int::MAX as usize {
            return Err(AtomicError::InvalidOffset);
        }
    };
}

pub (crate) use do_atomic_checks;
pub (crate) use do_atomic_checks_bit_test;
