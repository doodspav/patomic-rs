// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::{c_int, c_uchar, c_uint, c_ulong, c_void};

use crate::PATOMIC_MAX_CACHE_LINE_SIZE_ABI_UNSTABLE;

pub type patomic_transaction_flag_t = c_uchar;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_padded_flag_holder_abi_unstable_t {
    pub _padding_head: [c_uchar; PATOMIC_MAX_CACHE_LINE_SIZE_ABI_UNSTABLE - 1],
    pub flag: patomic_transaction_flag_t,
    pub _padding_tail: [c_uchar; PATOMIC_MAX_CACHE_LINE_SIZE_ABI_UNSTABLE],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_cmpxchg_t {
    pub obj: *mut c_void,
    pub expected: *mut c_void,
    pub desired: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_config_t {
    pub width: usize,
    pub attempts: c_ulong,
    pub flag_nullable: *const patomic_transaction_flag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_config_wfb_t {
    pub width: usize,
    pub attempts: c_ulong,
    pub fallback_attempts: c_ulong,
    pub flag_nullable: *const patomic_transaction_flag_t,
    pub fallback_flag_nullable: *const patomic_transaction_flag_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_result_t {
    pub status: c_ulong,
    pub attempts_made: c_ulong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct patomic_transaction_result_wfb_t {
    pub status: c_ulong,
    pub fallback_status: c_ulong,
    pub attempts_made: c_ulong,
    pub fallback_attempts_made: c_ulong,
}

pub type patomic_transaction_exit_code_t = c_int;

pub const patomic_TSUCCESS: patomic_transaction_exit_code_t = 0;
pub const patomic_TABORT_UNKNOWN: patomic_transaction_exit_code_t = 255;
pub const patomic_TABORT_EXPLICIT: patomic_transaction_exit_code_t = 1;
pub const patomic_TABORT_CONFLICT: patomic_transaction_exit_code_t = 2;
pub const patomic_TABORT_CAPACITY: patomic_transaction_exit_code_t = 3;
pub const patomic_TABORT_DEBUG: patomic_transaction_exit_code_t = 4;

pub type patomic_transaction_exit_info_t = c_int;

pub const patomic_TINFO_NONE: patomic_transaction_exit_info_t = 0;
pub const patomic_TINFO_ZERO_ATTEMPTS: patomic_transaction_exit_info_t = 1 << 0;
pub const patomic_TINFO_FLAG_SET: patomic_transaction_exit_info_t = 1 << 1;
pub const patomic_TINFO_RETRY: patomic_transaction_exit_info_t = 1 << 0;
pub const patomic_TINFO_NESTED: patomic_transaction_exit_info_t = 1 << 1;

#[inline]
pub const fn PATOMIC_TRANSACTION_STATUS_EXIT_CODE(
    status: c_ulong,
) -> patomic_transaction_exit_code_t {
    (status & 0xff) as c_int
}

#[inline]
pub const fn PATOMIC_TRANSACTION_STATUS_EXIT_INFO(
    status: c_ulong,
) -> c_uint {
    ((status >> 16) & 0xff) as c_uint
}

#[inline]
pub const fn PATOMIC_TRANSACTION_STATUS_ABORT_REASON(
    status: c_ulong,
) -> c_uchar {
    ((status >> 8) & 0xff) as c_uchar
}

unsafe extern "C" {
    pub fn patomic_transaction_status_exit_code(
        status: c_ulong,
    ) -> patomic_transaction_exit_code_t;

    pub fn patomic_transaction_status_exit_info(
        status: c_ulong,
    ) -> c_uint;

    pub fn patomic_transaction_status_abort_reason(
        status: c_ulong,
    ) -> c_uchar;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_code_is_first_8_bits_of_status() {
        // go through all combinations of 8 bits
        for code in 0..=0xFF as c_ulong {
            let status = code;
            assert_eq!(
                code as patomic_transaction_exit_code_t,
                PATOMIC_TRANSACTION_STATUS_EXIT_CODE(status)
            );
        }

        // go into 9th bit with first 8 bits zeroed
        let status = (1 as c_ulong) << 8;
        assert_eq!(0, PATOMIC_TRANSACTION_STATUS_EXIT_CODE(status));
    }

    #[test]
    fn abort_reason_is_second_8_bits_of_status() {
        // go through all combinations of 8 bits (offset by 8)
        for reason in 0..=0xFF as c_ulong {
            let status = reason << 8;
            assert_eq!(
                reason as c_uchar,
                PATOMIC_TRANSACTION_STATUS_ABORT_REASON(status)
            );
        }

        // go into 9th bit with first 8 bits zeroed (offset by 8)
        let status = (1 as c_ulong) << 16;
        assert_eq!(0, PATOMIC_TRANSACTION_STATUS_ABORT_REASON(status));
    }

    #[test]
    fn exit_info_is_third_8_bits_of_status() {
        // go through all combinations of 8 bits (offset by 16)
        for info in 0..=0xFF as c_ulong {
            let status = info << 16;
            assert_eq!(
                info as c_uint,
                PATOMIC_TRANSACTION_STATUS_EXIT_INFO(status)
            );
        }

        // go into 9th bit with first 8 bits zeroed (offset by 8)
        let status = (1 as c_ulong) << 24;
        assert_eq!(0, PATOMIC_TRANSACTION_STATUS_EXIT_INFO(status));

    }
}
