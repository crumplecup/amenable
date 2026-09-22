//! Verus accommodation model for `jiff::tz::TimeZoneOffsetInfo`'s
//! `TimeZone::to_offset_info` behavior for a `TimeZone::fixed(offset)`.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! tz_time_zone_offset_info`'s own harness for the identical claim,
//! checked directly against jiff's real API, independently confirms
//! (Kani can't check this claim at all — see
//! `amenable_kani::ext::jiff`'s own doc comment for the real,
//! confirmed `TimeZone` `Repr`-dispatch CBMC wall).
//!
//! Models `TimeZoneOffsetInfo` scoped to the `TimeZone::fixed` case
//! only, the same NARROWER scope `tz_time_zone_offset_info.rs`'s own
//! Creusot doc comment documents: no `Timestamp` payload modeled at
//! all (out of scope, since it's irrelevant for a fixed-offset time
//! zone), just the resulting offset seconds and the fact that DST is
//! always inactive.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::tz::TimeZoneOffsetInfo`, scoped to what
/// `TimeZone::fixed(offset).to_offset_info(ts)` needs (see this
/// module's own doc comment for why `ts` is out of scope).
pub struct TimeZoneOffsetInfoModel {
    /// Models `.offset()`'s own seconds.
    pub offset_seconds: i32,
}

impl TimeZoneOffsetInfoModel {
    /// Models `TimeZone::fixed(offset).to_offset_info(ts)`.
    pub fn from_fixed_offset_seconds(secs: i32) -> (result: TimeZoneOffsetInfoModel)
        ensures
            result.offset_seconds == secs,
    {
        TimeZoneOffsetInfoModel { offset_seconds: secs }
    }

    /// Models `.dst()`: always `Dst::No` for this scope.
    pub fn is_dst(&self) -> (result: bool)
        ensures
            !result,
    {
        false
    }
}

/// Exercises `from_fixed_offset_seconds`/`is_dst` — the same claim
/// `amenable_creusot::ext_jiff::tz_time_zone_offset_info` checks
/// against jiff's real API.
pub fn verify_tz_time_zone_offset_info_from_fixed_time_zone_model(
    secs: i32,
) -> (result: bool)
    ensures
        result,
{
    let info = TimeZoneOffsetInfoModel::from_fixed_offset_seconds(secs);
    info.offset_seconds == secs && !info.is_dst()
}

} // verus!
