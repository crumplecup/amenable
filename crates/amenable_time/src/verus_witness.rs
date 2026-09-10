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
    CalendarMonthInRangeOneToTwelve, HourInRangeZeroToTwentyFour, MinuteInRangeZeroToFiftyNine,
    SecondInRangeZeroToSixty, UtcOffsetHourInRangeZeroToTwentyThree,
    UtcOffsetMinuteInRangeZeroToFiftyNine,
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
