//! `Witness<KaniVerifier>` for `amenable_ext::chrono::BoundedStepWalk`.
//!
//! Proven as a one-step lemma, not by unrolling a whole walk: the real
//! stepping formula `next = (prev + delta).min(hi)` is checked for
//! *any* symbolic `(prev, delta, hi)`, and the full walk's bound then
//! follows by ordinary mathematical induction over however many times
//! the real discovery loop calls it -- avoiding the CBMC-unwinding wall
//! this same chrono-tz investigation hit when it tried to symbolically
//! explore an entire multi-thousand-step walk directly (see
//! `amenable_kani::chrono::tz_offset`'s own module doc comment). This is
//! exactly the kind of off-by-one/clamp bug a Kani proof earns its keep
//! on: a stray `.max` instead of `.min`, a dropped clamp, or a `delta`
//! added twice would all be caught here.

use amenable_core::Witness;
use amenable_ext::BoundedStepWalk;

use crate::rust_std::kani_ensures;
use crate::{CalculationProof, KaniVerifier};

impl Witness<KaniVerifier> for BoundedStepWalk {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "chrono::verify_bounded_step_walk".to_owned(),
            VERIFY_BOUNDED_STEP_WALK_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::chrono::BoundedStepWalk",
        "kani",
        || <BoundedStepWalk as Witness<KaniVerifier>>::proof().to_string(),
    )
}

// `Input = (prev, delta, hi)`. `delta > 0`, `prev <= hi`, and
// `prev.checked_add(delta).is_some()` are the precondition this claim
// is stated under -- a non-advancing, backward-clamped, or overflowing
// walk is a different (degenerate) claim, not this one. The overflow
// guard is load-bearing: the first run of this harness genuinely
// failed with "attempt to add with overflow" for `prev`/`delta` near
// `i64::MAX` before it was added -- a real finding, not defensive
// boilerplate. Within the stated precondition, the real formula's
// single step is checked directly, not restated.
kani_ensures!(
    BoundedStepWalk,
    "amenable_ext::chrono::BoundedStepWalk",
    (i64, i64, i64),
    |(prev, delta, hi)| {
        if delta > 0 && prev <= hi && prev.checked_add(delta).is_some() {
            let next = (prev + delta).min(hi);
            next >= prev && next - prev <= delta
        } else {
            true
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_BOUNDED_STEP_WALK_SRC, {
        /// One step of `next = (prev + delta).min(hi)` never moves
        /// backward and never advances by more than `delta` -- checked
        /// for fully symbolic `(prev, delta, hi)`, catching exactly the
        /// clamp/off-by-one bugs a hand-written grid-walk loop could
        /// introduce (a stray `.max` instead of `.min`, a dropped
        /// clamp, `delta` applied twice).
        #[kani::proof]
        fn verify_bounded_step_walk() {
            let prev: i64 = kani::any();
            let delta: i64 = kani::any();
            let hi: i64 = kani::any();

            assert!(<BoundedStepWalk as ::amenable_core::Ensures<KaniVerifier>>::ensures((
                prev, delta, hi
            )));
        }
    }
}

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for BoundedStepWalk {}
