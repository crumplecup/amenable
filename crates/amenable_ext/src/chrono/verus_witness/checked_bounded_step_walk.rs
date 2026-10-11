//! `Witness<VerusVerifier>` for `amenable_ext::BoundedStepWalk`.
//!
//! `BoundedStepWalk` isn't an `ExtStandard<T>`-wrapped foreign type (it's
//! a native `i64`-arithmetic claim, like `amenable_time::
//! IntervalStartPrecedesEnd`), so `impl_verus_witness_checked_ext!`
//! (built around `ExtStandard<T>`) doesn't apply -- this mirrors
//! `amenable_time::verus_witness::interval_ordering`'s own hand-rolled
//! shape instead, `include_str!`-ing the real carrier file by relative
//! path (no Cargo dependency on `amenable_verus` needed, same reason
//! that crate isn't one).

use amenable_core::{ClassifiedWitness, VerusVerifier, Witness};
use amenable_time::TemporalVerusProof;

use crate::BoundedStepWalk;

const BOUNDED_STEP_WALK_VERUS_SRC: &str =
    include_str!("../../../../amenable_verus/src/chrono/bounded_step_walk.rs");

impl Witness<VerusVerifier> for BoundedStepWalk {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new("verify_bounded_step_walk", BOUNDED_STEP_WALK_VERUS_SRC)
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for BoundedStepWalk {}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::BoundedStepWalk",
        "verus",
        || <BoundedStepWalk as Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    BoundedStepWalk,
    "amenable_ext::BoundedStepWalk",
    "bounded_step_walk_result_matches"
);

amenable_derive::verus_requires_predicate!(
    BoundedStepWalk,
    "amenable_ext::BoundedStepWalk",
    ["bounded_step_walk_requires"]
);
