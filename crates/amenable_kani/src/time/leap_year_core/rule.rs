//! The Gregorian leap-year rule itself (divisible-by-4, centennial exception) and the shared day-counting helpers.
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
use amenable_time::{
    CentennialYearDivisibleByOneHundred,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
};

/// The Gregorian leap-year rule (ISO 8601-1:2019, 3.1.1.21 note 1),
/// shared by the year-length and month/day-bound harnesses.
#[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
pub(crate) fn is_gregorian_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// A Gregorian calendar year is 366 days when a leap year, else 365.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub(crate) fn days_in_year(year: i32) -> i32 {
    if is_gregorian_leap_year(year) {
        366
    } else {
        365
    }
}

/// The number of calendar days in month `m` of year `y` (m in 1..=12;
/// months outside that range yield 0).
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub(crate) fn days_in_month(year: i32, month: u8) -> u8 {
    if month == 2 {
        if is_gregorian_leap_year(year) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else if matches!(month, 1 | 3 | 5 | 7 | 8 | 10 | 12) {
        31
    } else {
        0
    }
}

/// Whether `day` is a valid day-of-month for month `m` of year `y`.
#[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
pub(crate) fn is_valid_calendar_day(year: i32, month: u8, day: u8) -> bool {
    (1..=days_in_month(year, month)).contains(&day)
}

// ── GregorianLeapYearUsesDivisibleByFourAndFourHundredException ──────────────────

impl Witness<KaniVerifier> for GregorianLeapYearUsesDivisibleByFourAndFourHundredException {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_gregorian_leap_year".to_owned(),
            VERIFY_GREGORIAN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
        "kani",
        || <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    "amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException",
    i32,
    |year| year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
);

/// The `y%4 && (y%100 || y%400)` divisibility rule agrees, over every
/// `i32`, with ISO 8601-1:2019, 3.1.1.21 note 1's own stated form: a
/// centennial year needs the /400 rule, every other year only the /4
/// rule. A genuinely different formulation of the same law, not a
/// restatement of it — worth naming and checking in its own right.
pub struct GregorianLeapYearRuleMatchesCentennialCaseSplit;

impl Standard for GregorianLeapYearRuleMatchesCentennialCaseSplit {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for GregorianLeapYearRuleMatchesCentennialCaseSplit {
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

impl Witness<KaniVerifier> for GregorianLeapYearRuleMatchesCentennialCaseSplit {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_gregorian_leap_year".to_owned(),
            VERIFY_GREGORIAN_LEAP_YEAR_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::GregorianLeapYearRuleMatchesCentennialCaseSplit",
        "kani",
        || <GregorianLeapYearRuleMatchesCentennialCaseSplit as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    GregorianLeapYearRuleMatchesCentennialCaseSplit,
    "amenable_kani::GregorianLeapYearRuleMatchesCentennialCaseSplit",
    i32,
    |year| {
        let leap = <GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<
            KaniVerifier,
        >>::ensures(year);
        let case_split = if year % 100 == 0 {
            year % 400 == 0
        } else {
            year % 4 == 0
        };
        leap == case_split
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for GregorianLeapYearRuleMatchesCentennialCaseSplit
{
}

amenable_derive::harness! {
    kani, VERIFY_GREGORIAN_LEAP_YEAR_SRC, {
        /// ISO 8601-1:2019, 3.1.1.21 note 1 — a year is a leap year if
        /// divisible by 4, except a centennial year is a leap year only
        /// if also divisible by 400: the divisibility rule agrees with
        /// the standard's own case-split over every `i32`, plus six
        /// dated anchors (the classic off-by-a-century bugs).
        #[kani::proof]
        fn verify_gregorian_leap_year() {
            let year: i32 = kani::any();
            assert!(<GregorianLeapYearRuleMatchesCentennialCaseSplit as Ensures<
                KaniVerifier,
            >>::ensures(year));

            assert!(<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2000), "2000 is a leap year");
            assert!(<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(1600), "1600 is a leap year");
            assert!(<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2024), "2024 is a leap year");
            assert!(!<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(1900), "1900 is not a leap year");
            assert!(!<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2100), "2100 is not a leap year");
            assert!(!<GregorianLeapYearUsesDivisibleByFourAndFourHundredException as Ensures<KaniVerifier>>::ensures(2023), "2023 is not a leap year");
        }
    }
}
// ── CentennialYearDivisibleByOneHundred ──────────────────

impl Witness<KaniVerifier> for CentennialYearDivisibleByOneHundred {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_centennial_year_divisible_by_one_hundred".to_owned(),
            VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CentennialYearDivisibleByOneHundred",
        "kani",
        || <CentennialYearDivisibleByOneHundred as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CentennialYearDivisibleByOneHundred,
    "amenable_time::CentennialYearDivisibleByOneHundred",
    i32,
    |year| year % 100 == 0
);

/// `y % 100 == 0` agrees, over every `i32`, with `y%4 == 0 && y%25 ==
/// 0` (100 = 4·25, coprime factors) — a genuinely different
/// factorization of the same divisibility claim, not a restatement.
pub struct CentennialDivisibilityMatchesCoprimeFactorization;

impl Standard for CentennialDivisibilityMatchesCoprimeFactorization {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl Evidence for CentennialDivisibilityMatchesCoprimeFactorization {
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

impl Witness<KaniVerifier> for CentennialDivisibilityMatchesCoprimeFactorization {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_centennial_year_divisible_by_one_hundred".to_owned(),
            VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CentennialDivisibilityMatchesCoprimeFactorization",
        "kani",
        || <CentennialDivisibilityMatchesCoprimeFactorization as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CentennialDivisibilityMatchesCoprimeFactorization,
    "amenable_kani::CentennialDivisibilityMatchesCoprimeFactorization",
    i32,
    |year| {
        let centennial =
            <CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(year);
        centennial == (year % 4 == 0 && year % 25 == 0)
    }
);

impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CentennialDivisibilityMatchesCoprimeFactorization
{
}

amenable_derive::harness! {
    kani, VERIFY_CENTENNIAL_YEAR_DIVISIBLE_BY_ONE_HUNDRED_SRC, {
        /// ISO 8601-1:2019, 3.1.1.22 — a centennial year is one whose
        /// year number is an exact multiple of 100: `y % 100 == 0`
        /// agrees with its coprime factorization over every `i32`,
        /// plus dated anchors.
        #[kani::proof]
        fn verify_centennial_year_divisible_by_one_hundred() {
            let year: i32 = kani::any();
            assert!(<CentennialDivisibilityMatchesCoprimeFactorization as Ensures<
                KaniVerifier,
            >>::ensures(year));

            assert!(<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(0), "year 0 is centennial");
            assert!(<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(1900), "1900 is centennial");
            assert!(<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(2000), "2000 is centennial");
            assert!(!<CentennialYearDivisibleByOneHundred as Ensures<KaniVerifier>>::ensures(2024), "2024 is not centennial");
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for GregorianLeapYearUsesDivisibleByFourAndFourHundredException
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CentennialYearDivisibleByOneHundred {}
