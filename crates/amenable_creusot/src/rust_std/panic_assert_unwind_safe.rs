#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::panic::AssertUnwindSafe;
amenable_derive::harness! {
    creusot, VERIFY_ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// `AssertUnwindSafe` is a transparent tuple wrapper around the
        /// carried value; Creusot can prove the wrapper-level round trip
        /// directly through its public field.
        #[requires(true)]
        #[ensures(assert_unwind_safe_derefs_transparently(value, updated, result))]
        fn verify_assert_unwind_safe_derefs_transparently(
            value: i32,
            updated: i32,
        ) -> (i32, i32) {
            let mut wrapped = AssertUnwindSafe(value);
            let first = wrapped.0;
            wrapped.0 = updated;
            (first, wrapped.0)
        }
    }
}

amenable_derive::harness! {
    creusot, ASSERT_UNWIND_SAFE_DEREFS_TRANSPARENTLY_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// AssertUnwindSafe<i32>>` postcondition -- real, callable
        /// Pearlite content, not just descriptive text alongside it.
        #[logic(open)]
        fn assert_unwind_safe_derefs_transparently(
            value: i32,
            updated: i32,
            wrapper_result: (i32, i32),
        ) -> bool {
            pearlite! { wrapper_result.0 == value && wrapper_result.1 == updated }
        }
    }
}
