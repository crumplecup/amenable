//! Calendar-field bounds: ordinal day, and century/decade/year ordinals.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_time::{
        CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
        CenturyOrdinalInRangeZeroToNinetyNine, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
        OrdinalDayInRangeOneToThreeHundredSixtySix,
    };

    use crate::CreusotVerifier;

    use super::{
        CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
        CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_HOLDS_SRC,
        DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_HOLDS_SRC,
        ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_HOLDS_SRC,
        VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC,
        VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC,
        VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC,
        VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC,
    };

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

    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
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
