// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

macro_rules! do_atomic_checks {
    (
        $self:ident,
        $ops:expr,
        $fp:ident,
        $obj:ident
        $(, $bytes:ident)* $(,)?
    ) => {
        // check that operation is supported
        if $ops.$fp.is_none() {
            return Err(
                $crate::error::AtomicOpError::UnsupportedOperation.into()
            );
        }

        // check that atomic object is suitably aligned
        if !$self.alignment().is_met_by($obj) {
            return Err(
                $crate::error::AtomicOpError::InvalidAlignment.into()
            );
        }

        // check that all objects have the expected width
        {
            let width = $self.width().get();
            if $obj.len() != width {
                return Err(
                    $crate::error::AtomicOpError::InvalidSize.into()
                );
            }
            $(
                if $bytes.len() != width {
                    return Err(
                        $crate::error::AtomicOpError::InvalidSize.into()
                    );
                }
            )*
        }
    };
}

macro_rules! do_atomic_checks_bit_test {
    (
        $self:ident,
        $ops:expr,
        $fp:ident,
        $obj:ident
        $(, $bytes:ident)* ;
        $offset:ident $(,)?
    ) => {
        // do initial checks
        do_atomic_checks!($self, $ops, $fp, $obj $(, $bytes)*);

        // check that offset does not go out of bounds
        let bit_width = $obj.len() * (u8::BITS as usize);
        if $offset >= bit_width || $offset > ::core::ffi::c_int::MAX as usize {
            return Err(
                $crate::error::AtomicBitwiseOpError::InvalidOffset.into()
            );
        }
    };
}

macro_rules! do_transaction_checks {
    (
        $ops:expr,
        $fp:ident,
        $config:ident,
        $obj:ident
        $(, $bytes:ident)* $(,)?
    ) => {
        // check that operation is supported
        if $ops.$fp.is_none() {
            return Err(
                $crate::error::TransactionOpError::UnsupportedOperation.into()
            );
        }

        // check that all objects have the expected width
        {
            let width = $config.width;
            if $obj.len() != width {
                return Err(
                    $crate::error::TransactionOpError::InvalidSize.into()
                );
            }
            $(
                if $bytes.len() != width {
                    return Err(
                        $crate::error::TransactionOpError::InvalidSize.into()
                    );
                }
            )*
        }
    };
}

macro_rules! do_transaction_checks_bit_test {
    (
        $ops:expr,
        $fp:ident,
        $config:ident,
        $obj:ident
        $(, $bytes:ident)* ;
        $offset:ident $(,)?
    ) => {
        // do initial checks
        do_transaction_checks!($ops, $fp, $config, $obj $(, $bytes)*);

        // check that offset does not go out of bounds
        let bit_width = $config.width * (u8::BITS as usize);
        if $offset >= bit_width || $offset > ::core::ffi::c_int::MAX as usize {
            return Err(
                $crate::error::TransactionBitwiseOpError::InvalidOffset.into()
            )
        }
    }
}

pub (crate) use do_atomic_checks;
pub (crate) use do_atomic_checks_bit_test;
pub (crate) use do_transaction_checks;
pub (crate) use do_transaction_checks_bit_test;
