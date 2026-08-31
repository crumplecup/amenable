use super::CheckedProof;

use std::ops::{Bound, ControlFlow, RangeFull, RangeTo};

use crate::{
    BOUND_ROUND_TRIPS_ITS_ENDPOINT_HOLDS_SRC,
    CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_HOLDS_SRC, CreusotVerifier, CreusotWitness,
    RANGE_TO_CONTAINS_MATCHES_BOUND_HOLDS_SRC, VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC,
    VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC,
    VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC, VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<RangeTo<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_to_contains_matches_bound".to_string(),
            VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<RangeTo<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeTo<i32>>",
        "creusot",
        || <RustStdStandard<RangeTo<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::RANGE_TO_CONTAINS_MATCHES_BOUND_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn range_to_contains_matches_bound_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<RangeTo<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        RANGE_TO_CONTAINS_MATCHES_BOUND_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeTo<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<RangeTo<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<RangeFull> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_full_contains_everything".to_string(),
            VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<RangeFull>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeFull>",
        "creusot",
        || <RustStdStandard<RangeFull> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<Bound<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_bound_round_trips_its_endpoint".to_string(),
            VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Bound<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        "creusot",
        || <RustStdStandard<Bound<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::BOUND_ROUND_TRIPS_ITS_ENDPOINT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn bound_round_trips_its_endpoint_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Bound<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BOUND_ROUND_TRIPS_ITS_ENDPOINT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Bound<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<ControlFlow<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_control_flow_continue_and_break_are_disjoint".to_string(),
            VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<ControlFlow<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        "creusot",
        || {
            <RustStdStandard<ControlFlow<i32, i32>> as CreusotWitness>::proof().to_string()
        },
    )
}

/// Returns
/// `amenable_creusot::CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// control_flow_continue_and_break_are_disjoint_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<ControlFlow<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<ControlFlow<i32, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
