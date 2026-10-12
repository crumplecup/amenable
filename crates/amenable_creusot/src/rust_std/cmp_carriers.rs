#[cfg(creusot)]
use creusot_std::macros::{check, ensures, extern_spec, logic, requires, trusted};
#[cfg(creusot)]
use std::cmp::{Ordering, Reverse};
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

// No local `extern_spec! { impl Add for Wrapping<i32> }` any more:
// `creusot-std` 0.14.0 added its own native coverage (`spec_wrapping!`
// in `creusot_std::std::num`, instantiated for every integer width
// including `i32`) with the identical postcondition
// (`result.0 == self.0 + rhs.0`) this crate used to hand-roll here —
// confirmed by real `cargo creusot` output ("duplicate extern
// specification for <Wrapping<i32> as Add>::add") after bumping to
// 0.14.0, and by reading `creusot-std` 0.14.0's own source directly.
// `rust_std::num::wrapping_saturating`'s real harness (`verify_wrapping_
// i32_add_wraps`) still exercises `Wrapping<i32>`'s `+` operator; it now
// rests on `creusot-std`'s own axiom instead of this crate's.

// No local `extern_spec! { impl Ord for Reverse<T> }` any more: it worked
// under creusot-std 0.11 (a generic `T: Ord + OrdLogic` axiom on `cmp`),
// but Creusot 0.13's PR #2174 split `OrdLogic` into `PartialOrdLogic` +
// `OrdLogic: PartialOrdLogic` and gave `Reverse<T>` only the *partial*
// instance (`impl<T: PartialOrdLogic> PartialOrdLogic for Reverse<T>`,
// confirmed on both v0.13.0 and master). Specifying `Ord::cmp` -- which
// returns a total `Ordering` -- then requires `Reverse<T>: OrdLogic`,
// which no longer exists and can't be added here (orphan rule: both
// `OrdLogic` and `Reverse` are foreign). The `Reverse<i32>` harness below
// is `#[trusted]` as a result -- see its own doc comment.

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
        /// checks by symbolic execution.
        ///
        /// `#[trusted]`, unlike the `Ordering`/`Wrapping` harnesses in
        /// this file which rest on a real local `extern_spec!`. Creusot
        /// 0.13's PR #2174 split `OrdLogic` into `PartialOrdLogic` +
        /// `OrdLogic` and left `Reverse<T>` with only the partial
        /// instance, so `Ord::cmp` on `Reverse` can no longer be given a
        /// spec (it returns a total `Ordering`, which needs
        /// `Reverse<T>: OrdLogic`) and the orphan rule blocks adding that
        /// impl here. Same honest "asserted, not mechanically discharged"
        /// treatment as `num::nonzero`'s `verify_nonzero_i16_roundtrips`;
        /// revisit if creusot-std adds `OrdLogic for Reverse`.
        #[trusted]
        #[requires(true)]
        #[ensures(reverse_inverts_comparison_holds(a, b, result))]
        fn verify_reverse_inverts_comparison(a: i32, b: i32) -> (Ordering, i32) {
            (Reverse(a).cmp(&Reverse(b)), Reverse(a).0)
        }
    }
}
