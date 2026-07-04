// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;
use core::sync::atomic::Ordering as StdOrdering;

use patomic_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum Ordering {
    Relaxed = patomic_RELAXED as isize,
    Consume = patomic_CONSUME as isize,
    Acquire = patomic_ACQUIRE as isize,
    Release = patomic_RELEASE as isize,
    AcqRel = patomic_ACQ_REL as isize,
    SeqCst = patomic_SEQ_CST as isize,
}

impl Ordering {
    pub const fn is_valid_store_ordering(&self) -> bool {
        PATOMIC_IS_VALID_STORE_ORDER((*self) as c_int)
    }

    pub const fn is_valid_load_ordering(&self) -> bool {
        PATOMIC_IS_VALID_LOAD_ORDER((*self) as c_int)
    }

    pub const fn is_valid_fail_ordering_for(&self, succ: Ordering) -> bool {
        PATOMIC_IS_VALID_FAIL_ORDER(
            succ as c_int, (*self) as c_int,
        )
    }

    pub fn fail_ordering(&self) -> Ordering {
        PATOMIC_CMPXCHG_FAIL_ORDER((*self) as c_int).into()
    }
}

impl From<c_int> for Ordering {
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

impl From<StdOrdering> for Ordering {
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
