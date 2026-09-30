//! `KaniWitness` for `amenable_ext::
//! ExtStandard<jiff::civil::WeekdaysReverse>` — a real, checked
//! periodicity property over jiff's actual public API
//! (`Weekday::cycle_reverse`/`Iterator::next`), not a trusted stub.
//!
//! Confirmed via jiff's real source (`src/civil/weekday.rs`), not
//! assumed from `WeekdaysForward`'s own confirmed shape:
//! `WeekdaysReverse::next()` has the identical infallible
//! `jcore`-delegation shape — `self.it.next().map(Weekday::
//! from_jcore)`, no `Result`/`jiff::Error` anywhere — so the same
//! real reason `WeekdaysForward` is Kani-checkable applies here too,
//! re-verified rather than assumed.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use super::macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::WeekdaysReverse> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor".to_owned(),
            VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::WeekdaysReverse>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::WeekdaysReverse>",
        "kani",
        || <ExtStandard<jiff::civil::WeekdaysReverse> as crate::KaniWitness>::proof().to_string(),
    )
}

/// All seven real `Weekday` variants, for exhaustive enumeration —
/// the same constant `civil_weekdays_forward.rs`'s own witness uses.
const ALL_WEEKDAYS: [jiff::civil::Weekday; 7] = [
    jiff::civil::Weekday::Monday,
    jiff::civil::Weekday::Tuesday,
    jiff::civil::Weekday::Wednesday,
    jiff::civil::Weekday::Thursday,
    jiff::civil::Weekday::Friday,
    jiff::civil::Weekday::Saturday,
    jiff::civil::Weekday::Sunday,
];

kani_ensures_ext!(
    ExtStandard<jiff::civil::WeekdaysReverse>,
    "amenable_ext::ExtStandard<jiff::civil::WeekdaysReverse>",
    usize,
    |start_index| {
        if start_index >= ALL_WEEKDAYS.len() {
            true
        } else {
            let start = ALL_WEEKDAYS[start_index];
            let mut it = start.cycle_reverse();
            it.next() == Some(start) && it.next() == Some(start.previous())
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC, {
        /// `weekday.cycle_reverse()`'s first `next()` call yields
        /// `weekday` exactly, and the second yields
        /// `weekday.previous()` — checked exhaustively over all
        /// seven real `Weekday` variants, not an assumed slice of
        /// one.
        #[kani::proof]
        fn verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor() {
            let start_index: usize = kani::any();
            assert!(
                ExtStandard::<jiff::civil::WeekdaysReverse>::ensures(start_index),
                "Weekday::cycle_reverse()'s first two next() calls must yield the start weekday then its predecessor"
            );
        }
    }
}
