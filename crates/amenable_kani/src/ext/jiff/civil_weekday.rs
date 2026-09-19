//! `KaniWitness` for `amenable_ext::ExtStandard<jiff::civil::Weekday>`
//! — a real, checked round-trip property over jiff's actual public
//! API (`Weekday::from_monday_one_offset`/`to_monday_one_offset`),
//! not a trusted stub.
//!
//! `Weekday` is a plain 7-variant enum, but unlike `RoundMode` (no
//! public methods beyond derives) it has real, checked public
//! production/inspection methods — checked directly against jiff's
//! real source, not assumed trusted just because it's fieldless (the
//! same lesson `Era` already established: fieldless doesn't mean
//! nothing to check). `from_monday_one_offset`/`to_monday_one_offset`
//! is the same round-trip law already extern-spec'd once for
//! `ISOWeekDate`'s own witness (`civil_iso_week_date.rs`), restated
//! here as `Weekday`'s own primary witness.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::Weekday> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_weekday_monday_one_offset_round_trips".to_owned(),
            VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::Weekday>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::Weekday>",
        "kani",
        || <ExtStandard<jiff::civil::Weekday> as crate::KaniWitness>::proof().to_string(),
    )
}

/// jiff's own documented valid range for `Weekday::
/// from_monday_one_offset` (`1..=7`).
const WEEKDAY_MONDAY_ONE_OFFSET_MIN: i8 = 1;
const WEEKDAY_MONDAY_ONE_OFFSET_MAX: i8 = 7;

kani_ensures_ext!(
    ExtStandard<jiff::civil::Weekday>,
    "amenable_ext::ExtStandard<jiff::civil::Weekday>",
    i8,
    |offset| {
        if !(WEEKDAY_MONDAY_ONE_OFFSET_MIN..=WEEKDAY_MONDAY_ONE_OFFSET_MAX).contains(&offset) {
            true
        } else {
            let w = jiff::civil::Weekday::from_monday_one_offset(offset)
                .expect("offset is already checked to be in from_monday_one_offset's valid range");
            w.to_monday_one_offset() == offset
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC, {
        /// `Weekday::from_monday_one_offset(offset)`, whenever it
        /// succeeds, always returns a `Weekday` whose own
        /// `to_monday_one_offset()` is exactly `offset` back —
        /// checked over the full documented valid range `1..=7`, not
        /// an assumed slice of one.
        #[kani::proof]
        fn verify_civil_weekday_monday_one_offset_round_trips() {
            let offset: i8 = kani::any();
            assert!(
                ExtStandard::<jiff::civil::Weekday>::ensures(offset),
                "Weekday::from_monday_one_offset(offset).to_monday_one_offset() must equal offset whenever construction succeeds"
            );
        }
    }
}
