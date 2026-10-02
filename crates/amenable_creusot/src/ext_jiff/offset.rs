//! Real Creusot proof content for `jiff::tz::Offset`'s round-trip
//! property (`ext::jiff::offset` holds the `CreusotWitness` bridge that
//! references the `_SRC` constant this file's `harness!` call emits).
//!
//! `jiff::tz::Offset` is uncontracted everywhere — not `creusot-std`
//! (checked: no third-party-crate coverage at all, only std itself),
//! not `elicitation` (checked: no Creusot prior art for jiff). A real
//! `extern_spec!` needs a trusted logic accessor for the value `Offset::
//! seconds()` returns, the same shape `rust_std::mem_carrier`'s
//! `manually_drop_value` axiom uses for `ManuallyDrop<T>`'s wrapped
//! value: an ordinary method call (`.seconds()`) can't appear inside an
//! `#[ensures(..)]`/`#[logic]` clause (confirmed real toolchain
//! restriction — see this workspace's own Creusot findings), so
//! `from_seconds`'s postcondition and `seconds`'s postcondition both
//! reference the same opaque `offset_seconds_value` axiom instead of
//! calling each other.
//!
//! `offset_seconds_value` lives in `ext_jiff::shared_trusted_
//! accessors` (see that module's own doc comment for the full list
//! of cross-file reusers) — Creusot only allows one `extern_spec!`
//! per real function crate-wide, so `Offset::seconds()`'s own
//! contract can't be redeclared at any of those call sites.

mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, OFFSET_FROM_SECONDS_ROUND_TRIPS_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::tz::Offset>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it. Sufficient, not exhaustive on
        /// the error side (states jiff's documented valid range rather
        /// than reproducing its bounds-check arithmetic).
        #[logic(open)]
        fn offset_from_seconds_round_trips(seconds: i32, round_trip: Result<i32, ()>) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => got == seconds,
                    Err(_) => seconds < -93_599i32 || seconds > 93_599i32,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::offset::offset_from_seconds_round_trips",
        "creusot",
        "ensures",
        || OFFSET_FROM_SECONDS_ROUND_TRIPS_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_OFFSET_FROM_SECONDS_ROUND_TRIPS_SRC, {
        /// `Offset::from_seconds(secs)`, whenever it succeeds, always
        /// returns an `Offset` whose own `seconds()` is exactly `secs`
        /// back — the same claim
        /// `amenable_kani::ext::jiff::offset::verify_offset_from_seconds_round_trips`
        /// checks by symbolic execution, restated as a real Creusot
        /// postcondition resting on the `extern_spec!` above.
        #[requires(true)]
        #[ensures(offset_from_seconds_round_trips(seconds, result))]
        fn verify_offset_from_seconds_round_trips(seconds: i32) -> Result<i32, ()> {
            match jiff::tz::Offset::from_seconds(seconds) {
                Ok(offset) => Ok(offset.seconds()),
                Err(_) => Err(()),
            }
        }
    }
}
