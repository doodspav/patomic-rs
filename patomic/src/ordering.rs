// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;
use core::sync::atomic::Ordering as StdOrdering;

use patomic_sys::*;

/// Atomic memory ordering.
///
/// Enum constants specifying how memory accesses are to be ordered around an
/// atomic operation.
///
/// Each variant's value and semantics are identical to C++11's
/// [`std::memory_order`]'s values and semantics, and by extension mostly mirror
/// those of [`core::sync::atomic::Ordering`].
///
/// For more information on the semantics of each ordering, see the [nomicon]
/// and the C++ reference for [`std::memory_order`].
///
/// [nomicon]: https://doc.rust-lang.org/nomicon/atomics.html
/// [`std::memory_order`]: https://en.cppreference.com/w/cpp/atomic/memory_order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum Ordering {
    /// No ordering constraints, only atomic operations.
    ///
    /// Corresponds to [`std::memory_order_relaxed`] in C++11 and
    /// [`core::sync::atomic::Ordering::Relaxed`] in Rust.
    ///
    /// [`std::memory_order_relaxed`]: https://en.cppreference.com/w/cpp/atomic/memory_order#Relaxed_ordering
    Relaxed = patomic_RELAXED as isize,

    /// Only present for compatibility. It will always be treated as
    /// [`Acquire`].
    Consume = patomic_CONSUME as isize,

    /// When coupled with a load, if the loaded value was written by a store
    /// operation with [`Release`] (or stronger) ordering, then all subsequent
    /// operations become ordered after that store. In particular, all
    /// subsequent loads will see data written before the store.
    ///
    /// Notice that using this ordering for an operation that combines loads
    /// and stores leads to a [`Relaxed`] store operation!
    ///
    /// This ordering is only applicable for operations that can perform a load.
    ///
    /// Corresponds to [`std::memory_order_acquire`] in C++11 and
    /// [`core::sync::atomic::Ordering::Acquire`] in Rust.
    ///
    /// [`std::memory_order_acquire`]: https://en.cppreference.com/w/cpp/atomic/memory_order#Release-Acquire_ordering
    Acquire = patomic_ACQUIRE as isize,

    /// When coupled with a store, all previous operations become ordered
    /// before any load of this value with [`Acquire`] (or stronger) ordering.
    /// In particular, all previous writes become visible to all threads
    /// that perform an [`Acquire`] (or stronger) load of this value.
    ///
    /// Notice that using this ordering for an operation that combines loads
    /// and stores leads to a [`Relaxed`] load operation!
    ///
    /// This ordering is only applicable for operations that can perform a
    /// store.
    ///
    /// Corresponds to [`std::memory_order_release`] in C++11 and
    /// [`core::sync::atomic::Ordering::Release`] in Rust.
    ///
    /// [`std::memory_order_release`]: https://en.cppreference.com/w/cpp/atomic/memory_order#Release-Acquire_ordering
    Release = patomic_RELEASE as isize,

    /// Has the effects of both [`Acquire`] and [`Release`] together:
    /// For loads it uses [`Acquire`] ordering. For stores, it uses the
    /// [`Release`] ordering.
    ///
    /// Notice that in the case of `compare_exchange` and other operations with
    /// a read-only fallback path, it is possible that the operation ends up
    /// not performing any store and hence it has just [`Acquire`] ordering.
    /// However, [`AcqRel`] will never perform [`Relaxed`] accesses.
    ///
    /// This ordering is only applicable for operations that combine both loads
    /// and stores.
    ///
    /// Corresponds to [`std::memory_order_acq_rel`] in C++11 and
    /// [`core::sync::atomic::Ordering::AcqRel`] in Rust.
    ///
    /// [`std::memory_order_acq_rel`]: https://en.cppreference.com/w/cpp/atomic/memory_order#Release-Acquire_ordering
    AcqRel = patomic_ACQ_REL as isize,

    /// Like [`Acquire`]/[`Release`]/[`AcqRel`] (for load, store, and
    /// load-with-store operations, respectively) with the additional guarantee
    /// that all threads see all sequentially consistent operations in the same
    /// order.
    ///
    /// Corresponds to [`std::memory_order_seq_cst`] in C++11 and
    /// [`core::sync::atomic::Ordering::SeqCst`] in Rust.
    ///
    /// [`std::memory_order_seq_cst`]: https://en.cppreference.com/w/cpp/atomic/memory_order#Sequentially-consistent_ordering
    SeqCst = patomic_SEQ_CST as isize,
}

impl Ordering {
    /// Checks that `self` is valid to use for an atomic store operation.
    ///
    /// The valid store orderings are [`Relaxed`], [`Release`], and [`SeqCst`].
    ///
    /// # Examples
    ///
    /// ```
    /// use patomic::Ordering;
    ///
    /// assert!(Ordering::Release.is_valid_store_ordering());
    /// assert!(!Ordering::Acquire.is_valid_store_ordering());
    /// ```
    pub const fn is_valid_store_ordering(&self) -> bool {
        PATOMIC_IS_VALID_STORE_ORDER((*self) as c_int)
    }

    /// Checks that `self` is valid to use for an atomic load operation.
    ///
    /// The valid load orderings are [`Relaxed`], [`Consume`], [`Acquire`], and
    /// [`SeqCst`].
    ///
    /// # Examples
    ///
    /// ```
    /// use patomic::Ordering;
    ///
    /// assert!(Ordering::Acquire.is_valid_load_ordering());
    /// assert!(!Ordering::Release.is_valid_load_ordering());
    /// ```
    pub const fn is_valid_load_ordering(&self) -> bool {
        PATOMIC_IS_VALID_LOAD_ORDER((*self) as c_int)
    }

    /// Checks that `self` is valid to use as the fail ordering for an atomic
    /// read-modify-write operation which has a read-only fallback operation,
    /// with `succ` as the success ordering, such as `compare_exchange`.
    ///
    /// This requires that `self` is a valid load ordering and that it is not
    /// stronger than `succ`.
    ///
    /// # Examples
    ///
    /// ```
    /// use patomic::Ordering;
    ///
    /// assert!(Ordering::Relaxed.is_valid_fail_ordering_for(Ordering::AcqRel));
    /// assert!(!Ordering::SeqCst.is_valid_fail_ordering_for(Ordering::Relaxed));
    /// ```
    pub const fn is_valid_fail_ordering_for(&self, succ: Ordering) -> bool {
        PATOMIC_IS_VALID_FAIL_ORDER(
            succ as c_int, (*self) as c_int,
        )
    }

    /// Returns the strictest memory order that is valid to use as a fail
    /// ordering when `self` is the success ordering.
    ///
    /// The returned ordering `fail` satisfies
    /// [`fail.is_valid_fail_ordering_for(self)`](Self::is_valid_ordering_for).
    ///
    /// # Examples
    ///
    /// ```
    /// use patomic::Ordering;
    ///
    /// assert_eq!(Ordering::AcqRel.fail_ordering(), Ordering::Acquire);
    /// assert_eq!(Ordering::SeqCst.fail_ordering(), Ordering::SeqCst);
    /// ```
    pub const fn fail_ordering(&self) -> Ordering {
        let fail = PATOMIC_CMPXCHG_FAIL_ORDER((*self) as c_int);
        #[allow(non_upper_case_globals)]
        match fail {
            patomic_RELAXED => Self::Relaxed,
            patomic_CONSUME => Self::Consume,
            patomic_ACQUIRE => Self::Acquire,
            patomic_RELEASE => Self::Release,
            patomic_ACQ_REL => Self::AcqRel,
            patomic_SEQ_CST => Self::SeqCst,
            _ => unreachable!(),
        }
    }
}

impl From<c_int> for Ordering {
    /// Converts a raw [`patomic_memory_order_t`] value into an [`Ordering`].
    ///
    /// Any value not corresponding to a label in [`patomic_memory_order_t`] is
    /// converted to the fallback value of [`SeqCst`].
    fn from(ordering: c_int) -> Self {
        #[allow(non_upper_case_globals)]
        match ordering {
            patomic_RELAXED => Self::Relaxed,
            patomic_CONSUME => Self::Consume,
            patomic_ACQUIRE => Self::Acquire,
            patomic_RELEASE => Self::Release,
            patomic_ACQ_REL => Self::AcqRel,
            patomic_SEQ_CST => Self::SeqCst,
            _ => Self::SeqCst,
        }
    }
}

impl From<Ordering> for c_int {
    /// Converts an [`Ordering`] into a raw [`patomic_memory_order_t`] value.
    ///
    /// This conversion is lossless.
    fn from(ordering: Ordering) -> Self {
        #[allow(non_upper_case_globals)]
        match ordering {
            Ordering::Relaxed => patomic_RELAXED,
            Ordering::Consume => patomic_CONSUME,
            Ordering::Acquire => patomic_ACQUIRE,
            Ordering::Release => patomic_RELEASE,
            Ordering::AcqRel => patomic_ACQ_REL,
            Ordering::SeqCst => patomic_SEQ_CST,
        }
    }
}

impl From<StdOrdering> for Ordering {
    /// Converts a [`core::sync::atomic::Ordering`] into an [`Ordering`].
    ///
    /// [`core::sync::atomic::Ordering`] is `'[non_exhaustive]`, so any variant
    /// added in the future is converted to the fallback value of [`SeqCst`].
    #[inline]
    fn from(ordering: StdOrdering) -> Self {
        match ordering {
            StdOrdering::Relaxed => Self::Relaxed,
            StdOrdering::Acquire => Self::Acquire,
            StdOrdering::Release => Self::Release,
            StdOrdering::AcqRel => Self::AcqRel,
            StdOrdering::SeqCst => Self::SeqCst,
            _ => Self::SeqCst,
        }
    }
}

impl From<Ordering> for StdOrdering {
    /// Converts an [`Ordering`] into a [`core::sync::atomic::Ordering`].
    ///
    /// [`Consume`] has no standard library equivalent and
    /// is converted to [`Acquire`](StdOrdering::Acquire), matching how it is
    /// treated by the underlying C library.
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Relaxed => Self::Relaxed,
            Ordering::Consume => Self::Acquire,
            Ordering::Acquire => Self::Acquire,
            Ordering::Release => Self::Release,
            Ordering::AcqRel => Self::AcqRel,
            Ordering::SeqCst => Self::SeqCst,
        }
    }
}
