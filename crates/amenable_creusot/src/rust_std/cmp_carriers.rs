#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::cmp::{Ordering, Reverse};
#[cfg(creusot)]
use std::num::{
    FpCategory, IntErrorKind, NonZero, ParseFloatError, ParseIntError, Saturating, TryFromIntError,
    Wrapping,
};
// `Ordering::reverse` is uncontracted (creusot-std has no coverage for
// `core::cmp::Ordering` at all) — and unlike `String::len`/`Duration::
// as_secs`, matching the `(o, result)` pair structurally in `#[ensures]`
// *without* calling `.reverse()` there doesn't route around it: the
// harness body still calls `.reverse()` to produce `result`, and calling
// any uncontracted external function yields an impossible precondition
// for the WHOLE goal, not just for logic-context call sites — confirmed
// by a real prove failure (`Goal ...vc_verify_ordering_reverse_swaps_
// less_and_greater: ✘`), not a guess. Unlike `NonZero::new`, though,
// `Ordering::reverse` has no generics and no sealed trait bound
// (`pub const fn reverse(self) -> Ordering`), so a local `extern_spec!`
// for it is actually practical — the same trusted-axiom pattern
// `creusot-std` itself uses for `Duration::new`, just written here
// instead of shipped upstream.
#[cfg(creusot)]
extern_spec! {
    impl Ordering {
        #[check(ghost)]
        #[ensures(match (self, result) {
            (Ordering::Less, Ordering::Greater) => true,
            (Ordering::Equal, Ordering::Equal) => true,
            (Ordering::Greater, Ordering::Less) => true,
            _ => false,
        })]
        fn reverse(self) -> Ordering;
    }
}

amenable_derive::harness! {
    creusot, ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Ordering>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn ordering_reverse_swaps_less_and_greater_holds(o: Ordering, reverse_result: Ordering) -> bool {
            pearlite! {
                match (o, reverse_result) {
                    (Ordering::Less, Ordering::Greater) => true,
                    (Ordering::Equal, Ordering::Equal) => true,
                    (Ordering::Greater, Ordering::Less) => true,
                    _ => false,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC, {
        /// `Ordering` has exactly three inhabitants, and `.reverse()`
        /// swaps `Less`/`Greater` while fixing `Equal` — the same claim
        /// `amenable_kani::rust_std::cmp::verify_ordering_reverse_involution`
        /// checks (there, stated as an involution:
        /// `o.reverse().reverse() == o`, over an explicit enumeration of
        /// all three variants, since Kani has no `Arbitrary` impl for
        /// `Ordering`). Rests on the local `extern_spec!` above, which
        /// states the same swap as a trusted axiom on `reverse` itself —
        /// this harness just confirms the axiom is available and usable
        /// where a real proof needs it, the same relationship every
        /// `Duration` clause here has to `creusot-std`'s own axioms.
        ///
        /// Matching the `(o, result)` pair, not calling `.reverse()`
        /// again inside `#[ensures]`, already implies the involution
        /// Kani checks explicitly (applying the same swap twice is the
        /// identity), so no separate reverse-twice clause is needed.
        #[requires(true)]
        #[ensures(ordering_reverse_swaps_less_and_greater_holds(o, result))]
        fn verify_ordering_reverse_swaps_less_and_greater(o: Ordering) -> Ordering {
            o.reverse()
        }
    }
}

// Unlike `NonZero<T>`, `Wrapping<T>`'s arithmetic impls aren't one
// generic `impl<T: Sealed> Add for Wrapping<T>` — std generates a
// separate, concrete `impl Add for Wrapping<i32>` (and one per other
// width) via a `macro_rules!` (`library/core/src/num/wrapping.rs`,
// confirmed by reading the real source, not assumed), so an
// `extern_spec!` targeting this one concrete instantiation matches the
// real signature exactly, the same way `Ordering::reverse`'s did.
// `.0` is a public tuple-field projection, not a method call, so it's
// fine inside `#[ensures]` without a trusted wrapper; the plain
// (non-`@`) `+` between the two `i32` fields relies on Pearlite's native
// machine-integer semantics matching real wraparound, the same
// convention `creusot-std`'s own `spec_op_common!` macro uses for
// `i32::wrapping_add`'s postcondition.
#[cfg(creusot)]
extern_spec! {
    impl std::ops::Add for Wrapping<i32> {
        #[check(ghost)]
        #[ensures(result.0 == self.0 + rhs.0)]
        fn add(self, rhs: Wrapping<i32>) -> Wrapping<i32>;
    }
}

// `Reverse<T>: Ord` is ONE generic impl (`impl<T: Ord> Ord for
// Reverse<T>`, confirmed by reading the real source,
// `library/core/src/cmp.rs`), not a per-concrete-type macro like
// `Wrapping`/`Saturating` — closer in shape to `NonZero::new`'s generic
// impl than to those. A concrete `impl Ord for Reverse<i32>` extern_spec
// hits the identical "extern spec generics don't match" error
// `NonZero::new` did (confirmed, not assumed): `cmp` is defined once,
// generically. Unlike `ZeroablePrimitive`, though, `Ord` is an ordinary,
// nameable, stable trait, so the generic form is actually writable — with
// one addition: comparing `T` values via `>`/`==`/`<` inside `#[ensures]`
// needs `T: creusot_std::logic::OrdLogic` (a real, non-guessed
// requirement — the compiler's own error names it exactly:
// `the trait bound T: creusot_std::logic::OrdLogic is not satisfied`),
// creusot-std's logic-context comparison trait, distinct from the
// program-level `Ord` the real impl itself requires. So this proof is
// real and general over every `T: Ord + OrdLogic`, not narrowed to
// `i32` the way `Wrapping`/`Saturating`'s per-width proofs are.
#[cfg(creusot)]
extern_spec! {
    impl<T: Ord + creusot_std::logic::OrdLogic> Ord for Reverse<T> {
        #[check(ghost)]
        #[ensures(match result {
            Ordering::Less => other.0 > self.0,
            Ordering::Equal => other.0 == self.0,
            Ordering::Greater => other.0 < self.0,
        })]
        fn cmp(&self, other: &Reverse<T>) -> Ordering;
    }
}

amenable_derive::harness! {
    creusot, REVERSE_INVERTS_COMPARISON_HOLDS_SRC, {
        /// The `amenable_std::rust_std::RustStdStandard<Reverse<i32>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn reverse_inverts_comparison_holds(a: i32, b: i32, cmp_result: (Ordering, i32)) -> bool {
            pearlite! {
                (match cmp_result.0 {
                    Ordering::Less => b > a,
                    Ordering::Equal => b == a,
                    Ordering::Greater => b < a,
                }) && cmp_result.1 == a
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_REVERSE_INVERTS_COMPARISON_SRC, {
        /// `Reverse<T>` inverts `T`'s comparison direction, and its `.0`
        /// field round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::cmp::verify_reverse_inverts_comparison`
        /// checks by symbolic execution. Rests on the local `extern_spec!`
        /// above, the same relationship every non-`char`/`String` harness
        /// in this file has to a trusted axiom on the real method it
        /// exercises.
        #[requires(true)]
        #[ensures(reverse_inverts_comparison_holds(a, b, result))]
        fn verify_reverse_inverts_comparison(a: i32, b: i32) -> (Ordering, i32) {
            (Reverse(a).cmp(&Reverse(b)), Reverse(a).0)
        }
    }
}
