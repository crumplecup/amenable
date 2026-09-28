//! Real Creusot proof content for `jiff::civil::Weekday`'s round-trip
//! property (`ext::jiff::civil_weekday` holds the `CreusotWitness`
//! bridge that references the `_SRC` constant this file's `harness!`
//! call emits) — the same claim
//! `amenable_kani::ext::jiff::civil_weekday`'s own doc comment checks
//! by symbolic execution.
//!
//! Reuses `civil_iso_week_date.rs`'s existing `extern_spec!` for
//! `Weekday::from_monday_one_offset`/`to_monday_one_offset` directly
//! (calling the real methods, not redeclaring their contracts) —
//! Creusot allows only one `extern_spec!` per real function
//! crate-wide, the same real finding `civil_era.rs`'s own doc comment
//! documents (see `reference_creusot_toolchain_findings` finding 8).
//! No opaque accessor needed here: the postcondition states the
//! round-trip directly in terms of the plain `i8` offset, never
//! needing to reference `Weekday`'s own opaque discriminant.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::Weekday>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn civil_weekday_monday_one_offset_round_trips(
            offset: i8,
            round_trip: Result<i8, ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got == offset,
                    Err(_) => offset < 1i8 || offset > 7i8,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_weekday::civil_weekday_monday_one_offset_round_trips",
        "creusot",
        "ensures",
        || CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_WEEKDAY_MONDAY_ONE_OFFSET_ROUND_TRIPS_SRC, {
        /// `Weekday::from_monday_one_offset(offset)`, whenever it
        /// succeeds, always returns a `Weekday` whose own
        /// `to_monday_one_offset()` is exactly `offset` back — the
        /// same claim `amenable_kani::ext::jiff::civil_weekday::
        /// verify_civil_weekday_monday_one_offset_round_trips` checks
        /// by symbolic execution, restated as a real Creusot
        /// postcondition resting on `civil_iso_week_date.rs`'s own
        /// `extern_spec!`.
        #[requires(true)]
        #[ensures(civil_weekday_monday_one_offset_round_trips(offset, result))]
        fn verify_civil_weekday_monday_one_offset_round_trips(offset: i8) -> Result<i8, ()> {
            match jiff::civil::Weekday::from_monday_one_offset(offset) {
                Ok(w) => Ok(w.to_monday_one_offset()),
                Err(_) => Err(()),
            }
        }
    }
}
