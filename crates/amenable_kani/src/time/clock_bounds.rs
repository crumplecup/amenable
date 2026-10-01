//! Hour/minute/second and UTC-offset-hour/minute range contracts.
//!
//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use crate::rust_std::kani_ensures;
use crate::{CalculationProof, KaniVerifier};
use amenable_core::Witness;
use amenable_time::{
    HourInRangeZeroToTwentyFour, MinuteInRangeZeroToFiftyNine, SecondInRangeZeroToSixty,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
};

// ── HourInRangeZeroToTwentyFour ──────────────────────────────────

impl Witness<KaniVerifier> for HourInRangeZeroToTwentyFour {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_hour_in_range_zero_to_twenty_four".to_owned(),
            VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::HourInRangeZeroToTwentyFour",
        "kani",
        || <HourInRangeZeroToTwentyFour as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    HourInRangeZeroToTwentyFour,
    "amenable_time::HourInRangeZeroToTwentyFour",
    u8,
    |hour| hour <= 24
);

amenable_derive::harness! {
    kani, VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 / 5.3.2 — an hour is 00
        /// through 24 (24 reserved for end-of-day). The `0..=24`
        /// predicate holds at both boundaries and fails just past the
        /// upper one.
        #[kani::proof]
        fn verify_hour_in_range_zero_to_twenty_four() {
            assert!(<HourInRangeZeroToTwentyFour as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<HourInRangeZeroToTwentyFour as ::amenable_core::Ensures<KaniVerifier>>::ensures(24));
            assert!(!<HourInRangeZeroToTwentyFour as ::amenable_core::Ensures<KaniVerifier>>::ensures(25));
        }
    }
}
// ── MinuteInRangeZeroToFiftyNine ──────────────────────────────────

impl Witness<KaniVerifier> for MinuteInRangeZeroToFiftyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_minute_in_range_zero_to_fifty_nine".to_owned(),
            VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::MinuteInRangeZeroToFiftyNine",
        "kani",
        || <MinuteInRangeZeroToFiftyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MinuteInRangeZeroToFiftyNine,
    "amenable_time::MinuteInRangeZeroToFiftyNine",
    u8,
    |minute| minute <= 59
);

amenable_derive::harness! {
    kani, VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.1 — a minute is 00 through 59. The
        /// `0..=59` predicate holds at both boundaries and fails just
        /// past the upper one.
        #[kani::proof]
        fn verify_minute_in_range_zero_to_fifty_nine() {
            assert!(<MinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<MinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(59));
            assert!(!<MinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(60));
        }
    }
}
// ── SecondInRangeZeroToSixty ──────────────────────────────────

impl Witness<KaniVerifier> for SecondInRangeZeroToSixty {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_second_in_range_zero_to_sixty".to_owned(),
            VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::SecondInRangeZeroToSixty",
        "kani",
        || <SecondInRangeZeroToSixty as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    SecondInRangeZeroToSixty,
    "amenable_time::SecondInRangeZeroToSixty",
    u8,
    |second| second <= 60
);

amenable_derive::harness! {
    kani, VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.1 — a second is 00 through 60 (60
        /// admits a leap second). The `0..=60` predicate holds at both
        /// boundaries and fails just past the upper one.
        #[kani::proof]
        fn verify_second_in_range_zero_to_sixty() {
            assert!(<SecondInRangeZeroToSixty as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<SecondInRangeZeroToSixty as ::amenable_core::Ensures<KaniVerifier>>::ensures(60));
            assert!(!<SecondInRangeZeroToSixty as ::amenable_core::Ensures<KaniVerifier>>::ensures(61));
        }
    }
}
// ── UtcOffsetHourInRangeZeroToTwentyThree ──────────────────────────────────

impl Witness<KaniVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_utc_offset_hour_in_range_zero_to_twenty_three".to_owned(),
            VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::UtcOffsetHourInRangeZeroToTwentyThree",
        "kani",
        || <UtcOffsetHourInRangeZeroToTwentyThree as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    UtcOffsetHourInRangeZeroToTwentyThree,
    "amenable_time::UtcOffsetHourInRangeZeroToTwentyThree",
    u8,
    |hour| hour <= 23
);

amenable_derive::harness! {
    kani, VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset hour is 00
        /// through 23. The `0..=23` predicate holds at both boundaries
        /// and fails just past the upper one.
        #[kani::proof]
        fn verify_utc_offset_hour_in_range_zero_to_twenty_three() {
            assert!(<UtcOffsetHourInRangeZeroToTwentyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<UtcOffsetHourInRangeZeroToTwentyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(23));
            assert!(!<UtcOffsetHourInRangeZeroToTwentyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(24));
        }
    }
}
// ── UtcOffsetMinuteInRangeZeroToFiftyNine ──────────────────────────────────

impl Witness<KaniVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_utc_offset_minute_in_range_zero_to_fifty_nine".to_owned(),
            VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine",
        "kani",
        || <UtcOffsetMinuteInRangeZeroToFiftyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    UtcOffsetMinuteInRangeZeroToFiftyNine,
    "amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine",
    u8,
    |minute| minute <= 59
);

amenable_derive::harness! {
    kani, VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset minute is 00
        /// through 59. The `0..=59` predicate holds at both boundaries
        /// and fails just past the upper one.
        #[kani::proof]
        fn verify_utc_offset_minute_in_range_zero_to_fifty_nine() {
            assert!(<UtcOffsetMinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<UtcOffsetMinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(59));
            assert!(!<UtcOffsetMinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(60));
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for HourInRangeZeroToTwentyFour {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for MinuteInRangeZeroToFiftyNine {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for SecondInRangeZeroToSixty {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {}
