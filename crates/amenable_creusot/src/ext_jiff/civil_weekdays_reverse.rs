//! Real Creusot proof content for `jiff::civil::WeekdaysReverse`'s
//! periodicity property (`ext::jiff::civil_weekdays_reverse` holds
//! the `CreusotWitness` bridge) — the same claim
//! `amenable_verus::jiff::civil_weekdays_reverse`'s
//! hand-verified model checks, and the same claim
//! `amenable_kani::ext::jiff::civil_weekdays_reverse`'s own harness
//! checks directly against jiff's real API by symbolic execution (no
//! `jiff::Error` Drop-glue wall here either, the same real reason
//! `WeekdaysForward` is checkable — see that module's own doc
//! comment).
//!
//! Accommodation model, not a real `extern_spec!` against jiff's
//! actual `Iterator for WeekdaysReverse` impl, matching this crate's
//! own established precedent for iterator types lacking real
//! contract coverage (see `timestamp_series.rs`'s own doc comment).
//! Modeled purely in terms of the plain `i8` Monday-one offset,
//! matching `civil_weekdays_forward.rs`'s own choice.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use super::civil_weekdays_forward::weekday_offset_in_monday_one_range;
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::WeekdaysReverse>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds(
            start_offset: i8,
            observed: (i8, i8),
        ) -> bool {
            pearlite! {
                observed.0 == start_offset
                    && observed.1 == if start_offset == 1i8 { 7i8 } else { start_offset - 1i8 }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_weekdays_reverse::civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds",
        "creusot",
        "ensures",
        || CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_WEEKDAYS_REVERSE_NEXT_YIELDS_START_THEN_ITS_PREDECESSOR_SRC, {
        /// `weekday.cycle_reverse()`'s first `next()` call yields
        /// `weekday` exactly, and the second yields
        /// `weekday.previous()` (wrapping `Monday -> Sunday`, i.e.
        /// offset `1 -> 7`) — the same claim `amenable_kani::
        /// ext::jiff::civil_weekdays_reverse::
        /// verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor`
        /// checks by symbolic execution against jiff's real API,
        /// restated here as an accommodation model since Creusot has
        /// no real contract coverage for jiff's `Iterator` impl.
        #[requires(weekday_offset_in_monday_one_range(start_offset))]
        #[ensures(civil_weekdays_reverse_next_yields_start_then_its_predecessor_holds(start_offset, result))]
        fn verify_civil_weekdays_reverse_next_yields_start_then_its_predecessor(
            start_offset: i8,
        ) -> (i8, i8) {
            let previous_offset = if start_offset == 1 { 7 } else { start_offset - 1 };
            (start_offset, previous_offset)
        }
    }
}
