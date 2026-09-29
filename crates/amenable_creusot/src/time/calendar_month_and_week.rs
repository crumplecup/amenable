//! Calendar-field bounds: calendar month, weekday, and week number.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_time::{
        CalendarMonthInRangeOneToTwelve, WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    };

    use crate::CreusotVerifier;

    use super::{
        CALENDAR_MONTH_IN_RANGE_HOLDS_SRC, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC,
        VERIFY_WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_SRC,
        VERIFY_WEEKDAY_IN_RANGE_ONE_TO_SEVEN_SRC,
        WEEK_NUMBER_IN_RANGE_ONE_TO_FIFTY_THREE_HOLDS_SRC, WEEKDAY_IN_RANGE_ONE_TO_SEVEN_HOLDS_SRC,
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

    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for CalendarMonthInRangeOneToTwelve {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for WeekdayInRangeOneToSeven {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for WeekNumberInRangeOneToFiftyThree {}
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
