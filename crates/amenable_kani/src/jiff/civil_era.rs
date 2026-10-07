//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::civil::Era>` — a
//! real, checked property over jiff's actual public API
//! (`Date::era_year`), not a trusted stub.
//!
//! `Era` is a plain fieldless 2-variant enum (`BCE`/`CE`, no `Ord`
//! unlike `Unit`, only `PartialEq`) with no constructor of its own —
//! its only real production path is `Date::era_year() -> (i16, Era)`,
//! whose real behavior jiff documents precisely: `year >= 1` maps to
//! `(year, Era::CE)`; `year <= 0` maps to `(-year + 1, Era::BCE)`.
//! Checked directly against jiff's real source (`Date::era_year`'s
//! own implementation matches this exactly), a genuinely checkable,
//! non-tautological law — not assumed trusted just because `Era` has
//! no fields of its own.
//!
//! Uses the same always-valid day (`1`) as `civil_date.rs`'s own
//! witness, since `era_year` depends only on the year component.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::Era> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_era_year_classifies_bce_and_ce_correctly".to_owned(),
            VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::Era>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Era>",
        "kani",
        || <ExtStandard<jiff::civil::Era> as crate::KaniWitness>::proof().to_string(),
    )
}

/// jiff's own documented valid range for `civil::Date::new`'s year
/// (`-9999..=9999`) — the same constant `civil_date.rs`'s own witness
/// uses.
const DATE_YEAR_MIN: i16 = -9999;
const DATE_YEAR_MAX: i16 = 9999;

kani_ensures_ext!(
    ExtStandard<jiff::civil::Era>,
    "amenable_ext::ExtStandard<jiff::civil::Era>",
    i16,
    |year| {
        if !(DATE_YEAR_MIN..=DATE_YEAR_MAX).contains(&year) {
            true
        } else {
            let d = jiff::civil::Date::new(year, 1, 1).expect(
                "year is already checked to be in Date::new's valid range, month/day fixed at 1",
            );
            let (era_year, era) = d.era_year();
            if year >= 1 {
                era_year == year && era == jiff::civil::Era::CE
            } else {
                era_year == -year + 1 && era == jiff::civil::Era::BCE
            }
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_ERA_YEAR_CLASSIFIES_BCE_AND_CE_CORRECTLY_SRC, {
        /// `Date::new(year, 1, 1).era_year()` classifies `year >= 1`
        /// as `(year, Era::CE)` and `year <= 0` as `(-year + 1,
        /// Era::BCE)` — jiff's own documented law, checked over the
        /// full year range, not an assumed slice of one.
        #[kani::proof]
        fn verify_civil_era_year_classifies_bce_and_ce_correctly() {
            let year: i16 = kani::any();
            assert!(
                ExtStandard::<jiff::civil::Era>::ensures(year),
                "Date::era_year() must classify BCE/CE exactly as jiff documents"
            );
        }
    }
}
