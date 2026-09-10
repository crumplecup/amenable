//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use amenable_core::Witness;
use amenable_time::{
    CalendarMonthInRangeOneToTwelve, HourInRangeZeroToTwentyFour, MinuteInRangeZeroToFiftyNine,
    SecondInRangeZeroToSixty, UtcOffsetHourInRangeZeroToTwentyThree,
    UtcOffsetMinuteInRangeZeroToFiftyNine,
};

use crate::rust_std::kani_ensures;
use crate::{CalculationProof, KaniVerifier};

// ── CalendarMonthInRangeOneToTwelve ──────────────────────────────────
//
// ISO 8601-1:2019, 3.1.1.2 — a calendar month is one of twelve named
// intervals within a calendar year. The `bool` predicate is the range
// check `1..=12`; the harness proves it agrees, over the entire `u8`
// domain, with the independently-written twelve-way enumeration of the
// legal set (so a wrong bound or off-by-one cannot pass).

impl Witness<KaniVerifier> for CalendarMonthInRangeOneToTwelve {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_month_in_range".to_owned(),
            VERIFY_CALENDAR_MONTH_IN_RANGE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CalendarMonthInRangeOneToTwelve",
        "kani",
        || <CalendarMonthInRangeOneToTwelve as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarMonthInRangeOneToTwelve,
    "amenable_time::CalendarMonthInRangeOneToTwelve::ensures",
    u8,
    |month| (1..=12).contains(&month)
);

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC, {
        /// The `1..=12` month-range predicate agrees, over the whole `u8`
        /// domain, with the twelve-way enumeration of the legal calendar
        /// months.
        #[kani::proof]
        fn verify_calendar_month_in_range() {
            let month: u8 = kani::any();

            let arithmetic = <CalendarMonthInRangeOneToTwelve as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(month);

            let enumerated = month == 1
                || month == 2
                || month == 3
                || month == 4
                || month == 5
                || month == 6
                || month == 7
                || month == 8
                || month == 9
                || month == 10
                || month == 11
                || month == 12;

            assert_eq!(arithmetic, enumerated);
        }
    }
}

// ── HourInRangeZeroToTwentyFour ──────────────────────────────────

impl Witness<KaniVerifier> for HourInRangeZeroToTwentyFour {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::HourInRangeZeroToTwentyFour::ensures",
    u8,
    |hour| hour <= 24
);

amenable_derive::harness! {
    kani, VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 / 5.3.2 — an hour is 00 through 24 (24 reserved for end-of-day). The `0..=24` predicate (`hour <= 24`) agrees, over the
        /// whole `u8` domain, with the independently-written `hour < 25`.
        #[kani::proof]
        fn verify_hour_in_range_zero_to_twenty_four() {
            let hour: u8 = kani::any();

            let inclusive =
                <HourInRangeZeroToTwentyFour as ::amenable_core::Ensures<KaniVerifier>>::ensures(hour);
            let strict_below_next = hour < 25;

            assert_eq!(inclusive, strict_below_next);
        }
    }
}

// ── MinuteInRangeZeroToFiftyNine ──────────────────────────────────

impl Witness<KaniVerifier> for MinuteInRangeZeroToFiftyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::MinuteInRangeZeroToFiftyNine::ensures",
    u8,
    |minute| minute <= 59
);

amenable_derive::harness! {
    kani, VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.1 — a minute is 00 through 59. The `0..=59` predicate (`minute <= 59`) agrees, over the
        /// whole `u8` domain, with the independently-written `minute < 60`.
        #[kani::proof]
        fn verify_minute_in_range_zero_to_fifty_nine() {
            let minute: u8 = kani::any();

            let inclusive =
                <MinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(minute);
            let strict_below_next = minute < 60;

            assert_eq!(inclusive, strict_below_next);
        }
    }
}

// ── SecondInRangeZeroToSixty ──────────────────────────────────

impl Witness<KaniVerifier> for SecondInRangeZeroToSixty {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::SecondInRangeZeroToSixty::ensures",
    u8,
    |second| second <= 60
);

amenable_derive::harness! {
    kani, VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.1 — a second is 00 through 60 (60 admits a leap second). The `0..=60` predicate (`second <= 60`) agrees, over the
        /// whole `u8` domain, with the independently-written `second < 61`.
        #[kani::proof]
        fn verify_second_in_range_zero_to_sixty() {
            let second: u8 = kani::any();

            let inclusive =
                <SecondInRangeZeroToSixty as ::amenable_core::Ensures<KaniVerifier>>::ensures(second);
            let strict_below_next = second < 61;

            assert_eq!(inclusive, strict_below_next);
        }
    }
}

// ── UtcOffsetHourInRangeZeroToTwentyThree ──────────────────────────────────

impl Witness<KaniVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::UtcOffsetHourInRangeZeroToTwentyThree::ensures",
    u8,
    |hour| hour <= 23
);

amenable_derive::harness! {
    kani, VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset hour is 00 through 23. The `0..=23` predicate (`hour <= 23`) agrees, over the
        /// whole `u8` domain, with the independently-written `hour < 24`.
        #[kani::proof]
        fn verify_utc_offset_hour_in_range_zero_to_twenty_three() {
            let hour: u8 = kani::any();

            let inclusive =
                <UtcOffsetHourInRangeZeroToTwentyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(hour);
            let strict_below_next = hour < 24;

            assert_eq!(inclusive, strict_below_next);
        }
    }
}

// ── UtcOffsetMinuteInRangeZeroToFiftyNine ──────────────────────────────────

impl Witness<KaniVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine::ensures",
    u8,
    |minute| minute <= 59
);

amenable_derive::harness! {
    kani, VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset minute is 00 through 59. The `0..=59` predicate (`minute <= 59`) agrees, over the
        /// whole `u8` domain, with the independently-written `minute < 60`.
        #[kani::proof]
        fn verify_utc_offset_minute_in_range_zero_to_fifty_nine() {
            let minute: u8 = kani::any();

            let inclusive =
                <UtcOffsetMinuteInRangeZeroToFiftyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(minute);
            let strict_below_next = minute < 60;

            assert_eq!(inclusive, strict_below_next);
        }
    }
}
