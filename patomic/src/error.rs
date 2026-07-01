// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

macro_rules! widen_error {
    ($from:ty => $to:ty { $($variant:ident),+ $(,)? }) => {
        impl From<$from> for $to {
            fn from(e: $from) -> Self {
                match e {
                    $(<$from>::$variant => Self::$variant,)+
                }
            }
        }
    };
}

// applies to most operations
pub enum AtomicOpError {
    UnsupportedOperation,
    InvalidSize,
    InvalidAlignment,
}

widen_error!(AtomicOpError => AtomicBitwiseOpError {
    UnsupportedOperation, InvalidSize, InvalidAlignment,
});

widen_error!(AtomicOpError => AtomicExplicitAccessOpError {
    UnsupportedOperation, InvalidSize, InvalidAlignment,
});

widen_error!(AtomicOpError => AtomicExplicitBitTestOpError {
    UnsupportedOperation, InvalidSize, InvalidAlignment,
});

// applies to bitwise operations (that have an offset)
pub enum AtomicBitwiseOpError {
    UnsupportedOperation,
    InvalidSize,
    InvalidAlignment,
    InvalidOffset,
}

widen_error!(AtomicBitwiseOpError => AtomicExplicitBitTestOpError {
    UnsupportedOperation, InvalidSize, InvalidAlignment, InvalidOffset,
});

// applies to explicit operations that have special ordering
// these are operations that only do load or store, not rmw
// load, store, cmpxchg (because of fail)
pub enum AtomicExplicitAccessOpError {
    UnsupportedOperation,
    InvalidSize,
    InvalidAlignment,
    InvalidOrdering,
}

// applies only to explicit bit test
// has both special ordering (load) and offset
pub enum AtomicExplicitBitTestOpError {
    UnsupportedOperation,
    InvalidSize,
    InvalidAlignment,
    InvalidOffset,
    InvalidOrdering,
}
