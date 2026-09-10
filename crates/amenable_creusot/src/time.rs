//! Creusot proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets a `#[logic]` postcondition, a `#[requires]`/`#[ensures]`-contracted
//! function Creusot machine-checks against it, and (on the
//! `#[cfg(not(creusot))]` side, matching `ledger::contract_bounds`) a
//! `Witness<CreusotVerifier>` + `Ensures<CreusotVerifier>` tying the real
//! `amenable_time` type to that Pearlite content. Structural contracts
//! stay `Standard`-only and never reach this module.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

// ── CalendarMonthInRangeOneToTwelve ──────────────────────────────────

/// The `#[cfg(not(creusot))]` `Witness`/`Ensures` impls for
/// `CalendarMonthInRangeOneToTwelve` — gated on the `mod` (see
/// `ledger::contract_bounds`'s `not_creusot_mirror` for the rationale:
/// every item is a trait impl, and nothing establishes a token *from*
/// this contract, so real Creusot translation never needs to see it).
#[cfg(not(creusot))]
mod not_creusot_mirror {
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

    use crate::CreusotVerifier;

    use super::{
        CALENDAR_DAY_WITHIN_MONTH_BOUNDS_HOLDS_SRC, CALENDAR_MONTH_IN_RANGE_HOLDS_SRC,
        CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
        CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_HOLDS_SRC,
        CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_HOLDS_SRC,
        COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_HOLDS_SRC,
        DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
        GREGORIAN_LEAP_YEAR_HOLDS_SRC, HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_HOLDS_SRC,
        INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC, INTERVAL_START_PRECEDES_END_HOLDS_SRC,
        LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_HOLDS_SRC,
        LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC,
        MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
        MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_HOLDS_SRC,
        ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_HOLDS_SRC,
        SECOND_IN_RANGE_ZERO_TO_SIXTY_HOLDS_SRC,
        UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_HOLDS_SRC,
        UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
        UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC,
        VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC,
        VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC,
        VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC,
        VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC,
        VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC,
        VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC,
        VERIFY_GREGORIAN_LEAP_YEAR_SRC, VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC,
        VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, VERIFY_INTERVAL_START_PRECEDES_END_SRC,
        VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC,
        VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC,
        VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC,
        VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC,
        VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC,
        VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC,
        VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC,
        VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC,
        VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC,
        VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC,
        VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC,
        VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC,
        WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_HOLDS_SRC, WEEKDAY_IN_RANGE_ONE_TO_SEVEN_HOLDS_SRC,
        YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC,
    };

    impl amenable_core::Witness<CreusotVerifier> for CalendarMonthInRangeOneToTwelve {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_calendar_month_in_range".to_owned(),
                VERIFY_CALENDAR_MONTH_IN_RANGE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for CalendarMonthInRangeOneToTwelve {
        type Input = u8;
        // Pearlite predicates have no exec representation — carry the real
        // bound's source for audit, not a checked value (the checking is
        // `check_calendar_month_in_range` itself).
        type Bound = &'static str;

        fn ensures(_month: u8) -> Self::Bound {
            CALENDAR_MONTH_IN_RANGE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::CalendarMonthInRangeOneToTwelve",
            "creusot",
            || {
                <CalendarMonthInRangeOneToTwelve as amenable_core::Witness<CreusotVerifier>>::proof()
                    .to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for HourInRangeZeroToTwentyFour {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_hour_in_range_zero_to_twenty_four".to_owned(),
                VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for HourInRangeZeroToTwentyFour {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_hour: u8) -> Self::Bound {
            HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::HourInRangeZeroToTwentyFour",
            "creusot",
            || {
                <HourInRangeZeroToTwentyFour as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for MinuteInRangeZeroToFiftyNine {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_minute_in_range_zero_to_fifty_nine".to_owned(),
                VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for MinuteInRangeZeroToFiftyNine {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_minute: u8) -> Self::Bound {
            MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::MinuteInRangeZeroToFiftyNine",
            "creusot",
            || {
                <MinuteInRangeZeroToFiftyNine as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for SecondInRangeZeroToSixty {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_second_in_range_zero_to_sixty".to_owned(),
                VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for SecondInRangeZeroToSixty {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_second: u8) -> Self::Bound {
            SECOND_IN_RANGE_ZERO_TO_SIXTY_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::SecondInRangeZeroToSixty",
            "creusot",
            || {
                <SecondInRangeZeroToSixty as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_utc_offset_hour_in_range_zero_to_twenty_three".to_owned(),
                VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_hour: u8) -> Self::Bound {
            UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::UtcOffsetHourInRangeZeroToTwentyThree",
            "creusot",
            || {
                <UtcOffsetHourInRangeZeroToTwentyThree as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_utc_offset_minute_in_range_zero_to_fifty_nine".to_owned(),
                VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_minute: u8) -> Self::Bound {
            UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine",
            "creusot",
            || {
                <UtcOffsetMinuteInRangeZeroToFiftyNine as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for WeekdayInRangeOneToSeven {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_weekday_in_range_one_to_seven".to_owned(),
                VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for WeekdayInRangeOneToSeven {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_weekday: u8) -> Self::Bound {
            WEEKDAY_IN_RANGE_ONE_TO_SEVEN_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::WeekdayInRangeOneToSeven",
            "creusot",
            || {
                <WeekdayInRangeOneToSeven as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for WeekNumberInRangeOneToFiftyThree {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_week_number_in_range_one_to_fifty_three".to_owned(),
                VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for WeekNumberInRangeOneToFiftyThree {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_week: u8) -> Self::Bound {
            WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::WeekNumberInRangeOneToFiftyThree",
            "creusot",
            || {
                <WeekNumberInRangeOneToFiftyThree as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for OrdinalDayInRangeOneToThreeHundredSixtySix {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_ordinal_day_in_range_one_to_three_hundred_sixty_six".to_owned(),
                VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for OrdinalDayInRangeOneToThreeHundredSixtySix {
        type Input = u16;
        type Bound = &'static str;

        fn ensures(_day: u16) -> Self::Bound {
            ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
            "creusot",
            || {
                <OrdinalDayInRangeOneToThreeHundredSixtySix as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_century_ordinal_in_range_zero_to_ninety_nine".to_owned(),
                VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {
        type Input = u8;
        type Bound = &'static str;

        fn ensures(_ordinal: u8) -> Self::Bound {
            CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
            "creusot",
            || {
                <CenturyOrdinalInRangeZeroToNinetyNine as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for DecadeOrdinalInRangeZeroToNineHundredNinetyNine {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine".to_owned(),
                VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for DecadeOrdinalInRangeZeroToNineHundredNinetyNine {
        type Input = u16;
        type Bound = &'static str;

        fn ensures(_ordinal: u16) -> Self::Bound {
            DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
            "creusot",
            || {
                <DecadeOrdinalInRangeZeroToNineHundredNinetyNine as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier>
        for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
    {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine"
                    .to_owned(),
                VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC
                    .to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier>
        for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
    {
        type Input = u16;
        type Bound = &'static str;

        fn ensures(_year: u16) -> Self::Bound {
            CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
            "creusot",
            || {
                <CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for IntervalStartPrecedesEnd {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_interval_start_precedes_end".to_owned(),
                VERIFY_INTERVAL_START_PRECEDES_END_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for IntervalStartPrecedesEnd {
        type Input = (i32, i32);
        type Bound = &'static str;

        fn ensures(_endpoints: (i32, i32)) -> Self::Bound {
            INTERVAL_START_PRECEDES_END_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::IntervalStartPrecedesEnd",
            "creusot",
            || {
                <IntervalStartPrecedesEnd as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for IntervalDurationIsNonNegative {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_interval_duration_is_non_negative".to_owned(),
                VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for IntervalDurationIsNonNegative {
        type Input = (i32, i32);
        type Bound = &'static str;

        fn ensures(_endpoints: (i32, i32)) -> Self::Bound {
            INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::IntervalDurationIsNonNegative",
            "creusot",
            || {
                <IntervalDurationIsNonNegative as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_utc_timeline_ordering_applies_to_fixed_instants".to_owned(),
                VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for UtcTimelineOrderingAppliesToFixedInstants {
        type Input = (i32, i32);
        type Bound = &'static str;

        fn ensures(_endpoints: (i32, i32)) -> Self::Bound {
            UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::UtcTimelineOrderingAppliesToFixedInstants",
            "creusot",
            || {
                <UtcTimelineOrderingAppliesToFixedInstants as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier>
        for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
    {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_gregorian_leap_year".to_owned(),
                VERIFY_GREGORIAN_LEAP_YEAR_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier>
        for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
    {
        type Input = i32;
        type Bound = &'static str;

        fn ensures(_year: i32) -> Self::Bound {
            GREGORIAN_LEAP_YEAR_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
            "creusot",
            || {
                <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for CentennialYearDivisibleByOneHundred {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_centennial_year_divisible_by_one_hundred".to_owned(),
                VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for CentennialYearDivisibleByOneHundred {
        type Input = i32;
        type Bound = &'static str;

        fn ensures(_year: i32) -> Self::Bound {
            CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::CentennialYearDivisibleByOneHundred",
            "creusot",
            || {
                <CentennialYearDivisibleByOneHundred as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for LeapYearHasThreeHundredSixtySixCalendarDays {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_leap_year_has_three_hundred_sixty_six_calendar_days".to_owned(),
                VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for LeapYearHasThreeHundredSixtySixCalendarDays {
        type Input = i32;
        type Bound = &'static str;

        fn ensures(_year: i32) -> Self::Bound {
            LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
            "creusot",
            || {
                <LeapYearHasThreeHundredSixtySixCalendarDays as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for CommonYearHasThreeHundredSixtyFiveCalendarDays {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_common_year_has_three_hundred_sixty_five_calendar_days".to_owned(),
                VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for CommonYearHasThreeHundredSixtyFiveCalendarDays {
        type Input = i32;
        type Bound = &'static str;

        fn ensures(_year: i32) -> Self::Bound {
            COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
            "creusot",
            || {
                <CommonYearHasThreeHundredSixtyFiveCalendarDays as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier>
        for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
    {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days".to_owned(),
                VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier>
        for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
    {
        type Input = i32;
        type Bound = &'static str;

        fn ensures(_year: i32) -> Self::Bound {
            YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
            "creusot",
            || {
                <YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier>
        for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
    {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days".to_owned(),
                VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC
                    .to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier>
        for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
    {
        type Input = (i32, u8);
        type Bound = &'static str;

        fn ensures(_input: (i32, u8)) -> Self::Bound {
            MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
            "creusot",
            || {
                <MonthDurationInRangeTwentyEightToThirtyOneCalendarDays as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for CalendarDayWithinMonthBounds {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_calendar_day_within_month_bounds".to_owned(),
                VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for CalendarDayWithinMonthBounds {
        type Input = (i32, u8, u8);
        type Bound = &'static str;

        fn ensures(_input: (i32, u8, u8)) -> Self::Bound {
            CALENDAR_DAY_WITHIN_MONTH_BOUNDS_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::CalendarDayWithinMonthBounds",
            "creusot",
            || {
                <CalendarDayWithinMonthBounds as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    impl amenable_core::Witness<CreusotVerifier> for LeapDayOccursOnlyInLeapYear {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

        fn support() -> amenable_core::WitnessSupportSummary {
            amenable_core::WitnessSupportSummary::checked_leaf()
        }

        fn proof() -> Self::ProofArtifact {
            crate::witness::MultiCheckProof::new(vec![(
                "check_leap_day_occurs_only_in_leap_year".to_owned(),
                VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC.to_owned(),
            )])
        }
    }

    impl amenable_core::Ensures<CreusotVerifier> for LeapDayOccursOnlyInLeapYear {
        type Input = (i32, u8, u8);
        type Bound = &'static str;

        fn ensures(_input: (i32, u8, u8)) -> Self::Bound {
            LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_HOLDS_SRC
        }
    }

    ::inventory::submit! {
        ::amenable_core::ProofRecord::new(
            "amenable_time::LeapDayOccursOnlyInLeapYear",
            "creusot",
            || {
                <LeapDayOccursOnlyInLeapYear as amenable_core::Witness<CreusotVerifier>>::proof().to_string()
            },
        )
    }

    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for CalendarMonthInRangeOneToTwelve {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for HourInRangeZeroToTwentyFour {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for MinuteInRangeZeroToFiftyNine {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for SecondInRangeZeroToSixty {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for WeekdayInRangeOneToSeven {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for WeekNumberInRangeOneToFiftyThree {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for OrdinalDayInRangeOneToThreeHundredSixtySix
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for DecadeOrdinalInRangeZeroToNineHundredNinetyNine
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for IntervalStartPrecedesEnd {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for IntervalDurationIsNonNegative {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for UtcTimelineOrderingAppliesToFixedInstants
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for CentennialYearDivisibleByOneHundred {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for LeapYearHasThreeHundredSixtySixCalendarDays
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for CommonYearHasThreeHundredSixtyFiveCalendarDays
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for CalendarDayWithinMonthBounds {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for LeapDayOccursOnlyInLeapYear {}
}

amenable_derive::harness! {
    creusot, CALENDAR_MONTH_IN_RANGE_HOLDS_SRC, {
        /// The month-range postcondition (ISO 8601-1:2019, 3.1.1.2): the
        /// outcome equals the twelve-way enumeration of the legal
        /// calendar months.
        #[logic(open)]
        pub fn calendar_month_in_range_holds(month: u8, outcome: bool) -> bool {
            pearlite! {
                outcome
                    == (month == 1u8
                        || month == 2u8
                        || month == 3u8
                        || month == 4u8
                        || month == 5u8
                        || month == 6u8
                        || month == 7u8
                        || month == 8u8
                        || month == 9u8
                        || month == 10u8
                        || month == 11u8
                        || month == 12u8)
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::calendar_month_in_range_holds",
        "creusot",
        "ensures",
        || CALENDAR_MONTH_IN_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC, {
        /// The `1..=12` arithmetic month-range form satisfies the
        /// enumerated spec, for every `u8`.
        #[requires(true)]
        #[ensures(calendar_month_in_range_holds(month, result))]
        fn check_calendar_month_in_range(month: u8) -> bool {
            month >= 1u8 && month <= 12u8
        }
    }
}

amenable_derive::harness! {
    creusot, HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_HOLDS_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 / 5.3.2 — an hour is 00 through 24 (24 reserved for end-of-day). The `0..=24` postcondition, stated as `hour < 25`.
        #[logic(open)]
        pub fn hour_in_range_zero_to_twenty_four_holds(hour: u8, outcome: bool) -> bool {
            pearlite! { outcome == (hour < 25u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::hour_in_range_zero_to_twenty_four_holds",
        "creusot",
        "ensures",
        || HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC, {
        /// The `0..=24` inclusive form (`hour <= 24`) satisfies the
        /// `hour < 25` spec, for every `u8`.
        #[requires(true)]
        #[ensures(hour_in_range_zero_to_twenty_four_holds(hour, result))]
        fn check_hour_in_range_zero_to_twenty_four(hour: u8) -> bool {
            hour <= 24u8
        }
    }
}

amenable_derive::harness! {
    creusot, MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.1 — a minute is 00 through 59. The `0..=59` postcondition, stated as `minute < 60`.
        #[logic(open)]
        pub fn minute_in_range_zero_to_fifty_nine_holds(minute: u8, outcome: bool) -> bool {
            pearlite! { outcome == (minute < 60u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::minute_in_range_zero_to_fifty_nine_holds",
        "creusot",
        "ensures",
        || MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, {
        /// The `0..=59` inclusive form (`minute <= 59`) satisfies the
        /// `minute < 60` spec, for every `u8`.
        #[requires(true)]
        #[ensures(minute_in_range_zero_to_fifty_nine_holds(minute, result))]
        fn check_minute_in_range_zero_to_fifty_nine(minute: u8) -> bool {
            minute <= 59u8
        }
    }
}

amenable_derive::harness! {
    creusot, SECOND_IN_RANGE_ZERO_TO_SIXTY_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.1 — a second is 00 through 60 (60 admits a leap second). The `0..=60` postcondition, stated as `second < 61`.
        #[logic(open)]
        pub fn second_in_range_zero_to_sixty_holds(second: u8, outcome: bool) -> bool {
            pearlite! { outcome == (second < 61u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::second_in_range_zero_to_sixty_holds",
        "creusot",
        "ensures",
        || SECOND_IN_RANGE_ZERO_TO_SIXTY_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC, {
        /// The `0..=60` inclusive form (`second <= 60`) satisfies the
        /// `second < 61` spec, for every `u8`.
        #[requires(true)]
        #[ensures(second_in_range_zero_to_sixty_holds(second, result))]
        fn check_second_in_range_zero_to_sixty(second: u8) -> bool {
            second <= 60u8
        }
    }
}

amenable_derive::harness! {
    creusot, UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset hour is 00 through 23. The `0..=23` postcondition, stated as `hour < 24`.
        #[logic(open)]
        pub fn utc_offset_hour_in_range_zero_to_twenty_three_holds(hour: u8, outcome: bool) -> bool {
            pearlite! { outcome == (hour < 24u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::utc_offset_hour_in_range_zero_to_twenty_three_holds",
        "creusot",
        "ensures",
        || UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC, {
        /// The `0..=23` inclusive form (`hour <= 23`) satisfies the
        /// `hour < 24` spec, for every `u8`.
        #[requires(true)]
        #[ensures(utc_offset_hour_in_range_zero_to_twenty_three_holds(hour, result))]
        fn check_utc_offset_hour_in_range_zero_to_twenty_three(hour: u8) -> bool {
            hour <= 23u8
        }
    }
}

amenable_derive::harness! {
    creusot, UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset minute is 00 through 59. The `0..=59` postcondition, stated as `minute < 60`.
        #[logic(open)]
        pub fn utc_offset_minute_in_range_zero_to_fifty_nine_holds(minute: u8, outcome: bool) -> bool {
            pearlite! { outcome == (minute < 60u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::utc_offset_minute_in_range_zero_to_fifty_nine_holds",
        "creusot",
        "ensures",
        || UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, {
        /// The `0..=59` inclusive form (`minute <= 59`) satisfies the
        /// `minute < 60` spec, for every `u8`.
        #[requires(true)]
        #[ensures(utc_offset_minute_in_range_zero_to_fifty_nine_holds(minute, result))]
        fn check_utc_offset_minute_in_range_zero_to_fifty_nine(minute: u8) -> bool {
            minute <= 59u8
        }
    }
}

amenable_derive::harness! {
    creusot, WEEKDAY_IN_RANGE_ONE_TO_SEVEN_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.4.1 — a weekday is 1 (Monday) through 7 (Sunday). The outcome equals the seven-way enumeration of the ISO weekdays (Mon..Sun).
        #[logic(open)]
        pub fn weekday_in_range_one_to_seven_holds(weekday: u8, outcome: bool) -> bool {
            pearlite! { outcome == (weekday == 1u8 || weekday == 2u8 || weekday == 3u8 || weekday == 4u8 || weekday == 5u8 || weekday == 6u8 || weekday == 7u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::weekday_in_range_one_to_seven_holds",
        "creusot",
        "ensures",
        || WEEKDAY_IN_RANGE_ONE_TO_SEVEN_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC, {
        /// The `1..=7` range form satisfies the enumerated spec, for every `u8`.
        #[requires(true)]
        #[ensures(weekday_in_range_one_to_seven_holds(weekday, result))]
        fn check_weekday_in_range_one_to_seven(weekday: u8) -> bool {
            weekday >= 1u8 && weekday <= 7u8
        }
    }
}

amenable_derive::harness! {
    creusot, WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.4.1 — a calendar-week number is 01 through 53. The `1..=53` postcondition, stated as `1 <= week && week < 54`.
        #[logic(open)]
        pub fn week_number_in_range_one_to_fifty_three_holds(week: u8, outcome: bool) -> bool {
            pearlite! { outcome == (week >= 1u8 && week < 54u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::week_number_in_range_one_to_fifty_three_holds",
        "creusot",
        "ensures",
        || WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC, {
        /// The `1..=53` inclusive form (`1 <= week && week <= 53`) satisfies the `1 <= week && week < 54` spec, for every `u8`.
        #[requires(true)]
        #[ensures(week_number_in_range_one_to_fifty_three_holds(week, result))]
        fn check_week_number_in_range_one_to_fifty_three(week: u8) -> bool {
            week >= 1u8 && week <= 53u8
        }
    }
}

amenable_derive::harness! {
    creusot, ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.3.1 — an ordinal day-of-year is 001 through 365, or 366 in a leap year. The `1..=366` postcondition, stated as `1 <= day && day < 367`.
        #[logic(open)]
        pub fn ordinal_day_in_range_one_to_three_hundred_sixty_six_holds(day: u16, outcome: bool) -> bool {
            pearlite! { outcome == (day >= 1u16 && day < 367u16) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::ordinal_day_in_range_one_to_three_hundred_sixty_six_holds",
        "creusot",
        "ensures",
        || ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC, {
        /// The `1..=366` inclusive form (`1 <= day && day <= 366`) satisfies the `1 <= day && day < 367` spec, for every `u16`.
        #[requires(true)]
        #[ensures(ordinal_day_in_range_one_to_three_hundred_sixty_six_holds(day, result))]
        fn check_ordinal_day_in_range_one_to_three_hundred_sixty_six(day: u16) -> bool {
            day >= 1u16 && day <= 366u16
        }
    }
}

amenable_derive::harness! {
    creusot, CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_HOLDS_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.12 — a Gregorian century ordinal is 00 through 99. The `0..=99` postcondition, stated as `ordinal < 100`.
        #[logic(open)]
        pub fn century_ordinal_in_range_zero_to_ninety_nine_holds(ordinal: u8, outcome: bool) -> bool {
            pearlite! { outcome == (ordinal < 100u8) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::century_ordinal_in_range_zero_to_ninety_nine_holds",
        "creusot",
        "ensures",
        || CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC, {
        /// The `0..=99` inclusive form (`ordinal <= 99`) satisfies the `ordinal < 100` spec, for every `u8`.
        #[requires(true)]
        #[ensures(century_ordinal_in_range_zero_to_ninety_nine_holds(ordinal, result))]
        fn check_century_ordinal_in_range_zero_to_ninety_nine(ordinal: u8) -> bool {
            ordinal <= 99u8
        }
    }
}

amenable_derive::harness! {
    creusot, DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.11 — a Gregorian decade ordinal is 000 through 999. The `0..=999` postcondition, stated as `ordinal < 1000`.
        #[logic(open)]
        pub fn decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds(ordinal: u16, outcome: bool) -> bool {
            pearlite! { outcome == (ordinal < 1000u16) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds",
        "creusot",
        "ensures",
        || DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// The `0..=999` inclusive form (`ordinal <= 999`) satisfies the `ordinal < 1000` spec, for every `u16`.
        #[requires(true)]
        #[ensures(decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds(ordinal, result))]
        fn check_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine(ordinal: u16) -> bool {
            ordinal <= 999u16
        }
    }
}

amenable_derive::harness! {
    creusot, CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.2.1 — a non-expanded calendar year is 0000 through 9999. The `0..=9999` postcondition, stated as `year < 10000`.
        #[logic(open)]
        pub fn calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds(year: u16, outcome: bool) -> bool {
            pearlite! { outcome == (year < 10000u16) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds",
        "creusot",
        "ensures",
        || CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// The `0..=9999` inclusive form (`year <= 9999`) satisfies the `year < 10000` spec, for every `u16`.
        #[requires(true)]
        #[ensures(calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds(year, result))]
        fn check_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine(year: u16) -> bool {
            year <= 9999u16
        }
    }
}

amenable_derive::harness! {
    creusot, INTERVAL_START_PRECEDES_END_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.6 / 3.1.1.8 — an interval's first endpoint is no later than its second on the relevant timeline: the outcome equals `start` preceding-or-equal `end` on the timeline.
        #[logic(open)]
        pub fn interval_start_precedes_end_holds(start: i32, end: i32, outcome: bool) -> bool {
            pearlite! { outcome == (start@ <= end@) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::interval_start_precedes_end_holds",
        "creusot",
        "ensures",
        || INTERVAL_START_PRECEDES_END_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_INTERVAL_START_PRECEDES_END_SRC, {
        /// `start <= end` satisfies the precedence spec, its negation form, and the non-negative-span form, for every `i32` pair.
        #[requires(true)]
        #[ensures(interval_start_precedes_end_holds(start, end, result))]
        #[ensures((start@ <= end@) == !(end@ < start@))]
        #[ensures((start@ <= end@) == (end@ - start@ >= 0))]
        fn check_interval_start_precedes_end(start: i32, end: i32) -> bool {
            start <= end
        }
    }
}

amenable_derive::harness! {
    creusot, INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.8 — the span between an interval's endpoints is zero or positive, never negative: the outcome equals the `end@ - start@` span being non-negative.
        #[logic(open)]
        pub fn interval_duration_is_non_negative_holds(start: i32, end: i32, outcome: bool) -> bool {
            pearlite! { outcome == (end@ - start@ >= 0) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::interval_duration_is_non_negative_holds",
        "creusot",
        "ensures",
        || INTERVAL_DURATION_IS_NON_NEGATIVE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_INTERVAL_DURATION_IS_NON_NEGATIVE_SRC, {
        /// `start <= end` is the panic-free witness that the span is non-negative, and pins the zero-span case to endpoint equality, for every `i32` pair.
        #[requires(true)]
        #[ensures(interval_duration_is_non_negative_holds(start, end, result))]
        #[ensures((end@ - start@ >= 0) == (start@ <= end@))]
        #[ensures((end@ - start@ == 0) == (start@ == end@))]
        fn check_interval_duration_is_non_negative(start: i32, end: i32) -> bool {
            start <= end
        }
    }
}

amenable_derive::harness! {
    creusot, UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC, {
        /// RFC 3339, 5.1 — two fixed instants are totally ordered by their position on the UTC timeline: the outcome equals `earlier` being at or before `later`.
        #[logic(open)]
        pub fn utc_timeline_ordering_applies_to_fixed_instants_holds(earlier: i32, later: i32, outcome: bool) -> bool {
            pearlite! { outcome == (earlier@ <= later@) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::utc_timeline_ordering_applies_to_fixed_instants_holds",
        "creusot",
        "ensures",
        || UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_UTC_TIMELINE_ORDERING_APPLIES_TO_FIXED_INSTANTS_SRC, {
        /// `a <= b` satisfies the ordering spec, and the relation is reflexive, total, antisymmetric, and transitive over `i32` instants.
        #[requires(true)]
        #[ensures(utc_timeline_ordering_applies_to_fixed_instants_holds(a, b, result))]
        #[ensures(utc_timeline_ordering_applies_to_fixed_instants_holds(a, a, true))]
        #[ensures(a@ <= b@ || b@ <= a@)]
        #[ensures(a@ <= b@ && b@ <= a@ ==> a@ == b@)]
        #[ensures(a@ <= b@ && b@ <= c@ ==> a@ <= c@)]
        fn check_utc_timeline_ordering_applies_to_fixed_instants(a: i32, b: i32, c: i32) -> bool {
            a <= b
        }
    }
}

amenable_derive::harness! {
    creusot, GREGORIAN_LEAP_YEAR_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — a year is a leap year if divisible by 4, except a centennial year is a leap year only if also divisible by 400: the outcome equals the divisible-by-4-with-the-centennial-/400-exception rule.
        #[logic(open)]
        pub fn gregorian_leap_year_holds(year: i32, outcome: bool) -> bool {
            pearlite! { outcome == (year@ % 4 == 0 && (year@ % 100 != 0 || year@ % 400 == 0)) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::gregorian_leap_year_holds",
        "creusot",
        "ensures",
        || GREGORIAN_LEAP_YEAR_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_GREGORIAN_LEAP_YEAR_SRC, {
        /// The exec leap rule satisfies the spec, and every leap year is divisible by four, for every `i32`.
        #[requires(true)]
        #[ensures(gregorian_leap_year_holds(year, result))]
        #[ensures(year@ % 4 == 0 && (year@ % 100 != 0 || year@ % 400 == 0) ==> year@ % 4 == 0)]
        fn check_gregorian_leap_year(year: i32) -> bool {
            year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
        }
    }
}

amenable_derive::harness! {
    creusot, CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.22 — a centennial year is one whose year number is an exact multiple of 100: the outcome equals `year` being an exact multiple of 100.
        #[logic(open)]
        pub fn centennial_year_divisible_by_one_hundred_holds(year: i32, outcome: bool) -> bool {
            pearlite! { outcome == (year@ % 100 == 0) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::centennial_year_divisible_by_one_hundred_holds",
        "creusot",
        "ensures",
        || CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC, {
        /// The exec `year % 100 == 0` check satisfies the spec, for every `i32`.
        #[requires(true)]
        #[ensures(centennial_year_divisible_by_one_hundred_holds(year, result))]
        fn check_centennial_year_divisible_by_one_hundred(year: i32) -> bool {
            year % 100 == 0
        }
    }
}

amenable_derive::harness! {
    creusot, IS_GREGORIAN_LEAP_YEAR_SRC, {
        /// The Gregorian leap-year rule (ISO 8601-1:2019, 3.1.1.21 note 1),
        /// shared by the year-length and month/day-bound checks.
        #[logic(open)]
        pub fn is_gregorian_leap_year(year: i32) -> bool {
            pearlite! { year@ % 4 == 0 && (year@ % 100 != 0 || year@ % 400 == 0) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::is_gregorian_leap_year",
        "creusot",
        "logic",
        || IS_GREGORIAN_LEAP_YEAR_SRC,
    )
}

amenable_derive::harness! {
    creusot, LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 — a leap year contains 366 calendar days: the outcome equals `year` being a leap year (a leap year has 366 days).
        #[logic(open)]
        pub fn leap_year_has_three_hundred_sixty_six_calendar_days_holds(year: i32, outcome: bool) -> bool {
            pearlite! { outcome == is_gregorian_leap_year(year) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::leap_year_has_three_hundred_sixty_six_calendar_days_holds",
        "creusot",
        "ensures",
        || LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// The exec year-length check equals `year` being a leap year, for every non-negative `i32`.
        #[requires(year@ >= 0)]
        #[ensures(leap_year_has_three_hundred_sixty_six_calendar_days_holds(year, result))]
        #[ensures(is_gregorian_leap_year(year) == result)]
        fn check_leap_year_has_three_hundred_sixty_six_calendar_days(year: i32) -> bool {
            let days: i32 = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
            days == 366
        }
    }
}

amenable_derive::harness! {
    creusot, COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.20 — a common year contains 365 calendar days: the outcome equals `year` not being a leap year (a common year has 365 days).
        #[logic(open)]
        pub fn common_year_has_three_hundred_sixty_five_calendar_days_holds(year: i32, outcome: bool) -> bool {
            pearlite! { outcome == (!is_gregorian_leap_year(year)) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::common_year_has_three_hundred_sixty_five_calendar_days_holds",
        "creusot",
        "ensures",
        || COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC, {
        /// The exec year-length check equals `year` not being a leap year, for every non-negative `i32`.
        #[requires(year@ >= 0)]
        #[ensures(common_year_has_three_hundred_sixty_five_calendar_days_holds(year, result))]
        #[ensures(is_gregorian_leap_year(year) ==> !result)]
        fn check_common_year_has_three_hundred_sixty_five_calendar_days(year: i32) -> bool {
            let days: i32 = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
            days == 365
        }
    }
}

amenable_derive::harness! {
    creusot, YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC, {
        /// ISO 8601-1:2019, 2.2.14 — a year's duration is 365 or 366 calendar days: the outcome always holds — the length is 365 or 366.
        #[logic(open)]
        pub fn year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds(_year: i32, outcome: bool) -> bool {
            pearlite! { outcome }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds",
        "creusot",
        "ensures",
        || YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// The exec year-length check is always in `365..=366`, for every non-negative `i32`.
        #[requires(year@ >= 0)]
        #[ensures(year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds(year, result))]
        #[ensures(is_gregorian_leap_year(year) ==> result)]
        #[ensures(!is_gregorian_leap_year(year) ==> result)]
        fn check_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days(year: i32) -> bool {
            let days: i32 = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
            days >= 365 && days <= 366
        }
    }
}

amenable_derive::harness! {
    creusot, MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_HOLDS_SRC, {
        /// ISO 8601-1:2019, 2.2.12 — a month's duration is 28, 29, 30, or 31 calendar days according to the month and year: the outcome always holds — the duration is 28, 29, 30, or 31.
        #[logic(open)]
        pub fn month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds(_year: i32, _month: u8, outcome: bool) -> bool {
            pearlite! { outcome }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds",
        "creusot",
        "ensures",
        || MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC, {
        /// The exec month-duration check is always in `28..=31`, for every year and month `1..=12`.
        #[requires(month@ >= 1 && month@ <= 12)]
        #[ensures(month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds(year, month, result))]
        fn check_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days(year: i32, month: u8) -> bool {
            let dim: u8 = if month == 2 {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 }
            } else if month == 4 || month == 6 || month == 9 || month == 11 {
                30
            } else {
                31
            };
            dim >= 28 && dim <= 31
        }
    }
}

amenable_derive::harness! {
    creusot, CALENDAR_DAY_WITHIN_MONTH_BOUNDS_HOLDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.2.1 — a calendar-date day component is within the valid day count for that month and year: the outcome equals `day` lying in `1..=days_in_month(year, month)`.
        #[logic(open)]
        pub fn calendar_day_within_month_bounds_holds(year: i32, month: u8, day: u8, outcome: bool) -> bool {
            pearlite! { outcome == (day@ >= 1 && day@ <= (if month@ == 2 {
                if is_gregorian_leap_year(year) { 29 } else { 28 }
            } else if month@ == 4 || month@ == 6 || month@ == 9 || month@ == 11 {
                30
            } else {
                31
            })) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::calendar_day_within_month_bounds_holds",
        "creusot",
        "ensures",
        || CALENDAR_DAY_WITHIN_MONTH_BOUNDS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC, {
        /// The exec bounds check equals `day` lying in month bounds, and a valid February 29 forces a leap year.
        #[requires(month@ >= 1 && month@ <= 12)]
        #[ensures(calendar_day_within_month_bounds_holds(year, month, day, result))]
        #[ensures(month@ == 2 && day@ == 29 && result ==> is_gregorian_leap_year(year))]
        fn check_calendar_day_within_month_bounds(year: i32, month: u8, day: u8) -> bool {
            let dim: u8 = if month == 2 {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 }
            } else if month == 4 || month == 6 || month == 9 || month == 11 {
                30
            } else {
                31
            };
            day >= 1 && day <= dim
        }
    }
}

amenable_derive::harness! {
    creusot, LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_HOLDS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — the 29th of February is a valid calendar date only when the year is a leap year: the outcome equals: if the date is February 29, the year is a leap year.
        #[logic(open)]
        pub fn leap_day_occurs_only_in_leap_year_holds(year: i32, month: u8, day: u8, outcome: bool) -> bool {
            pearlite! { outcome == (!(month@ == 2 && day@ == 29) || is_gregorian_leap_year(year)) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::leap_day_occurs_only_in_leap_year_holds",
        "creusot",
        "ensures",
        || LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC, {
        /// The exec predicate equals: a February-29 date forces a leap year.
        #[requires(true)]
        #[ensures(leap_day_occurs_only_in_leap_year_holds(year, month, day, result))]
        fn check_leap_day_occurs_only_in_leap_year(year: i32, month: u8, day: u8) -> bool {
            !(month == 2 && day == 29) || (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
        }
    }
}
