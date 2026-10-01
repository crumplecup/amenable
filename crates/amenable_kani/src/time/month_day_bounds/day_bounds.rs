//! Within-month calendar-day bounds and the leap-day-only-in-leap-year contract.
//!
//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use crate::rust_std::kani_ensures;
use crate::{CalculationProof, KaniVerifier};
use amenable_core::{Ensures, Evidence, Standard, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};
use amenable_time::{CalendarDayWithinMonthBounds, LeapDayOccursOnlyInLeapYear};

// Only referenced inside the `#[kani::proof]` body `amenable_derive::harness!`
// gates behind `#[cfg(kani)]` below -- plain `cargo check` can't see that
// usage, so gate the import to match rather than leaving it "unused".
#[cfg(kani)]
use amenable_time::CalendarMonthInRangeOneToTwelve;

use super::super::leap_year_core::{is_gregorian_leap_year, is_valid_calendar_day};

// ── CalendarDayWithinMonthBounds ──────────────────

impl Witness<KaniVerifier> for CalendarDayWithinMonthBounds {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_day_within_month_bounds".to_owned(),
            VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CalendarDayWithinMonthBounds",
        "kani",
        || <CalendarDayWithinMonthBounds as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarDayWithinMonthBounds,
    "amenable_time::CalendarDayWithinMonthBounds",
    (i32, u8, u8),
    |(year, month, day)| is_valid_calendar_day(year, month, day)
);

/// A valid calendar day always lies in `1..=31`, and February 29 is
/// valid exactly in a leap year — two real characterizations of
/// [`CalendarDayWithinMonthBounds`]'s own claim, not restatements of
/// it.
pub struct CalendarDayWithinMonthBoundsCharacterization;

impl Standard for CalendarDayWithinMonthBoundsCharacterization {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for CalendarDayWithinMonthBoundsCharacterization {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for CalendarDayWithinMonthBoundsCharacterization {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_day_within_month_bounds".to_owned(),
            VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CalendarDayWithinMonthBoundsCharacterization",
        "kani",
        || <CalendarDayWithinMonthBoundsCharacterization as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarDayWithinMonthBoundsCharacterization,
    "amenable_kani::CalendarDayWithinMonthBoundsCharacterization",
    (i32, u8, u8),
    |(year, month, day)| {
        let valid =
            <CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((year, month, day));
        (!valid || (1..=31).contains(&day))
            && (!(month == 2 && day == 29) || valid == is_gregorian_leap_year(year))
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CalendarDayWithinMonthBoundsCharacterization
{
}

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_DAY_WITHIN_MONTH_BOUNDS_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.2.1 — a calendar-date day
        /// component is within the valid day count for that month and
        /// year: a valid day lies in `1..=31`, and February 29 is valid
        /// exactly in a leap year, over every `i32` year and month
        /// `1..=12`; plus dated anchors.
        #[kani::proof]
        fn verify_calendar_day_within_month_bounds() {
            let year: i32 = kani::any();
            let month: u8 = kani::any();
            let day: u8 = kani::any();
            kani::assume(<CalendarMonthInRangeOneToTwelve as ::amenable_core::Requires<KaniVerifier>>::requires(month));

            assert!(<CalendarDayWithinMonthBoundsCharacterization as Ensures<
                KaniVerifier,
            >>::ensures((year, month, day)));

            assert!(<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2020, 2, 29)), "2020-02-29 is a valid date");
            assert!(!<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 2, 29)), "2021-02-29 is not");
            assert!(!<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 4, 31)), "April has 30 days");
            assert!(<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 1, 31)), "January has 31 days");
            assert!(!<CalendarDayWithinMonthBounds as Ensures<KaniVerifier>>::ensures((2021, 1, 0)), "day 0 is invalid");
        }
    }
}
// ── LeapDayOccursOnlyInLeapYear ──────────────────

impl Witness<KaniVerifier> for LeapDayOccursOnlyInLeapYear {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_day_occurs_only_in_leap_year".to_owned(),
            VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::LeapDayOccursOnlyInLeapYear",
        "kani",
        || <LeapDayOccursOnlyInLeapYear as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapDayOccursOnlyInLeapYear,
    "amenable_time::LeapDayOccursOnlyInLeapYear",
    (i32, u8, u8),
    |(year, month, day)| !(month == 2 && day == 29) || is_gregorian_leap_year(year)
);

/// February 29 as a full calendar date is valid exactly when the
/// year is a leap year — a real biconditional over `is_valid_
/// calendar_day`, strictly stronger than [`LeapDayOccursOnlyInLeapYear`]'s
/// own one-directional "not valid unless leap" claim (which never
/// asserts Feb 29 *is* valid in a leap year).
pub struct LeapDayValidityMatchesGregorianRule;

impl Standard for LeapDayValidityMatchesGregorianRule {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for LeapDayValidityMatchesGregorianRule {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl Witness<KaniVerifier> for LeapDayValidityMatchesGregorianRule {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_day_occurs_only_in_leap_year".to_owned(),
            VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::LeapDayValidityMatchesGregorianRule",
        "kani",
        || <LeapDayValidityMatchesGregorianRule as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapDayValidityMatchesGregorianRule,
    "amenable_kani::LeapDayValidityMatchesGregorianRule",
    i32,
    |year| is_valid_calendar_day(year, 2, 29) == is_gregorian_leap_year(year)
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for LeapDayValidityMatchesGregorianRule {}

amenable_derive::harness! {
    kani, VERIFY_LEAP_DAY_OCCURS_ONLY_IN_LEAP_YEAR_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — the 29th of February is a
        /// valid calendar date only when the year is a leap year:
        /// checked as a full biconditional over every `i32` year, plus
        /// dated anchors on the weaker one-directional claim.
        #[kani::proof]
        fn verify_leap_day_occurs_only_in_leap_year() {
            let year: i32 = kani::any();
            assert!(<LeapDayValidityMatchesGregorianRule as Ensures<KaniVerifier>>::ensures(year));

            assert!(!<LeapDayOccursOnlyInLeapYear as Ensures<KaniVerifier>>::ensures((2023, 2, 29)), "2023 is not a leap year");
            assert!(<LeapDayOccursOnlyInLeapYear as Ensures<KaniVerifier>>::ensures((2024, 2, 29)), "2024 is a leap year");
            assert!(<LeapDayOccursOnlyInLeapYear as Ensures<KaniVerifier>>::ensures((2021, 4, 31)), "not Feb 29, trivially satisfied");
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CalendarDayWithinMonthBounds {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for LeapDayOccursOnlyInLeapYear {}
