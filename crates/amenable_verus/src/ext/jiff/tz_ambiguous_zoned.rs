//! Verus accommodation model for `jiff::tz::AmbiguousZoned`'s
//! `TimeZone::to_ambiguous_zoned` behavior for a `TimeZone::
//! fixed(offset)` — the same shape `tz_ambiguous_timestamp.rs`
//! already established for the sibling type.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! tz_ambiguous_zoned`'s own harness for the identical claim, checked
//! directly against jiff's real API, independently confirms (Kani
//! can't check this claim at all — see `amenable_kani::ext::jiff`'s
//! own doc comment for the real, confirmed `TimeZone` `Repr`-dispatch
//! CBMC wall, reached here via a direct call to the same already-
//! confirmed-timing-out function).
//!
//! Models `AmbiguousZoned` scoped to the `TimeZone::fixed` case only,
//! the same NARROWER scope `tz_ambiguous_zoned.rs`'s own Creusot doc
//! comment documents: no `DateTime`/`dt` or `TimeZone`/`time_zone()`
//! payload modeled at all.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::tz::AmbiguousZoned`, scoped to what
/// `TimeZone::fixed(offset).to_ambiguous_zoned(dt)` needs (see this
/// module's own doc comment for why `dt`/`time_zone()` are out of
/// scope).
pub struct AmbiguousZonedModel {
    /// Models `.offset()`'s `AmbiguousOffset::Unambiguous { offset }`
    /// seconds — always populated in this model, since `TimeZone::
    /// fixed` never produces `Gap`/`Fold`.
    pub offset_seconds: i32,
}

impl AmbiguousZonedModel {
    /// Models `TimeZone::fixed(offset).to_ambiguous_zoned(dt)`.
    pub fn from_fixed_offset_seconds(secs: i32) -> (result: AmbiguousZonedModel)
        ensures
            result.offset_seconds == secs,
    {
        AmbiguousZonedModel { offset_seconds: secs }
    }

    /// Models `.is_ambiguous()`: always `false` for this scope.
    pub fn is_ambiguous(&self) -> (result: bool)
        ensures
            !result,
    {
        false
    }
}

/// Exercises `from_fixed_offset_seconds`/`is_ambiguous` — the same
/// claim `amenable_creusot::ext_jiff::tz_ambiguous_zoned` checks
/// against jiff's real API.
pub fn verify_tz_ambiguous_zoned_from_fixed_time_zone_is_always_unambiguous_model(
    secs: i32,
) -> (result: bool)
    ensures
        result,
{
    let zdt = AmbiguousZonedModel::from_fixed_offset_seconds(secs);
    zdt.offset_seconds == secs && !zdt.is_ambiguous()
}

} // verus!
