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
    use amenable_time::CalendarMonthInRangeOneToTwelve;

    use crate::CreusotVerifier;

    use super::{CALENDAR_MONTH_IN_RANGE_HOLDS_SRC, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC};

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
