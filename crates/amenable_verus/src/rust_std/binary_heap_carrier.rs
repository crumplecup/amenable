//! Verus accommodation model for `alloc::collections::BinaryHeap<i32>`.
//!
//! `vstd` gives `BinaryHeap` no spec support at all — not one
//! `assume_specification`/`external_type_specification` anywhere in the
//! installed `vstd` names it (confirmed by inspecting `vstd`'s own
//! source directly, not just its absence from generated coverage
//! reports). That's a different kind of gap than the ones the rest of
//! this crate documents (a missing per-type `next` spec, an unstable
//! feature requirement): it's not that Verus chokes trying to reason
//! about the real type, the way Kani sometimes does — Verus never
//! reasons about real `std`/`alloc` internals *at all*, for any type,
//! covered or not. Every carrier in this crate that touches a real std
//! method does so through `assume_specification`: a trusted axiom
//! asserting the native function behaves a certain way, never checked
//! against the real (often `unsafe`) implementation. `vstd` covering
//! `Vec`/`BTreeMap`/`VecDeque` means someone already wrote and vouched
//! for those axioms upstream; `BinaryHeap` simply has none yet.
//!
//! Writing that axiom ourselves — asserting `BinaryHeap::push`/`pop`
//! behave a certain way — was the other option, and it isn't wrong, but
//! it would be a new unaudited trust assumption about real `alloc`
//! internals with nothing checking whether we got it right. This carrier
//! takes the same route `amenable_kani` already takes for the ordering
//! collections it can't run its real proof against
//! (`amenable_kani::btree_model::KaniBTreeMap`/`KaniBTreeSet`,
//! `linked_list_extract_model`): model the semantics we mean explicitly,
//! in real code Verus checks completely (no axiom, no trust), and say so
//! plainly. `VerusMaxHeapPair` below isn't `BinaryHeap` and doesn't
//! pretend to be — it's the max-first pop-order law a max-heap is
//! supposed to satisfy, spelled out as a two-push/two-pop model. The
//! proof is conditional: sound if the real `BinaryHeap` refines this
//! law, which `amenable_kani`'s own `verify_binary_heap_pop_yields_the_
//! maximum_first` harness (checking the *real* `BinaryHeap<i32>`
//! directly — Kani has no trouble with a two-element real heap) already
//! confirms independently, for the identical claim.
//!
//! `binary_heap::PeekMut<'static, i32>` checks the same max-exposure
//! law in `amenable_kani`
//! (`verify_binary_heap_peek_mut_exposes_the_maximum`) — unlike
//! `Drain`/`IntoIter`/`Iter` (see `unordered_pair_carrier.rs`),
//! `PeekMut`'s whole contract genuinely *is* "exposes the current
//! maximum," so reusing this deterministic max-first model here is
//! accurate, not an overclaim — so it's registered against this same
//! carrier and harness too.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Models the max-first pop-order law a two-element max-heap must
/// satisfy, regardless of push order — not `BinaryHeap` itself.
pub struct VerusMaxHeapPair {
    pub max: i32,
    pub min: i32,
}

impl VerusMaxHeapPair {
    /// Push two values in either order; the model records which is the
    /// greater, matching a max-heap's ordering law.
    pub fn from_two_pushes(a: i32, b: i32) -> (result: Self)
        ensures
            result.max == if a >= b { a } else { b },
            result.min == if a >= b { b } else { a },
    {
        if a >= b {
            Self { max: a, min: b }
        } else {
            Self { max: b, min: a }
        }
    }

    /// Pop the maximum first, then the remaining element — the law the
    /// real `BinaryHeap` is expected to refine.
    pub fn pop_max_then_remaining(self) -> (result: (i32, i32))
        ensures
            result.0 == self.max,
            result.1 == self.min,
    {
        (self.max, self.min)
    }
}

/// Pushing two values in either order and popping twice always yields
/// the greater value first, then the lesser — the max-heap pop-order
/// law `BinaryHeap<i32>::push`/`pop` are expected to refine.
pub fn verify_max_heap_pair_pops_the_maximum_first(a: i32, b: i32) -> (result: (i32, i32))
    ensures
        result.0 == if a >= b { a } else { b },
        result.1 == if a >= b { b } else { a },
{
    let heap = VerusMaxHeapPair::from_two_pushes(a, b);
    heap.pop_max_then_remaining()
}

} // verus!
