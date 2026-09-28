//! Verus accommodation model for `jiff::fmt::strtime::Meridiem`'s
//! `From<jiff::civil::Time> for Meridiem` conversion.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_strtime_meridiem`'s own harness for the identical claim,
//! checked directly against jiff's real API, independently confirms.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::strtime::Meridiem`'s two real variants.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MeridiemModel {
    /// Models `jiff::fmt::strtime::Meridiem::AM`.
    Am,
    /// Models `jiff::fmt::strtime::Meridiem::PM`.
    Pm,
}

/// The documented threshold this model establishes: `AM` for `hour <
/// 12`, `PM` otherwise — the same claim `amenable_kani::ext::jiff::
/// fmt_strtime_meridiem::
/// verify_fmt_strtime_meridiem_from_time_matches_hour_threshold` and
/// `amenable_creusot::ext_jiff::fmt_strtime_meridiem::
/// verify_fmt_strtime_meridiem_from_time_matches_hour_threshold`
/// check against jiff's real API.
pub open spec fn meridiem_model_from_hour_holds(hour: i8, result: MeridiemModel) -> bool {
    if hour < 12 {
        result == MeridiemModel::Am
    } else {
        result == MeridiemModel::Pm
    }
}

/// `civil::Time`'s own real valid hour range (`0..=23`), the domain
/// this model's `Meridiem::from(time)` conversion is restricted to.
pub open spec fn meridiem_hour_in_range(hour: i8) -> bool {
    0 <= hour && hour <= 23
}

/// A model of `Meridiem::from(time)`, restricted to `civil::Time`'s
/// own real valid hour range (`0..=23`).
pub fn verify_fmt_strtime_meridiem_from_time_matches_hour_threshold_model(
    hour: i8,
) -> (result: MeridiemModel)
    requires
        meridiem_hour_in_range(hour),
    ensures
        meridiem_model_from_hour_holds(hour, result),
{
    if hour < 12 { MeridiemModel::Am } else { MeridiemModel::Pm }
}

} // verus!
