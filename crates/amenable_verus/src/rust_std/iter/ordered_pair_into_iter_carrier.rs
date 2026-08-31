//! Verus accommodation model for the "yields two owned values in push
//! order, then `None`" law shared by `alloc::vec::Drain`,
//! `alloc::collections::vec_deque::IntoIter`, and
//! `alloc::collections::linked_list::IntoIter`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. All three real types check the identical law in
//! `amenable_kani` — push (or collect) two elements, then consume in
//! order (`verify_vec_drain_removes_and_yields_in_order`,
//! `verify_vec_deque_into_iter_yields_owned_values_in_order`,
//! `verify_linked_list_into_iter_yields_owned_values_in_order`) — so
//! `VerusOrderedPairIntoIterModel` models that one shared law once, and
//! `amenable_std::verus_witness` registers all three real types against
//! this single carrier and harness, the same way `weak_carrier.rs`
//! backs both `Rc`'s and `Arc`'s `Weak`. `VerusOrderedPairIntoIterModel`
//! isn't any of these three types and doesn't claim to be — the proof
//! is conditional: sound if the real types refine this law, which each
//! type's own `amenable_kani` harness (checking the real type directly)
//! already confirms independently, for the identical claim.
//!
//! `alloc::collections::linked_list::Iter<'static, i32>` checks the same
//! positional order law in `amenable_kani`
//! (`verify_linked_list_iter_yields_references_in_order`) — only by
//! reference rather than by value, a distinction this model doesn't
//! encode either way (it tracks position and value, not real borrow
//! semantics) — so it's registered against this same carrier and
//! harness too.
//!
//! `alloc::string::Drain<'static>` checks an even weaker version of the
//! same shape in `amenable_kani`: the direct `String::drain` path times
//! out under Kani even for a single ASCII character (see
//! `amenable_kani::string_drain_model`'s own doc comment), so that
//! crate's own witness is a documented *assumption* (a `Standard` with
//! `basis = "Self"`), not a proof, that draining yields the source
//! content in order and leaves it empty — the identical "in order, then
//! exhausted" shape this model states, over `i32` in place of `char`
//! matching this crate's own established representative-element
//! convention throughout. Registered against this same carrier and
//! harness too.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::{
    observed_option_matches_input, observed_pair_matches_input, value_unchanged,
};

verus! {

/// A singleton claim, shared identically by all five real types this
/// carrier backs (`Vec::Drain`/`VecDeque::IntoIter`/`LinkedList::
/// IntoIter`/`LinkedList::Iter`/`String::Drain`) even though nothing
/// else in this crate ever calls it again: a freshly-constructed model
/// is always positioned before the first element. Named, not inlined,
/// so the assumption has an explicit, auditable source.
pub open spec fn ordered_pair_into_iter_model_starts_at_position_zero(position: u8) -> bool {
    position == 0
}

/// `advance`'s whole postcondition: yields `first` then `second` in
/// order, advancing the position each time, then `None` once exhausted,
/// with the position pinned once it reaches or passes `2`.
pub open spec fn ordered_pair_into_iter_advance_result_matches(old_position: u8, first: i32, second: i32, result: Option<i32>, new_position: u8) -> bool {
    (old_position == 0 ==> result == Some(first) && new_position == 1)
        && (old_position == 1 ==> result == Some(second) && new_position == 2)
        && (old_position >= 2 ==> result is None && new_position == old_position)
}

/// Models the "yields two owned values in order, then `None`" law —
/// not `Vec::Drain`/`VecDeque::IntoIter`/`LinkedList::IntoIter`
/// themselves.
pub struct VerusOrderedPairIntoIterModel {
    /// The first element yielded.
    pub first: i32,
    /// The second element yielded.
    pub second: i32,
    /// The next index `next()` yields.
    pub position: u8,
}

impl VerusOrderedPairIntoIterModel {
    /// Construct the model from two values, in order, positioned
    /// before the first.
    pub fn from_pair(first: i32, second: i32) -> (result: Self)
        ensures
            observed_pair_matches_input((result.first, result.second), (first, second)),
            ordered_pair_into_iter_model_starts_at_position_zero(result.position),
    {
        Self { first, second, position: 0 }
    }

    /// Advance and yield the next value by value, or `None` once both
    /// have been yielded.
    pub fn advance(&mut self) -> (result: Option<i32>)
        ensures
            ordered_pair_into_iter_advance_result_matches(
                old(self).position, old(self).first, old(self).second, result, final(self).position,
            ),
            value_unchanged(old(self).first as int, final(self).first as int),
            value_unchanged(old(self).second as int, final(self).second as int),
    {
        let result = if self.position == 0 {
            Some(self.first)
        } else if self.position == 1 {
            Some(self.second)
        } else {
            None
        };

        if self.position < 2 {
            self.position += 1;
        }

        result
    }
}

/// Two values pushed/collected in order are yielded back in the same
/// order, then `None` once exhausted — the law the real
/// `Vec::Drain`/`VecDeque::IntoIter`/`LinkedList::IntoIter` are each
/// expected to refine.
pub fn verify_ordered_pair_into_iter_model_yields_owned_values_in_order(a: i32, b: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, a),
        observed_option_matches_input(result.1, b),
        result.2 is None,
{
    let mut it = VerusOrderedPairIntoIterModel::from_pair(a, b);

    let first = it.advance();
    let second = it.advance();
    let exhausted = it.advance();

    (first, second, exhausted)
}

} // verus!
