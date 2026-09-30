use amenable_core::VerusVerifier;

use super::proof::TemporalVerusProof;
use crate::{
    HourInRangeZeroToTwentyFour, MinuteInRangeZeroToFiftyNine, SecondInRangeZeroToSixty,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
};

// ── HourInRangeZeroToTwentyFour ──────────────────────────────────

const HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/hour_in_range_zero_to_twenty_four_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for HourInRangeZeroToTwentyFour {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_hour_in_range_zero_to_twenty_four",
            HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for HourInRangeZeroToTwentyFour {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::HourInRangeZeroToTwentyFour",
        "verus",
        || {
            <HourInRangeZeroToTwentyFour as amenable_core::Witness<VerusVerifier>>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    HourInRangeZeroToTwentyFour,
    "amenable_time::HourInRangeZeroToTwentyFour",
    "hour_in_range_zero_to_twenty_four_result_matches"
);

// ── MinuteInRangeZeroToFiftyNine ──────────────────────────────────

const MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/minute_in_range_zero_to_fifty_nine_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for MinuteInRangeZeroToFiftyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_minute_in_range_zero_to_fifty_nine",
            MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for MinuteInRangeZeroToFiftyNine {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::MinuteInRangeZeroToFiftyNine",
        "verus",
        || {
            <MinuteInRangeZeroToFiftyNine as amenable_core::Witness<VerusVerifier>>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    MinuteInRangeZeroToFiftyNine,
    "amenable_time::MinuteInRangeZeroToFiftyNine",
    "minute_in_range_zero_to_fifty_nine_result_matches"
);

// ── SecondInRangeZeroToSixty ──────────────────────────────────

const SECOND_IN_RANGE_ZERO_TO_SIXTY_VERUS_SRC: &str =
    include_str!("../../../amenable_verus/src/time/second_in_range_zero_to_sixty_carrier.rs");

impl amenable_core::Witness<VerusVerifier> for SecondInRangeZeroToSixty {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_second_in_range_zero_to_sixty",
            SECOND_IN_RANGE_ZERO_TO_SIXTY_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for SecondInRangeZeroToSixty {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::SecondInRangeZeroToSixty",
        "verus",
        || {
            <SecondInRangeZeroToSixty as amenable_core::Witness<VerusVerifier>>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    SecondInRangeZeroToSixty,
    "amenable_time::SecondInRangeZeroToSixty",
    "second_in_range_zero_to_sixty_result_matches"
);

// ── UtcOffsetHourInRangeZeroToTwentyThree ──────────────────────────────────

const UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/utc_offset_hour_in_range_zero_to_twenty_three_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_utc_offset_hour_in_range_zero_to_twenty_three",
            UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::UtcOffsetHourInRangeZeroToTwentyThree",
        "verus",
        || {
            <UtcOffsetHourInRangeZeroToTwentyThree as amenable_core::Witness<VerusVerifier>>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    UtcOffsetHourInRangeZeroToTwentyThree,
    "amenable_time::UtcOffsetHourInRangeZeroToTwentyThree",
    "utc_offset_hour_in_range_zero_to_twenty_three_result_matches"
);

// ── UtcOffsetMinuteInRangeZeroToFiftyNine ──────────────────────────────────

const UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_VERUS_SRC: &str = include_str!(
    "../../../amenable_verus/src/time/utc_offset_minute_in_range_zero_to_fifty_nine_carrier.rs"
);

impl amenable_core::Witness<VerusVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = TemporalVerusProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        TemporalVerusProof::new(
            "verify_utc_offset_minute_in_range_zero_to_fifty_nine",
            UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_VERUS_SRC,
        )
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> amenable_core::WitnessSupportSummary {
        amenable_core::WitnessSupportSummary::checked_leaf()
    }
}

impl amenable_core::ClassifiedWitness<VerusVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {}

inventory::submit! {
    amenable_core::ProofRecord::new(
        "amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine",
        "verus",
        || {
            <UtcOffsetMinuteInRangeZeroToFiftyNine as amenable_core::Witness<VerusVerifier>>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    UtcOffsetMinuteInRangeZeroToFiftyNine,
    "amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine",
    "utc_offset_minute_in_range_zero_to_fifty_nine_result_matches"
);
