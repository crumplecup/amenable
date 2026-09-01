#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::alloc::System;
#[cfg(creusot)]
use std::boxed::Box;
amenable_derive::harness! {
    creusot, SYSTEM_ALLOCATION_ROUND_TRIPS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<System>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn system_allocation_round_trips(value: i32, alloc_result: i32) -> bool {
            pearlite! { alloc_result == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SYSTEM_ALLOCATES_AND_DEALLOCATES_A_LAYOUT_SRC, {
        /// `System` is the process's default global allocator in this crate,
        /// so a `Box` allocation and drop is serviced by `System` even though
        /// the allocator hooks stay behind ordinary Rust library code.
        #[trusted]
        #[requires(true)]
        // Canonical home: RustStdStandard<System>'s Ensures<CreusotVerifier>
        // impl (amenable_std::creusot_witness) names this exact fragment.
        #[ensures(system_allocation_round_trips(value, result))]
        fn verify_system_allocates_and_deallocates_a_layout(value: i32) -> i32 {
            let _allocator = System;
            let boxed = Box::new(value);
            let round_trip = *boxed;
            drop(boxed);
            round_trip
        }
    }
}
