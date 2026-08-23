//! Verus accommodation model for `alloc::collections::LinkedList<i32>`.
//!
//! `vstd` gives `LinkedList` no spec support at all — the same kind of
//! zero-coverage gap `binary_heap_carrier.rs` documents (not a missing
//! per-type `next` spec or an unstable-feature requirement: Verus never
//! reasons about real `alloc`/`std` internals for this type at all,
//! covered or not). Writing `assume_specification`s for
//! `LinkedList::push_back`/`pop_front` ourselves would be a new,
//! unaudited trust assumption about real `alloc` internals with nothing
//! checking whether we got it right. Instead this carrier models the
//! semantics explicitly, in real code Verus checks completely: a
//! two-push/two-pop FIFO law, mirroring
//! `amenable_kani::linked_list_extract_model`'s own accommodation-model
//! precedent for the same type. `VerusFifoQueuePair` isn't `LinkedList`
//! and doesn't claim to be — the proof is conditional: sound if the
//! real `LinkedList` refines this law, which `amenable_kani`'s own
//! `verify_linked_list_is_fifo_through_back_and_front` harness (checking
//! the real `LinkedList<i32>` directly) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Models the FIFO push-back/pop-front order law a queue must satisfy —
/// not `LinkedList` itself.
pub struct VerusFifoQueuePair {
    pub first: i32,
    pub second: i32,
}

impl VerusFifoQueuePair {
    /// Push two values onto the back, in order.
    pub fn from_two_pushes(a: i32, b: i32) -> (result: Self)
        ensures
            result.first == a,
            result.second == b,
    {
        Self { first: a, second: b }
    }

    /// Pop from the front: the first-pushed value comes out first, then
    /// the second — the law the real `LinkedList` is expected to
    /// refine.
    pub fn pop_front_then_remaining(self) -> (result: (i32, i32))
        ensures
            result.0 == self.first,
            result.1 == self.second,
    {
        (self.first, self.second)
    }
}

/// Pushing two values onto the back and popping twice from the front
/// always yields them in push order — the FIFO law
/// `LinkedList<i32>::push_back`/`pop_front` are expected to refine.
pub fn verify_fifo_queue_pair_pops_in_push_order(a: i32, b: i32) -> (result: (i32, i32))
    ensures
        result.0 == a,
        result.1 == b,
{
    let queue = VerusFifoQueuePair::from_two_pushes(a, b);
    queue.pop_front_then_remaining()
}

} // verus!
