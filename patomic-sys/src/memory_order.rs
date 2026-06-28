// Copyright (c) doodspav.
// SPDX-License-Identifier: MIT OR Apache-2.0

use core::ffi::c_int;

pub type patomic_memory_order_t = c_int;

pub const patomic_RELAXED: patomic_memory_order_t = 0;
pub const patomic_CONSUME: patomic_memory_order_t = 1;
pub const patomic_ACQUIRE: patomic_memory_order_t = 2;
pub const patomic_RELEASE: patomic_memory_order_t = 3;
pub const patomic_ACQ_REL: patomic_memory_order_t = 4;
pub const patomic_SEQ_CST: patomic_memory_order_t = 5;

#[inline]
pub const fn PATOMIC_IS_VALID_ORDER(order: c_int) -> bool {
    matches!(
        order,
        patomic_RELAXED
            | patomic_CONSUME
            | patomic_ACQUIRE
            | patomic_RELEASE
            | patomic_ACQ_REL
            | patomic_SEQ_CST
    )
}

#[inline]
pub const fn PATOMIC_IS_VALID_STORE_ORDER(order: c_int) -> bool {
    matches!(
        order,
        patomic_RELAXED
            | patomic_RELEASE
            | patomic_SEQ_CST
    )
}

#[inline]
pub const fn PATOMIC_IS_VALID_LOAD_ORDER(order: c_int) -> bool {
    matches!(
        order,
        patomic_RELAXED
            | patomic_CONSUME
            | patomic_ACQUIRE
            | patomic_SEQ_CST
    )
}

#[inline]
pub const fn PATOMIC_IS_VALID_FAIL_ORDER(
    succ: c_int,
    fail: c_int,
) -> bool {
    succ >= fail
        && PATOMIC_IS_VALID_ORDER(succ)
        && PATOMIC_IS_VALID_LOAD_ORDER(fail)
}

#[inline]
pub const fn PATOMIC_CMPXCHG_FAIL_ORDER(
    succ: c_int,
) -> c_int {
    if succ == patomic_ACQ_REL || succ == patomic_RELEASE {
        patomic_ACQUIRE
    } else {
        succ
    }
}

unsafe extern "C" {
    pub fn patomic_is_valid_order(order: c_int) -> c_int;

    pub fn patomic_is_valid_store_order(order: c_int) -> c_int;

    pub fn patomic_is_valid_load_order(order: c_int) -> c_int;

    pub fn patomic_is_valid_fail_order(
        succ: c_int,
        fail: c_int,
    ) -> c_int;

    pub fn patomic_cmpxchg_fail_order(
        succ: c_int,
    ) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_ORDERS: [c_int; 5] = [-1, -10, 10, c_int::MIN, c_int::MAX];

    const VALID_ORDERS: [patomic_memory_order_t; 6] = [
        patomic_RELAXED,
        patomic_CONSUME,
        patomic_ACQUIRE,
        patomic_RELEASE,
        patomic_ACQ_REL,
        patomic_SEQ_CST,
    ];

    const STORE_ORDERS: [patomic_memory_order_t; 3] = [
        patomic_RELAXED,
        patomic_RELEASE,
        patomic_SEQ_CST,
    ];

    const LOAD_ORDERS: [patomic_memory_order_t; 4] = [
        patomic_RELAXED,
        patomic_CONSUME,
        patomic_ACQUIRE,
        patomic_SEQ_CST,
    ];

    #[test]
    fn is_valid_order_allows_all_valid_orders() {
        for order in VALID_ORDERS {
            assert!(PATOMIC_IS_VALID_ORDER(order));
        }
    }

    #[test]
    fn is_valid_order_rejects_invalid_orders() {
        for order in INVALID_ORDERS {
            assert!(!PATOMIC_IS_VALID_ORDER(order));
        }
    }

    #[test]
    fn is_valid_store_order_allows_all_valid_store_orders() {
        for order in STORE_ORDERS {
            assert!(PATOMIC_IS_VALID_STORE_ORDER(order));
        }
    }

    #[test]
    fn is_valid_store_order_rejects_invalid_orders() {
        for order in INVALID_ORDERS {
            assert!(!PATOMIC_IS_VALID_STORE_ORDER(order));
        }
    }

    #[test]
    fn is_valid_store_order_rejects_non_store_orders() {
        for order in VALID_ORDERS {
            if !STORE_ORDERS.contains(&order) {
                assert!(!PATOMIC_IS_VALID_STORE_ORDER(order));
            }
        }
    }

    #[test]
    fn is_valid_load_order_allows_all_valid_load_orders() {
        for order in LOAD_ORDERS {
            assert!(PATOMIC_IS_VALID_LOAD_ORDER(order));
        }
    }

    #[test]
    fn is_valid_load_order_rejects_invalid_load_orders() {
        for order in INVALID_ORDERS {
            assert!(!PATOMIC_IS_VALID_LOAD_ORDER(order));
        }
    }

    #[test]
    fn is_valid_load_order_rejects_non_load_orders() {
        for order in VALID_ORDERS {
            if !LOAD_ORDERS.contains(&order) {
                assert!(!PATOMIC_IS_VALID_LOAD_ORDER(order));
            }
        }
    }

    #[test]
    fn is_valid_fail_order_allows_all_valid_pairs() {
        for succ in VALID_ORDERS {
            for fail in VALID_ORDERS {
                if fail > succ || !LOAD_ORDERS.contains(&fail) {
                    continue;
                }
                assert!(PATOMIC_IS_VALID_FAIL_ORDER(succ, fail));
            }
        }
    }

    #[test]
    fn is_valid_fail_order_rejects_succ_lt_fail() {
        for succ in VALID_ORDERS {
            for fail in VALID_ORDERS {
                if succ < fail {
                    assert!(!PATOMIC_IS_VALID_FAIL_ORDER(succ, fail));
                }
            }
        }
    }

    #[test]
    fn is_valid_fail_order_rejects_invalid_succ_order() {
        for succ in INVALID_ORDERS {
            for fail in VALID_ORDERS {
                assert!(!PATOMIC_IS_VALID_FAIL_ORDER(succ, fail));
            }
        }
    }

    #[test]
    fn is_valid_fail_order_rejects_invalid_fail_order() {
        for fail in INVALID_ORDERS {
            for succ in VALID_ORDERS {
                assert!(!PATOMIC_IS_VALID_FAIL_ORDER(succ, fail));
            }
        }
    }

    #[test]
    fn is_valid_fail_order_rejects_non_load_fail_order() {
        for fail in VALID_ORDERS {
            if !LOAD_ORDERS.contains(&fail) {
                for succ in VALID_ORDERS {
                    assert!(!PATOMIC_IS_VALID_FAIL_ORDER(succ, fail));
                }
            }
        }
    }

    #[test]
    fn cmpxchg_fail_order_converts_valid_succ_order() {
        for order in VALID_ORDERS {
            let fail_order = PATOMIC_CMPXCHG_FAIL_ORDER(order);
            if LOAD_ORDERS.contains(&order) {
                assert_eq!(order, fail_order);
            } else {
                assert_eq!(patomic_ACQUIRE, fail_order);
            }
        }
    }

    #[test]
    fn cmpxchg_fail_order_returns_invalid_succ_order() {
        for order in INVALID_ORDERS {
            assert_eq!(order, PATOMIC_CMPXCHG_FAIL_ORDER(order));
        }
    }
}
