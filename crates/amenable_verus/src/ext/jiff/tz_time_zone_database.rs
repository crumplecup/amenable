//! Verus accommodation model for `jiff::tz::TimeZoneDatabase`'s
//! `none`/`is_definitively_empty` behavior.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! tz_time_zone_database`'s and `amenable_creusot::ext_jiff::
//! tz_time_zone_database`'s own harnesses for the identical claim,
//! checked directly against jiff's real API, independently confirm.
//!
//! Models `TimeZoneDatabase` scoped to `none()`/`is_definitively_
//! empty()` only — see `amenable_kani::ext::jiff::
//! tz_time_zone_database`'s own doc comment for why `get()`/
//! `bundled()`/etc. are out of scope.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::tz::TimeZoneDatabase`, scoped to `none()`.
pub struct TimeZoneDatabaseModel {
    /// Models `.is_definitively_empty()`.
    pub is_definitively_empty: bool,
}

impl TimeZoneDatabaseModel {
    /// Models `TimeZoneDatabase::none()`.
    pub fn none() -> (result: TimeZoneDatabaseModel)
        ensures
            result.is_definitively_empty,
    {
        TimeZoneDatabaseModel { is_definitively_empty: true }
    }
}

/// Exercises `none()` — the same claim `amenable_kani::ext::jiff::
/// tz_time_zone_database` and `amenable_creusot::ext_jiff::
/// tz_time_zone_database` both check against jiff's real API.
pub fn verify_tz_time_zone_database_none_is_definitively_empty_model() -> (result: bool)
    ensures
        result,
{
    TimeZoneDatabaseModel::none().is_definitively_empty
}

} // verus!
