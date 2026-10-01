//! Calendar-month, weekday, and week-number range contracts.
//!
//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use crate::rust_std::{kani_ensures, kani_requires};
use crate::{CalculationProof, KaniVerifier};
use amenable_core::Witness;
use amenable_time::{
    CalendarMonthInRangeOneToTwelve, WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
};

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

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CalendarMonthInRangeOneToTwelve {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for WeekdayInRangeOneToSeven {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for WeekNumberInRangeOneToFiftyThree {}
