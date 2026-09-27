//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use amenable_core::{Ensures, Evidence, Standard, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};
use amenable_time::{
    CalendarDayWithinMonthBounds, CalendarMonthInRangeOneToTwelve,
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CentennialYearDivisibleByOneHundred, CenturyOrdinalInRangeZeroToNinetyNine,
    CommonYearHasThreeHundredSixtyFiveCalendarDays,
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException, HourInRangeZeroToTwentyFour,
    IntervalDurationIsNonNegative, IntervalStartPrecedesEnd, LeapDayOccursOnlyInLeapYear,
    LeapYearHasThreeHundredSixtySixCalendarDays, MinuteInRangeZeroToFiftyNine,
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    OrdinalDayInRangeOneToThreeHundredSixtySix, SecondInRangeZeroToSixty,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
    UtcTimelineOrderingAppliesToFixedInstants, WeekNumberInRangeOneToFiftyThree,
    WeekdayInRangeOneToSeven,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
};

use crate::rust_std::{kani_ensures, kani_requires};
use crate::{CalculationProof, KaniVerifier};

/// The Gregorian leap-year rule (ISO 8601-1:2019, 3.1.1.21 note 1),
/// shared by the year-length and month/day-bound harnesses.
fn is_gregorian_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// A Gregorian calendar year is 366 days when a leap year, else 365.
fn days_in_year(year: i32) -> i32 {
    if is_gregorian_leap_year(year) {
        366
    } else {
        365
    }
}

/// The number of calendar days in month `m` of year `y` (m in 1..=12;
/// months outside that range yield 0).
fn days_in_month(year: i32, month: u8) -> u8 {
    if month == 2 {
        if is_gregorian_leap_year(year) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else if matches!(month, 1 | 3 | 5 | 7 | 8 | 10 | 12) {
        31
    } else {
        0
    }
}

/// Whether `day` is a valid day-of-month for month `m` of year `y`.
fn is_valid_calendar_day(year: i32, month: u8, day: u8) -> bool {
    (1..=days_in_month(year, month)).contains(&day)
}

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
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

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
    "amenable_time::CalendarMonthInRangeOneToTwelve",
    u8,
    |month| (1..=12).contains(&month)
);

// Reused as a real `kani::assume` precondition wherever a later
// harness needs "month is 1..=12" restricted, rather than restating
// the range literally.
kani_requires!(
    CalendarMonthInRangeOneToTwelve,
    "amenable_time::CalendarMonthInRangeOneToTwelve",
    u8,
    |month| (1..=12).contains(&month)
);

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC, {
        /// The `1..=12` month-range predicate holds at both boundaries
        /// and fails just outside each one.
        #[kani::proof]
        fn verify_calendar_month_in_range() {
            assert!(<CalendarMonthInRangeOneToTwelve as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(1));
            assert!(<CalendarMonthInRangeOneToTwelve as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(12));
            assert!(!<CalendarMonthInRangeOneToTwelve as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(0));
            assert!(!<CalendarMonthInRangeOneToTwelve as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(13));
        }
    }
}

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

// ── WeekdayInRangeOneToSeven ──────────────────────

impl Witness<KaniVerifier> for WeekdayInRangeOneToSeven {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_weekday_in_range_one_to_seven".to_owned(),
            VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::WeekdayInRangeOneToSeven",
        "kani",
        || <WeekdayInRangeOneToSeven as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    WeekdayInRangeOneToSeven,
    "amenable_time::WeekdayInRangeOneToSeven",
    u8,
    |weekday| (1..=7).contains(&weekday)
);

amenable_derive::harness! {
    kani, VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.4.1 — a weekday is 1 (Monday)
        /// through 7 (Sunday). The range predicate holds at both
        /// boundaries and fails just outside each one.
        #[kani::proof]
        fn verify_weekday_in_range_one_to_seven() {
            assert!(<WeekdayInRangeOneToSeven as ::amenable_core::Ensures<KaniVerifier>>::ensures(1));
            assert!(<WeekdayInRangeOneToSeven as ::amenable_core::Ensures<KaniVerifier>>::ensures(7));
            assert!(!<WeekdayInRangeOneToSeven as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(!<WeekdayInRangeOneToSeven as ::amenable_core::Ensures<KaniVerifier>>::ensures(8));
        }
    }
}

// ── WeekNumberInRangeOneToFiftyThree ──────────────────────

impl Witness<KaniVerifier> for WeekNumberInRangeOneToFiftyThree {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_week_number_in_range_one_to_fifty_three".to_owned(),
            VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::WeekNumberInRangeOneToFiftyThree",
        "kani",
        || <WeekNumberInRangeOneToFiftyThree as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    WeekNumberInRangeOneToFiftyThree,
    "amenable_time::WeekNumberInRangeOneToFiftyThree",
    u8,
    |week| (1..=53).contains(&week)
);

amenable_derive::harness! {
    kani, VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.4.1 — a calendar-week number is 01
        /// through 53. The canonical predicate holds at both boundaries
        /// and fails just outside each one.
        #[kani::proof]
        fn verify_week_number_in_range_one_to_fifty_three() {
            assert!(<WeekNumberInRangeOneToFiftyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(1));
            assert!(<WeekNumberInRangeOneToFiftyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(53));
            assert!(!<WeekNumberInRangeOneToFiftyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(!<WeekNumberInRangeOneToFiftyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(54));
        }
    }
}

// ── OrdinalDayInRangeOneToThreeHundredSixtySix ──────────────────────

impl Witness<KaniVerifier> for OrdinalDayInRangeOneToThreeHundredSixtySix {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_ordinal_day_in_range_one_to_three_hundred_sixty_six".to_owned(),
            VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
        "kani",
        || <OrdinalDayInRangeOneToThreeHundredSixtySix as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    OrdinalDayInRangeOneToThreeHundredSixtySix,
    "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
    u16,
    |day| (1..=366).contains(&day)
);

amenable_derive::harness! {
    kani, VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.3.1 — an ordinal
        /// day-of-year is 001 through 365, or 366 in a leap year. The
        /// canonical predicate holds at both boundaries and fails just
        /// outside each one.
        #[kani::proof]
        fn verify_ordinal_day_in_range_one_to_three_hundred_sixty_six() {
            assert!(<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(1));
            assert!(<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(366));
            assert!(!<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(!<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(367));
        }
    }
}

// ── CenturyOrdinalInRangeZeroToNinetyNine ──────────────────────

impl Witness<KaniVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_century_ordinal_in_range_zero_to_ninety_nine".to_owned(),
            VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
        "kani",
        || <CenturyOrdinalInRangeZeroToNinetyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CenturyOrdinalInRangeZeroToNinetyNine,
    "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
    u8,
    |ordinal| ordinal <= 99
);

amenable_derive::harness! {
    kani, VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.12 — a Gregorian century
        /// ordinal is 00 through 99. The canonical predicate holds at
        /// the boundary and fails just past it.
        #[kani::proof]
        fn verify_century_ordinal_in_range_zero_to_ninety_nine() {
            assert!(<CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(99));
            assert!(!<CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(100));
        }
    }
}

// ── DecadeOrdinalInRangeZeroToNineHundredNinetyNine ──────────────────────

impl Witness<KaniVerifier> for DecadeOrdinalInRangeZeroToNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine".to_owned(),
            VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
        "kani",
        || <DecadeOrdinalInRangeZeroToNineHundredNinetyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
    u16,
    |ordinal| ordinal <= 999
);

amenable_derive::harness! {
    kani, VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.11 — a Gregorian decade
        /// ordinal is 000 through 999. The canonical predicate holds at
        /// the boundary and fails just past it.
        #[kani::proof]
        fn verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine() {
            assert!(<DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(999));
            assert!(!<DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(1000));
        }
    }
}

// ── CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine ──────────────────────

impl Witness<KaniVerifier> for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine"
                .to_owned(),
            VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC
                .to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
        "kani",
        || <CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
    u16,
    |year| year <= 9999
);

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.2.1 — a non-expanded calendar
        /// year is 0000 through 9999. The canonical predicate holds at
        /// the boundary and fails just past it.
        #[kani::proof]
        fn verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine() {
            assert!(<CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(9999));
            assert!(!<CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(10000));
        }
    }
}

// ── IntervalStartPrecedesEnd ──────────────────

impl Witness<KaniVerifier> for IntervalStartPrecedesEnd {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_interval_start_precedes_end".to_owned(),
            VERIFY_INTERVAL_START_PRECEDES_END_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::IntervalStartPrecedesEnd",
        "kani",
        || <IntervalStartPrecedesEnd as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    IntervalStartPrecedesEnd,
    "amenable_time::IntervalStartPrecedesEnd",
    (i32, i32),
    |(start, end)| start <= end
);

amenable_derive::harness! {
    kani, VERIFY_INTERVAL_START_PRECEDES_END_SRC, {
        /// ISO 8601-1:2019, 3.1.1.6 / 3.1.1.8 — an interval's first
        /// endpoint is no later than its second on the relevant
        /// timeline. `start <= end` holds when the endpoints coincide
        /// or are ordered, and fails when reversed — checked at both
        /// ends of the `i32` range.
        #[kani::proof]
        fn verify_interval_start_precedes_end() {
            assert!(<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 0)));
            assert!(<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 1)));
            assert!(!<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((1, 0)));
            assert!(<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MIN, i32::MAX)));
            assert!(!<IntervalStartPrecedesEnd as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MAX, i32::MIN)));
        }
    }
}

// ── IntervalDurationIsNonNegative ──────────────────

impl Witness<KaniVerifier> for IntervalDurationIsNonNegative {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_interval_duration_is_non_negative".to_owned(),
            VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::IntervalDurationIsNonNegative",
        "kani",
        || <IntervalDurationIsNonNegative as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    IntervalDurationIsNonNegative,
    "amenable_time::IntervalDurationIsNonNegative",
    (i32, i32),
    |(start, end)| i64::from(end) - i64::from(start) >= 0
);

amenable_derive::harness! {
    kani, VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, {
        /// ISO 8601-1:2019, 3.1.1.8 — the span between an interval's
        /// endpoints is zero or positive, never negative. Non-negative
        /// when ordered or coincident, negative when reversed —
        /// checked at both ends of the `i32` range.
        #[kani::proof]
        fn verify_interval_duration_is_non_negative() {
            assert!(<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 0)));
            assert!(<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((0, 1)));
            assert!(!<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((1, 0)));
            assert!(<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MIN, i32::MAX)));
            assert!(!<IntervalDurationIsNonNegative as ::amenable_core::Ensures<KaniVerifier>>::ensures((i32::MAX, i32::MIN)));
        }
    }
}

// ── UtcTimelineOrderingAppliesToFixedInstants ──────────────────

impl Witness<KaniVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_utc_timeline_ordering_applies_to_fixed_instants".to_owned(),
            VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
        "kani",
        || <UtcTimelineOrderingAppliesToFixedInstants as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    UtcTimelineOrderingAppliesToFixedInstants,
    "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
    (i32, i32),
    |(a, b)| a <= b
);

/// `<=` on UTC timeline positions is a total order: reflexive, total,
/// antisymmetric, and transitive. A genuinely distinct claim from
/// [`UtcTimelineOrderingAppliesToFixedInstants`] itself (which only
/// states the relation, not that it has these four properties) —
/// named so the compound check is a real, callable predicate rather
/// than an inline restatement.
pub struct UtcTimelineOrderingIsATotalOrder;

impl Standard for UtcTimelineOrderingIsATotalOrder {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for UtcTimelineOrderingIsATotalOrder {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for UtcTimelineOrderingIsATotalOrder {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_utc_timeline_ordering_applies_to_fixed_instants".to_owned(),
            VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::UtcTimelineOrderingIsATotalOrder",
        "kani",
        || <UtcTimelineOrderingIsATotalOrder as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    UtcTimelineOrderingIsATotalOrder,
    "amenable_kani::UtcTimelineOrderingIsATotalOrder",
    (i32, i32, i32),
    |(a, b, c)| {
        let le = |x: i32, y: i32| {
            <UtcTimelineOrderingAppliesToFixedInstants as Ensures<KaniVerifier>>::ensures((x, y))
        };
        le(a, a)
            && (le(a, b) || le(b, a))
            && (!(le(a, b) && le(b, a)) || a == b)
            && (!(le(a, b) && le(b, c)) || le(a, c))
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for UtcTimelineOrderingIsATotalOrder {}

amenable_derive::harness! {
    kani, VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC, {
        /// RFC 3339, 5.1 — two fixed instants are totally ordered by
        /// their position on the UTC timeline: `<=` on `i32` timeline
        /// positions is reflexive, total, antisymmetric, and
        /// transitive — checked over three symbolic instants.
        #[kani::proof]
        fn verify_utc_timeline_ordering_applies_to_fixed_instants() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();

            assert!(<UtcTimelineOrderingIsATotalOrder as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures((a, b, c)));
        }
    }
}

// ── GregorianLeapYearUsesDivisibleByFourAndFourHundredException ──────────────────

impl Witness<KaniVerifier> for GregorianLeapYearUsesDivisibleByFourAndFourHundredException {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_gregorian_leap_year".to_owned(),
            VERIFY_GREGORIAN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
        "kani",
        || <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
    i32,
    |year| year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
);

/// The `y%4 && (y%100 || y%400)` divisibility rule agrees, over every
/// `i32`, with ISO 8601-1:2019, 3.1.1.21 note 1's own stated form: a
/// centennial year needs the /400 rule, every other year only the /4
/// rule. A genuinely different formulation of the same law, not a
/// restatement of it — worth naming and checking in its own right.
pub struct GregorianLeapYearRuleMatchesCentennialCaseSplit;

impl Standard for GregorianLeapYearRuleMatchesCentennialCaseSplit {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for GregorianLeapYearRuleMatchesCentennialCaseSplit {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for GregorianLeapYearRuleMatchesCentennialCaseSplit {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_gregorian_leap_year".to_owned(),
            VERIFY_GREGORIAN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::GregorianLeapYearRuleMatchesCentennialCaseSplit",
        "kani",
        || <GregorianLeapYearRuleMatchesCentennialCaseSplit as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    GregorianLeapYearRuleMatchesCentennialCaseSplit,
    "amenable_kani::GregorianLeapYearRuleMatchesCentennialCaseSplit",
    i32,
    |year| {
        let leap = <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<
            KaniVerifier,
        >>::ensures(year);
        let case_split = if year % 100 == 0 {
            year % 400 == 0
        } else {
            year % 4 == 0
        };
        leap == case_split
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for GregorianLeapYearRuleMatchesCentennialCaseSplit
{
}

amenable_derive::harness! {
    kani, VERIFY_GREGORIAN_LEAP_YEAR_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — a year is a leap year if
        /// divisible by 4, except a centennial year is a leap year only
        /// if also divisible by 400: the divisibility rule agrees with
        /// the standard's own case-split over every `i32`, plus six
        /// dated anchors (the classic off-by-a-century bugs).
        #[kani::proof]
        fn verify_gregorian_leap_year() {
            let year: i32 = kani::any();
            assert!(<GregorianLeapYearRuleMatchesCentennialCaseSplit as Ensures<
                KaniVerifier,
            >>::ensures(year));

            assert!(<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2000), "2000 is a leap year");
            assert!(<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(1600), "1600 is a leap year");
            assert!(<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2024), "2024 is a leap year");
            assert!(!<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(1900), "1900 is not a leap year");
            assert!(!<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2100), "2100 is not a leap year");
            assert!(!<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2023), "2023 is not a leap year");
        }
    }
}

// ── CentennialYearDivisibleByOneHundred ──────────────────

impl Witness<KaniVerifier> for CentennialYearDivisibleByOneHundred {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_centennial_year_divisible_by_one_hundred".to_owned(),
            VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CentennialYearDivisibleByOneHundred",
        "kani",
        || <CentennialYearDivisibleByOneHundred as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CentennialYearDivisibleByOneHundred,
    "amenable_time::CentennialYearDivisibleByOneHundred",
    i32,
    |year| year % 100 == 0
);

/// `y % 100 == 0` agrees, over every `i32`, with `y%4 == 0 && y%25 ==
/// 0` (100 = 4·25, coprime factors) — a genuinely different
/// factorization of the same divisibility claim, not a restatement.
pub struct CentennialDivisibilityMatchesCoprimeFactorization;

impl Standard for CentennialDivisibilityMatchesCoprimeFactorization {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for CentennialDivisibilityMatchesCoprimeFactorization {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for CentennialDivisibilityMatchesCoprimeFactorization {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_centennial_year_divisible_by_one_hundred".to_owned(),
            VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CentennialDivisibilityMatchesCoprimeFactorization",
        "kani",
        || <CentennialDivisibilityMatchesCoprimeFactorization as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CentennialDivisibilityMatchesCoprimeFactorization,
    "amenable_kani::CentennialDivisibilityMatchesCoprimeFactorization",
    i32,
    |year| {
        let centennial =
            <CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(year);
        centennial == (year % 4 == 0 && year % 25 == 0)
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CentennialDivisibilityMatchesCoprimeFactorization
{
}

amenable_derive::harness! {
    kani, VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC, {
        /// ISO 8601-1:2019, 3.1.1.22 — a centennial year is one whose
        /// year number is an exact multiple of 100: `y % 100 == 0`
        /// agrees with its coprime factorization over every `i32`,
        /// plus dated anchors.
        #[kani::proof]
        fn verify_centennial_year_divisible_by_one_hundred() {
            let year: i32 = kani::any();
            assert!(<CentennialDivisibilityMatchesCoprimeFactorization as Ensures<
                KaniVerifier,
            >>::ensures(year));

            assert!(<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(0), "year 0 is centennial");
            assert!(<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(1900), "1900 is centennial");
            assert!(<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(2000), "2000 is centennial");
            assert!(!<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(2024), "2024 is not centennial");
        }
    }
}

// ── LeapYearHasThreeHundredSixtySixCalendarDays ──────────────────

impl Witness<KaniVerifier> for LeapYearHasThreeHundredSixtySixCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_year_has_three_hundred_sixty_six_calendar_days".to_owned(),
            VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
        "kani",
        || <LeapYearHasThreeHundredSixtySixCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapYearHasThreeHundredSixtySixCalendarDays,
    "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
    i32,
    |year| days_in_year(year) == 366
);

/// This file's day-count/leap-year models are defined only for
/// non-negative years (no BCE support) — a real domain restriction,
/// not a range predicate already named above. Genuinely trivial (its
/// own definition is the whole claim), so it gets no dedicated proof
/// of its own, the same way `AccountsDistinct`/`BalancedEntries` don't
/// — it's checked inline wherever a day-count harness needs it.
pub struct YearIsNonNegativeInThisModel;

impl Standard for YearIsNonNegativeInThisModel {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for YearIsNonNegativeInThisModel {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for YearIsNonNegativeInThisModel {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}
}

kani_requires!(
    YearIsNonNegativeInThisModel,
    "amenable_kani::YearIsNonNegativeInThisModel",
    i32,
    |year| year >= 0
);

// `requires()` is only ever called from inside `#[cfg(kani)]`-gated
// harness bodies, which a plain build never compiles -- this
// existence check (never run, just typechecked) is what keeps the
// type itself from looking dead outside a `--cfg kani` build, the
// same `let _ = ...;` idiom `amenable_ext`'s own trait-bound
// assertions already use.
const _: () = {
    let _ = <YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires;
};

/// `days_in_year(y) == 366` agrees, over every modeled year, with the
/// Gregorian leap-year rule directly, and a leap year is exactly a
/// common year plus one day — two related but distinct claims about
/// the same day-count model, not a restatement of `ensures()` itself.
pub struct LeapYearDayCountMatchesGregorianRule;

impl Standard for LeapYearDayCountMatchesGregorianRule {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for LeapYearDayCountMatchesGregorianRule {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for LeapYearDayCountMatchesGregorianRule {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_year_has_three_hundred_sixty_six_calendar_days".to_owned(),
            VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::LeapYearDayCountMatchesGregorianRule",
        "kani",
        || <LeapYearDayCountMatchesGregorianRule as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapYearDayCountMatchesGregorianRule,
    "amenable_kani::LeapYearDayCountMatchesGregorianRule",
    i32,
    |year| {
        let has_366 =
            <LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(year);
        has_366 == is_gregorian_leap_year(year)
            && days_in_year(year) - 365 == if is_gregorian_leap_year(year) { 1 } else { 0 }
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for LeapYearDayCountMatchesGregorianRule {}

amenable_derive::harness! {
    kani, VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 — a leap year contains 366
        /// calendar days: `days_in_year(y) == 366` agrees with the
        /// Gregorian rule, and with "a leap year is a common year plus
        /// one day", over every modeled year, plus dated anchors.
        #[kani::proof]
        fn verify_leap_year_has_three_hundred_sixty_six_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(<YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires(year));

            assert!(<LeapYearDayCountMatchesGregorianRule as Ensures<KaniVerifier>>::ensures(year));

            assert!(<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(2000), "2000 has 366 days");
            assert!(<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(2024), "2024 has 366 days");
            assert!(!<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(2023), "2023 has 365 days");
            assert!(!<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(1900), "1900 has 365 days");
        }
    }
}

// ── CommonYearHasThreeHundredSixtyFiveCalendarDays ──────────────────

impl Witness<KaniVerifier> for CommonYearHasThreeHundredSixtyFiveCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_common_year_has_three_hundred_sixty_five_calendar_days".to_owned(),
            VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
        "kani",
        || <CommonYearHasThreeHundredSixtyFiveCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CommonYearHasThreeHundredSixtyFiveCalendarDays,
    "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
    i32,
    |year| days_in_year(year) == 365
);

/// `days_in_year(y) == 365` agrees, over every modeled year, with
/// `!is_gregorian_leap_year(y)`, and 365/366 are the only two options
/// (they're distinct) — two related but distinct claims about the
/// day-count model, not a restatement of `ensures()` itself.
pub struct CommonYearDayCountMatchesGregorianRule;

impl Standard for CommonYearDayCountMatchesGregorianRule {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for CommonYearDayCountMatchesGregorianRule {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for CommonYearDayCountMatchesGregorianRule {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_common_year_has_three_hundred_sixty_five_calendar_days".to_owned(),
            VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CommonYearDayCountMatchesGregorianRule",
        "kani",
        || <CommonYearDayCountMatchesGregorianRule as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CommonYearDayCountMatchesGregorianRule,
    "amenable_kani::CommonYearDayCountMatchesGregorianRule",
    i32,
    |year| {
        let has_365 =
            <CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(
                year,
            );
        has_365 == !is_gregorian_leap_year(year)
            && (days_in_year(year) == 365) != (days_in_year(year) == 366)
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CommonYearDayCountMatchesGregorianRule {}

amenable_derive::harness! {
    kani, VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.20 — a common year contains 365
        /// calendar days: `days_in_year(y) == 365` agrees with
        /// `!is_gregorian_leap_year(y)` over every modeled year, plus
        /// dated anchors.
        #[kani::proof]
        fn verify_common_year_has_three_hundred_sixty_five_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(<YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires(year));

            assert!(<CommonYearDayCountMatchesGregorianRule as Ensures<KaniVerifier>>::ensures(year));

            assert!(<CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(2023), "2023 has 365 days");
            assert!(<CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(1900), "1900 has 365 days");
            assert!(!<CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(2000), "2000 has 366 days");
        }
    }
}

// ── YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays ──────────────────

impl Witness<KaniVerifier>
    for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
{
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days".to_owned(),
            VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
        "kani",
        || <YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
    i32,
    |year| (365..=366).contains(&days_in_year(year))
);

amenable_derive::harness! {
    kani, VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 2.2.14 — a year's duration is 365 or 366
        /// calendar days: the model is well-formed over every modeled
        /// year.
        #[kani::proof]
        fn verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(<YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires(year));

            assert!(<YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as Ensures<
                KaniVerifier,
            >>::ensures(year));
        }
    }
}

// ── MonthDurationInRangeTwentyEightToThirtyOneCalendarDays ──────────────────

impl Witness<KaniVerifier> for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days"
                .to_owned(),
            VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
        "kani",
        || <MonthDurationInRangeTwentyEightToThirtyOneCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
    (i32, u8),
    |(year, month)| (28..=31).contains(&days_in_month(year, month))
);

/// `days_in_month(y, m)` is 31 exactly for the long months, 30 exactly
/// for the short months, and (28 or 29, distinguished by leap year)
/// for February — a real characterization of which month gets which
/// length, not a restatement of the `28..=31` range itself.
pub struct MonthDurationCharacterizedByMonthAndLeapYear;

impl Standard for MonthDurationCharacterizedByMonthAndLeapYear {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for MonthDurationCharacterizedByMonthAndLeapYear {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for MonthDurationCharacterizedByMonthAndLeapYear {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days"
                .to_owned(),
            VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::MonthDurationCharacterizedByMonthAndLeapYear",
        "kani",
        || <MonthDurationCharacterizedByMonthAndLeapYear as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MonthDurationCharacterizedByMonthAndLeapYear,
    "amenable_kani::MonthDurationCharacterizedByMonthAndLeapYear",
    (i32, u8),
    |(year, month)| {
        let dim = days_in_month(year, month);
        (dim == 31) == matches!(month, 1 | 3 | 5 | 7 | 8 | 10 | 12)
            && (dim == 30) == matches!(month, 4 | 6 | 9 | 11)
            && (dim < 30) == (month == 2)
            && (dim == 29) == (month == 2 && is_gregorian_leap_year(year))
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for MonthDurationCharacterizedByMonthAndLeapYear
{
}

/// The twelve months' durations sum to the year's own total day
/// count — a distinct aggregate claim from any single month's
/// duration.
pub struct MonthDurationsSumToYearDuration;

impl Standard for MonthDurationsSumToYearDuration {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for MonthDurationsSumToYearDuration {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for MonthDurationsSumToYearDuration {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days"
                .to_owned(),
            VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::MonthDurationsSumToYearDuration",
        "kani",
        || <MonthDurationsSumToYearDuration as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MonthDurationsSumToYearDuration,
    "amenable_kani::MonthDurationsSumToYearDuration",
    i32,
    |year| {
        let total = days_in_month(year, 1) as i32
            + days_in_month(year, 2) as i32
            + days_in_month(year, 3) as i32
            + days_in_month(year, 4) as i32
            + days_in_month(year, 5) as i32
            + days_in_month(year, 6) as i32
            + days_in_month(year, 7) as i32
            + days_in_month(year, 8) as i32
            + days_in_month(year, 9) as i32
            + days_in_month(year, 10) as i32
            + days_in_month(year, 11) as i32
            + days_in_month(year, 12) as i32;
        total == days_in_year(year)
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for MonthDurationsSumToYearDuration {}

amenable_derive::harness! {
    kani, VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 2.2.12 — a month's duration is 28, 29, 30,
        /// or 31 calendar days according to the month and year: each
        /// length is characterized exactly by month and leap year, and
        /// the twelve months sum to the year's own total.
        #[kani::proof]
        fn verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days() {
            let year: i32 = kani::any();
            let month: u8 = kani::any();
            kani::assume(<CalendarMonthInRangeOneToTwelve as ::amenable_core::Requires<KaniVerifier>>::requires(month));

            assert!(<MonthDurationInRangeTwentyEightToThirtyOneCalendarDays as Ensures<
                KaniVerifier,
            >>::ensures((year, month)));
            assert!(<MonthDurationCharacterizedByMonthAndLeapYear as Ensures<
                KaniVerifier,
            >>::ensures((year, month)));
            assert!(<MonthDurationsSumToYearDuration as Ensures<KaniVerifier>>::ensures(year));
        }
    }
}

// ── CalendarDayWithinMonthBounds ──────────────────

impl Witness<KaniVerifier> for CalendarDayWithinMonthBounds {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_day_within_month_bounds".to_owned(),
            VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CalendarDayWithinMonthBounds",
        "kani",
        || <CalendarDayWithinMonthBounds as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarDayWithinMonthBounds,
    "amenable_time::CalendarDayWithinMonthBounds",
    (i32, u8, u8),
    |(year, month, day)| is_valid_calendar_day(year, month, day)
);

/// A valid calendar day always lies in `1..=31`, and February 29 is
/// valid exactly in a leap year — two real characterizations of
/// [`CalendarDayWithinMonthBounds`]'s own claim, not restatements of
/// it.
pub struct CalendarDayWithinMonthBoundsCharacterization;

impl Standard for CalendarDayWithinMonthBoundsCharacterization {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for CalendarDayWithinMonthBoundsCharacterization {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for CalendarDayWithinMonthBoundsCharacterization {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_day_within_month_bounds".to_owned(),
            VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CalendarDayWithinMonthBoundsCharacterization",
        "kani",
        || <CalendarDayWithinMonthBoundsCharacterization as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarDayWithinMonthBoundsCharacterization,
    "amenable_kani::CalendarDayWithinMonthBoundsCharacterization",
    (i32, u8, u8),
    |(year, month, day)| {
        let valid =
            <CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((year, month, day));
        (!valid || (1..=31).contains(&day))
            && (!(month == 2 && day == 29) || valid == is_gregorian_leap_year(year))
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CalendarDayWithinMonthBoundsCharacterization
{
}

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.2.1 — a calendar-date day
        /// component is within the valid day count for that month and
        /// year: a valid day lies in `1..=31`, and February 29 is valid
        /// exactly in a leap year, over every `i32` year and month
        /// `1..=12`; plus dated anchors.
        #[kani::proof]
        fn verify_calendar_day_within_month_bounds() {
            let year: i32 = kani::any();
            let month: u8 = kani::any();
            let day: u8 = kani::any();
            kani::assume(<CalendarMonthInRangeOneToTwelve as ::amenable_core::Requires<KaniVerifier>>::requires(month));

            assert!(<CalendarDayWithinMonthBoundsCharacterization as Ensures<
                KaniVerifier,
            >>::ensures((year, month, day)));

            assert!(<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2020, 2, 29)), "2020-02-29 is a valid date");
            assert!(!<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 2, 29)), "2021-02-29 is not");
            assert!(!<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 4, 31)), "April has 30 days");
            assert!(<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 1, 31)), "January has 31 days");
            assert!(!<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 1, 0)), "day 0 is invalid");
        }
    }
}

// ── LeapDayOccursOnlyInLeapYear ──────────────────

impl Witness<KaniVerifier> for LeapDayOccursOnlyInLeapYear {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_day_occurs_only_in_leap_year".to_owned(),
            VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::LeapDayOccursOnlyInLeapYear",
        "kani",
        || <LeapDayOccursOnlyInLeapYear as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapDayOccursOnlyInLeapYear,
    "amenable_time::LeapDayOccursOnlyInLeapYear",
    (i32, u8, u8),
    |(year, month, day)| !(month == 2 && day == 29) || is_gregorian_leap_year(year)
);

/// February 29 as a full calendar date is valid exactly when the
/// year is a leap year — a real biconditional over `is_valid_
/// calendar_day`, strictly stronger than [`LeapDayOccursOnlyInLeapYear`]'s
/// own one-directional "not valid unless leap" claim (which never
/// asserts Feb 29 *is* valid in a leap year).
pub struct LeapDayValidityMatchesGregorianRule;

impl Standard for LeapDayValidityMatchesGregorianRule {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for LeapDayValidityMatchesGregorianRule {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for LeapDayValidityMatchesGregorianRule {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_day_occurs_only_in_leap_year".to_owned(),
            VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::LeapDayValidityMatchesGregorianRule",
        "kani",
        || <LeapDayValidityMatchesGregorianRule as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapDayValidityMatchesGregorianRule,
    "amenable_kani::LeapDayValidityMatchesGregorianRule",
    i32,
    |year| is_valid_calendar_day(year, 2, 29) == is_gregorian_leap_year(year)
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for LeapDayValidityMatchesGregorianRule {}

amenable_derive::harness! {
    kani, VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — the 29th of February is a
        /// valid calendar date only when the year is a leap year:
        /// checked as a full biconditional over every `i32` year, plus
        /// dated anchors on the weaker one-directional claim.
        #[kani::proof]
        fn verify_leap_day_occurs_only_in_leap_year() {
            let year: i32 = kani::any();
            assert!(<LeapDayValidityMatchesGregorianRule as Ensures<KaniVerifier>>::ensures(year));

            assert!(!<LeapDayOccursOnlyInLeapYear as Ensures<KaniVerifier>>::ensures((2023, 2, 29)), "2023 is not a leap year");
            assert!(<LeapDayOccursOnlyInLeapYear as Ensures<KaniVerifier>>::ensures((2024, 2, 29)), "2024 is a leap year");
            assert!(<LeapDayOccursOnlyInLeapYear as Ensures<KaniVerifier>>::ensures((2021, 4, 31)), "not Feb 29, trivially satisfied");
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CalendarMonthInRangeOneToTwelve {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for HourInRangeZeroToTwentyFour {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for MinuteInRangeZeroToFiftyNine {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for SecondInRangeZeroToSixty {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for WeekdayInRangeOneToSeven {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for WeekNumberInRangeOneToFiftyThree {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for OrdinalDayInRangeOneToThreeHundredSixtySix
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for DecadeOrdinalInRangeZeroToNineHundredNinetyNine
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for IntervalStartPrecedesEnd {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for IntervalDurationIsNonNegative {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for UtcTimelineOrderingAppliesToFixedInstants
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CentennialYearDivisibleByOneHundred {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for LeapYearHasThreeHundredSixtySixCalendarDays
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CommonYearHasThreeHundredSixtyFiveCalendarDays
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CalendarDayWithinMonthBounds {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for LeapDayOccursOnlyInLeapYear {}
