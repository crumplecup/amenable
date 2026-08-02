//! Gallery case for `AtomicPtr::compare_exchange`'s failure path under Kani.
//!
//! The production `rust_std::sync_atomic::verify_atomic_ptr_load_store_swap_and_compare_exchange`
//! proof covers `new`/`load`, `store`/`load`, `swap`, and
//! `compare_exchange`'s success path, all of which verify cleanly. Adding a
//! failure-path check — call `compare_exchange` with a `current` that does
//! not match the atomic's real value, and assert the returned `Err` reports
//! that real value unchanged — looked like the obvious next assertion to
//! bolt on, and it compiles fine. It does not verify.
//!
//! This case reduces that failure to its minimal form: a single
//! `compare_exchange` call against a freshly constructed `AtomicPtr`, no
//! `swap` or prior exchange involved, still fails the same way. That rules
//! out interference from the rest of the production proof's operation
//! sequence — this is Kani's model of `atomic_compare_exchange`'s failure
//! branch for pointer types itself losing track of the reported value, not
//! a logic error anywhere in the production harness or in std.

#[cfg(kani)]
use std::sync::atomic::{AtomicPtr, Ordering};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::atomic_ptr_compare_exchange::mismatch_does_not_report_the_real_current_value".to_owned(),
            harness: "gallery::atomic_ptr_compare_exchange::mismatch_does_not_report_the_real_current_value".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "AtomicPtr::compare_exchange's failure branch loses the reported value under Kani".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, MISMATCH_DOES_NOT_REPORT_THE_REAL_CURRENT_VALUE_SRC, {
        /// Minimal reproduction: one `AtomicPtr`, one `compare_exchange`
        /// call with a `current` that cannot match (a `null_mut` current
        /// against an atomic freshly constructed from a real stack
        /// address), asserting the returned `Err` equals the atomic's real
        /// value. This should hold per `compare_exchange`'s documented
        /// contract, but Kani cannot verify it.
        #[kani::proof]
        fn mismatch_does_not_report_the_real_current_value() {
            let mut real_slot: i32 = kani::any();
            let real: *mut i32 = &mut real_slot;
            let atomic = AtomicPtr::new(real);

            let wrong_current: *mut i32 = std::ptr::null_mut();
            let new_value: *mut i32 = std::ptr::null_mut();
            let result =
                atomic.compare_exchange(wrong_current, new_value, Ordering::SeqCst, Ordering::SeqCst);

            assert_eq!(
                result,
                Err(real),
                "a mismatched compare_exchange should report the real current value unchanged"
            );
        }
    }
}
