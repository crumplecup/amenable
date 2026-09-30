use amenable_core::VerusVerifier;

use super::proof::TemporalVerusProof;
use crate::{
    CalendarDayWithinMonthBounds, LeapDayOccursOnlyInLeapYear,
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
};

// ── MonthDurationInRangeTwentyEightToThirtyOneCalendarDays ──────────────────

const MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier>
    for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
{
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days",
            MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
        "verus",
        || <MonthDurationInRangeTwentyEightToThirtyOneCalendarDays as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
    [
        "month_duration_result_matches",
        "month_durations_sum_to_year_duration"
    ]
);

amenable_derive::verus_requires_predicate!(
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
    "month_in_range_one_to_twelve_for_requires"
);

// ── CalendarDayWithinMonthBounds ──────────────────

const CALENDAR_DAY_WITHIN_MONTH_BOUNDS_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/calendar_day_within_month_bounds_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for CalendarDayWithinMonthBounds {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_calendar_day_within_month_bounds",
            CALENDAR_DAY_WITHIN_MONTH_BOUNDS_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for CalendarDayWithinMonthBounds {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CalendarDayWithinMonthBounds",
        "verus",
        || <CalendarDayWithinMonthBounds as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    CalendarDayWithinMonthBounds,
    "amenable_time::CalendarDayWithinMonthBounds",
    [
        "calendar_day_within_bounds_result_matches",
        "calendar_day_within_bounds_result_implies_day_in_range",
        "calendar_day_feb29_result_implies_leap_year",
    ]
);

amenable_derive::verus_requires_predicate!(
    CalendarDayWithinMonthBounds,
    "amenable_time::CalendarDayWithinMonthBounds",
    "month_in_range_one_to_twelve_for_requires"
);

// ── LeapDayOccursOnlyInLeapYear ──────────────────

const LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/leap_day_occurs_only_in_leap_year_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for LeapDayOccursOnlyInLeapYear {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_leap_day_occurs_only_in_leap_year",
            LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for LeapDayOccursOnlyInLeapYear {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::LeapDayOccursOnlyInLeapYear",
        "verus",
        || <LeapDayOccursOnlyInLeapYear as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    LeapDayOccursOnlyInLeapYear,
    "amenable_time::LeapDayOccursOnlyInLeapYear",
    [
        "leap_day_result_matches",
        "leap_day_validity_matches_gregorian_rule"
    ]
);
