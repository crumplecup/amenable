//! Verus `Witness` impls for `amenable_time`'s genuinely-checkable
//! temporal contracts (`AMENABLE_TIME_PLAN.md` Phase 6).
//!
//! Unlike Kani and Creusot, the Verus toolchain is invoked as a bare
//! compiler over `amenable_verus/src/lib.rs` — it never resolves Cargo
//! dependencies, so the *proof* content (the real `verus! { ... }` spec
//! functions) lives there. This module is the ordinary-Rust half: it
//! ties each `amenable_time` contract to the Verus spec file that
//! machine-checks it (`include_str!`, so the two can never drift), and
//! registers the `Witness<VerusVerifier>` the `proof_composition`
//! composites need to resolve under `VerusVerifier`.

use amenable_std::VerusVerifier;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    CalendarMonthInRangeOneToTwelve, CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CenturyOrdinalInRangeZeroToNinetyNine, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    HourInRangeZeroToTwentyFour, IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
    MinuteInRangeZeroToFiftyNine, OrdinalDayInRangeOneToThreeHundredSixtySix,
    SecondInRangeZeroToSixty, UtcOffsetHourInRangeZeroToTwentyThree,
    UtcOffsetMinuteInRangeZeroToFiftyNine, UtcTimelineOrderingAppliesToFixedInstants,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
};

/// Proof artifact naming an `amenable_verus` Verus spec function that
/// machine-checks a temporal contract's invariant, and carrying the
/// whole spec file it lives in verbatim (`include_str!`) — the same
/// one-claim-per-file granularity `amenable_derive::harness!` gives the
/// Kani/Creusot artifacts.
#[derive(Debug, Clone, PartialEq, Eq, Getters, new)]
pub struct TemporalVerusProof {
    /// The Verus spec function that checks this contract's invariant.
    #[new(into)]
    harness: String,
    /// The spec's own source, verbatim.
    #[new(into)]
    claim: String,
}

impl core::fmt::Display for TemporalVerusProof {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, f)))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "harness: {}", self.harness)?;
        write!(f, "claim: {}", self.claim)
    }
}

// ── CalendarMonthInRangeOneToTwelve ──────────────────────────────────

const CALENDAR_MONTH_IN_RANGE_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/calendar_month_carrier.rs");

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

// ── HourInRangeZeroToTwentyFour ──────────────────────────────────

const HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/hour_in_range_zero_to_twenty_four_carrier.rs");

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

// ── MinuteInRangeZeroToFiftyNine ──────────────────────────────────

const MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/minute_in_range_zero_to_fifty_nine_carrier.rs");

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

// ── SecondInRangeZeroToSixty ──────────────────────────────────

const SECOND_IN_RANGE_ZERO_TO_SIXTY_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/second_in_range_zero_to_sixty_carrier.rs");

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

// ── UtcOffsetHourInRangeZeroToTwentyThree ──────────────────────────────────

const UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/utc_offset_hour_in_range_zero_to_twenty_three_carrier.rs"
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

// ── UtcOffsetMinuteInRangeZeroToFiftyNine ──────────────────────────────────

const UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/utc_offset_minute_in_range_zero_to_fifty_nine_carrier.rs"
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

// ── WeekdayInRangeOneToSeven ──────────────────

const WEEKDAY_IN_RANGE_ONE_TO_SEVEN_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/weekday_in_range_one_to_seven_carrier.rs");

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

// ── WeekNumberInRangeOneToFiftyThree ──────────────────

const WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/week_number_in_range_one_to_fifty_three_carrier.rs"
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

// ── OrdinalDayInRangeOneToThreeHundredSixtySix ──────────────────

const ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/ordinal_day_in_range_one_to_three_hundred_sixty_six_carrier.rs"
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

// ── CenturyOrdinalInRangeZeroToNinetyNine ──────────────────

const CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/century_ordinal_in_range_zero_to_ninety_nine_carrier.rs"
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

// ── DecadeOrdinalInRangeZeroToNineHundredNinetyNine ──────────────────

const DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_carrier.rs"
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

// ── CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine ──────────────────

const CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_carrier.rs"
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

// ── IntervalStartPrecedesEnd ──────────────────

const INTERVAL_START_PRECEDES_END_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/interval_start_precedes_end_carrier.rs");

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

// ── IntervalDurationIsNonNegative ──────────────────

const INTERVAL_DURATION_IS_NON_NEGATIVE_VERUS_SRC: &str =
    include_str!("../../amenable_verus/src/time/interval_duration_is_non_negative_carrier.rs");

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

// ── UtcTimelineOrderingAppliesToFixedInstants ──────────────────

const UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_VERUS_SRC: &str = include_str!(
    "../../amenable_verus/src/time/utc_timeline_ordering_applies_to_fixed_instants_carrier.rs"
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
