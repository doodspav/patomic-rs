// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::cmp::Ordering;
use core::num::NonZeroUsize;

use patomic_sys::*;

use crate::SharedBytesRef;

pub trait AtomicLayout {
    fn width(&self) -> NonZeroUsize;
    fn alignment(&self) -> Alignment;
}

/// The alignment required by atomic operations on buffers.
///
/// Each backend has an alignment required by all of its supported operations,
/// which must be met by the [`SharedBytesRef`] bytes on which the operations
/// take place.
///
/// Within the semantics of this type, buffers are considered aligned if they
/// meet either the **recommended** or **minimum** alignment requirements.
///
/// To meet the recommended alignment requirements, the buffer must be aligned
/// to at least [`recommended`].
///
/// To meet the minimum alignment requirements, the buffer must be aligned to at
/// least [`minimum`]. Additionally, if [`size_within`] is not `0`, the buffer
/// must also entirely reside within a hypothetical buffer with the size and
/// alignment of [`size_within`]. The intention of this is to communicate that a
/// buffer is suitably aligned if it does not cross a cache-line boundary.
///
/// These are meaningfully different in C, where the variety of platforms and
/// allowed integer representations is larger, however in Rust it is enough for
/// either requirement to be met for a buffer to be considered suitably aligned.
///
/// # Warning
///
/// Not meeting the alignment requirements when performing an unchecked atomic
/// operation will result in undefined behavior.
///
/// # Note
///
/// All APIs in this crate returning this type guarantee that [`recommended`]
/// and [`minimum`] are a power of `2`, and that [`minimum`] is never larger
/// than [`recommended`]. The [`size_within`] value will either be `0`, or a
/// power of `2`.
///
/// [`recommended`]: Self::recommended
/// [`minimum`]: Self::minimum
/// [`size_within`]: Self::size_within
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Alignment {
    /// Alignment required by the C language (unconditionally valid).
    pub recommended: NonZeroUsize,

    /// Alignment required by the architecture (conditionally valid).
    pub minimum: NonZeroUsize,

    /// Size and alignment of a hypothetical buffer within which the bytes must
    /// reside for [`minimum`] to be considered valid, unless it is `0` in which
    /// case [`minimum`] is always valid.
    ///
    /// [`minimum`]: Self::minimum
    pub size_within: usize,
}

impl Alignment {
    /// Checks that the buffer meets these alignment requirements, according to
    /// the semantics of this type.
    ///
    /// # Note
    ///
    /// The check will always fail if the buffer is empty.
    pub fn is_met_by(&self, bytes: SharedBytesRef) -> bool {
        let ptr = bytes.as_ptr().cast();
        let align = (*self).into();
        let Some(width) = NonZeroUsize::new(bytes.len()) else {
            return false;
        };
        PATOMIC_ALIGN_MEETS_RECOMMENDED(ptr, align)
            || PATOMIC_ALIGN_MEETS_MINIMUM(ptr, align, width)
    }
}

impl From<patomic_align_t> for Alignment {
    /// Converts a raw [`patomic_align_t`] value into an [`Alignment`].
    ///
    /// This conversion is lossless.
    fn from(value: patomic_align_t) -> Self {
        Self {
            recommended: value.recommended,
            minimum: value.minimum,
            size_within: value.size_within,
        }
    }
}

impl From<Alignment> for patomic_align_t {
    /// Converts an [`Alignment`] into a raw [`patomic_align_t`] value.
    ///
    /// This conversion is lossless.
    fn from(value: Alignment) -> Self {
        Self {
            recommended: value.recommended,
            minimum: value.minimum,
            size_within: value.size_within,
        }
    }
}

impl Ord for Alignment {
    /// Compares two [`Alignment`]s by the strictness of their requirements,
    /// where a greater value imposes stricter requirements.
    ///
    /// Fields are compared in the order [`recommended`], [`minimum`],
    /// [`size_within`]. For the first two fields, a larger alignment is
    /// stricter. For [`size_within`], smaller values are stricter, except for
    /// `0` which is the least strict value.
    ///
    /// [`recommended`]: Self::recommended
    /// [`minimum`]: Self::minimum
    /// [`size_within`]: Self::size_within
    fn cmp(&self, other: &Self) -> Ordering {
        self.recommended
            .cmp(&other.recommended)
            .then_with(|| self.minimum.cmp(&other.minimum))
            .then_with(|| match (self.size_within, other.size_within) {
                (a, b) if a == b => Ordering::Equal,
                (0, _) => Ordering::Less,
                (_, 0) => Ordering::Greater,
                (a, b) => b.cmp(&a),
            })
    }
}

impl PartialOrd for Alignment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
