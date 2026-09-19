//! Real Creusot proof content for `jiff::civil::WeekdaysForward`'s
//! periodicity property (`ext::jiff::civil_weekdays_forward` holds
//! the `CreusotWitness` bridge) — the same claim
//! `amenable_verus::ext::jiff::civil_weekdays_forward`'s
//! hand-verified model checks, and the same claim
//! `amenable_kani::ext::jiff::civil_weekdays_forward`'s own harness
//! checks directly against jiff's real API by symbolic execution
//! (unlike the `*Series` family, `WeekdaysForward` has NO `jiff::
//! Error` Drop-glue wall at all — see that module's own doc comment).
//!
//! Accommodation model, not a real `extern_spec!` against jiff's
//! actual `Iterator for WeekdaysForward` impl, matching this crate's
//! own established precedent for iterator types lacking real
//! contract coverage (see `timestamp_series.rs`'s own doc comment).
//! Modeled purely in terms of the plain `i8` Monday-one offset
//! (`civil_iso_week_date.rs`'s own numbering), the same choice
//! `civil_weekday.rs`'s own witness makes: the claim never needs
//! `Weekday`'s own variant identity, only that the offset advances
//! by exactly one, wrapping `7 -> 1`.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, CIVIL_WEEKDAYS_FORWARD_NEXT_YIELDS_START_THEN_ITS_SUCCESSOR_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::WeekdaysForward>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn civil_weekdays_forward_next_yields_start_then_its_successor_holds(
            start_offset: i8,
            observed: (i8, i8),
        ) -> bool {
            pearlite! {
                observed.0 == start_offset
                    && observed.1 == if start_offset == 7i8 { 1i8 } else { start_offset + 1i8 }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_WEEKDAYS_FORWARD_NEXT_YIELDS_START_THEN_ITS_SUCCESSOR_SRC, {
        /// `weekday.cycle_forward()`'s first `next()` call yields
        /// `weekday` exactly, and the second yields `weekday.next()`
        /// (wrapping `Sunday -> Monday`, i.e. offset `7 -> 1`) — the
        /// same claim `amenable_kani::ext::jiff::
        /// civil_weekdays_forward::
        /// verify_civil_weekdays_forward_next_yields_start_then_its_successor`
        /// checks by symbolic execution against jiff's real API,
        /// restated here as an accommodation model since Creusot has
        /// no real contract coverage for jiff's `Iterator` impl.
        #[requires(start_offset > 0i8 && start_offset < 8i8)]
        #[ensures(civil_weekdays_forward_next_yields_start_then_its_successor_holds(start_offset, result))]
        fn verify_civil_weekdays_forward_next_yields_start_then_its_successor(
            start_offset: i8,
        ) -> (i8, i8) {
            let next_offset = if start_offset == 7 { 1 } else { start_offset + 1 };
            (start_offset, next_offset)
        }
    }
}
