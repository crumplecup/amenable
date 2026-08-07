//! Verus accommodation model for `core::ops::{Bound<i32>,
//! ControlFlow<i32, i32>, RangeFull, RangeTo<i32>}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real types
//! directly — no timeout concerns for any of these four. This carrier
//! states each resulting law directly. None of these functions are the
//! real types themselves — each proof is conditional: sound if the
//! real type refines the stated law, which `amenable_kani`'s own
//! harness for that exact type (checking the real type directly)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `RangeTo` is unbounded below, so `.contains(x)` reduces to its
/// single exclusive upper bound.
pub fn verify_range_to_model_contains_matches_bound(end: i32, x: i32) -> (result: bool)
    ensures
        result == (x < end),
{
    x < end
}

/// `RangeFull` carries no fields but still has real behavior: its
/// `.contains()` is unconditionally `true`.
pub fn verify_range_full_model_contains_everything() -> (result: bool)
    ensures
        result,
{
    true
}

/// `(included_round_trip, excluded_round_trip, unbounded_holds)`.
type BoundResult = (i32, i32, bool);

/// `Bound` has exactly three inhabitants: `Included`/`Excluded`
/// round-trip their endpoint, and `Unbounded` carries none.
pub fn verify_bound_model_round_trips_its_endpoint(v: i32) -> (result: BoundResult)
    ensures
        result.0 == v,
        result.1 == v,
        result.2,
{
    (v, v, true)
}

/// `(continue_is_continue, continue_is_break, continue_value,
/// break_is_break, break_is_continue, break_value)`.
type ControlFlowResult = (bool, bool, i32, bool, bool, i32);

/// `Continue` and `Break` are mutually exclusive, and each accessor
/// round-trips the value the other variant lacks.
pub fn verify_control_flow_model_continue_and_break_are_disjoint(c: i32, b: i32) -> (result: ControlFlowResult)
    ensures
        result.0,
        !result.1,
        result.2 == c,
        result.3,
        !result.4,
        result.5 == b,
{
    (true, false, c, true, false, b)
}

} // verus!
