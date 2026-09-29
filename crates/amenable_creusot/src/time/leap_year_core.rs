//! The Gregorian leap-year rule and the year-length contracts built on it.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_time::{
        CentennialYearDivisibleByOneHundred, CommonYearHasThreeHundredSixtyFiveCalendarDays,
        GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
        LeapYearHasThreeHundredSixtySixCalendarDays,
        YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    };

    use crate::CreusotVerifier;

    use super::{
        CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_HOLDS_SRC,
        COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_HOLDS_SRC,
        GREGORIAN_LEAP_YEAR_HOLDS_SRC,
        LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC,
        VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC,
        VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC,
        VERIFY_GREGORIAN_LEAP_YEAR_SRC,
        VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC,
        VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC,
        YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_HOLDS_SRC,
    };

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

    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
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
    creusot, EVERY_LEAP_YEAR_IS_DIVISIBLE_BY_FOUR_HOLDS_SRC, {
        /// Every leap year is divisible by four — a real consequence
        /// of [`gregorian_leap_year_holds`]'s own rule, not a
        /// restatement of it (the rule's `#[ensures(..)]` couldn't be
        /// spelled `result ==> ..` directly, since `result` isn't in
        /// scope for a bare `#[logic(open)]` predicate).
        #[logic(open)]
        pub fn every_leap_year_is_divisible_by_four_holds(year: i32, is_leap: bool) -> bool {
            pearlite! { is_leap ==> year@ % 4 == 0 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::every_leap_year_is_divisible_by_four_holds",
        "creusot",
        "ensures",
        || EVERY_LEAP_YEAR_IS_DIVISIBLE_BY_FOUR_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_GREGORIAN_LEAP_YEAR_SRC, {
        /// The exec leap rule satisfies the spec, and every leap year is divisible by four, for every `i32`.
        #[requires(true)]
        #[ensures(gregorian_leap_year_holds(year, result))]
        #[ensures(every_leap_year_is_divisible_by_four_holds(year, result))]
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
    creusot, YEAR_IS_NON_NEGATIVE_IN_THIS_MODEL_HOLDS_SRC, {
        /// This file's day-count/leap-year checks are defined only for
        /// non-negative years (no BCE support) — a real domain
        /// restriction, reused as the real `#[requires(..)]`
        /// precondition wherever a day-count check needs it.
        #[logic(open)]
        pub fn year_is_non_negative_in_this_model(year: i32) -> bool {
            pearlite! { year@ >= 0 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::time::year_is_non_negative_in_this_model",
        "creusot",
        "requires",
        || YEAR_IS_NON_NEGATIVE_IN_THIS_MODEL_HOLDS_SRC,
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
        #[requires(year_is_non_negative_in_this_model(year))]
        #[ensures(leap_year_has_three_hundred_sixty_six_calendar_days_holds(year, result))]
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
        #[requires(year_is_non_negative_in_this_model(year))]
        #[ensures(common_year_has_three_hundred_sixty_five_calendar_days_holds(year, result))]
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
        #[requires(year_is_non_negative_in_this_model(year))]
        #[ensures(year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds(year, result))]
        fn check_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days(year: i32) -> bool {
            let days: i32 = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
            days >= 365 && days <= 366
        }
    }
}
