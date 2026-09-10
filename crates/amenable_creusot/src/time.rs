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
        CalendarMonthInRangeOneToTwelve,
        CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
        CenturyOrdinalInRangeZeroToNinetyNine, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
        HourInRangeZeroToTwentyFour, MinuteInRangeZeroToFiftyNine,
        OrdinalDayInRangeOneToThreeHundredSixtySix, SecondInRangeZeroToSixty,
        UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
        WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    };

    use crate::CreusotVerifier;

    use super::{
        CALENDAR_MONTH_IN_RANGE_HOLDS_SRC,
        CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
        CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_HOLDS_SRC,
        DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
        HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_HOLDS_SRC, MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
        ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_HOLDS_SRC,
        SECOND_IN_RANGE_ZERO_TO_SIXTY_HOLDS_SRC,
        UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_HOLDS_SRC,
        UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
        VERIFY_CALENDAR_MONTH_IN_RANGE_SRC,
        VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC,
        VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC,
        VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC,
        VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC,
        VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC,
        VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC,
        VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC,
        VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC,
        VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC,
        VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC,
        VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC,
        WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_HOLDS_SRC, WEEKDAY_IN_RANGE_ONE_TO_SEVEN_HOLDS_SRC,
    };

    impl amenable_core::Witness<CreusotVerifier> for CalendarMonthInRangeOneToTwelve {
        type SupportingEvidence = Self;
        type ProofArtifact = crate::witness::MultiCheckProof;

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
