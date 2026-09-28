//! Verus accommodation model for `jiff::tz::OffsetConflict::
//! resolve`'s `AlwaysOffset`/`AlwaysTimeZone` behavior — the same
//! scope `tz_offset_conflict.rs`'s own Creusot doc comment documents
//! (`PreferOffset`/`Reject` are out of scope, disproportionate to
//! add here).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! tz_offset_conflict`'s own harness for the identical claim, checked
//! directly against jiff's real API, independently confirms (Kani
//! can't check this claim at all — see `amenable_kani::ext::jiff`'s
//! own doc comment for the real `TimeZone` `Repr`-dispatch CBMC
//! wall, reached here via a direct call to the same
//! already-confirmed-timing-out function).
//!
//! Models the resolved offset seconds directly (no `AmbiguousZoned`/
//! `TimeZone`/`DateTime` payload at all): `AlwaysOffset` always
//! returns the claimed seconds, `AlwaysTimeZone` always returns the
//! actual (time-zone) seconds.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `resolve_always_offset_seconds`'s postcondition: always returns
/// the claimed offset.
pub open spec fn offset_conflict_always_offset_holds(claimed: i32, result: i32) -> bool {
    result == claimed
}

/// `resolve_always_time_zone_seconds`'s postcondition: always returns
/// the time zone's own (actual) offset.
pub open spec fn offset_conflict_always_time_zone_holds(actual: i32, result: i32) -> bool {
    result == actual
}

/// Models `OffsetConflict::AlwaysOffset.resolve(dt, claimed, tz)`'s
/// resolved offset seconds: always the claimed offset, ignoring
/// `tz`'s own.
pub fn resolve_always_offset_seconds(claimed: i32, actual: i32) -> (result: i32)
    ensures
        offset_conflict_always_offset_holds(claimed, result),
{
    let _ = actual;
    claimed
}

/// Models `OffsetConflict::AlwaysTimeZone.resolve(dt, claimed, tz)`'s
/// resolved offset seconds: always `tz`'s own (actual) offset,
/// ignoring the claimed one.
pub fn resolve_always_time_zone_seconds(claimed: i32, actual: i32) -> (result: i32)
    ensures
        offset_conflict_always_time_zone_holds(actual, result),
{
    let _ = claimed;
    actual
}

/// Exercises `resolve_always_offset_seconds`/`resolve_always_time_
/// zone_seconds` — the same claim `amenable_creusot::ext_jiff::
/// tz_offset_conflict` checks against jiff's real API.
pub fn verify_tz_offset_conflict_always_offset_and_always_time_zone_model(
    claimed: i32,
    actual: i32,
) -> (result: bool)
    ensures
        result,
{
    resolve_always_offset_seconds(claimed, actual) == claimed
        && resolve_always_time_zone_seconds(claimed, actual) == actual
}

} // verus!
