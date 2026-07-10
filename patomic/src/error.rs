// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[allow(non_upper_case_globals)]
mod messages {
    pub const UnsupportedOperation: &str = "The operation is not supported by this backend on this platform";
    pub const InvalidSize: &str = "The size of an object used by this operation does not match the expected width";
    pub const InvalidAlignment: &str = "The alignment of the shared object is insufficient for this operation on this backend and platform";
    pub const InvalidOffset: &str = "The offset used would exceed the bounds of the shared object";
    pub const InvalidOrdering: &str = "The ordering used is not valid for this operation";
}

macro_rules! define_error {
    // Without conversions.
    ($name:ident $variants:tt) => {
        define_error!(@define $name $variants);
    };

    // With conversions: targets in brackets.
    ($name:ident => [$($into:ty),+ $(,)?] $variants:tt) => {
        define_error!(@define $name $variants);
        $( define_error!(@from $name => $into $variants); )+
    };

    (@define $name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        pub enum $name {
            $($variant,)+
        }

        impl ::core::fmt::Display for $name {
            fn fmt(
                &self, f: &mut ::core::fmt::Formatter<'_>
            ) -> ::core::fmt::Result {
                f.write_str(match self {
                    $(Self::$variant => $crate::error::messages::$variant,)+
                })
            }
        }

        impl ::core::error::Error for $name {}
    };

    (@from $name:ident => $into:ty { $($variant:ident),+ $(,)? }) => {
        impl ::core::convert::From<$name> for $into {
            fn from(e: $name) -> Self {
                match e {
                    $(<$name>::$variant => Self::$variant,)+
                }
            }
        }
    };
}

define_error!(
    AtomicError {
        UnsupportedOperation,
        InvalidSize,
        InvalidAlignment,
        InvalidOffset,
        InvalidOrdering,
    }
);

pub type AtomicResult<T> = Result<T, AtomicError>;

define_error!(
    AtomicExplicitBitTestOpError => [
        AtomicError,
    ] {
        UnsupportedOperation,
        InvalidSize,
        InvalidAlignment,
        InvalidOffset,
        InvalidOrdering,
    }
);

pub type AtomicExplicitBitTestOpResult<T> =
    Result<T, AtomicExplicitBitTestOpError>;

define_error!(
    AtomicExplicitAccessOpError => [
        AtomicError,
    ] {
        UnsupportedOperation,
        InvalidSize,
        InvalidAlignment,
        InvalidOrdering,
    }
);

pub type AtomicExplicitAccessOpResult<T> =
    Result<T, AtomicExplicitAccessOpError>;

define_error!(
    AtomicBitwiseOpError => [
        AtomicExplicitBitTestOpError,
        AtomicError,
    ] {
        UnsupportedOperation,
        InvalidSize,
        InvalidAlignment,
        InvalidOffset,
    }
);

pub type AtomicBitwiseOpResult<T> = Result<T, AtomicBitwiseOpError>;

define_error!(
    AtomicOpError => [
        AtomicBitwiseOpError,
        AtomicExplicitAccessOpError,
        AtomicExplicitBitTestOpError,
        AtomicError,
    ] {
        UnsupportedOperation,
        InvalidSize,
        InvalidAlignment,
    }
);

pub type AtomicOpResult<T> = Result<T, AtomicOpError>;

define_error!(
    TransactionError {
        UnsupportedOperation,
        InvalidSize,
        InvalidOffset,
    }
);

pub type TransactionResult<T> = Result<T, TransactionError>;

define_error!(
    TransactionBitwiseOpError => [
        TransactionError,
    ] {
        UnsupportedOperation,
        InvalidSize,
        InvalidOffset,
    }
);

pub type TransactionBitwiseOpResult<T> = Result<T, TransactionBitwiseOpError>;

define_error!(
    TransactionOpError => [
        TransactionBitwiseOpError,
        TransactionError,
    ] {
        UnsupportedOperation,
        InvalidSize,
    }
);

pub type TransactionOpResult<T> = Result<T, TransactionOpError>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TransactionUnsupportedOpError;

impl core::fmt::Display for TransactionUnsupportedOpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(messages::UnsupportedOperation)
    }
}

impl core::error::Error for TransactionUnsupportedOpError {}

impl From<TransactionUnsupportedOpError> for TransactionOpError {
    fn from(_: TransactionUnsupportedOpError) -> Self {
        Self::UnsupportedOperation
    }
}

impl From<TransactionUnsupportedOpError> for TransactionBitwiseOpError {
    fn from(_: TransactionUnsupportedOpError) -> Self {
        Self::UnsupportedOperation
    }
}

impl From<TransactionUnsupportedOpError> for TransactionError {
    fn from(_: TransactionUnsupportedOpError) -> Self {
        Self::UnsupportedOperation
    }
}

pub type TransactionUnsupportedOpResult<T> =
    Result<T, TransactionUnsupportedOpError>;
