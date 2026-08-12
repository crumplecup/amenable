//! Verus accommodation model for the "writes through both positions,
//! in order" law shared by `alloc::collections::vec_deque::IterMut` and
//! `alloc::collections::linked_list::IterMut`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Both real types check the identical law in `amenable_kani`
//! — write through each position via the iterator, then observe both
//! writes landed (`verify_vec_deque_iter_mut_writes_through`,
//! `verify_linked_list_iter_mut_writes_through`) — so
//! `VerusOrderedPairIterMutModel` models that one shared law once, and
//! `amenable_std::verus_witness` registers both real types against this
//! single carrier and harness. Writes through explicit
//! `write_first`/`write_second` calls rather than a returned `&mut i32`
//! — a returned mutable reference hits Verus's prophecy-based
//! mutable-reference-return machinery, the same unrelated complication
//! `unsafe_cell_carrier.rs` already documents sidestepping.
//! `VerusOrderedPairIterMutModel` isn't `VecDeque::IterMut`/
//! `LinkedList::IterMut` and doesn't claim to be — the proof is
//! conditional: sound if the real types refine this law, which each
//! type's own `amenable_kani` harness (checking the real type directly)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

// The shared write-through postcondition `amenable_std::verus_witness`
// registers for plain slot-update laws across several carriers.
#[cfg(verus_keep_ghost)]
use crate::rust_std::cell_carrier::write_stores_new_value;

verus! {

/// Models the "writes through both positions, in order" law — not
/// `VecDeque::IterMut`/`LinkedList::IterMut` themselves.
pub struct VerusOrderedPairIterMutModel {
    pub first: i32,
    pub second: i32,
}

impl VerusOrderedPairIterMutModel {
    pub fn from_pair(first: i32, second: i32) -> (result: Self)
        ensures
            result.first == first,
            result.second == second,
    {
        Self { first, second }
    }

    /// Writes through the first position, mirroring
    /// `*iterator.next().unwrap() = value`.
    pub fn write_first(&mut self, value: i32)
        ensures
            write_stores_new_value(value as int, final(self).first as int),
            final(self).second == old(self).second,
    {
        self.first = value;
    }

    /// Writes through the second position, mirroring the second
    /// `*iterator.next().unwrap() = value` call.
    pub fn write_second(&mut self, value: i32)
        ensures
            write_stores_new_value(value as int, final(self).second as int),
            final(self).first == old(self).first,
    {
        self.second = value;
    }
}

/// Writing through both positions via the iterator lands both writes,
/// each at its own position — the law the real
/// `VecDeque::IterMut`/`LinkedList::IterMut` are each expected to
/// refine.
pub fn verify_ordered_pair_iter_mut_model_writes_through_in_order(first: i32, second: i32, updated_first: i32, updated_second: i32) -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    let mut model = VerusOrderedPairIterMutModel::from_pair(first, second);

    model.write_first(updated_first);
    model.write_second(updated_second);

    (model.first == updated_first, model.second == updated_second)
}

} // verus!
