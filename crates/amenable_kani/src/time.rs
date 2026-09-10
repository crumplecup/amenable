//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use amenable_core::Witness;
use amenable_time::{
    CalendarMonthInRangeOneToTwelve, CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CentennialYearDivisibleByOneHundred, CenturyOrdinalInRangeZeroToNinetyNine,
    CommonYearHasThreeHundredSixtyFiveCalendarDays,
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException, HourInRangeZeroToTwentyFour,
    IntervalDurationIsNonNegative, IntervalStartPrecedesEnd,
    LeapYearHasThreeHundredSixtySixCalendarDays, MinuteInRangeZeroToFiftyNine,
    OrdinalDayInRangeOneToThreeHundredSixtySix, SecondInRangeZeroToSixty,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
    UtcTimelineOrderingAppliesToFixedInstants, WeekNumberInRangeOneToFiftyThree,
    WeekdayInRangeOneToSeven,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
};

use crate::rust_std::kani_ensures;
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

// ── WeekdayInRangeOneToSeven ──────────────────────

impl Witness<KaniVerifier> for WeekdayInRangeOneToSeven {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::WeekdayInRangeOneToSeven::ensures",
    u8,
    |weekday| (1..=7).contains(&weekday)
);

amenable_derive::harness! {
    kani, VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.4.1 — a weekday is 1 (Monday) through 7 (Sunday). The range predicate agrees, over the whole `u8`
        /// domain, with the seven-way enumeration of the ISO weekdays (Mon..Sun).
        #[kani::proof]
        fn verify_weekday_in_range_one_to_seven() {
            let weekday: u8 = kani::any();

            let predicate =
                <WeekdayInRangeOneToSeven as ::amenable_core::Ensures<KaniVerifier>>::ensures(weekday);
            let enumerated = weekday == 1 || weekday == 2 || weekday == 3 || weekday == 4 || weekday == 5 || weekday == 6 || weekday == 7;

            assert_eq!(predicate, enumerated);
        }
    }
}

// ── WeekNumberInRangeOneToFiftyThree ──────────────────────

impl Witness<KaniVerifier> for WeekNumberInRangeOneToFiftyThree {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::WeekNumberInRangeOneToFiftyThree::ensures",
    u8,
    |week| (1..=53).contains(&week)
);

amenable_derive::harness! {
    kani, VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.4.1 — a calendar-week number is 01 through 53. The canonical predicate agrees, over the whole `u8`
        /// domain, with the independently-written `week >= 1 && week < 54`.
        #[kani::proof]
        fn verify_week_number_in_range_one_to_fifty_three() {
            let week: u8 = kani::any();

            let predicate =
                <WeekNumberInRangeOneToFiftyThree as ::amenable_core::Ensures<KaniVerifier>>::ensures(week);
            let restated = week >= 1 && week < 54;

            assert_eq!(predicate, restated);
        }
    }
}

// ── OrdinalDayInRangeOneToThreeHundredSixtySix ──────────────────────

impl Witness<KaniVerifier> for OrdinalDayInRangeOneToThreeHundredSixtySix {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix::ensures",
    u16,
    |day| (1..=366).contains(&day)
);

amenable_derive::harness! {
    kani, VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.3.1 — an ordinal day-of-year is 001 through 365, or 366 in a leap year. The canonical predicate agrees, over the whole `u16`
        /// domain, with the independently-written `day >= 1 && day < 367`.
        #[kani::proof]
        fn verify_ordinal_day_in_range_one_to_three_hundred_sixty_six() {
            let day: u16 = kani::any();

            let predicate =
                <OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(day);
            let restated = day >= 1 && day < 367;

            assert_eq!(predicate, restated);
        }
    }
}

// ── CenturyOrdinalInRangeZeroToNinetyNine ──────────────────────

impl Witness<KaniVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine::ensures",
    u8,
    |ordinal| ordinal <= 99
);

amenable_derive::harness! {
    kani, VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.12 — a Gregorian century ordinal is 00 through 99. The canonical predicate agrees, over the whole `u8`
        /// domain, with the independently-written `ordinal < 100`.
        #[kani::proof]
        fn verify_century_ordinal_in_range_zero_to_ninety_nine() {
            let ordinal: u8 = kani::any();

            let predicate =
                <CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(ordinal);
            let restated = ordinal < 100;

            assert_eq!(predicate, restated);
        }
    }
}

// ── DecadeOrdinalInRangeZeroToNineHundredNinetyNine ──────────────────────

impl Witness<KaniVerifier> for DecadeOrdinalInRangeZeroToNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine::ensures",
    u16,
    |ordinal| ordinal <= 999
);

amenable_derive::harness! {
    kani, VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.11 — a Gregorian decade ordinal is 000 through 999. The canonical predicate agrees, over the whole `u16`
        /// domain, with the independently-written `ordinal < 1000`.
        #[kani::proof]
        fn verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine() {
            let ordinal: u16 = kani::any();

            let predicate =
                <DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(ordinal);
            let restated = ordinal < 1000;

            assert_eq!(predicate, restated);
        }
    }
}

// ── CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine ──────────────────────

impl Witness<KaniVerifier> for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine::ensures",
    u16,
    |year| year <= 9999
);

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.2.1 — a non-expanded calendar year is 0000 through 9999. The canonical predicate agrees, over the whole `u16`
        /// domain, with the independently-written `year < 10000`.
        #[kani::proof]
        fn verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine() {
            let year: u16 = kani::any();

            let predicate =
                <CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(year);
            let restated = year < 10000;

            assert_eq!(predicate, restated);
        }
    }
}

// ── IntervalStartPrecedesEnd ──────────────────

impl Witness<KaniVerifier> for IntervalStartPrecedesEnd {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::IntervalStartPrecedesEnd::ensures",
    (i32, i32),
    |(start, end)| start <= end
);

amenable_derive::harness! {
    kani, VERIFY_INTERVAL_START_PRECEDES_END_SRC, {
        /// ISO 8601-1:2019, 3.1.1.6 / 3.1.1.8 — an interval's first endpoint is no later than its second on the relevant timeline. The `start <= end` predicate agrees, over every `i32` pair, with the
        /// negation form `!(end < start)` and with a non-negative `i64` span.
        #[kani::proof]
        fn verify_interval_start_precedes_end() {
            let start: i32 = kani::any();
            let end: i32 = kani::any();

            let precedes = <IntervalStartPrecedesEnd as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures((start, end));

            // Same fact stated as a negation and as a non-negative span.
            assert_eq!(precedes, !(end < start));
            assert_eq!(precedes, i64::from(end) - i64::from(start) >= 0);
        }
    }
}

// ── IntervalDurationIsNonNegative ──────────────────

impl Witness<KaniVerifier> for IntervalDurationIsNonNegative {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::IntervalDurationIsNonNegative::ensures",
    (i32, i32),
    |(start, end)| i64::from(end) - i64::from(start) >= 0
);

amenable_derive::harness! {
    kani, VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, {
        /// ISO 8601-1:2019, 3.1.1.8 — the span between an interval's endpoints is zero or positive, never negative. The non-negative-span predicate agrees, over every `i32` pair, with
        /// `start <= end`; the span is zero exactly when the endpoints coincide.
        #[kani::proof]
        fn verify_interval_duration_is_non_negative() {
            let start: i32 = kani::any();
            let end: i32 = kani::any();

            let non_negative = <IntervalDurationIsNonNegative as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures((start, end));

            // A non-negative span is exactly `start` preceding `end`, and the
            // span is zero exactly when the endpoints coincide.
            assert_eq!(non_negative, start <= end);
            assert_eq!(i64::from(end) - i64::from(start) == 0, start == end);
        }
    }
}

// ── UtcTimelineOrderingAppliesToFixedInstants ──────────────────

impl Witness<KaniVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::UtcTimelineOrderingAppliesToFixedInstants::ensures",
    (i32, i32),
    |(a, b)| a <= b
);

amenable_derive::harness! {
    kani, VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC, {
        /// RFC 3339, 5.1 — two fixed instants are totally ordered by their position on the UTC timeline. `<=` on `i32` timeline positions is reflexive, antisymmetric, total,
        /// and transitive — a total order — checked over three symbolic instants.
        #[kani::proof]
        fn verify_utc_timeline_ordering_applies_to_fixed_instants() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();

            let le = |x: i32, y: i32| {
                <UtcTimelineOrderingAppliesToFixedInstants as ::amenable_core::Ensures<
                    KaniVerifier,
                >>::ensures((x, y))
            };

            // `<=` on timeline positions is a total order.
            assert!(le(a, a), "reflexive");
            assert!(le(a, b) || le(b, a), "total");
            assert!(!(le(a, b) && le(b, a)) || a == b, "antisymmetric");
            assert!(!(le(a, b) && le(b, c)) || le(a, c), "transitive");
        }
    }
}

// ── GregorianLeapYearUsesDivisibleByFourAndFourHundredException ──────────────────

impl Witness<KaniVerifier> for GregorianLeapYearUsesDivisibleByFourAndFourHundredException {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException::ensures",
    i32,
    |year| year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
);

amenable_derive::harness! {
    kani, VERIFY_GREGORIAN_LEAP_YEAR_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — a year is a leap year if divisible by 4, except a centennial year is a leap year only if also divisible by 400. The `y%4 && (y%100 || y%400)` rule agrees, over every `i32`, with the
        /// case split on centennial years, and with six dated anchors.
        #[kani::proof]
        fn verify_gregorian_leap_year() {
            let year: i32 = kani::any();

            let leap = <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(year);

            // The standard's stated form: a centennial year needs the /400
            // rule, every other year only the /4 rule.
            let case_split = if year % 100 == 0 { year % 400 == 0 } else { year % 4 == 0 };
            assert_eq!(leap, case_split);

            // Dated anchors — the classic off-by-a-century bugs.
            let leap_of = |y: i32| {
                <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as ::amenable_core::Ensures<
                    KaniVerifier,
                >>::ensures(y)
            };
            assert!(leap_of(2000), "2000 is a leap year");
            assert!(leap_of(1600), "1600 is a leap year");
            assert!(leap_of(2024), "2024 is a leap year");
            assert!(!leap_of(1900), "1900 is not a leap year");
            assert!(!leap_of(2100), "2100 is not a leap year");
            assert!(!leap_of(2023), "2023 is not a leap year");
        }
    }
}

// ── CentennialYearDivisibleByOneHundred ──────────────────

impl Witness<KaniVerifier> for CentennialYearDivisibleByOneHundred {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::CentennialYearDivisibleByOneHundred::ensures",
    i32,
    |year| year % 100 == 0
);

amenable_derive::harness! {
    kani, VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC, {
        /// ISO 8601-1:2019, 3.1.1.22 — a centennial year is one whose year number is an exact multiple of 100. `y % 100 == 0` agrees, over every `i32`, with `y%4 == 0 && y%25 == 0`
        /// (100 = 4·25, coprime factors), plus dated anchors.
        #[kani::proof]
        fn verify_centennial_year_divisible_by_one_hundred() {
            let year: i32 = kani::any();

            let centennial = <CentennialYearDivisibleByOneHundred as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(year);

            // 100 = 4 * 25 and gcd(4, 25) = 1, so divisibility by 100 is
            // exactly divisibility by both 4 and 25.
            assert_eq!(centennial, year % 4 == 0 && year % 25 == 0);

            let centennial_of = |y: i32| {
                <CentennialYearDivisibleByOneHundred as ::amenable_core::Ensures<
                    KaniVerifier,
                >>::ensures(y)
            };
            assert!(centennial_of(0), "year 0 is centennial");
            assert!(centennial_of(1900), "1900 is centennial");
            assert!(centennial_of(2000), "2000 is centennial");
            assert!(!centennial_of(2024), "2024 is not centennial");
        }
    }
}

// ── LeapYearHasThreeHundredSixtySixCalendarDays ──────────────────

impl Witness<KaniVerifier> for LeapYearHasThreeHundredSixtySixCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays::ensures",
    i32,
    |year| days_in_year(year) == 366
);

amenable_derive::harness! {
    kani, VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 — a leap year contains 366 calendar days. Over every non-negative `i32`, `days_in_year(y) == 366` agrees with
        /// `leap(y)`; a leap year is a common year plus one day; dated anchors.
        #[kani::proof]
        fn verify_leap_year_has_three_hundred_sixty_six_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(year >= 0);

            let has_366 = <LeapYearHasThreeHundredSixtySixCalendarDays as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(year);

            assert_eq!(has_366, is_gregorian_leap_year(year));
            // A leap year is a common year plus exactly one day.
            assert_eq!(
                days_in_year(year) - 365,
                if is_gregorian_leap_year(year) { 1 } else { 0 }
            );

            let has_366_of = |y: i32| {
                <LeapYearHasThreeHundredSixtySixCalendarDays as ::amenable_core::Ensures<
                    KaniVerifier,
                >>::ensures(y)
            };
            assert!(has_366_of(2000), "2000 has 366 days");
            assert!(has_366_of(2024), "2024 has 366 days");
            assert!(!has_366_of(2023), "2023 has 365 days");
            assert!(!has_366_of(1900), "1900 has 365 days");
        }
    }
}

// ── CommonYearHasThreeHundredSixtyFiveCalendarDays ──────────────────

impl Witness<KaniVerifier> for CommonYearHasThreeHundredSixtyFiveCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

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
    "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays::ensures",
    i32,
    |year| days_in_year(year) == 365
);

amenable_derive::harness! {
    kani, VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.20 — a common year contains 365 calendar days. Over every non-negative `i32`, `days_in_year(y) == 365` agrees with
        /// `!leap(y)`; the two lengths are distinct; dated anchors.
        #[kani::proof]
        fn verify_common_year_has_three_hundred_sixty_five_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(year >= 0);

            let has_365 = <CommonYearHasThreeHundredSixtyFiveCalendarDays as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(year);

            assert_eq!(has_365, !is_gregorian_leap_year(year));
            // 365 and 366 are the only two options, and they are distinct.
            assert!((days_in_year(year) == 365) != (days_in_year(year) == 366));

            let has_365_of = |y: i32| {
                <CommonYearHasThreeHundredSixtyFiveCalendarDays as ::amenable_core::Ensures<
                    KaniVerifier,
                >>::ensures(y)
            };
            assert!(has_365_of(2023), "2023 has 365 days");
            assert!(has_365_of(1900), "1900 has 365 days");
            assert!(!has_365_of(2000), "2000 has 366 days");
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
    "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays::ensures",
    i32,
    |year| (365..=366).contains(&days_in_year(year))
);

amenable_derive::harness! {
    kani, VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 2.2.14 — a year's duration is 365 or 366 calendar days. Over every non-negative `i32`, `days_in_year(y)` is always 365 or
        /// 366 — the model is well-formed.
        #[kani::proof]
        fn verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(year >= 0);

            let in_range = <YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(year);

            // The model is well-formed: every year's length is 365 or 366.
            assert!(in_range);
            assert!(days_in_year(year) == 365 || days_in_year(year) == 366);
        }
    }
}
