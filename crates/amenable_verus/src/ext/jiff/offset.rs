//! Verus accommodation model for `jiff::tz::Offset::from_seconds`/
//! `Offset::seconds`.
//!
//! `jiff` has zero `vstd` coverage (a third-party crate, not `std`) and
//! Verus never resolves `Cargo.toml` at all — unlike Kani/Creusot, there
//! is no mechanism (real or `extern_spec`-style trusted axiom) for
//! Verus to reach jiff's actual code, full stop. The established
//! response to that gap in this codebase is the same one every other
//! zero-`vstd`-coverage carrier uses (see `rust_std::iter::
//! iter_transform_carrier`'s own doc comment for the precedent): a
//! hand-verified Verus-native model reproducing the type's documented
//! behavior, not a `#[verifier::external]`/`assume_specification`
//! trust-only axiom. This proof is conditional on that reproduction
//! being faithful — which `amenable_kani::ext::jiff::offset`'s and
//! `amenable_creusot::ext_jiff::offset`'s own harnesses for the
//! identical claim, checked directly against jiff's real
//! `Offset::from_seconds`/`Offset::seconds`, independently confirm.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own documented valid range for `Offset::from_seconds`
/// (`-25:59:59..=25:59:59`, in seconds) — the same constant the Kani
/// and Creusot proofs for this type independently confirm.
pub open spec fn offset_seconds_in_range(secs: int) -> bool {
    secs >= -93599 && secs <= 93599
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `secs` back out exactly;
/// out of range, it reports none, mirroring `Offset::from_seconds`'s
/// real `Result::Err` case.
pub open spec fn offset_from_seconds_model_round_trip_holds(secs: i32, result: Option<i32>) -> bool {
    if offset_seconds_in_range(secs as int) {
        result == Some(secs)
    } else {
        result is None
    }
}

/// A model of `Offset::from_seconds(secs).ok().map(|o| o.seconds())`:
/// in range, the value round-trips; out of range, `None` — the same
/// claim `amenable_kani::ext::jiff::offset::
/// verify_offset_from_seconds_round_trips` and `amenable_creusot::
/// ext_jiff::offset::verify_offset_from_seconds_round_trips` check
/// against jiff's real API.
pub fn verify_offset_from_seconds_model_round_trips(secs: i32) -> (result: Option<i32>)
    ensures
        offset_from_seconds_model_round_trip_holds(secs, result),
{
    if (-93599..=93599).contains(&secs) { Some(secs) } else { None }
}

} // verus!
