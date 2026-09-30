use amenable_core::VerusVerifier;

use super::proof::TemporalVerusProof;
use crate::{
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CenturyOrdinalInRangeZeroToNinetyNine, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    OrdinalDayInRangeOneToThreeHundredSixtySix,
};

// ── OrdinalDayInRangeOneToThreeHundredSixtySix ──────────────────

const ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/ordinal_day_in_range_one_to_three_hundred_sixty_six_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for OrdinalDayInRangeOneToThreeHundredSixtySix {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_ordinal_day_in_range_one_to_three_hundred_sixty_six",
            ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for OrdinalDayInRangeOneToThreeHundredSixtySix
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
        "verus",
        || <OrdinalDayInRangeOneToThreeHundredSixtySix as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    OrdinalDayInRangeOneToThreeHundredSixtySix,
    "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
    "ordinal_day_in_range_one_to_three_hundred_sixty_six_result_matches"
);

// ── CenturyOrdinalInRangeZeroToNinetyNine ──────────────────

const CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/century_ordinal_in_range_zero_to_ninety_nine_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_century_ordinal_in_range_zero_to_ninety_nine",
            CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
        "verus",
        || <CenturyOrdinalInRangeZeroToNinetyNine as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    CenturyOrdinalInRangeZeroToNinetyNine,
    "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
    "century_ordinal_in_range_zero_to_ninety_nine_result_matches"
);

// ── DecadeOrdinalInRangeZeroToNineHundredNinetyNine ──────────────────

const DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for DecadeOrdinalInRangeZeroToNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine",
            DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for DecadeOrdinalInRangeZeroToNineHundredNinetyNine
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
        "verus",
        || <DecadeOrdinalInRangeZeroToNineHundredNinetyNine as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
    "decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_result_matches"
);

// ── CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine ──────────────────

const CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier>
    for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
{
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine",
            CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier>
    for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
{
}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
        "verus",
        || <CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as amenable_core::Witness<VerusVerifier>>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
    "calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_result_matches"
);
