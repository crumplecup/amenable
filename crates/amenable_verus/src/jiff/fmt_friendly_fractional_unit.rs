//! Verus accommodation model for `jiff::fmt::friendly::
//! FractionalUnit`'s `From<FractionalUnit> for Unit` conversion.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_friendly_fractional_unit`'s own harness for the identical
//! claim, checked directly against jiff's real API, independently
//! confirms.
//!
//! Reuses `unit.rs`'s existing `UnitModel` directly, rather than
//! modeling `Unit`'s ten variants a second time — the same "don't
//! re-model what's already modeled" discipline `span_fieldwise.rs`'s
//! own reuse of `span.rs`'s `SpanUnitFields` follows.

use super::unit::UnitModel;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of one of `FractionalUnit`'s 5 documented variants.
#[derive(Clone, Copy)]
pub enum FractionalUnitModel {
    /// Models `jiff::fmt::friendly::FractionalUnit::Hour`.
    Hour,
    /// Models `jiff::fmt::friendly::FractionalUnit::Minute`.
    Minute,
    /// Models `jiff::fmt::friendly::FractionalUnit::Second`.
    Second,
    /// Models `jiff::fmt::friendly::FractionalUnit::Millisecond`.
    Millisecond,
    /// Models `jiff::fmt::friendly::FractionalUnit::Microsecond`.
    Microsecond,
}

/// The documented per-variant mapping this model establishes: each
/// `FractionalUnitModel` variant converts to the exactly corresponding
/// `UnitModel` variant, the same claim `amenable_kani::ext::jiff::
/// fmt_friendly_fractional_unit::
/// verify_fmt_friendly_fractional_unit_from_matches_documented_mapping`
/// and `amenable_creusot::ext_jiff::fmt_friendly_fractional_unit::
/// verify_fmt_friendly_fractional_unit_from_matches_documented_mapping`
/// check against jiff's real API.
pub open spec fn fractional_unit_model_conversion_holds(
    f: FractionalUnitModel,
    result: UnitModel,
) -> bool {
    match (f, result) {
        (FractionalUnitModel::Hour, UnitModel::Hour) => true,
        (FractionalUnitModel::Minute, UnitModel::Minute) => true,
        (FractionalUnitModel::Second, UnitModel::Second) => true,
        (FractionalUnitModel::Millisecond, UnitModel::Millisecond) => true,
        (FractionalUnitModel::Microsecond, UnitModel::Microsecond) => true,
        _ => false,
    }
}

/// A model of `jiff::fmt::friendly::FractionalUnit`'s real
/// `From<FractionalUnit> for Unit` conversion.
pub fn verify_fmt_friendly_fractional_unit_from_matches_documented_mapping_model(
    f: FractionalUnitModel,
) -> (result: UnitModel)
    ensures
        fractional_unit_model_conversion_holds(f, result),
{
    match f {
        FractionalUnitModel::Hour => UnitModel::Hour,
        FractionalUnitModel::Minute => UnitModel::Minute,
        FractionalUnitModel::Second => UnitModel::Second,
        FractionalUnitModel::Millisecond => UnitModel::Millisecond,
        FractionalUnitModel::Microsecond => UnitModel::Microsecond,
    }
}

} // verus!
