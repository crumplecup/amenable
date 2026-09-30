use amenable_core::VerusVerifier;

use super::proof::TemporalVerusProof;
use crate::{
    CentennialYearDivisibleByOneHundred, CommonYearHasThreeHundredSixtyFiveCalendarDays,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    LeapYearHasThreeHundredSixtySixCalendarDays,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
};

// ── GregorianLeapYearUsesDivisibleByFourAndFourHundredException ──────────────────

const GREGORIAN_LEAP_YEAR_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/gregorian_leap_year_carrier.rs");

impl amenable_core::Witness<VerusVerifier>
    for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
{
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new("verify_gregorian_leap_year", GREGORIAN_LEAP_YEAR_VERUS_SRC)
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
        "verus",
        || <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
    [
        "gregorian_leap_year_result_matches",
        "gregorian_leap_year_implies_divisible_by_four",
        "gregorian_leap_year_holds",
    ]
);

// ── CentennialYearDivisibleByOneHundred ──────────────────

const CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/centennial_year_divisible_by_one_hundred_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for CentennialYearDivisibleByOneHundred {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_centennial_year_divisible_by_one_hundred",
            CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for CentennialYearDivisibleByOneHundred {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CentennialYearDivisibleByOneHundred",
        "verus",
        || <CentennialYearDivisibleByOneHundred as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    CentennialYearDivisibleByOneHundred,
    "amenable_time::CentennialYearDivisibleByOneHundred",
    [
        "centennial_year_result_matches",
        "centennial_year_divisible_by_one_hundred_holds"
    ]
);

// ── LeapYearHasThreeHundredSixtySixCalendarDays ──────────────────

const LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/leap_year_has_three_hundred_sixty_six_calendar_days_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for LeapYearHasThreeHundredSixtySixCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_leap_year_has_three_hundred_sixty_six_calendar_days",
            LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for LeapYearHasThreeHundredSixtySixCalendarDays
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
        "verus",
        || <LeapYearHasThreeHundredSixtySixCalendarDays as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    LeapYearHasThreeHundredSixtySixCalendarDays,
    "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
    [
        "leap_year_has_366_days_result_matches",
        "leap_year_day_count_matches_gregorian_rule",
        "days_in_year_is_365_or_366",
    ]
);

amenable_derive::verus_requires_predicate!(
    LeapYearHasThreeHundredSixtySixCalendarDays,
    "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
    "year_is_non_negative"
);

// ── CommonYearHasThreeHundredSixtyFiveCalendarDays ──────────────────

const COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/common_year_has_three_hundred_sixty_five_calendar_days_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for CommonYearHasThreeHundredSixtyFiveCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_common_year_has_three_hundred_sixty_five_calendar_days",
            COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for CommonYearHasThreeHundredSixtyFiveCalendarDays
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
        "verus",
        || <CommonYearHasThreeHundredSixtyFiveCalendarDays as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    CommonYearHasThreeHundredSixtyFiveCalendarDays,
    "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
    [
        "common_year_has_365_days_result_matches",
        "common_year_day_count_matches_gregorian_rule",
        "days_in_year_is_365_or_366",
    ]
);

amenable_derive::verus_requires_predicate!(
    CommonYearHasThreeHundredSixtyFiveCalendarDays,
    "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
    "year_is_non_negative"
);

// ── YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays ──────────────────

const YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_carrier.rs");

impl amenable_core::Witness<VerusVerifier>
    for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
{
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new("verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days", YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_VERUS_SRC)
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
        "verus",
        || <YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
    ["year_duration_result_matches", "days_in_year_is_365_or_366"]
);

amenable_derive::verus_requires_predicate!(
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
    "year_is_non_negative"
);
