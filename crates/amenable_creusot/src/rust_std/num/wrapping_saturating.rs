//! `Wrapping<i32>` and `Saturating<i32>` arithmetic: `+` wraps / clamps
//! exactly like the inner type's `wrapping_add` / `saturating_add`.

#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires};
#[cfg(creusot)]
use std::num::{Saturating, Wrapping};

amenable_derive::harness! {
    creusot, WRAPPING_I32_ADD_WRAPS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// Wrapping<i32>>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn wrapping_i32_add_wraps_holds(
            a: Wrapping<i32>,
            b: Wrapping<i32>,
            add_result: Wrapping<i32>,
        ) -> bool {
            pearlite! { add_result.0 == a.0 + b.0 }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_WRAPPING_ADD_MATCHES_THE_INNER_WRAPPING_ADD_SRC, {
        /// `Wrapping<T>`'s `+` operator wraps on overflow exactly like the
        /// inner type's `wrapping_add` — the same claim
        /// `amenable_kani::rust_std::num::verify_wrapping_add_matches_the_inner_wrapping_add`
        /// checks by symbolic execution (there, comparing against
        /// `a.wrapping_add(b)` directly). Rests on the local `extern_spec!`
        /// above, the same relationship every other non-`char`/`String`
        /// harness in this file has to a trusted axiom on the real method
        /// it exercises.
        #[requires(true)]
        #[ensures(wrapping_i32_add_wraps_holds(a, b, result))]
        fn verify_wrapping_i32_add_wraps(a: Wrapping<i32>, b: Wrapping<i32>) -> Wrapping<i32> {
            a + b
        }
    }
}

// Same per-concrete-type macro shape as `Wrapping<T>` (confirmed by
// reading the real source, `library/core/src/num/saturating.rs`: `impl
// const Add for Saturating<$t>` generated once per width, not one
// generic sealed-trait impl), so a local `extern_spec!` is practical
// here too — but the semantics are clamping, not wraparound, so the
// postcondition restates `creusot-std`'s own three-way `@`-lifted
// contract for the plain `i32::saturating_add` method (`spec_op_common!`
// in `creusot_std::std::num`) in terms of the wrapper's `.0` fields,
// rather than reusing Wrapping's plain-`+` idiom.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Saturating<i32> {
        #[check(ghost)]
        #[ensures(
            (self.0@ + rhs.0@) >= i32::MIN@ && (self.0@ + rhs.0@) <= i32::MAX@
            ==> result.0@ == (self.0@ + rhs.0@)
        )]
        #[ensures((self.0@ + rhs.0@) < i32::MIN@ ==> result.0@ == i32::MIN@)]
        #[ensures((self.0@ + rhs.0@) > i32::MAX@ ==> result.0@ == i32::MAX@)]
        fn add(self, rhs: Saturating<i32>) -> Saturating<i32>;
    }
}

amenable_derive::harness! {
    creusot, SATURATING_I32_ADD_CLAMPS_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<
        /// Saturating<i32>>` postcondition -- real, callable Pearlite
        /// content, not just descriptive text alongside it.
        #[logic(open)]
        fn saturating_i32_add_clamps_holds(
            a: Saturating<i32>,
            b: Saturating<i32>,
            add_result: Saturating<i32>,
        ) -> bool {
            pearlite! {
                ((a.0@ + b.0@) >= i32::MIN@ && (a.0@ + b.0@) <= i32::MAX@
                    ==> add_result.0@ == (a.0@ + b.0@))
                    && ((a.0@ + b.0@) < i32::MIN@ ==> add_result.0@ == i32::MIN@)
                    && ((a.0@ + b.0@) > i32::MAX@ ==> add_result.0@ == i32::MAX@)
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_SATURATING_ADD_MATCHES_THE_INNER_SATURATING_ADD_SRC, {
        /// `Saturating<T>`'s `+` operator saturates at the numeric bounds
        /// exactly like the inner type's `saturating_add` — the same
        /// claim `amenable_kani::rust_std::num::verify_saturating_add_matches_the_inner_saturating_add`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, which restates `creusot-std`'s own trusted axiom for
        /// `i32::saturating_add` in terms of `Saturating<i32>`'s wrapper
        /// field.
        #[requires(true)]
        #[ensures(saturating_i32_add_clamps_holds(a, b, result))]
        fn verify_saturating_i32_add_clamps(a: Saturating<i32>, b: Saturating<i32>) -> Saturating<i32> {
            a + b
        }
    }
}
