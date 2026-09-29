//! Clock-face field bounds: hour, minute, second, and UTC-offset hour/minute.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(not(creusot))]
mod not_creusot_mirror {
    use amenable_time::{
        HourInRangeZeroToTwentyFour, MinuteInRangeZeroToFiftyNine, SecondInRangeZeroToSixty,
        UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetMinuteInRangeZeroToFiftyNine,
    };

    use crate::CreusotVerifier;

    use super::{
        HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_HOLDS_SRC, MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
        SECOND_IN_RANGE_ZERO_TO_SIXTY_HOLDS_SRC,
        UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_HOLDS_SRC,
        UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_HOLDS_SRC,
        VERIFY_HOUR_IN_RANGE_ZERO_TO_TWENTY_FOUR_SRC,
        VERIFY_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC, VERIFY_SECOND_IN_RANGE_ZERO_TO_SIXTY_SRC,
        VERIFY_UTC_OFFSET_HOUR_IN_RANGE_ZERO_TO_TWENTY_THREE_SRC,
        VERIFY_UTC_OFFSET_MINUTE_IN_RANGE_ZERO_TO_FIFTY_NINE_SRC,
    };

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

    // ClassifiedWitness: every checked leaf above closes over real Creusot
    // proof content (see its `support()` override).
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for HourInRangeZeroToTwentyFour {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for MinuteInRangeZeroToFiftyNine {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for SecondInRangeZeroToSixty {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for UtcOffsetHourInRangeZeroToTwentyThree {}
    impl amenable_core::ClassifiedWitness<CreusotVerifier> for UtcOffsetMinuteInRangeZeroToFiftyNine {}
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
