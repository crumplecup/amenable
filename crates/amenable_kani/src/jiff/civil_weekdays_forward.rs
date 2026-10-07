//! `KaniWitness` for `amenable_ext::
//! ExtStandard<jiff::civil::WeekdaysForward>` — a real, checked
//! periodicity property over jiff's actual public API
//! (`Weekday::cycle_forward`/`Iterator::next`), not a trusted stub.
//!
//! Unlike `TimestampSeries`/`DateSeries`/`DateTimeSeries`/
//! `TimeSeries` (all real Kani-uncheckable, sharing a `jiff::Error`
//! recursive-Arc Drop-glue wall — see `gallery::
//! jiff_error_drop_cost`'s own doc comment), `WeekdaysForward::next()`
//! is genuinely different, confirmed by reading jiff's real source
//! (`src/civil/weekday.rs`): it delegates to `jcore::civil::
//! WeekdaysForward::next()` and `Weekday::from_jcore` (a plain,
//! infallible `const fn` match) — no `Result`/`jiff::Error` anywhere
//! in the call chain at all, since cycling through 7 fixed weekdays
//! can never fail. So this is checked directly, not assumed trusted
//! by resemblance to the `*Series` family just because it's also an
//! "iterator over a jiff type."
//!
//! Real law checked (jiff's own documented example in `Weekday::
//! cycle_forward`'s doc comment): the first `next()` call yields the
//! starting weekday exactly, and the second yields its `next()`
//! (wrapping `Sunday -> Monday`).

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;

use crate::ext_macros::{ExtCheckedProof, kani_ensures_ext};
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<jiff::civil::WeekdaysForward> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "verify_civil_weekdays_forward_next_yields_start_then_its_successor".to_owned(),
            VERIFY_CIVIL_WEEKDAYS_FORWARD_NEXT_YIELDS_START_THEN_ITS_SUCCESSOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<jiff::civil::WeekdaysForward>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<jiff::civil::WeekdaysForward>",
        "kani",
        || <ExtStandard<jiff::civil::WeekdaysForward> as crate::KaniWitness>::proof().to_string(),
    )
}

/// All seven real `Weekday` variants, for exhaustive enumeration —
/// `Weekday` has no `kani::Arbitrary` derive of its own to draw from
/// directly (the same real constraint `civil_iso_week_date.rs`'s own
/// witness already works around).
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
    ExtStandard<jiff::civil::WeekdaysForward>,
    "amenable_ext::ExtStandard<jiff::civil::WeekdaysForward>",
    usize,
    |start_index| {
        if start_index >= ALL_WEEKDAYS.len() {
            true
        } else {
            let start = ALL_WEEKDAYS[start_index];
            let mut it = start.cycle_forward();
            it.next() == Some(start) && it.next() == Some(start.next())
        }
    }
);

amenable_derive::harness! {
    kani, VERIFY_CIVIL_WEEKDAYS_FORWARD_NEXT_YIELDS_START_THEN_ITS_SUCCESSOR_SRC, {
        /// `weekday.cycle_forward()`'s first `next()` call yields
        /// `weekday` exactly, and the second yields `weekday.next()`
        /// — checked exhaustively over all seven real `Weekday`
        /// variants, not an assumed slice of one.
        #[kani::proof]
        fn verify_civil_weekdays_forward_next_yields_start_then_its_successor() {
            let start_index: usize = kani::any();
            assert!(
                ExtStandard::<jiff::civil::WeekdaysForward>::ensures(start_index),
                "Weekday::cycle_forward()'s first two next() calls must yield the start weekday then its successor"
            );
        }
    }
}
