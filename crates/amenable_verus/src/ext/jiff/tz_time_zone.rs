//! Verus accommodation model for `jiff::tz::TimeZone`'s own
//! `unknown`/`is_unknown`/`to_fixed_offset` behavior.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! tz_time_zone`'s own harness for the identical claim, checked
//! directly against jiff's real API, independently confirms (Kani
//! can't check this claim at all — see `amenable_kani::ext::jiff`'s
//! own doc comment for the real `TimeZone` `Repr`-dispatch CBMC
//! wall).
//!
//! Models `TimeZone` as a plain struct (`{ is_unknown: bool,
//! fixed_offset_seconds: i32 }`), scoped to `unknown`/`fixed`-
//! constructed values only, matching every sibling `TimeZone`-
//! touching model in this directory.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::tz::TimeZone`, scoped to `unknown`/`fixed`.
pub struct TimeZoneModel {
    /// Models `.is_unknown()`.
    pub is_unknown: bool,
    /// Models `.to_fixed_offset()`'s own seconds, meaningful only
    /// when `is_unknown` is `false`.
    pub fixed_offset_seconds: i32,
}

/// `unknown`'s postcondition: always the unknown variant.
pub open spec fn time_zone_unknown_holds(result: TimeZoneModel) -> bool {
    result.is_unknown
}

/// `fixed`'s postcondition: never unknown, and the fixed offset
/// round-trips exactly.
pub open spec fn time_zone_fixed_holds(offset_seconds: i32, result: TimeZoneModel) -> bool {
    !result.is_unknown && result.fixed_offset_seconds == offset_seconds
}

impl TimeZoneModel {
    /// Models `TimeZone::unknown()`.
    pub fn unknown() -> (result: TimeZoneModel)
        ensures
            time_zone_unknown_holds(result),
    {
        TimeZoneModel { is_unknown: true, fixed_offset_seconds: 0 }
    }

    /// Models `TimeZone::fixed(offset)`.
    pub fn fixed(offset_seconds: i32) -> (result: TimeZoneModel)
        ensures
            time_zone_fixed_holds(offset_seconds, result),
    {
        TimeZoneModel { is_unknown: false, fixed_offset_seconds: offset_seconds }
    }
}

/// Exercises `unknown`/`fixed` — the same claim `amenable_creusot::
/// ext_jiff::tz_time_zone` checks against jiff's real API.
pub fn verify_tz_time_zone_unknown_and_fixed_round_trip_model(seconds: i32) -> (result: bool)
    ensures
        result,
{
    let unknown_ok = TimeZoneModel::unknown().is_unknown;

    let tz = TimeZoneModel::fixed(seconds);
    let not_unknown_ok = !tz.is_unknown;
    let fixed_offset_ok = tz.fixed_offset_seconds == seconds;

    unknown_ok && not_unknown_ok && fixed_offset_ok
}

} // verus!
