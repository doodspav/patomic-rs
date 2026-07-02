// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_ulong, c_void};
use core::cell::UnsafeCell;

use bitflags::bitflags;

use crossbeam_utils::CachePadded;

use patomic_sys::*;

use crate::{SharedBytesRef, SharedFlagRef};

pub struct TransactionFlag {
    flag: CachePadded<UnsafeCell<u8>>,
}

impl TransactionFlag {
    pub const fn new() -> Self {
        Self {
            flag: CachePadded::new(UnsafeCell::new(0)),
        }
    }

    pub fn as_ref(&'_ self) -> SharedFlagRef<'_> {
        SharedFlagRef::from_cell(&self.flag)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionExitCode {
    Success,
    AbortUnknown,
    AbortExplicit { abort_reason: u8 },
    AbortConflict,
    AbortCapacity,
    AbortDebug,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TransactionExitInfo: u8 {
        const NONE          = patomic_TINFO_NONE as u8;
        const ZERO_ATTEMPTS = patomic_TINFO_ZERO_ATTEMPTS as u8;
        const FLAG_SET      = patomic_TINFO_FLAG_SET as u8;
        const RETRY         = patomic_TINFO_RETRY as u8;
        const NESTED        = patomic_TINFO_NESTED as u8;
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionStatus {
    pub exit_code: TransactionExitCode,
    pub exit_info: TransactionExitInfo,
}

impl TransactionStatus {
    pub const fn from_ffi(status: c_ulong) -> Self {
        let exit_code = match PATOMIC_TRANSACTION_STATUS_EXIT_CODE(status) {
            patomic_TSUCCESS => TransactionExitCode::Success,
            patomic_TABORT_EXPLICIT => TransactionExitCode::AbortExplicit {
                abort_reason: PATOMIC_TRANSACTION_STATUS_ABORT_REASON(status),
            },
            patomic_TABORT_CONFLICT => TransactionExitCode::AbortConflict,
            patomic_TABORT_CAPACITY => TransactionExitCode::AbortCapacity,
            patomic_TABORT_DEBUG => TransactionExitCode::AbortDebug,
            _ => TransactionExitCode::AbortUnknown,
        };

        let exit_info = TransactionExitInfo::from_bits_truncate(
            PATOMIC_TRANSACTION_STATUS_EXIT_INFO(status) as u8
        );

        Self { exit_code, exit_info }
    }
}

#[must_use = "transaction may have aborted; check status"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionOutcome {
    pub status: TransactionStatus,
    pub attempts_made: u32,
}

impl TransactionOutcome {
    pub const fn from_ffi(result: patomic_transaction_result_t) -> Self {
        Self {
            status: TransactionStatus::from_ffi(result.status),
            attempts_made: result.attempts_made as u32,
        }
    }
}

#[must_use = "transaction may have aborted; check status"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionOutcomeWfb {
    pub status: TransactionStatus,
    pub fallback_status: TransactionStatus,
    pub attempts_made: u32,
    pub fallback_attempts_made: u32,
}

impl TransactionOutcomeWfb {
    pub const fn from_ffi(result: patomic_transaction_result_wfb_t) -> Self {
        Self {
            status: TransactionStatus::from_ffi(result.status),
            attempts_made: result.attempts_made as u32,
            fallback_status: TransactionStatus::from_ffi(result.fallback_status),
            fallback_attempts_made: result.fallback_attempts_made as u32,
        }
    }
}

pub struct CmpxchgOperands<'a> {
    pub obj: SharedBytesRef<'a>,
    pub expected: &'a mut [u8],
    pub desired: &'a [u8],
}

impl From<CmpxchgOperands<'_>> for patomic_transaction_cmpxchg_t {
    fn from(value: CmpxchgOperands<'_>) -> Self {
        Self {
            obj: value.obj.as_mut_ptr() as *mut c_void,
            expected: value.expected.as_mut_ptr() as *mut c_void,
            desired: value.desired.as_ptr() as *const c_void,
        }
    }
}

pub struct TransactionConfig<'a> {
    pub width: usize,
    pub attempts: u32,
    pub flag: Option<SharedFlagRef<'a>>
}

impl From<TransactionConfig<'_>> for patomic_transaction_config_t {
    fn from(value: TransactionConfig<'_>) -> Self {
        Self {
            width: value.width,
            attempts: value.attempts as c_ulong,
            flag_nullable: value.flag
                .map_or(core::ptr::null(), |f| f.as_ptr()),
        }
    }
}

pub struct TransactionConfigWfb<'a> {
    pub width: usize,
    pub attempts: u32,
    pub fallback_attempts: u32,
    pub flag: Option<SharedFlagRef<'a>>,
    pub fallback_flag: Option<SharedFlagRef<'a>>,
}

impl From<TransactionConfigWfb<'_>> for patomic_transaction_config_wfb_t {
    fn from(value: TransactionConfigWfb<'_>) -> Self {
        Self {
            width: value.width,
            attempts: value.attempts as c_ulong,
            fallback_attempts: value.fallback_attempts as c_ulong,
            flag_nullable: value.flag
                .map_or(core::ptr::null(), |f| f.as_ptr()),
            fallback_flag_nullable: value.fallback_flag
                .map_or(core::ptr::null(), |f| f.as_ptr()),
        }
    }
}
