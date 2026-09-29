//! Month-duration and within-month day bounds -- built on `leap_year_core`'s leap-year rule.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(creusot)]
use super::leap_year_core::is_gregorian_leap_year;

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_time::{
        CalendarDayWithinMonthBounds, LeapDayOccursOnlyInLeapYear,
        MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    };

    use crate::CreusotVerifier;

    use super::{
        CALENDAR_DAY_WITHIN_MONTH_BOUNDS_HOLDS_SRC, LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_HOLDS_SRC,
        MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_HOLDS_SRC,
        VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC, VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC,
        VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC,
    };

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
    impl amenable_core::ClassifiedWitness<CreusotVerifier>
        for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
    {
    }
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for CalendarDayWithinMonthBounds {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for LeapDayOccursOnlyInLeapYear {}
}

amenable_derive::harness! {
    creusot, MONTH_IN_RANGE_ONE_TO_TWELVE_FOR_REQUIRES_HOLDS_SRC, {
        /// The calendar-month range (`1..=12`), in the plain
        /// `#[requires(..)]`-shaped form (input only, no `outcome`) —
        /// distinct from [`calendar_month_in_range_holds`] itself
        /// (which relates an input to a computed `outcome`) since a
        /// precondition has no result to compare against. Reused
        /// wherever a month/day check needs to restrict `month` to its
        /// real range.
        #[logic(open)]
        pub fn month_in_range_one_to_twelve_for_requires(month: u8) -> bool {
            pearlite! { month@ >= 1 && month@ <= 12 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::month_in_range_one_to_twelve_for_requires",
        "creusot",
        "requires",
        || MONTH_IN_RANGE_ONE_TO_TWELVE_FOR_REQUIRES_HOLDS_SRC,
    )
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
        #[requires(month_in_range_one_to_twelve_for_requires(month))]
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
    creusot, VALID_FEBRUARY_TWENTY_NINE_FORCES_LEAP_YEAR_HOLDS_SRC, {
        /// A valid February 29 forces the year to be a leap year — a
        /// real consequence of [`calendar_day_within_month_bounds_holds`]'s
        /// own definition (day 29 only fits within February's bound
        /// when that bound is 29, which only happens in a leap year),
        /// not a restatement of it.
        #[logic(open)]
        pub fn valid_february_twenty_nine_forces_leap_year_holds(
            year: i32,
            month: u8,
            day: u8,
            valid: bool,
        ) -> bool {
            pearlite! { month@ == 2 && day@ == 29 && valid ==> is_gregorian_leap_year(year) }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::valid_february_twenty_nine_forces_leap_year_holds",
        "creusot",
        "ensures",
        || VALID_FEBRUARY_TWENTY_NINE_FORCES_LEAP_YEAR_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC, {
        /// The exec bounds check equals `day` lying in month bounds, and a valid February 29 forces a leap year.
        #[requires(month_in_range_one_to_twelve_for_requires(month))]
        #[ensures(calendar_day_within_month_bounds_holds(year, month, day, result))]
        #[ensures(valid_february_twenty_nine_forces_leap_year_holds(year, month, day, result))]
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
