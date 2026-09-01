#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::boxed::Box;
amenable_derive::harness! {
    creusot, VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, {
        /// `Box::new` stores the supplied `i32`, and `Box::as_ref`
        /// exposes that same wrapped value through a shared borrow.
        ///
        /// This leans directly on `creusot-std`'s own upstream
        /// contracts for `Box::new` (`*result == val`) and
        /// `Box::as_ref` (`**self == *result`) instead of postulating
        /// any local model.
        #[requires(true)]
        #[ensures(box_new_preserves_the_wrapped_value(value, result))]
        fn verify_box_new_preserves_the_wrapped_value(value: i32) -> i32 {
            let boxed = Box::new(value);
            *boxed.as_ref()
        }
    }
}

amenable_derive::harness! {
    creusot, BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Box<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn box_new_preserves_the_wrapped_value(value: i32, box_result: i32) -> bool {
            pearlite! { box_result == value }
        }
    }
}
