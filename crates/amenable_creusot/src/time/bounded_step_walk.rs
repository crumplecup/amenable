//! `Witness<CreusotVerifier>` for `amenable_ext::BoundedStepWalk`.
//!
//! Mirrors `amenable_kani::chrono::bounded_step_walk`'s own one-step
//! lemma exactly, in Pearlite rather than Kani's executable `bool`: the
//! real stepping formula `next = (prev + delta).min(hi)` never moves
//! backward and never advances by more than `delta`, under the same
//! precondition (`delta > 0 && prev <= hi &&
//! prev.checked_add(delta).is_some()`).

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_ext::BoundedStepWalk;

    use crate::CreusotVerifier;

    use super::{BOUNDED_STEP_WALK_HOLDS_SRC, VERIFY_BOUNDED_STEP_WALK_SRC};

    impl amenable_core::Witness<CreusotVerifier> for BoundedStepWalk {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_bounded_step_walk".to_owned(),
                VERIFY_BOUNDED_STEP_WALK_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for BoundedStepWalk {
        type Input = (i64, i64, i64);
        type Bound = &'static str;

        fn ensures(_step: (i64, i64, i64)) -> Self::Bound {
            BOUNDED_STEP_WALK_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_ext::BoundedStepWalk",
            "creusot",
            || {
                <BoundedStepWalk as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::ClassifiedWitness<CreusotVerifier> for BoundedStepWalk {}
}

amenable_derive::harness! {
    creusot, BOUNDED_STEP_WALK_REQUIRES_SRC, {
        /// The precondition this claim is stated under: `delta > 0`,
        /// `prev <= hi`, and `prev + delta` does not overflow `i64` --
        /// named so `#[requires(..)]` calls it rather than restating it
        /// inline (inline literal comparisons against a raw `i64`, not
        /// its `@` deep model, fail to type-check under the real
        /// Creusot compiler -- confirmed directly, not assumed).
        #[logic(open)]
        pub fn bounded_step_walk_requires(prev: i64, delta: i64, hi: i64) -> bool {
            pearlite! { delta@ > 0 && prev@ <= hi@ && prev@ + delta@ <= i64::MAX@ }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::bounded_step_walk_requires",
        "creusot",
        "requires",
        || BOUNDED_STEP_WALK_REQUIRES_SRC,
    )
}

amenable_derive::harness! {
    creusot, BOUNDED_STEP_WALK_HOLDS_SRC, {
        /// One step of `next = (prev + delta).min(hi)` never moves
        /// backward and never advances by more than `delta`, under the
        /// precondition `delta > 0 && prev <= hi`: the outcome equals
        /// `next` lying in `[prev, prev + delta]`.
        #[logic(open)]
        pub fn bounded_step_walk_holds(prev: i64, delta: i64, hi: i64, next: i64) -> bool {
            pearlite! { next@ >= prev@ && next@ - prev@ <= delta@ }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::bounded_step_walk_holds",
        "creusot",
        "ensures",
        || BOUNDED_STEP_WALK_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_BOUNDED_STEP_WALK_SRC, {
        /// `(prev + delta).min(hi)` satisfies the bounded-step spec, for
        /// every `i64` triple respecting the precondition -- the
        /// overflow guard is load-bearing, mirroring the real Kani
        /// finding (`prev + delta` genuinely overflows for `prev`/
        /// `delta` near `i64::MAX` without it).
        #[requires(bounded_step_walk_requires(prev, delta, hi))]
        #[ensures(bounded_step_walk_holds(prev, delta, hi, result))]
        fn check_bounded_step_walk(prev: i64, delta: i64, hi: i64) -> i64 {
            (prev + delta).min(hi)
        }
    }
}
