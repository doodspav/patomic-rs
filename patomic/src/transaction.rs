// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types used to configure transactional operations and inspect their
//! outcomes.
//!
//! Transactions allow for more complex operations to be performed, as well as
//! operations on larger buffers.
//!
//! This comes with the drawback that a transactional operation might never
//! succeed. The outcome always needs to be checked after calling the operation.

use core::ffi::c_ulong;
use core::cell::UnsafeCell;

use bitflags::bitflags;

use crossbeam_utils::CachePadded;

use patomic_sys::*;

use crate::{SharedBytesRef, SharedFlagRef};

/// A flag which can be provided to a transaction to intentionally cause it to
/// abort.
///
/// The flag is read from at the start of each transaction attempt, and the
/// attempt is aborted if the value is non-zero.
///
/// Writing to the flag while it is being used in a live transaction operation
/// will cause the transaction to abort. This is because any modification from
/// outside the transaction to any memory in a cache line being used by a
/// transaction will cause the transaction to abort.
///
/// The flag is padded to have its own cache line, in order to avoid false
/// sharing, which may cause a live transaction to unexpectedly abort.
///
/// # Note
///
/// You are not required to use this type to create a [`SharedFlagRef`] to pass
/// to the transaction; any padded or unpadded flag accessed through a
/// [`SharedFlagRef`] may be used.
pub struct TransactionFlag {
    flag: CachePadded<UnsafeCell<u8>>,
}

impl TransactionFlag {
    /// Creates a new unset [`TransactionFlag`].
    pub const fn new() -> Self {
        Self {
            flag: CachePadded::new(UnsafeCell::new(0)),
        }
    }

    /// Returns a [`SharedFlagRef`] referring to this flag, suitable for passing
    /// to a transaction via [`TransactionConfig`] or [`TransactionConfigWfb`].
    pub fn as_ref(&'_ self) -> SharedFlagRef<'_> {
        SharedFlagRef::from_cell(&self.flag)
    }
}

/// Denotes the success or failure of a transaction, along with the reason for
/// the failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransactionExitCode {
    /// The transaction was started and committed successfully.
    Success,

    /// The transaction failed for an unknown or implementation specific reason.
    AbortUnknown,

    /// The transaction was explicitly aborted by the user.
    AbortExplicit {
        /// The user-provided reason for the abort.
        abort_reason: u8
    },

    /// The transaction encountered a memory conflict with another thread.
    AbortConflict,

    /// The transaction accessed too much memory.
    AbortCapacity,

    /// The transaction encountered a debug trap or exception.
    AbortDebug,
}

bitflags! {
    /// A set of flags providing additional transaction abort information to
    /// supplement what is provided by the exit code.
    ///
    /// # Note
    ///
    /// The information should be taken as a hint and not be depended upon. Some
    /// implementations may provide more information for certain scenarios than
    /// other implementations.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TransactionExitInfo: u8 {
        /// No extra information is available.
        ///
        /// # Note
        ///
        /// This will always be the case if the transaction succeeds.
        const NONE          = patomic_TINFO_NONE as u8;

        /// The transaction explicitly aborted because attempts or fallback
        /// attempts was set to zero.
        ///
        /// # Note
        ///
        /// This is set by the underlying C library rather than by an
        /// implementation.
        const ZERO_ATTEMPTS = patomic_TINFO_ZERO_ATTEMPTS as u8;

        /// The transaction explicitly aborted because the flag was set.
        ///
        /// # Note
        ///
        /// This is set by the underlying C library rather than by an
        /// implementation.
        const FLAG_SET      = patomic_TINFO_FLAG_SET as u8;

        /// The transaction might not fail if retried.
        const RETRY         = patomic_TINFO_RETRY as u8;

        /// The transaction was aborted from an inner nested transaction.
        const NESTED        = patomic_TINFO_NESTED as u8;
    }
}


/// The status of a single transaction, combining its exit code with any
/// additional abort information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionStatus {
    /// The success or failure of the transaction.
    pub exit_code: TransactionExitCode,

    /// Additional abort information supplementing the exit code.
    pub exit_info: TransactionExitInfo,
}

impl TransactionStatus {
    /// Converts a raw transaction status value into a [`TransactionStatus`].
    ///
    /// # Note
    ///
    /// Any exit code value not corresponding to a variant in
    /// [`TransactionExitCode`] is converted to the fallback value of
    /// [`AbortUnknown`], and any exit info bits not corresponding to a label in
    /// [`TransactionExitInfo`] are discarded.
    ///
    /// [`AbortUnknown`]: TransactionExitCode::AbortUnknown
    pub const fn from_ffi(status: c_ulong) -> Self {
        #[allow(non_upper_case_globals)]
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

/// Represents the outcome of a transaction.
#[must_use = "transaction may have aborted; check status"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionOutcome {
    /// Status from the final attempt at committing the transaction.
    pub status: TransactionStatus,

    /// Attempts made to commit the transaction.
    pub attempts_made: u32,
}

impl From<patomic_transaction_result_t> for TransactionOutcome {
    /// Converts a raw [`patomic_transaction_result_t`] value into a
    /// [`TransactionOutcome`].
    fn from(result: patomic_transaction_result_t) -> Self {
        Self {
            status: TransactionStatus::from_ffi(result.status),
            attempts_made: result.attempts_made as u32,
        }
    }
}

/// Represents the outcome of a transaction with a fallback path.
#[must_use = "transaction may have aborted; check status"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionOutcomeWfb {
    /// Status from the final attempt at committing the primary transaction.
    pub status: TransactionStatus,

    /// Status from the final attempt at committing the fallback transaction.
    ///
    /// # Note
    ///
    /// The value of this field is unspecified if the primary transaction was
    /// successful.
    pub fallback_status: TransactionStatus,

    /// Attempts made to commit the primary transaction.
    pub attempts_made: u32,

    /// Attempts made to commit the fallback transaction.
    ///
    /// # Note
    ///
    /// The value of this field is unspecified if the primary transaction was
    /// successful.
    pub fallback_attempts_made: u32,
}

impl From<patomic_transaction_result_wfb_t> for TransactionOutcomeWfb {
    /// Converts a raw [`patomic_transaction_result_wfb_t`] value into a
    /// [`TransactionOutcomeWfb`].
    fn from(result: patomic_transaction_result_wfb_t) -> Self {
        Self {
            status: TransactionStatus::from_ffi(result.status),
            attempts_made: result.attempts_made as u32,
            fallback_status: TransactionStatus::from_ffi(result.fallback_status),
            fallback_attempts_made: result.fallback_attempts_made as u32,
        }
    }
}

/// Used in double and multi variants of `compare_exchange` to pass multiple
/// memory locations.
///
/// # Note
///
/// The byte width of all buffers at all memory locations is obtained from a
/// [`TransactionConfig`] or [`TransactionConfigWfb`] which is provided
/// separately.
pub struct CmpxchgOperands<'a> {
    /// Buffer on which to perform the transaction.
    pub obj: SharedBytesRef<'a>,

    /// Value [`obj`] must have for the transaction to succeed.
    ///
    /// [`obj`]: Self::obj
    pub expected: &'a mut [u8],

    /// Value [`obj`] will be set to if the transaction succeeds.
    ///
    /// [`obj`]: Self::obj
    pub desired: &'a [u8],
}

impl From<CmpxchgOperands<'_>> for patomic_transaction_cmpxchg_t {
    /// Converts [`CmpxchgOperands`] into a raw
    /// [`patomic_transaction_cmpxchg_t`] value.
    ///
    /// # Warning
    ///
    /// The raw value contains pointers to buffers referenced by the original
    /// [`CmpxchgOperands`] object.
    fn from(value: CmpxchgOperands<'_>) -> Self {
        Self {
            obj: value.obj.as_mut_ptr().cast(),
            expected: value.expected.as_mut_ptr().cast(),
            desired: value.desired.as_ptr().cast(),
        }
    }
}

/// Used to configure the execution limits of a transaction, pass a
/// user-provided flag, and determine the byte width of all buffers passed to
/// the transaction.
pub struct TransactionConfig<'a> {
    /// Size in bytes of buffers to operate on.
    pub width: usize,

    /// Number of attempts to make committing the transaction.
    pub attempts: u32,

    /// Read from at the start of each transaction attempt, which is aborted if
    /// the value is non-zero.
    ///
    /// May be [`None`], in which case no flag check is performed.
    pub flag: Option<SharedFlagRef<'a>>
}

impl From<TransactionConfig<'_>> for patomic_transaction_config_t {
    /// Converts a [`TransactionConfig`] into a raw
    /// [`patomic_transaction_config_t`] value.
    ///
    /// A [`flag`] of [`None`] is converted to a null pointer.
    ///
    /// [`flag`]: TransactionConfig::flag
    fn from(value: TransactionConfig<'_>) -> Self {
        Self {
            width: value.width,
            attempts: value.attempts as c_ulong,
            flag_nullable: value.flag
                .map_or(core::ptr::null(), |f| f.as_ptr()),
        }
    }
}

/// Used to configure the execution limits of primary and fallback transactions,
/// pass user-provided flags, and determine the byte width of all buffers passed
/// to the transactions.
///
/// # Note
///
/// The [`flag`] and [`fallback_flag`] may refer to the same flag. The [`flag`]
/// tends to guard a read-write code path, and the [`fallback_flag`] tends to
/// guard a read-only code path.
///
/// With this in mind, it is recommended to use [`flag`] as a unique writer lock
/// and [`fallback_flag`] as a shared reader lock.
///
/// [`flag`]: Self::flag
/// [`fallback_flag`]: Self::fallback_flag
pub struct TransactionConfigWfb<'a> {
    /// Size in bytes of buffers to operate on.
    pub width: usize,

    /// Number of attempts to make committing the primary transaction.
    pub attempts: u32,

    /// Number of attempts to make committing the fallback transaction.
    pub fallback_attempts: u32,

    /// Read from at the start of each primary transaction attempt, which is
    /// aborted if the value is non-zero.
    ///
    /// May be [`None`], in which case no primary flag check is performed.
    pub flag: Option<SharedFlagRef<'a>>,

    /// Read from at the start of each fallback transaction attempt, which is
    /// aborted if the value is non-zero.
    ///
    /// May be [`None`], in which case no fallback flag check is performed.
    pub fallback_flag: Option<SharedFlagRef<'a>>,
}

impl From<TransactionConfigWfb<'_>> for patomic_transaction_config_wfb_t {
    /// Converts a [`TransactionConfigWfb`] into a raw
    /// [`patomic_transaction_config_wfb_t`] value.
    ///
    /// A [`flag`] or [`fallback_flag`] of [`None`] is converted to a null
    /// pointer.
    ///
    /// [`flag`]: TransactionConfigWfb::flag
    /// [`fallback_flag`]: TransactionConfigWfb::fallback_flag
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
