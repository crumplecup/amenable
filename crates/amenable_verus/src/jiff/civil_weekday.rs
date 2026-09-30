//! Verus accommodation model for `jiff::civil::Weekday::
//! from_monday_one_offset`/`to_monday_one_offset`.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! civil_weekday`'s and `amenable_creusot::ext_jiff::civil_weekday`'s
//! own harnesses for the identical claim, checked directly against
//! jiff's real API, independently confirm.
//!
//! Unlike `unit.rs`'s own `UnitModel` enum (built to state a real
//! ordering law needing variant identity), this model states the
//! round-trip law purely in terms of the plain `i8` offset — no
//! `Weekday`-shaped enum needed at all, since the claim never
//! inspects which day of the week it is, only that the offset
//! survives the round trip.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own documented valid range for `Weekday::
/// from_monday_one_offset` (`1..=7`) — the same constant the Kani and
/// Creusot proofs for this type independently confirm.
pub open spec fn weekday_monday_one_offset_in_range(offset: int) -> bool {
    offset >= 1 && offset <= 7
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `offset` back out exactly;
/// out of range, it reports none, mirroring `Weekday::
/// from_monday_one_offset`'s real `Result::Err` case.
pub open spec fn civil_weekday_monday_one_offset_model_round_trip_holds(
    offset: i8,
    result: Option<i8>,
) -> bool {
    if weekday_monday_one_offset_in_range(offset as int) {
        result == Some(offset)
    } else {
        result is None
    }
}

/// A model of `Weekday::from_monday_one_offset(offset).ok().map(|w|
/// w.to_monday_one_offset())`: in range, the value round-trips; out
/// of range, `None` — the same claim `amenable_kani::ext::jiff::
/// civil_weekday::verify_civil_weekday_monday_one_offset_round_trips`
/// and `amenable_creusot::ext_jiff::civil_weekday::
/// verify_civil_weekday_monday_one_offset_round_trips` check against
/// jiff's real API.
pub fn verify_civil_weekday_monday_one_offset_round_trips_model(offset: i8) -> (result: Option<
    i8,
>)
    ensures
        civil_weekday_monday_one_offset_model_round_trip_holds(offset, result),
{
    if (1..=7).contains(&offset) { Some(offset) } else { None }
}

} // verus!
