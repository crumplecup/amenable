#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::borrow::Cow;
amenable_derive::harness! {
    creusot, VERIFY_COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// `Cow` stores either a borrowed or owned value, and
        /// destructuring the enum recovers that wrapped `i32`
        /// unchanged.
        ///
        /// `creusot-std` 0.11.0 ships no contracts for
        /// `alloc::borrow::Cow`, and calling uncontracted external
        /// methods such as `Deref::deref` or `Cow::into_owned` would
        /// poison the whole goal. So this uses only local construction
        /// and pattern matching on the enum itself.
        #[requires(true)]
        #[ensures(cow_destructure_recovers_the_wrapped_value(value, result))]
        fn verify_cow_destructure_recovers_the_wrapped_value(value: Cow<'static, i32>) -> i32 {
            match value {
                Cow::Borrowed(borrowed) => *borrowed,
                Cow::Owned(owned) => owned,
            }
        }
    }
}

amenable_derive::harness! {
    creusot, COW_DESTRUCTURE_RECOVERS_THE_WRAPPED_VALUE_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Cow<'static,
        /// i32>>` postcondition -- real, callable Pearlite content,
        /// not just descriptive text alongside it.
        #[logic(open)]
        fn cow_destructure_recovers_the_wrapped_value(
            value: Cow<'static, i32>,
            cow_result: i32,
        ) -> bool {
            pearlite! {
                match value {
                    Cow::Borrowed(borrowed) => cow_result == *borrowed,
                    Cow::Owned(owned) => cow_result == owned,
                }
            }
        }
    }
}
