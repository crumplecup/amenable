//! Verus accommodation model for `jiff::Unit`'s documented ordering
//! law.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani, there is no mechanism for Verus
//! to reach jiff's actual code, full stop. The established response
//! (see `offset.rs`'s own doc comment for the precedent) is a
//! hand-verified Verus-native model reproducing the type's documented
//! behavior. This proof is conditional on that reproduction being
//! faithful — which `amenable_kani::ext::jiff::unit`'s own harness for
//! the identical claim, checked directly against jiff's real
//! `Ord`/`PartialOrd` impl by exhaustive enumeration, independently
//! confirms. `amenable_creusot::ext::jiff`'s own doc comment documents
//! why this claim is Creusot-uncheckable specifically (`jiff::Unit`
//! lacks `creusot_std::model::DeepModel`, the same class of constraint
//! already documented there for `Reverse<T>: OrdLogic`) — the same
//! real finding that makes an accommodation model the honest choice
//! here, not merely the default one.
//!
//! Models the ten real variants by their real discriminant values
//! (`Year = 9` down to `Nanosecond = 0`, confirmed against jiff's real
//! source, `src/span.rs`) directly as an `i8`, since Verus has no
//! reason to reproduce jiff's own enum layout — only the ordering law
//! itself.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of one of `Unit`'s ten real variants, by its real
/// discriminant value.
#[derive(Clone, Copy)]
pub enum UnitModel {
    /// Models `jiff::Unit::Year` (discriminant 9).
    Year,
    /// Models `jiff::Unit::Month` (discriminant 8).
    Month,
    /// Models `jiff::Unit::Week` (discriminant 7).
    Week,
    /// Models `jiff::Unit::Day` (discriminant 6).
    Day,
    /// Models `jiff::Unit::Hour` (discriminant 5).
    Hour,
    /// Models `jiff::Unit::Minute` (discriminant 4).
    Minute,
    /// Models `jiff::Unit::Second` (discriminant 3).
    Second,
    /// Models `jiff::Unit::Millisecond` (discriminant 2).
    Millisecond,
    /// Models `jiff::Unit::Microsecond` (discriminant 1).
    Microsecond,
    /// Models `jiff::Unit::Nanosecond` (discriminant 0).
    Nanosecond,
}

/// The real discriminant value jiff assigns each variant — the same
/// values `amenable_kani::ext::jiff::unit`'s own harness confirms
/// against jiff's real source.
pub open spec fn unit_model_discriminant(u: UnitModel) -> int {
    match u {
        UnitModel::Year => 9,
        UnitModel::Month => 8,
        UnitModel::Week => 7,
        UnitModel::Day => 6,
        UnitModel::Hour => 5,
        UnitModel::Minute => 4,
        UnitModel::Second => 3,
        UnitModel::Millisecond => 2,
        UnitModel::Microsecond => 1,
        UnitModel::Nanosecond => 0,
    }
}

/// The ordering law this model establishes: comparing two units by
/// their real discriminant value reproduces the documented "bigger
/// units compare greater" law exactly.
pub open spec fn unit_model_ordering_holds(a: UnitModel, b: UnitModel, result: i8) -> bool {
    let da = unit_model_discriminant(a);
    let db = unit_model_discriminant(b);
    (result == 1 ==> da > db) && (result == 0 ==> da == db) && (result == -1 ==> da < db) && (
    result == 1 || result == 0 || result == -1)
}

/// A model of `a.cmp(&b)` for two `Unit` values, collapsed to `1`
/// (greater), `0` (equal), or `-1` (less) — the same claim
/// `amenable_kani::ext::jiff::unit::
/// verify_unit_ordering_matches_discriminant_order` checks against
/// jiff's real API by exhaustive enumeration.
pub fn verify_unit_ordering_matches_discriminant_order_model(
    a: UnitModel,
    b: UnitModel,
) -> (result: i8)
    ensures
        unit_model_ordering_holds(a, b, result),
{
    let da = unit_model_discriminant_exec(a);
    let db = unit_model_discriminant_exec(b);
    if da > db {
        1
    } else if da == db {
        0
    } else {
        -1
    }
}

/// Executable mirror of `unit_model_discriminant`, needed because spec
/// functions cannot be called from executable code directly.
fn unit_model_discriminant_exec(u: UnitModel) -> (result: i8)
    ensures
        result as int == unit_model_discriminant(u),
{
    match u {
        UnitModel::Year => 9,
        UnitModel::Month => 8,
        UnitModel::Week => 7,
        UnitModel::Day => 6,
        UnitModel::Hour => 5,
        UnitModel::Minute => 4,
        UnitModel::Second => 3,
        UnitModel::Millisecond => 2,
        UnitModel::Microsecond => 1,
        UnitModel::Nanosecond => 0,
    }
}

} // verus!
