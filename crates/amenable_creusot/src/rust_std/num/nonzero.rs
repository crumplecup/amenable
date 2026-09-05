//! `NonZero<i16>`'s Creusot postconditions: `new` succeeds iff the input is
//! nonzero, and `.get()` round-trips the wrapped value. One representative
//! width -- the coverage checklist resolves every `NonZero{I,U}*` alias back
//! to `RustStdStandard<NonZero<i16>>`.

/// The `#[cfg(creusot)]` imports and trusted logic wrapper this file needs,
/// consolidated onto one `mod` gate -- see `stoplight::mirror` for the
/// rationale. `harness! { .. }` blocks reference all of it unqualified.
#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires, trusted};
    pub(super) use std::num::NonZero;

    // `NonZero::get` is a plain program function -- creusot-std has no
    // extern_spec for `NonZero<T>` at all. Trusted wrapper, same shape as
    // `string_len`.
    #[trusted]
    #[logic(opaque)]
    pub(super) fn nonzero_i16_get(_nz: &NonZero<i16>) -> i16 {
        dead
    }
}
#[cfg(creusot)]
use mirror::{NonZero, ensures, logic, nonzero_i16_get, requires, trusted};

amenable_derive::harness! {
    creusot, NONZERO_I16_NEW_SUCCEEDS_EXACTLY_WHEN_NONZERO_SRC, {
        /// The first `amenable_std::rust_std::RustStdStandard<NonZero<i16>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn nonzero_i16_new_succeeds_exactly_when_nonzero(
            value: i16,
            new_result: Option<NonZero<i16>>,
        ) -> bool {
            pearlite! {
                match new_result {
                    Some(_) => value != 0i16,
                    None => value == 0i16,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, NONZERO_I16_GET_ROUND_TRIPS_SRC, {
        /// The second `amenable_std::rust_std::RustStdStandard<NonZero<i16>>`
        /// postcondition -- real, callable Pearlite content, not just
        /// descriptive text alongside it.
        ///
        /// `opaque`, not `open`: it calls the module-private
        /// `nonzero_i16_get` (itself `#[trusted] #[logic(opaque)]`), and
        /// Creusot rejects an `open` (transparent) definition that would
        /// expose a less-visible item to its own callers.
        #[logic(opaque)]
        fn nonzero_i16_get_round_trips(value: i16, new_result: Option<NonZero<i16>>) -> bool {
            pearlite! {
                match new_result {
                    Some(nz) => nonzero_i16_get(&nz) == value,
                    None => true,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_NONZERO_I16_ROUNDTRIPS_SRC, {
        /// `NonZero::new` succeeds iff the input is nonzero, and `.get()`
        /// round-trips the wrapped value unchanged — the same claim
        /// `amenable_kani::rust_std::num::verify_nonzero_i16` checks by
        /// symbolic execution, restated as a Creusot postcondition.
        ///
        /// `#[trusted]`, unlike every other harness in this file: `new`
        /// is uncontracted (creusot-std covers plain integers and
        /// Duration, not `NonZero<T>` at all), and giving it one myself
        /// isn't practical — `extern_spec!` requires matching the real
        /// generic signature exactly (confirmed: `extern spec generics
        /// don't match` when targeting the concrete `NonZero<i16>`
        /// alone), and the real bound is `T: ZeroablePrimitive`, an
        /// `unsafe`, sealed, doc-comment-flagged-"currently permanently
        /// unstable" trait — not something nameable from outside `std`
        /// on stable Rust. So this states the same claim Kani checks by
        /// symbolic execution, honestly marked as asserted rather than
        /// mechanically discharged, the same way `elicitation`'s own
        /// reference pattern uses `#[trusted]` for claims judged "too
        /// hard to prove" rather than silently weakening them.
        ///
        /// One width, not all twelve `amenable_kani` proves separately
        /// (`i8` through `u128`/`usize`): the coverage checklist resolves
        /// every `NonZero{I,U}*` type alias back to the same evidence,
        /// `RustStdStandard<NonZero<i16>>`, so one representative case is
        /// what actually closes the gap there.
        ///
        /// Both `#[ensures]` clauses below are the canonical home
        /// `RustStdStandard<NonZero<i16>>`'s own `Ensures<CreusotVerifier>`
        /// impl (`amenable_creusot::rust_std_witness`) names.
        #[trusted]
        #[requires(true)]
        #[ensures(nonzero_i16_new_succeeds_exactly_when_nonzero(value, result))]
        #[ensures(nonzero_i16_get_round_trips(value, result))]
        fn verify_nonzero_i16_roundtrips(value: i16) -> Option<NonZero<i16>> {
            NonZero::new(value)
        }
    }
}
