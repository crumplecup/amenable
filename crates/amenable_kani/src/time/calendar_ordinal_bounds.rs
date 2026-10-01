//! Calendar-year, century, decade, and ordinal-day range contracts.
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
use amenable_core::Witness;
use amenable_time::{
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CenturyOrdinalInRangeZeroToNinetyNine, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    OrdinalDayInRangeOneToThreeHundredSixtySix,
};

// ── CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine ──────────────────────

impl Witness<KaniVerifier> for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine"
                .to_owned(),
            VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC
                .to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
        "kani",
        || <CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    "amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine",
    u16,
    |year| year <= 9999
);

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_YEAR_IN_RANGE_ZERO_TO_NINE_THOUSAND_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// ISO/WD 8601-1:2016(E), 4.1.2.1 — a non-expanded calendar
        /// year is 0000 through 9999. The canonical predicate holds at
        /// the boundary and fails just past it.
        #[kani::proof]
        fn verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine() {
            assert!(<CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(9999));
            assert!(!<CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(10000));
        }
    }
}
// ── CenturyOrdinalInRangeZeroToNinetyNine ──────────────────────

impl Witness<KaniVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_century_ordinal_in_range_zero_to_ninety_nine".to_owned(),
            VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
        "kani",
        || <CenturyOrdinalInRangeZeroToNinetyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    CenturyOrdinalInRangeZeroToNinetyNine,
    "amenable_time::CenturyOrdinalInRangeZeroToNinetyNine",
    u8,
    |ordinal| ordinal <= 99
);

amenable_derive::harness! {
    kani, VERIFY_CENTURY_ORDINAL_IN_RANGE_ZERO_TO_NINETY_NINE_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.12 — a Gregorian century
        /// ordinal is 00 through 99. The canonical predicate holds at
        /// the boundary and fails just past it.
        #[kani::proof]
        fn verify_century_ordinal_in_range_zero_to_ninety_nine() {
            assert!(<CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(99));
            assert!(!<CenturyOrdinalInRangeZeroToNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(100));
        }
    }
}
// ── DecadeOrdinalInRangeZeroToNineHundredNinetyNine ──────────────────────

impl Witness<KaniVerifier> for DecadeOrdinalInRangeZeroToNineHundredNinetyNine {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine".to_owned(),
            VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
        "kani",
        || <DecadeOrdinalInRangeZeroToNineHundredNinetyNine as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    "amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine",
    u16,
    |ordinal| ordinal <= 999
);

amenable_derive::harness! {
    kani, VERIFY_DECADE_ORDINAL_IN_RANGE_ZERO_TO_NINE_HUNDRED_NINETY_NINE_SRC, {
        /// ISO 8601-1:2019/Amd 1:2022, 4.3.11 — a Gregorian decade
        /// ordinal is 000 through 999. The canonical predicate holds at
        /// the boundary and fails just past it.
        #[kani::proof]
        fn verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine() {
            assert!(<DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(<DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(999));
            assert!(!<DecadeOrdinalInRangeZeroToNineHundredNinetyNine as ::amenable_core::Ensures<KaniVerifier>>::ensures(1000));
        }
    }
}
// ── OrdinalDayInRangeOneToThreeHundredSixtySix ──────────────────────

impl Witness<KaniVerifier> for OrdinalDayInRangeOneToThreeHundredSixtySix {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> ::amenable_core::WitnessSupportSummary {
        ::amenable_core::WitnessSupportSummary::checked_leaf()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "time::verify_ordinal_day_in_range_one_to_three_hundred_sixty_six".to_owned(),
            VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC.to_owned(),
        )
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
        "kani",
        || <OrdinalDayInRangeOneToThreeHundredSixtySix as Witness<KaniVerifier>>::proof().to_string(),
    )
}

kani_ensures!(
    OrdinalDayInRangeOneToThreeHundredSixtySix,
    "amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix",
    u16,
    |day| (1..=366).contains(&day)
);

amenable_derive::harness! {
    kani, VERIFY_ORDINAL_DAY_IN_RANGE_ONE_TO_THREE_HUNDRED_SIXTY_SIX_SRC, {
        /// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.3.1 — an ordinal
        /// day-of-year is 001 through 365, or 366 in a leap year. The
        /// canonical predicate holds at both boundaries and fails just
        /// outside each one.
        #[kani::proof]
        fn verify_ordinal_day_in_range_one_to_three_hundred_sixty_six() {
            assert!(<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(1));
            assert!(<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(366));
            assert!(!<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(0));
            assert!(!<OrdinalDayInRangeOneToThreeHundredSixtySix as ::amenable_core::Ensures<KaniVerifier>>::ensures(367));
        }
    }
}

// ── ClassifiedWitness: every checked leaf above closes over real,
// machine-checked Kani proof content (see its `support()` override).
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier> for CenturyOrdinalInRangeZeroToNinetyNine {}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for DecadeOrdinalInRangeZeroToNineHundredNinetyNine
{
}
impl ::amenable_core::ClassifiedWitness<KaniVerifier>
    for OrdinalDayInRangeOneToThreeHundredSixtySix
{
}
