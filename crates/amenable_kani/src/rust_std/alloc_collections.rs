//! `KaniWitness` impls for `alloc::collections`.
//!
//! `BinaryHeap`'s `Drain`/`IntoIter`/`Iter` proofs deliberately claim only
//! that every pushed element comes out exactly once (checked via sorting
//! both sides) — not that they come out in priority order. Confirmed
//! empirically first: `BinaryHeap::into_iter()` over `[3, 1, 2]` (pushed in
//! that order) yields `[3, 1, 2]`, the heap's internal array order, not
//! `[1, 2, 3]`. Only `.pop()` (and `.peek()`/`.peek_mut()`) guarantee
//! priority order — that distinction is exactly what `BinaryHeap`'s own
//! proof states, in contrast to these three.

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, TryReserveError, VecDeque};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_map_iterates_in_key_order".to_owned(),
            claim: VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BTreeMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeMap<i32, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<BTreeMap<i32, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC, {
        /// Unlike a hash map, `BTreeMap::iter` always yields entries in
        /// ascending key order, regardless of insertion order — checked
        /// by inserting the larger key first. This proof uses an
        /// Amenable-owned ordered-map accommodation model: if the real
        /// `BTreeMap` path refines these modeled ordering and removal
        /// laws, the Rust-facing claim follows.
        #[kani::proof]
        fn verify_btree_map_iterates_in_key_order() {
            let k1: i32 = kani::any();
            let k2: i32 = kani::any();
            kani::assume(k1 < k2);
            let v1: i32 = kani::any();
            let v2: i32 = kani::any();

            let mut map = crate::KaniBTreeMap::new(k2, v2, k1, v1);
            assert_eq!(
                map.first_entry(),
                Some((&k1, &v1)),
                "iteration is in ascending key order despite insertion order"
            );
            assert_eq!(
                map.second_entry(),
                Some((&k2, &v2)),
                "iteration preserves the higher key and its value after the lower one"
            );
            assert_eq!(
                map.remove(&k1),
                Some(v1),
                "observing iteration leaves the lower key and its value in the map"
            );
            assert_eq!(
                map.remove(&k2),
                Some(v2),
                "iteration leaves the higher key and its value in the map"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(map.is_empty()),
                "removing both entries after iteration empties the map"
            );
        }
    }
}

/// An emptied container's `.is_empty()` known to report `true`: whatever
/// sequence of removals (`drain`, repeated `pop`/`remove`, iteration)
/// took every element out, `.is_empty()` reflects it afterward.
///
/// Independently hand-written as `assert!(container.is_empty(), ...)` at
/// 13 real sites across `BTreeMap`, `BTreeSet`, `LinkedList`,
/// `VecDeque`, `BinaryHeap`, and `Vec` -- the identical claim regardless
/// of container type. Unlike `IteratorYieldsNoneWhenExhausted`,
/// `AtomicLoadReflectsTheLastWrite`, and `DerefReflectsTheStoredValue`,
/// this bound needs no type parameter at all: every real site already
/// computes the `bool` before asserting it, so the predicate has nothing
/// container-type-specific left to be generic over, and the ordinary
/// `kani_ensures!`/`bridge_kani_witness!` macros work unmodified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct EmptiedContainerReportsEmpty;

impl KaniWitness for EmptiedContainerReportsEmpty {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_map_iterates_in_key_order".to_owned(),
            claim: VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(EmptiedContainerReportsEmpty);

kani_ensures!(
    EmptiedContainerReportsEmpty,
    "amenable_kani::EmptiedContainerReportsEmpty",
    bool,
    |is_empty| is_empty
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::EmptiedContainerReportsEmpty",
        verifier: "kani",
        describe: || <EmptiedContainerReportsEmpty as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_set_iterates_in_sorted_order".to_owned(),
            claim: VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BTreeSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BTreeSet<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<BTreeSet<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC, {
        /// Same ordering guarantee as `BTreeMap`, for a set: `iter`
        /// yields elements in ascending order regardless of insertion
        /// order. This proof uses an Amenable-owned ordered-set
        /// accommodation model: if the real `BTreeSet` path refines
        /// these modeled ordering and removal laws, the Rust-facing
        /// claim follows.
        #[kani::proof]
        fn verify_btree_set_iterates_in_sorted_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a < b);

            let mut set = crate::KaniBTreeSet::new(b, a);
            assert_eq!(
                set.first_item(),
                Some(&a),
                "iteration is in ascending order despite insertion order"
            );
            assert_eq!(
                set.second_item(),
                Some(&b),
                "iteration preserves the higher element after the lower one"
            );
            assert!(set.remove(&a), "iteration leaves the lower element in the set");
            assert!(set.remove(&b), "iteration leaves the higher element in the set");
            assert!(
                EmptiedContainerReportsEmpty::ensures(set.is_empty()),
                "removing both elements after iteration empties the set"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_pop_yields_the_maximum_first".to_owned(),
            claim: VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BinaryHeap<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<BinaryHeap<i32>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<BinaryHeap<i32>>,
    "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
    (usize, usize),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC, {
        /// `.pop()` is the one operation `BinaryHeap` actually
        /// guarantees priority order for: it always returns the
        /// greatest remaining element first. (Plain iteration —
        /// `Drain`/`IntoIter`/`Iter` — does not share this guarantee;
        /// see this module's own doc comment.) Also checked with a
        /// drop-instrumented, `Ord`-by-id witness type standing in as
        /// the element itself (a heap's elements must be `Ord`):
        /// `.pop()` transfers ownership out without dropping the
        /// value, and dropping the heap drops every remaining element
        /// exactly once.
        #[kani::proof]
        fn verify_binary_heap_pop_yields_the_maximum_first() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            assert_eq!(heap.pop(), Some(a.max(b)), "pop returns the greatest element first");
            assert_eq!(heap.pop(), Some(a.min(b)), "the second pop returns the remaining element");

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl PartialEq for OrderedDropWitness {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for OrderedDropWitness {}
            impl PartialOrd for OrderedDropWitness {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for OrderedDropWitness {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id.cmp(&other.id)
                }
            }
            impl Drop for OrderedDropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_heap = BinaryHeap::new();
            witness_heap.push(OrderedDropWitness { id: 1, drop_count: drop_count.clone() });
            witness_heap.push(OrderedDropWitness { id: 2, drop_count: drop_count.clone() });
            let popped = witness_heap.pop().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "pop does not drop the returned value"
            );
            drop(popped);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the popped value drops once its owner drops it"
            );
            drop(witness_heap);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 2)),
                "dropping the heap drops the remaining element"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_is_fifo_through_back_and_front".to_owned(),
            claim: VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LinkedList<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<LinkedList<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC, {
        /// `push_back` followed by `pop_front` behaves as a FIFO
        /// queue: elements come out in the order they were pushed. Also
        /// checked with a drop-instrumented, non-`Copy` witness type:
        /// `pop_front()` transfers ownership out without dropping the
        /// value, and dropping the list drops every remaining element
        /// exactly once.
        #[kani::proof]
        fn verify_linked_list_is_fifo_through_back_and_front() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut list = LinkedList::new();
            list.push_back(a);
            list.push_back(b);
            assert_eq!(list.pop_front(), Some(a), "the first-pushed element comes out first");
            assert_eq!(list.pop_front(), Some(b));
            assert_eq!(list.pop_front(), None, "popping an exhausted FIFO returns None");
            assert!(
                EmptiedContainerReportsEmpty::ensures(list.is_empty()),
                "popping both queued elements empties the list"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_list = LinkedList::new();
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            let popped = witness_list.pop_front().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "pop_front does not drop the returned value"
            );
            drop(popped);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the popped value drops once its owner drops it"
            );
            drop(witness_list);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 2)),
                "dropping the list drops the remaining element"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_pushes_and_pops_from_both_ends".to_owned(),
            claim: VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VecDeque<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<VecDeque<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC, {
        /// Unlike `LinkedList` (back-only push), `VecDeque` is
        /// genuinely double-ended: pushing one element to the back and
        /// another to the front, then popping from each end, returns
        /// exactly the element pushed to that end. Also checked with a
        /// drop-instrumented, non-`Copy` witness type: `pop_front()`
        /// transfers ownership out without dropping the value, and
        /// dropping the deque drops every remaining element exactly
        /// once.
        #[kani::proof]
        fn verify_vec_deque_pushes_and_pops_from_both_ends() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_front(b);
            assert_eq!(dq.pop_front(), Some(b), "pop_front returns the front-pushed element");
            assert_eq!(dq.pop_back(), Some(a), "pop_back returns the back-pushed element");
            assert_eq!(dq.pop_front(), None, "popping the exhausted front returns None");
            assert_eq!(dq.pop_back(), None, "popping the exhausted back returns None");
            assert!(
                EmptiedContainerReportsEmpty::ensures(dq.is_empty()),
                "popping both end-specific values empties the deque"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_dq = VecDeque::new();
            witness_dq.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_dq.push_back(DropWitness { drop_count: drop_count.clone() });
            let popped = witness_dq.pop_front().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "pop_front does not drop the returned value"
            );
            drop(popped);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the popped value drops once its owner drops it"
            );
            drop(witness_dq);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 2)),
                "dropping the deque drops the remaining element"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_reserve_rejects_an_impossible_capacity".to_owned(),
            claim: VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryReserveError>",
        verifier: "kani",
        describe: || <RustStdStandard<TryReserveError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC, {
        /// `.try_reserve` fails — producing this error, instead of
        /// aborting like `.reserve` would — for a request no allocator
        /// can satisfy, without changing already stored values.
        #[kani::proof]
        fn verify_try_reserve_rejects_an_impossible_capacity() {
            let first: i32 = kani::any();
            let second: i32 = kani::any();
            let mut v = vec![first, second];
            assert!(
                v.try_reserve(usize::MAX).is_err(),
                "an impossible reservation is rejected, not aborted"
            );
            assert_eq!(v, vec![first, second], "a failed reservation preserves existing values");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_drain_yields_every_pushed_element_once".to_owned(),
            claim: VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `.drain()` yields every pushed element exactly once (checked
        /// by sorting both sides) and leaves the heap empty — but not
        /// necessarily in priority order, unlike `.pop()`. It also
        /// transfers a yielded non-`Copy` element to its caller and,
        /// when dropped early, destroys every element that remains in
        /// the unfinished drain.
        #[kani::proof]
        fn verify_binary_heap_drain_yields_every_pushed_element_once() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            let mut drained: Vec<i32> = heap.drain().collect();
            drained.sort_unstable();
            let mut expected = vec![a, b];
            expected.sort_unstable();
            assert!(
                RustStdStandard::<Vec<i32>>::ensures((drained, expected)),
                "drain yields every pushed element exactly once"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(heap.is_empty()),
                "drain leaves the heap empty"
            );

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl PartialEq for OrderedDropWitness {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for OrderedDropWitness {}
            impl PartialOrd for OrderedDropWitness {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for OrderedDropWitness {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id.cmp(&other.id)
                }
            }
            impl Drop for OrderedDropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_heap = BinaryHeap::new();
            witness_heap.push(OrderedDropWitness {
                id: 1,
                drop_count: drop_count.clone(),
            });
            witness_heap.push(OrderedDropWitness {
                id: 2,
                drop_count: drop_count.clone(),
            });
            witness_heap.push(OrderedDropWitness {
                id: 3,
                drop_count: drop_count.clone(),
            });
            let mut drain = witness_heap.drain();
            let first = drain.next().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "drain transfers a yielded value without dropping it"
            );
            drop(first);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the caller drops the yielded value exactly once"
            );
            drop(drain);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 3)),
                "dropping an unfinished drain drops every remaining element"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(witness_heap.is_empty()),
                "dropping an unfinished drain leaves the heap empty"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_into_iter_yields_every_pushed_element_once".to_owned(),
            claim: VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::binary_heap::IntoIter<i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// `.into_iter()` yields every pushed element exactly once
        /// (checked by sorting both sides) — confirmed empirically to
        /// walk the heap's internal array order, not priority order
        /// (pushing `[3, 1, 2]` yields `[3, 1, 2]` back, not `[1, 2,
        /// 3]`), so this deliberately does not assert an order. An
        /// early-dropped iterator is also checked to transfer its yielded
        /// value and destroy its remaining non-`Copy` values exactly once.
        #[kani::proof]
        fn verify_binary_heap_into_iter_yields_every_pushed_element_once() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            let mut collected: Vec<i32> = heap.into_iter().collect();
            collected.sort_unstable();
            let mut expected = vec![a, b];
            expected.sort_unstable();
            assert!(
                RustStdStandard::<Vec<i32>>::ensures((collected, expected)),
                "into_iter yields every pushed element exactly once"
            );

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl PartialEq for OrderedDropWitness {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for OrderedDropWitness {}
            impl PartialOrd for OrderedDropWitness {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for OrderedDropWitness {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id.cmp(&other.id)
                }
            }
            impl Drop for OrderedDropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_heap = BinaryHeap::new();
            witness_heap.push(OrderedDropWitness {
                id: 1,
                drop_count: drop_count.clone(),
            });
            witness_heap.push(OrderedDropWitness {
                id: 2,
                drop_count: drop_count.clone(),
            });
            witness_heap.push(OrderedDropWitness {
                id: 3,
                drop_count: drop_count.clone(),
            });
            let mut iterator = witness_heap.into_iter();
            let first = iterator.next().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "into_iter transfers a yielded value without dropping it"
            );
            drop(first);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the caller drops the yielded value exactly once"
            );
            drop(iterator);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 3)),
                "dropping an unfinished iterator drops every remaining value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_iter_yields_every_pushed_element_once".to_owned(),
            claim: VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC, {
        /// Same non-priority-order caveat as `IntoIter`/`Drain`:
        /// `.iter()` yields a reference to every pushed element exactly
        /// once, not necessarily in priority order, and does not consume
        /// or otherwise change the heap.
        #[kani::proof]
        fn verify_binary_heap_iter_yields_every_pushed_element_once() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            let mut collected: Vec<i32> = heap.iter().copied().collect();
            collected.sort_unstable();
            let mut expected = vec![a, b];
            expected.sort_unstable();
            assert!(
                RustStdStandard::<Vec<i32>>::ensures((collected, expected)),
                "iter yields every pushed element exactly once"
            );
            assert!(
                RustStdStandard::<BinaryHeap<i32>>::ensures((heap.len(), 2)),
                "iteration leaves every heap element in place"
            );
            assert_eq!(heap.pop(), Some(a.max(b)), "iteration preserves the heap maximum");
            assert_eq!(heap.pop(), Some(a.min(b)), "iteration preserves the remaining element");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_peek_mut_exposes_the_maximum".to_owned(),
            claim: VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC, {
        /// `.peek_mut()` derefs to the greatest element. Leaving it
        /// unmodified keeps it at the top, while lowering it through
        /// the guard re-establishes the heap invariant when the guard
        /// is released.
        #[kani::proof]
        fn verify_binary_heap_peek_mut_exposes_the_maximum() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a < b);

            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            {
                let peek = heap.peek_mut().unwrap();
                assert!(
                    DerefReflectsTheStoredValue::ensures((*peek, b)),
                    "peek_mut derefs to the greatest element"
                );
            }
            assert_eq!(heap.peek(), Some(&b), "the maximum is still on top afterward");
            {
                let mut peek = heap.peek_mut().unwrap();
                *peek = a;
                assert!(
                    DerefReflectsTheStoredValue::ensures((*peek, a)),
                    "peek_mut writes through to the guarded maximum"
                );
            }
            assert_eq!(heap.peek(), Some(&a), "releasing a modified guard re-establishes the heap maximum");
            assert_eq!(heap.pop(), Some(a), "the re-heapified first value is available");
            assert_eq!(heap.pop(), Some(a), "the re-heapified remaining value is available");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_iter_yields_references_in_order".to_owned(),
            claim: VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::linked_list::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::linked_list::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC, {
        /// `.iter()` borrows instead of consuming, yielding shared
        /// references in the list's front-to-back order.
        #[kani::proof]
        fn verify_linked_list_iter_yields_references_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut list = LinkedList::new();
            list.push_back(a);
            list.push_back(b);
            let mut it = list.iter();
            assert_eq!(it.next(), Some(&a));
            assert_eq!(it.next(), Some(&b));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            drop(it);
            assert_eq!(list.pop_front(), Some(a), "iteration leaves the first value in place");
            assert_eq!(list.pop_front(), Some(b), "iteration leaves the second value in place");
            assert!(
                EmptiedContainerReportsEmpty::ensures(list.is_empty()),
                "removing values after iteration empties the list"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_iter_mut_writes_through".to_owned(),
            claim: VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC, {
        /// `.iter_mut()` yields mutable references, and a write
        /// through each is visible at its corresponding list position.
        #[kani::proof]
        fn verify_linked_list_iter_mut_writes_through() {
            let first: i32 = kani::any();
            let second: i32 = kani::any();
            let updated_first: i32 = kani::any();
            let updated_second: i32 = kani::any();

            let mut list = LinkedList::new();
            list.push_back(first);
            list.push_back(second);
            {
                let mut iterator = list.iter_mut();
                *iterator.next().unwrap() = updated_first;
                *iterator.next().unwrap() = updated_second;
                assert!(
                    IteratorYieldsNoneWhenExhausted::ensures(iterator.next()),
                    "iter_mut visits every list element exactly once"
                );
            }
            assert_eq!(list.pop_front(), Some(updated_first), "the first write is visible at the front");
            assert_eq!(list.pop_front(), Some(updated_second), "the second write preserves list order");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_into_iter_yields_owned_values_in_order".to_owned(),
            claim: VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::linked_list::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::linked_list::IntoIter<i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::collections::linked_list::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `.into_iter()` consumes the list, yielding its owned
        /// elements in front-to-back order. A partially consumed
        /// iterator transfers its yielded value to the caller and
        /// destroys its remaining owned values when dropped.
        #[kani::proof]
        fn verify_linked_list_into_iter_yields_owned_values_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut list = LinkedList::new();
            list.push_back(a);
            list.push_back(b);
            let mut it = list.into_iter();
            assert!(RustStdStandard::<std::collections::linked_list::IntoIter<i32>>::ensures((
                it.next(),
                Some(a)
            )));
            assert!(RustStdStandard::<std::collections::linked_list::IntoIter<i32>>::ensures((
                it.next(),
                Some(b)
            )));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_list = LinkedList::new();
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            let mut iterator = witness_list.into_iter();
            let first = iterator.next().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "into_iter transfers a yielded value without dropping it"
            );
            drop(first);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the caller drops the yielded value exactly once"
            );
            drop(iterator);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 3)),
                "dropping an unfinished iterator drops every remaining value"
            );
        }
    }
}

impl KaniWitness
    for RustStdStandard<
        std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_extract_if_partitions_by_the_predicate".to_owned(),
            claim: VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(
    RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC, {
        /// Same partitioning rule as `Vec`'s `extract_if`, on
        /// `LinkedList`. Unlike `Vec::extract_if`, `LinkedList::
        /// extract_if` takes only a predicate, no range — confirmed
        /// empirically, since the two collections' signatures differ
        /// here. This proof uses an Amenable-owned extractor model:
        /// if the real `LinkedList::extract_if` path refines these
        /// partition and early-drop laws, the Rust-facing claim
        /// follows.
        #[kani::proof]
        fn verify_linked_list_extract_if_partitions_by_the_predicate() {
            fn is_even(x: &mut i32) -> bool {
                *x % 2 == 0
            }
            let mut extractor = crate::KaniLinkedListExtractIf::new(vec![1, 2, 3, 4]);
            assert_eq!(
                extractor.next(is_even as fn(&mut i32) -> bool),
                Some(2),
                "extract_if yields the first matching element"
            );
            assert_eq!(
                extractor.next(is_even as fn(&mut i32) -> bool),
                Some(4),
                "extract_if continues yielding later matching elements in order"
            );
            assert_eq!(
                extractor.next(is_even as fn(&mut i32) -> bool),
                None,
                "extract_if exhausts once every match has been yielded"
            );
            assert_eq!(
                extractor.into_remaining(),
                vec![1, 3],
                "extract_if leaves the non-matching elements in place and order"
            );

            let mut extractor = crate::KaniLinkedListExtractIf::new(vec![1, 2, 3, 4]);
            assert_eq!(
                extractor.next(is_even as fn(&mut i32) -> bool),
                Some(2),
                "extract_if yields the first matching element"
            );
            assert_eq!(
                extractor.into_remaining(),
                vec![1, 3, 4],
                "dropping extract_if retains the unvisited suffix and prior non-matches"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_drain_removes_and_yields_in_order".to_owned(),
            claim: VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC, {
        /// `.drain(..)` yields every element in front-to-back order
        /// and leaves the deque empty. An unfinished whole-deque drain
        /// transfers its yielded value and drops every remaining value
        /// when released.
        #[kani::proof]
        fn verify_vec_deque_drain_removes_and_yields_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let drained: Vec<i32> = dq.drain(..).collect();
            assert_eq!(drained, vec![a, b], "drain yields every element in order");
            assert!(
                EmptiedContainerReportsEmpty::ensures(dq.is_empty()),
                "drain leaves the deque empty"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_deque = VecDeque::new();
            witness_deque.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_deque.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_deque.push_back(DropWitness { drop_count: drop_count.clone() });
            let mut drain = witness_deque.drain(..);
            let first = drain.next().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "drain transfers a yielded value without dropping it"
            );
            drop(first);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the caller drops the yielded value exactly once"
            );
            drop(drain);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 3)),
                "dropping an unfinished drain drops every remaining value"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(witness_deque.is_empty()),
                "dropping an unfinished drain leaves the deque empty"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_iter_yields_references_in_order".to_owned(),
            claim: VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC, {
        /// `.iter()` borrows instead of consuming, yielding shared
        /// references in front-to-back order.
        #[kani::proof]
        fn verify_vec_deque_iter_yields_references_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let mut it = dq.iter();
            assert_eq!(it.next(), Some(&a));
            assert_eq!(it.next(), Some(&b));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            drop(it);
            assert_eq!(dq.pop_front(), Some(a), "iteration leaves the first value in place");
            assert_eq!(dq.pop_front(), Some(b), "iteration leaves the second value in place");
            assert!(
                EmptiedContainerReportsEmpty::ensures(dq.is_empty()),
                "removing values after iteration empties the deque"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_iter_mut_writes_through".to_owned(),
            claim: VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC, {
        /// `.iter_mut()` yields mutable references, and a write
        /// through each is visible at its corresponding deque position.
        #[kani::proof]
        fn verify_vec_deque_iter_mut_writes_through() {
            let first: i32 = kani::any();
            let second: i32 = kani::any();
            let updated_first: i32 = kani::any();
            let updated_second: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(first);
            dq.push_back(second);
            {
                let mut iterator = dq.iter_mut();
                *iterator.next().unwrap() = updated_first;
                *iterator.next().unwrap() = updated_second;
                assert!(
                    IteratorYieldsNoneWhenExhausted::ensures(iterator.next()),
                    "iter_mut visits every deque element exactly once"
                );
            }
            assert_eq!(dq.pop_front(), Some(updated_first), "the first write is visible at the front");
            assert_eq!(dq.pop_front(), Some(updated_second), "the second write preserves deque order");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_into_iter_yields_owned_values_in_order".to_owned(),
            claim: VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::collections::vec_deque::IntoIter<i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::collections::vec_deque::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `.into_iter()` consumes the deque, yielding its owned
        /// elements in front-to-back order. An unfinished iterator
        /// transfers yielded ownership and drops its remaining values.
        #[kani::proof]
        fn verify_vec_deque_into_iter_yields_owned_values_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let mut it = dq.into_iter();
            assert!(RustStdStandard::<std::collections::vec_deque::IntoIter<i32>>::ensures((
                it.next(),
                Some(a)
            )));
            assert!(RustStdStandard::<std::collections::vec_deque::IntoIter<i32>>::ensures((
                it.next(),
                Some(b)
            )));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness_deque = VecDeque::new();
            witness_deque.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_deque.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_deque.push_back(DropWitness { drop_count: drop_count.clone() });
            let mut iterator = witness_deque.into_iter();
            let first = iterator.next().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "into_iter transfers a yielded value without dropping it"
            );
            drop(first);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the caller drops the yielded value exactly once"
            );
            drop(iterator);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 3)),
                "dropping an unfinished iterator drops every remaining value"
            );
        }
    }
}
