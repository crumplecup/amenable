//! Real Creusot proof content for `chrono::FixedOffset`'s round-trip property
//! (`chrono::fixed_offset` holds the `CreusotWitness` bridge that references the
//! `_SRC` constant this file's `harness!` call emits).
//!
//! `chrono::FixedOffset` has no Creusot-std or elicitation prior coverage. A real
//! `extern_spec!` needs a trusted logic accessor for the value `local_minus_utc()`
//! returns — an ordinary method call can't appear inside a Pearlite
//! `#[ensures(..)]`/`#[logic]` clause — so `east_opt`/`west_opt`'s postconditions
//! and `local_minus_utc`'s postcondition all reference the same opaque
//! `fixed_offset_local_minus_utc_value` axiom instead of calling each other. That
//! accessor lives in `ext_chrono::shared_trusted_accessors`, chrono's counterpart
//! to `ext_jiff`'s own module of the same name, for the same reason.

mod logic;
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<chrono::FixedOffset>` postcondition —
        /// real, callable Pearlite content, not just descriptive text alongside
        /// it. States chrono's documented valid range rather than reproducing its
        /// bounds-check arithmetic.
        #[logic(open)]
        fn fixed_offset_east_and_west_round_trip(
            secs: i32,
            east: bool,
            round_trip: Result<i32, ()>,
        ) -> bool {
            pearlite! {
                match round_trip {
                    Ok(got) => if east { got == secs } else { got == -secs },
                    Err(_) => secs <= -86_400i32 || secs >= 86_400i32,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::fixed_offset::fixed_offset_east_and_west_round_trip",
        "creusot",
        "ensures",
        || FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_FIXED_OFFSET_EAST_AND_WEST_ROUND_TRIP_SRC, {
        /// `FixedOffset::east_opt(secs)`/`west_opt(secs)`, whenever either
        /// succeeds, always returns a `FixedOffset` whose own `local_minus_utc()`
        /// reports back `secs` (east) or `-secs` (west) — the same claim
        /// `amenable_kani::chrono::fixed_offset::
        /// verify_fixed_offset_east_and_west_round_trip` checks by symbolic
        /// execution, restated as a real Creusot postcondition resting on the
        /// `extern_spec!` above.
        #[requires(true)]
        #[ensures(fixed_offset_east_and_west_round_trip(secs, east, result))]
        fn verify_fixed_offset_east_and_west_round_trip(secs: i32, east: bool) -> Result<i32, ()> {
            let constructed = if east {
                chrono::FixedOffset::east_opt(secs)
            } else {
                chrono::FixedOffset::west_opt(secs)
            };
            match constructed {
                Some(offset) => Ok(offset.local_minus_utc()),
                None => Err(()),
            }
        }
    }
}
