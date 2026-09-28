//! Real Creusot proof content for `jiff::Error`'s classification
//! predicates (`ext::jiff::error` holds the `CreusotWitness` bridge).
//!
//! `jiff::Error` is uncontracted everywhere (no `creusot-std`/
//! `elicitation` prior art — a third-party type), and unlike `Offset`
//! its value isn't a simple bounded scalar: `Display`/`is_range`/
//! `is_invalid_parameter`/`is_crate_feature` all read a private
//! `ErrorKind` enum this crate has no access to. Each predicate gets
//! its own opaque boolean accessor (the same "opaque logic accessor"
//! shape `offset.rs`'s `offset_seconds_value` uses, just boolean-valued
//! instead of scalar-valued).
//!
//! Two designs were tried and dropped before the pairwise-`ensures`
//! shape below: (1) one shared enum-valued accessor — worked logically,
//! but its variants are only ever *matched*, never *constructed* (the
//! opaque accessor's body is a builtin placeholder, never real
//! construction), which `cargo creusot`'s own compiler pass flags as
//! dead code with no clean fix short of `#[allow]`; (2) a single named
//! `error_classifications_are_disjoint(&Error) -> bool` transparent
//! logic function computing the conjunction from the three opaque
//! accessors — real-toolchain-confirmed rejected too, a different real
//! error this time ("Cannot make `error_is_range` transparent in
//! `error_classifications_are_disjoint` as it would call a
//! less-visible item" — Creusot's proof-transparency check requires a
//! transparent function's own callees to be at least as visible as
//! itself, and this one needed to be visible enough to appear in an
//! `extern_spec!` on a fully-public foreign type). Stating the three
//! pairwise disjointness facts directly as `#[ensures(..)]` clauses on
//! each `extern_spec!` method avoids both: no intermediate function,
//! nothing to be "too transparent" or "never constructed." The claim
//! itself is a genuine fact, independently confirmed by reading jiff
//! 0.2.35's real match arms directly, not assumed: `is_range`/
//! `is_invalid_parameter`/`is_crate_feature` match on three disjoint
//! `ErrorKind` variant sets (`Bounds|SpecialBounds|JcoreRange` vs
//! `UnitConfig|Civil(..)` vs `CrateFeature`). This is the honest limit
//! of what's checkable about `jiff::Error`: `Display`/`chain()` content
//! is untouchable by Kani (see `amenable_kani::gallery::
//! jiff_error_drop_cost`) and isn't restated here either — this proof
//! is scoped to the classification predicates' own contract with each
//! other, not to what `Display` renders.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, extern_spec, logic, requires, trusted};

    #[trusted]
    #[logic(opaque)]
    pub(super) fn error_is_range(_e: &jiff::Error) -> bool {
        dead
    }

    #[trusted]
    #[logic(opaque)]
    pub(super) fn error_is_invalid_parameter(_e: &jiff::Error) -> bool {
        dead
    }

    #[trusted]
    #[logic(opaque)]
    pub(super) fn error_is_crate_feature(_e: &jiff::Error) -> bool {
        dead
    }
}
#[cfg(creusot)]
use mirror::{
    ensures, error_is_crate_feature, error_is_invalid_parameter, error_is_range, extern_spec,
    logic, requires,
};

#[cfg(creusot)]
extern_spec! {
    impl jiff::Error {
        #[ensures(result == error_is_range(self))]
        #[ensures(!(error_is_range(self) && error_is_invalid_parameter(self)))]
        #[ensures(!(error_is_range(self) && error_is_crate_feature(self)))]
        fn is_range(&self) -> bool;

        #[ensures(result == error_is_invalid_parameter(self))]
        #[ensures(!(error_is_invalid_parameter(self) && error_is_range(self)))]
        #[ensures(!(error_is_invalid_parameter(self) && error_is_crate_feature(self)))]
        fn is_invalid_parameter(&self) -> bool;

        #[ensures(result == error_is_crate_feature(self))]
        #[ensures(!(error_is_crate_feature(self) && error_is_range(self)))]
        #[ensures(!(error_is_crate_feature(self) && error_is_invalid_parameter(self)))]
        fn is_crate_feature(&self) -> bool;
    }
}

amenable_derive::harness! {
    creusot, ERROR_CLASSIFICATION_PREDICATES_ARE_MUTUALLY_EXCLUSIVE_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::Error>` postcondition —
        /// real, callable Pearlite content, not just descriptive text
        /// alongside it.
        #[logic(open)]
        fn error_classification_predicates_are_mutually_exclusive_holds(
            flags: (bool, bool, bool),
        ) -> bool {
            pearlite! {
                !(flags.0 && flags.1) && !(flags.0 && flags.2) && !(flags.1 && flags.2)
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::error::error_classification_predicates_are_mutually_exclusive_holds",
        "creusot",
        "ensures",
        || ERROR_CLASSIFICATION_PREDICATES_ARE_MUTUALLY_EXCLUSIVE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_ERROR_CLASSIFICATION_PREDICATES_ARE_MUTUALLY_EXCLUSIVE_SRC, {
        /// `is_range`/`is_invalid_parameter`/`is_crate_feature` never
        /// agree on the same error, resting on the disjoint-match-arms
        /// axiom named in this file's own module doc comment.
        #[requires(true)]
        #[ensures(error_classification_predicates_are_mutually_exclusive_holds(result))]
        fn verify_error_classification_predicates_are_mutually_exclusive(
            e: jiff::Error,
        ) -> (bool, bool, bool) {
            (e.is_range(), e.is_invalid_parameter(), e.is_crate_feature())
        }
    }
}
