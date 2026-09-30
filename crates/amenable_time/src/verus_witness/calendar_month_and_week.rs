use amenable_core::VerusVerifier;

use super::proof::TemporalVerusProof;
use crate::{
    CalendarMonthInRangeOneToTwelve, WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
};

// ── CalendarMonthInRangeOneToTwelve ──────────────────────────────────

const CALENDAR_MONTH_IN_RANGE_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/calendar_month_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for CalendarMonthInRangeOneToTwelve {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_calendar_month_in_range",
            CALENDAR_MONTH_IN_RANGE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for CalendarMonthInRangeOneToTwelve {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CalendarMonthInRangeOneToTwelve",
        "verus",
        || {
            <CalendarMonthInRangeOneToTwelve as amenable_core::Witness<VerusVerifier>>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    CalendarMonthInRangeOneToTwelve,
    "amenable_time::CalendarMonthInRangeOneToTwelve",
    "calendar_month_range_result_matches"
);

// ── WeekdayInRangeOneToSeven ──────────────────

const WEEKDAY_IN_RANGE_ONE_TO_SEVEN_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/weekday_in_range_one_to_seven_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for WeekdayInRangeOneToSeven {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_weekday_in_range_one_to_seven",
            WEEKDAY_IN_RANGE_ONE_TO_SEVEN_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for WeekdayInRangeOneToSeven {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::WeekdayInRangeOneToSeven",
        "verus",
        || <WeekdayInRangeOneToSeven as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    WeekdayInRangeOneToSeven,
    "amenable_time::WeekdayInRangeOneToSeven",
    "weekday_in_range_one_to_seven_result_matches"
);

// ── WeekNumberInRangeOneToFiftyThree ──────────────────

const WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/week_number_in_range_one_to_fifty_three_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for WeekNumberInRangeOneToFiftyThree {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_week_number_in_range_one_to_fifty_three",
            WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for WeekNumberInRangeOneToFiftyThree {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::WeekNumberInRangeOneToFiftyThree",
        "verus",
        || <WeekNumberInRangeOneToFiftyThree as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    WeekNumberInRangeOneToFiftyThree,
    "amenable_time::WeekNumberInRangeOneToFiftyThree",
    "week_number_in_range_one_to_fifty_three_result_matches"
);
