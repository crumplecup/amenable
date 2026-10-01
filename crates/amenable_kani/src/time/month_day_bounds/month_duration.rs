//! The month-duration (28-31 calendar days) range contract.
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
use amenable_core::{Evidence, Standard, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};
use amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays;

// `Ensures`/`CalendarMonthInRangeOneToTwelve` are only referenced inside
// the `#[kani::proof]` body `amenable_derive::harness!` gates behind
// `#[cfg(kani)]` below -- plain `cargo check` can't see that usage, so
// gate the imports to match rather than leaving them "unused".
#[cfg(kani)]
use amenable_core::Ensures;
#[cfg(kani)]
use amenable_time::CalendarMonthInRangeOneToTwelve;

use super::super::leap_year_core::{days_in_month, days_in_year, is_gregorian_leap_year};

// ── MonthDurationInRangeTwentyEightToThirtyOneCalendarDays ──────────────────

impl Witness<KaniVerifier> for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days"
                .to_owned(),
            VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
        "kani",
        || <MonthDurationInRangeTwentyEightToThirtyOneCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    "amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays",
    (i32, u8),
    |(year, month)| (28..=31).contains(&days_in_month(year, month))
);

/// `days_in_month(y, m)` is 31 exactly for the long months, 30 exactly
/// for the short months, and (28 or 29, distinguished by leap year)
/// for February — a real characterization of which month gets which
/// length, not a restatement of the `28..=31` range itself.
pub struct MonthDurationCharacterizedByMonthAndLeapYear;

impl Standard for MonthDurationCharacterizedByMonthAndLeapYear {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for MonthDurationCharacterizedByMonthAndLeapYear {
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

impl Witness<KaniVerifier> for MonthDurationCharacterizedByMonthAndLeapYear {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days"
                .to_owned(),
            VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::MonthDurationCharacterizedByMonthAndLeapYear",
        "kani",
        || <MonthDurationCharacterizedByMonthAndLeapYear as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MonthDurationCharacterizedByMonthAndLeapYear,
    "amenable_kani::MonthDurationCharacterizedByMonthAndLeapYear",
    (i32, u8),
    |(year, month)| {
        let dim = days_in_month(year, month);
        (dim == 31) == matches!(month, 1 | 3 | 5 | 7 | 8 | 10 | 12)
            && (dim == 30) == matches!(month, 4 | 6 | 9 | 11)
            && (dim < 30) == (month == 2)
            && (dim == 29) == (month == 2 && is_gregorian_leap_year(year))
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for MonthDurationCharacterizedByMonthAndLeapYear
{
}

/// The twelve months' durations sum to the year's own total day
/// count — a distinct aggregate claim from any single month's
/// duration.
pub struct MonthDurationsSumToYearDuration;

impl Standard for MonthDurationsSumToYearDuration {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for MonthDurationsSumToYearDuration {
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

impl Witness<KaniVerifier> for MonthDurationsSumToYearDuration {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days"
                .to_owned(),
            VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::MonthDurationsSumToYearDuration",
        "kani",
        || <MonthDurationsSumToYearDuration as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    MonthDurationsSumToYearDuration,
    "amenable_kani::MonthDurationsSumToYearDuration",
    i32,
    |year| {
        let total = days_in_month(year, 1) as i32
            + days_in_month(year, 2) as i32
            + days_in_month(year, 3) as i32
            + days_in_month(year, 4) as i32
            + days_in_month(year, 5) as i32
            + days_in_month(year, 6) as i32
            + days_in_month(year, 7) as i32
            + days_in_month(year, 8) as i32
            + days_in_month(year, 9) as i32
            + days_in_month(year, 10) as i32
            + days_in_month(year, 11) as i32
            + days_in_month(year, 12) as i32;
        total == days_in_year(year)
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for MonthDurationsSumToYearDuration {}

amenable_derive::harness! {
    kani, VERIFY_MONTH_DURATION_IN_RANGE_TWENTY_EIGHT_TO_THIRTY_ONE_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 2.2.12 — a month's duration is 28, 29, 30,
        /// or 31 calendar days according to the month and year: each
        /// length is characterized exactly by month and leap year, and
        /// the twelve months sum to the year's own total.
        #[kani::proof]
        fn verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days() {
            let year: i32 = kani::any();
            let month: u8 = kani::any();
            kani::assume(<CalendarMonthInRangeOneToTwelve as ::amenable_core::Requires<KaniVerifier>>::requires(month));

            assert!(<MonthDurationInRangeTwentyEightToThirtyOneCalendarDays as Ensures<
                KaniVerifier,
            >>::ensures((year, month)));
            assert!(<MonthDurationCharacterizedByMonthAndLeapYear as Ensures<
                KaniVerifier,
            >>::ensures((year, month)));
            assert!(<MonthDurationsSumToYearDuration as Ensures<KaniVerifier>>::ensures(year));
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for MonthDurationInRangeTwentyEightToThirtyOneCalendarDays
{
}
