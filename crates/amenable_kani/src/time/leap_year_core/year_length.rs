//! Year-length contracts (365 vs 366 calendar days) built on the leap-year rule.
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
use amenable_core::{Ensures, Evidence, Standard, Witness};
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};
use amenable_time::{
    CommonYearHasThreeHundredSixtyFiveCalendarDays, LeapYearHasThreeHundredSixtySixCalendarDays,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
};

use super::rule::{days_in_year, is_gregorian_leap_year};

// ── LeapYearHasThreeHundredSixtySixCalendarDays ──────────────────

impl Witness<KaniVerifier> for LeapYearHasThreeHundredSixtySixCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_year_has_three_hundred_sixty_six_calendar_days".to_owned(),
            VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
        "kani",
        || <LeapYearHasThreeHundredSixtySixCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapYearHasThreeHundredSixtySixCalendarDays,
    "amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays",
    i32,
    |year| days_in_year(year) == 366
);

/// This file's day-count/leap-year models are defined only for
/// non-negative years (no BCE support) — a real domain restriction,
/// not a range predicate already named above. Genuinely trivial (its
/// own definition is the whole claim), so it gets no dedicated proof
/// of its own, the same way `AccountsDistinct`/`BalancedEntries` don't
/// — it's checked inline wherever a day-count harness needs it.
pub struct YearIsNonNegativeInThisModel;

impl Standard for YearIsNonNegativeInThisModel {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for YearIsNonNegativeInThisModel {
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

impl Witness<KaniVerifier> for YearIsNonNegativeInThisModel {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}
}

kani_requires!(
    YearIsNonNegativeInThisModel,
    "amenable_kani::YearIsNonNegativeInThisModel",
    i32,
    |year| year >= 0
);

// `requires()` is only ever called from inside `#[cfg(kani)]`-gated
// harness bodies, which a plain build never compiles -- this
// existence check (never run, just typechecked) is what keeps the
// type itself from looking dead outside a `--cfg kani` build, the
// same `let _ = ...;` idiom `amenable_ext`'s own trait-bound
// assertions already use.
const _: () = {
    let _ = <YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires;
};

/// `days_in_year(y) == 366` agrees, over every modeled year, with the
/// Gregorian leap-year rule directly, and a leap year is exactly a
/// common year plus one day — two related but distinct claims about
/// the same day-count model, not a restatement of `ensures()` itself.
pub struct LeapYearDayCountMatchesGregorianRule;

impl Standard for LeapYearDayCountMatchesGregorianRule {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for LeapYearDayCountMatchesGregorianRule {
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

impl Witness<KaniVerifier> for LeapYearDayCountMatchesGregorianRule {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_leap_year_has_three_hundred_sixty_six_calendar_days".to_owned(),
            VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::LeapYearDayCountMatchesGregorianRule",
        "kani",
        || <LeapYearDayCountMatchesGregorianRule as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    LeapYearDayCountMatchesGregorianRule,
    "amenable_kani::LeapYearDayCountMatchesGregorianRule",
    i32,
    |year| {
        let has_366 =
            <LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(year);
        has_366 == is_gregorian_leap_year(year)
            && days_in_year(year) - 365 == if is_gregorian_leap_year(year) { 1 } else { 0 }
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for LeapYearDayCountMatchesGregorianRule {}

amenable_derive::harness! {
    kani, VERIFY_LEAP_YEAR_HAS_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 — a leap year contains 366
        /// calendar days: `days_in_year(y) == 366` agrees with the
        /// Gregorian rule, and with "a leap year is a common year plus
        /// one day", over every modeled year, plus dated anchors.
        #[kani::proof]
        fn verify_leap_year_has_three_hundred_sixty_six_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(<YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires(year));

            assert!(<LeapYearDayCountMatchesGregorianRule as Ensures<KaniVerifier>>::ensures(year));

            assert!(<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(2000), "2000 has 366 days");
            assert!(<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(2024), "2024 has 366 days");
            assert!(!<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(2023), "2023 has 365 days");
            assert!(!<LeapYearHasThreeHundredSixtySixCalendarDays as Ensures<KaniVerifier>>::ensures(1900), "1900 has 365 days");
        }
    }
}
// ── CommonYearHasThreeHundredSixtyFiveCalendarDays ──────────────────

impl Witness<KaniVerifier> for CommonYearHasThreeHundredSixtyFiveCalendarDays {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_common_year_has_three_hundred_sixty_five_calendar_days".to_owned(),
            VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
        "kani",
        || <CommonYearHasThreeHundredSixtyFiveCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CommonYearHasThreeHundredSixtyFiveCalendarDays,
    "amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays",
    i32,
    |year| days_in_year(year) == 365
);

/// `days_in_year(y) == 365` agrees, over every modeled year, with
/// `!is_gregorian_leap_year(y)`, and 365/366 are the only two options
/// (they're distinct) — two related but distinct claims about the
/// day-count model, not a restatement of `ensures()` itself.
pub struct CommonYearDayCountMatchesGregorianRule;

impl Standard for CommonYearDayCountMatchesGregorianRule {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for CommonYearDayCountMatchesGregorianRule {
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

impl Witness<KaniVerifier> for CommonYearDayCountMatchesGregorianRule {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_common_year_has_three_hundred_sixty_five_calendar_days".to_owned(),
            VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CommonYearDayCountMatchesGregorianRule",
        "kani",
        || <CommonYearDayCountMatchesGregorianRule as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CommonYearDayCountMatchesGregorianRule,
    "amenable_kani::CommonYearDayCountMatchesGregorianRule",
    i32,
    |year| {
        let has_365 =
            <CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(
                year,
            );
        has_365 == !is_gregorian_leap_year(year)
            && (days_in_year(year) == 365) != (days_in_year(year) == 366)
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CommonYearDayCountMatchesGregorianRule {}

amenable_derive::harness! {
    kani, VERIFY_COMMON_YEAR_HAS_THREE_HUNDRED_SIXTY_FIVE_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 3.1.1.20 — a common year contains 365
        /// calendar days: `days_in_year(y) == 365` agrees with
        /// `!is_gregorian_leap_year(y)` over every modeled year, plus
        /// dated anchors.
        #[kani::proof]
        fn verify_common_year_has_three_hundred_sixty_five_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(<YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires(year));

            assert!(<CommonYearDayCountMatchesGregorianRule as Ensures<KaniVerifier>>::ensures(year));

            assert!(<CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(2023), "2023 has 365 days");
            assert!(<CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(1900), "1900 has 365 days");
            assert!(!<CommonYearHasThreeHundredSixtyFiveCalendarDays as Ensures<KaniVerifier>>::ensures(2000), "2000 has 366 days");
        }
    }
}
// ── YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays ──────────────────

impl Witness<KaniVerifier>
    for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
{
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days".to_owned(),
            VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
        "kani",
        || <YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    "amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays",
    i32,
    |year| (365..=366).contains(&days_in_year(year))
);

amenable_derive::harness! {
    kani, VERIFY_YEAR_DURATION_IN_RANGE_THREE_HUNDRED_SIXTY_FIVE_TO_THREE_HUNDRED_SIXTY_SIX_CALENDAR_DAYS_SRC, {
        /// ISO 8601-1:2019, 2.2.14 — a year's duration is 365 or 366
        /// calendar days: the model is well-formed over every modeled
        /// year.
        #[kani::proof]
        fn verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days() {
            let year: i32 = kani::any();
            kani::assume(<YearIsNonNegativeInThisModel as ::amenable_core::Requires<KaniVerifier>>::requires(year));

            assert!(<YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays as Ensures<
                KaniVerifier,
            >>::ensures(year));
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for LeapYearHasThreeHundredSixtySixCalendarDays
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CommonYearHasThreeHundredSixtyFiveCalendarDays
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays
{
}
