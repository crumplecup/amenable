//! Kani proofs for `amenable_time`'s genuinely-checkable temporal
//! contracts (`AMENABLE_TIME_PLAN.md` Phase 6). Each atomic contract type
//! gets its own real `bool` predicate (`kani_ensures!`, Kani's own DFCC
//! representation), a `Witness<KaniVerifier>` citing the harness that
//! machine-checks it, and a `#[kani::proof]` harness over the whole input
//! domain. Structural contracts ("uses a hyphen separator") stay
//! `Standard`-only and never reach this module.

use amenable_core::Witness;
use amenable_time::CalendarMonthInRangeOneToTwelve;

use crate::rust_std::kani_ensures;
use crate::{CalculationProof, KaniVerifier};

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
    "amenable_time::CalendarMonthInRangeOneToTwelve::ensures",
    u8,
    |month| (1..=12).contains(&month)
);

amenable_derive::harness! {
    kani, VERIFY_CALENDAR_MONTH_IN_RANGE_SRC, {
        /// The `1..=12` month-range predicate agrees, over the whole `u8`
        /// domain, with the twelve-way enumeration of the legal calendar
        /// months.
        #[kani::proof]
        fn verify_calendar_month_in_range() {
            let month: u8 = kani::any();

            let arithmetic = <CalendarMonthInRangeOneToTwelve as ::amenable_core::Ensures<
                KaniVerifier,
            >>::ensures(month);

            let enumerated = month == 1
                || month == 2
                || month == 3
                || month == 4
                || month == 5
                || month == 6
                || month == 7
                || month == 8
                || month == 9
                || month == 10
                || month == 11
                || month == 12;

            assert_eq!(arithmetic, enumerated);
        }
    }
}
