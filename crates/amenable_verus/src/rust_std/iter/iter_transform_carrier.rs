//! Verus accommodation model for the single-item transform adapters
//! `core::iter::{Map, Filter, FilterMap, MapWhile, Cloned, Copied}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Each of these six real types checks a "the adapter's
//! `.next()` matches its transformation applied directly to the
//! underlying item" claim in `amenable_kani`, over a single symbolic
//! item — this carrier states each one directly as the same closure
//! applied to the same symbolic input, not `.next()` calls against a
//! modeled iterator state machine (unlike the ordered/unordered-pair
//! carriers, there's no multi-step position to track here: every one of
//! these six claims is fully determined by a single input value).
//! None of these functions are `Map`/`Filter`/`FilterMap`/`MapWhile`/
//! `Cloned`/`Copied` themselves — each proof is conditional: sound if
//! the real adapter refines the stated transformation, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! adapter directly) already confirms independently, for the identical
//! claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

// The single-increment-headroom precondition `amenable_std::
// IncrementHeadroom` names -- a real spec fn defined once in
// `iter_sequence_carrier`, called from here directly rather than
// restated inline. `#[cfg(verus_keep_ghost)]`-gated like every other
// spec-only import in this crate (see `chars_carrier.rs`): plain
// `spec fn`s carry no runtime representation, so this import only
// resolves when Verus's own ghost content is retained, not under
// ordinary `cargo check`.
#[cfg(verus_keep_ghost)]
use crate::rust_std::iter::single_increment_headroom_holds;
#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::values_are_equal;

verus! {

/// `Map::next` applies its closure to the underlying item — modeled
/// here as `add_one`, the exact closure `amenable_kani`'s own
/// `verify_map_applies_its_closure_to_each_item` harness uses.
pub fn verify_map_model_applies_its_closure_to_each_item(x: i32) -> (result: (i32, i32))
    requires
        single_increment_headroom_holds(x),
    ensures
        values_are_equal(result.0, result.1),
{
    let mapped = x + 1;
    let expected = x + 1;
    (mapped, expected)
}

/// `Filter`'s predicate and `FilterMap`'s closure are two different
/// real mechanisms landing on the identical law over this shared
/// symbolic item: "is nonzero" (`Filter`'s own predicate) and "`None`
/// for `0`, `Some(x)` otherwise" (`FilterMap`'s own closure) are the
/// same function, so both `verify_filter_model_...`/`verify_filter_
/// map_model_...` below state it once by calling this instead of
/// restating it twice.
pub open spec fn nonzero_item_survives_filtering(x: i32, result: Option<i32>) -> bool {
    (x != 0 ==> result == Some(x)) && (x == 0 ==> result is None)
}

/// `Filter::next` yields the item only when the predicate holds —
/// modeled here as "is nonzero", the exact predicate
/// `amenable_kani`'s own `verify_filter_yields_only_items_matching_
/// the_predicate` harness uses.
pub fn verify_filter_model_yields_only_items_matching_the_predicate(x: i32) -> (result: Option<i32>)
    ensures
        nonzero_item_survives_filtering(x, result),
{
    if x != 0 { Some(x) } else { None }
}

/// `FilterMap::next` matches its closure applied to the item — modeled
/// here as `amenable_kani`'s own
/// `verify_filter_map_applies_and_filters_in_one_step` harness's exact
/// closure (`None` for `0`, `Some(x)` otherwise).
pub fn verify_filter_map_model_applies_and_filters_in_one_step(x: i32) -> (result: Option<i32>)
    ensures
        nonzero_item_survives_filtering(x, result),
{
    if x == 0 { None } else { Some(x) }
}

/// A margin narrow enough that `x * 2` (`MapWhile`'s own doubling
/// closure) never overflows `i32`.
pub open spec fn is_within_map_while_doubling_headroom(x: int) -> bool {
    -1000 < x < 1000
}

/// `MapWhile`'s own closure result: `Some(x * 2)` for even `x`, `None`
/// otherwise.
pub open spec fn map_while_closure_result_matches(x: i32, result: Option<i32>) -> bool {
    (x % 2 == 0 ==> result == Some((x * 2) as i32)) && (x % 2 != 0 ==> result is None)
}

/// `MapWhile::next` matches its closure applied to the item — modeled
/// here as `amenable_kani`'s own
/// `verify_map_while_maps_items_while_the_closure_returns_some`
/// harness's exact closure (`Some(x * 2)` for even `x`, `None`
/// otherwise).
pub fn verify_map_while_model_maps_items_while_the_closure_returns_some(x: i32) -> (result: Option<i32>)
    requires
        is_within_map_while_doubling_headroom(x as int),
    ensures
        map_while_closure_result_matches(x, result),
{
    if x % 2 == 0 { Some(x * 2) } else { None }
}

/// `Cloned::next` yields an owned clone of the referenced item — for
/// `i32` (a `Copy` type), cloning is the identity.
pub fn verify_cloned_model_clones_each_referenced_item(value: i32) -> (result: (i32, i32))
    ensures
        values_are_equal(result.0, result.1),
{
    (value, value)
}

/// `Copied::next` yields an owned copy of the referenced item — the
/// identity, for `i32`.
pub fn verify_copied_model_copies_each_referenced_item(value: i32) -> (result: (i32, i32))
    ensures
        values_are_equal(result.0, result.1),
{
    (value, value)
}

} // verus!
