#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
amenable_derive::harness! {
    creusot, VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// Dereferencing a shared reference recovers exactly the value it
        /// borrows.
        #[requires(true)]
        #[ensures(shared_reference_dereferences_to_the_referent(value, result))]
        fn verify_shared_reference_dereferences_to_the_referent(value: i32) -> i32 {
            let reference = &value;
            *reference
        }
    }
}

amenable_derive::harness! {
    creusot, SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<&'static i32>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn shared_reference_dereferences_to_the_referent(
            value: i32,
            reference_result: i32,
        ) -> bool {
            pearlite! { reference_result == value }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// Dereferencing a mutable reference recovers the borrowed value,
        /// and writing through it updates the referent.
        #[requires(true)]
        #[ensures(mutable_reference_dereferences_to_and_updates_the_referent(initial, next, result))]
        fn verify_mutable_reference_dereferences_to_and_updates_the_referent(
            initial: i32,
            next: i32,
        ) -> (i32, i32) {
            let mut value = initial;
            let reference = &mut value;
            let before = *reference;
            *reference = next;
            (before, *reference)
        }
    }
}

amenable_derive::harness! {
    creusot, MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<&'static mut
        /// i32>` postcondition -- real, callable Pearlite content, not
        /// just descriptive text alongside it.
        #[logic(open)]
        fn mutable_reference_dereferences_to_and_updates_the_referent(
            initial: i32,
            next: i32,
            reference_result: (i32, i32),
        ) -> bool {
            pearlite! { reference_result.0 == initial && reference_result.1 == next }
        }
    }
}
