//! Verus accommodation model for the "yields every pushed element
//! exactly once, order unspecified" law shared by
//! `alloc::collections::binary_heap::Drain`,
//! `alloc::collections::binary_heap::IntoIter`, and
//! `alloc::collections::binary_heap::Iter`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Unlike `ordered_pair_into_iter_carrier.rs`'s law, this one
//! is deliberately weaker: `BinaryHeap`'s iteration order is not
//! sorted-pop order and isn't guaranteed at all by its real docs, so
//! `amenable_kani`'s own harnesses
//! (`verify_binary_heap_drain_yields_every_pushed_element_once`/
//! `verify_binary_heap_into_iter_yields_every_pushed_element_once`/
//! `verify_binary_heap_iter_yields_every_pushed_element_once`) all
//! `sort_unstable()` both sides before comparing — checking multiset
//! equality, not sequence equality. `VerusUnorderedPairModel` mirrors
//! that exact weaker shape: yielding in either order (a caller-chosen
//! `bool`, standing in for the real type's unspecified internal order)
//! still recovers the same two values, none lost or duplicated. Not
//! `binary_heap_carrier.rs`'s `VerusMaxHeapPair` reused here — that
//! model's `pop_max_then_remaining` proves a *stronger*, deterministic
//! order than `Drain`/`IntoIter`/`Iter` actually guarantee, which would
//! overclaim what these three iteration-order-unspecified types are
//! known to do. `VerusUnorderedPairModel` isn't
//! `Drain`/`IntoIter`/`Iter` and doesn't claim to be — the proof is
//! conditional: sound if the real types refine this law, which each
//! type's own `amenable_kani` harness (checking the real type directly)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_pair_matches_input;

verus! {

/// Models the "yields every element exactly once, order unspecified"
/// law — not `binary_heap::Drain`/`IntoIter`/`Iter` themselves.
pub struct VerusUnorderedPairModel {
    pub first: i32,
    pub second: i32,
}

/// `drain_in_either_order`'s whole postcondition: yields `(first,
/// second)` or `(second, first)` depending on which order was
/// requested.
pub open spec fn drain_result_matches_order(first_first: bool, first: i32, second: i32, result: (i32, i32)) -> bool {
    (first_first ==> result == (first, second)) && (!first_first ==> result == (second, first))
}

impl VerusUnorderedPairModel {
    pub fn new(first: i32, second: i32) -> (result: Self)
        ensures
            observed_pair_matches_input((result.first, result.second), (first, second)),
    {
        Self { first, second }
    }

    /// Yields both values in either order — `first_first` stands in for
    /// the real type's unspecified internal iteration order.
    pub fn drain_in_either_order(self, first_first: bool) -> (result: (i32, i32))
        ensures
            drain_result_matches_order(first_first, self.first, self.second, result),
    {
        if first_first {
            (self.first, self.second)
        } else {
            (self.second, self.first)
        }
    }
}

/// Regardless of which order the model yields in, the two values that
/// come out are exactly the two that went in, once each — the
/// multiset-equality law the real
/// `binary_heap::Drain`/`IntoIter`/`Iter` are each expected to refine.
pub fn verify_unordered_pair_model_yields_every_element_once(a: i32, b: i32, first_first: bool) -> (result: bool)
    ensures
        result,
{
    let model = VerusUnorderedPairModel::new(a, b);
    let (x, y) = model.drain_in_either_order(first_first);

    (x == a && y == b) || (x == b && y == a)
}

} // verus!
