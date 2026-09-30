use amenable_core::VerusVerifier;

use super::proof::TemporalVerusProof;
use crate::{
    IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
    UtcTimelineOrderingAppliesToFixedInstants,
};

// ── IntervalStartPrecedesEnd ──────────────────

const INTERVAL_START_PRECEDES_END_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/interval_start_precedes_end_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for IntervalStartPrecedesEnd {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_interval_start_precedes_end",
            INTERVAL_START_PRECEDES_END_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for IntervalStartPrecedesEnd {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::IntervalStartPrecedesEnd",
        "verus",
        || <IntervalStartPrecedesEnd as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    IntervalStartPrecedesEnd,
    "amenable_time::IntervalStartPrecedesEnd",
    [
        "interval_start_precedes_end_result_matches",
        "interval_start_precedes_end_matches_negated_form",
        "interval_start_precedes_end_matches_nonneg_span_form",
    ]
);

// ── IntervalDurationIsNonNegative ──────────────────

const INTERVAL_DURATION_IS_NON_NEGATIVE_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/interval_duration_is_non_negative_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for IntervalDurationIsNonNegative {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_interval_duration_is_non_negative",
            INTERVAL_DURATION_IS_NON_NEGATIVE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for IntervalDurationIsNonNegative {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::IntervalDurationIsNonNegative",
        "verus",
        || <IntervalDurationIsNonNegative as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    IntervalDurationIsNonNegative,
    "amenable_time::IntervalDurationIsNonNegative",
    [
        "interval_duration_is_non_negative_result_matches",
        "interval_duration_is_non_negative_matches_start_le_end_form",
        "interval_zero_span_iff_endpoints_equal",
    ]
);

// ── UtcTimelineOrderingAppliesToFixedInstants ──────────────────

const UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/utc_timeline_ordering_applies_to_fixed_instants_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_utc_timeline_ordering_applies_to_fixed_instants",
            UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for UtcTimelineOrderingAppliesToFixedInstants {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
        "verus",
        || <UtcTimelineOrderingAppliesToFixedInstants as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    UtcTimelineOrderingAppliesToFixedInstants,
    "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
    [
        "utc_timeline_ordering_result_matches",
        "utc_timeline_ordering_applies_to_fixed_instants_holds",
        "utc_timeline_ordering_is_total",
        "utc_timeline_ordering_is_antisymmetric",
        "utc_timeline_ordering_is_transitive",
    ]
);
