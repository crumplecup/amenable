//! Verus accommodation model for `chrono::FixedOffset::east_opt`/
//! `FixedOffset::west_opt`/`FixedOffset::local_minus_utc`.
//!
//! `chrono` has zero `vstd` coverage and Verus never resolves `Cargo.toml`
//! at all — unlike Kani/Creusot, there is no mechanism for Verus to reach
//! chrono's actual code, full stop. The established response (see
//! `jiff/offset.rs`'s own doc comment for the precedent this mirrors) is
//! a hand-verified Verus-native model reproducing the type's documented
//! behavior. This proof is conditional on that reproduction being
//! faithful — which `amenable_kani::chrono::fixed_offset`'s and
//! `amenable_creusot::ext_chrono::fixed_offset`'s own harnesses for the
//! identical claim, checked directly against chrono's real
//! `east_opt`/`west_opt`/`local_minus_utc`, independently confirm.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// chrono's own documented valid range for `FixedOffset::east_opt`/
/// `west_opt` (`-23:59:59..=23:59:59`, in seconds) — the same constant
/// the Kani and Creusot proofs for this type independently confirm.
pub open spec fn fixed_offset_seconds_in_range(secs: int) -> bool {
    secs > -86400 && secs < 86400
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `secs` back out (unchanged for
/// `east`, negated for `west`); out of range, it reports none,
/// mirroring `east_opt`/`west_opt`'s real `Option::None` case.
pub open spec fn fixed_offset_east_and_west_model_round_trip_holds(
    secs: i32,
    east: bool,
    result: Option<i32>,
) -> bool {
    if fixed_offset_seconds_in_range(secs as int) {
        if east {
            result == Some(secs)
        } else {
            result == Some((-secs) as i32)
        }
    } else {
        result is None
    }
}

/// A model of `(if east { FixedOffset::east_opt(secs) } else {
/// FixedOffset::west_opt(secs) }).map(|o| o.local_minus_utc())`: in
/// range, the value round-trips (sign-flipped for `west`); out of
/// range, `None` — the same claim `amenable_kani::chrono::
/// fixed_offset::verify_fixed_offset_east_and_west_round_trip` and
/// `amenable_creusot::ext_chrono::fixed_offset::
/// verify_fixed_offset_east_and_west_round_trip` check against
/// chrono's real API.
pub fn verify_fixed_offset_east_and_west_round_trips_model(secs: i32, east: bool) -> (result: Option<i32>)
    ensures
        fixed_offset_east_and_west_model_round_trip_holds(secs, east, result),
{
    if secs > -86400 && secs < 86400 {
        if east { Some(secs) } else { Some(-secs) }
    } else {
        None
    }
}

} // verus!
