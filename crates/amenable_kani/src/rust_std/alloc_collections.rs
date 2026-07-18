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

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_map_iterates_in_key_order",
            claim: VERIFY_BTREE_MAP_ITERATES_IN_KEY_ORDER_SRC,
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
        /// by inserting the larger key first. Also checked with a
        /// drop-instrumented, non-`Copy` value type: `remove()`
        /// transfers the value out without dropping it, and dropping
        /// the map drops every remaining value exactly once — `i32`
        /// alone has no drop glue to distinguish "moved out cleanly"
        /// from "dropped early" or "leaked".
        #[kani::proof]
        fn verify_btree_map_iterates_in_key_order() {
            let k1: i32 = kani::any();
            let k2: i32 = kani::any();
            kani::assume(k1 < k2);
            let v1: i32 = kani::any();
            let v2: i32 = kani::any();

            let mut map = BTreeMap::new();
            map.insert(k2, v2);
            map.insert(k1, v1);
            let entries: Vec<(&i32, &i32)> = map.iter().collect();
            assert_eq!(
                entries,
                vec![(&k1, &v1), (&k2, &v2)],
                "iteration is in ascending key order despite insertion order"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witness_map = BTreeMap::new();
            witness_map.insert(1, DropWitness { drop_count: drop_count.clone() });
            witness_map.insert(2, DropWitness { drop_count: drop_count.clone() });
            let removed = witness_map.remove(&1).unwrap();
            assert_eq!(drop_count.get(), 0, "remove does not drop the returned value");
            drop(removed);
            assert_eq!(drop_count.get(), 1, "the removed value drops once its owner drops it");
            drop(witness_map);
            assert_eq!(drop_count.get(), 2, "dropping the map drops the remaining value");
        }
    }
}

impl KaniWitness for RustStdStandard<BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_btree_set_iterates_in_sorted_order",
            claim: VERIFY_BTREE_SET_ITERATES_IN_SORTED_ORDER_SRC,
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
        /// order. Also checked with a drop-instrumented, `Ord`-by-id
        /// witness type standing in as the element itself (a set's
        /// elements, unlike a map's values, must be `Ord`): `.take()`
        /// transfers the real stored element out without dropping it —
        /// only the probe key used to look it up gets dropped — and
        /// dropping the set drops every remaining element exactly once.
        #[kani::proof]
        fn verify_btree_set_iterates_in_sorted_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a < b);

            let mut set = BTreeSet::new();
            set.insert(b);
            set.insert(a);
            let items: Vec<&i32> = set.iter().collect();
            assert_eq!(
                items,
                vec![&a, &b],
                "iteration is in ascending order despite insertion order"
            );

            struct OrderedDropWitness {
                id: i32,
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
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

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witness_set = BTreeSet::new();
            witness_set.insert(OrderedDropWitness { id: 1, drop_count: drop_count.clone() });
            witness_set.insert(OrderedDropWitness { id: 2, drop_count: drop_count.clone() });
            let probe = OrderedDropWitness { id: 1, drop_count: drop_count.clone() };
            let taken = witness_set.take(&probe);
            assert_eq!(drop_count.get(), 0, "take does not drop the probe key or the stored element");
            assert!(taken.is_some());
            drop(probe);
            assert_eq!(drop_count.get(), 1, "the probe key drops once its owner drops it");
            drop(taken);
            assert_eq!(drop_count.get(), 2, "the taken element drops once its owner drops it");
            drop(witness_set);
            assert_eq!(drop_count.get(), 3, "dropping the set drops the remaining element");
        }
    }
}

impl KaniWitness for RustStdStandard<BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_pop_yields_the_maximum_first",
            claim: VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC,
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
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
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

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witness_heap = BinaryHeap::new();
            witness_heap.push(OrderedDropWitness { id: 1, drop_count: drop_count.clone() });
            witness_heap.push(OrderedDropWitness { id: 2, drop_count: drop_count.clone() });
            let popped = witness_heap.pop().unwrap();
            assert_eq!(drop_count.get(), 0, "pop does not drop the returned value");
            drop(popped);
            assert_eq!(drop_count.get(), 1, "the popped value drops once its owner drops it");
            drop(witness_heap);
            assert_eq!(drop_count.get(), 2, "dropping the heap drops the remaining element");
        }
    }
}

impl KaniWitness for RustStdStandard<LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_is_fifo_through_back_and_front",
            claim: VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC,
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

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witness_list = LinkedList::new();
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_list.push_back(DropWitness { drop_count: drop_count.clone() });
            let popped = witness_list.pop_front().unwrap();
            assert_eq!(drop_count.get(), 0, "pop_front does not drop the returned value");
            drop(popped);
            assert_eq!(drop_count.get(), 1, "the popped value drops once its owner drops it");
            drop(witness_list);
            assert_eq!(drop_count.get(), 2, "dropping the list drops the remaining element");
        }
    }
}

impl KaniWitness for RustStdStandard<VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_pushes_and_pops_from_both_ends",
            claim: VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC,
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

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witness_dq = VecDeque::new();
            witness_dq.push_back(DropWitness { drop_count: drop_count.clone() });
            witness_dq.push_back(DropWitness { drop_count: drop_count.clone() });
            let popped = witness_dq.pop_front().unwrap();
            assert_eq!(drop_count.get(), 0, "pop_front does not drop the returned value");
            drop(popped);
            assert_eq!(drop_count.get(), 1, "the popped value drops once its owner drops it");
            drop(witness_dq);
            assert_eq!(drop_count.get(), 2, "dropping the deque drops the remaining element");
        }
    }
}

impl KaniWitness for RustStdStandard<TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_reserve_rejects_an_impossible_capacity",
            claim: VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC,
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
        /// `.try_reserve` succeeds for a small, reasonable request and
        /// fails — producing this error, instead of aborting like
        /// `.reserve` would — for a request no allocator can satisfy.
        #[kani::proof]
        fn verify_try_reserve_rejects_an_impossible_capacity() {
            let mut v: Vec<i32> = Vec::new();
            assert!(v.try_reserve(4).is_ok(), "a small reservation succeeds");
            assert!(
                v.try_reserve(usize::MAX).is_err(),
                "an impossible reservation is rejected, not aborted"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_drain_yields_every_pushed_element_once",
            claim: VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
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
        /// necessarily in priority order, unlike `.pop()`.
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
            assert_eq!(drained, expected, "drain yields every pushed element exactly once");
            assert!(heap.is_empty(), "drain leaves the heap empty");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_into_iter_yields_every_pushed_element_once",
            claim: VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
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
        /// 3]`), so this deliberately does not assert an order.
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
            assert_eq!(collected, expected, "into_iter yields every pushed element exactly once");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_iter_yields_every_pushed_element_once",
            claim: VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC,
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
        /// once, not necessarily in priority order.
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
            assert_eq!(collected, expected, "iter yields every pushed element exactly once");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_binary_heap_peek_mut_exposes_the_maximum",
            claim: VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC,
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
        /// `.peek_mut()` derefs to the greatest element, and leaving it
        /// unmodified (no re-heapify needed) leaves it as the top of
        /// the heap afterward.
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
                assert_eq!(*peek, b, "peek_mut derefs to the greatest element");
            }
            assert_eq!(heap.peek(), Some(&b), "the maximum is still on top afterward");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_iter_yields_references_in_order",
            claim: VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
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
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_iter_mut_writes_through",
            claim: VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC,
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
        /// through one is visible when the list is read back.
        #[kani::proof]
        fn verify_linked_list_iter_mut_writes_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();

            let mut list = LinkedList::new();
            list.push_back(value);
            for x in list.iter_mut() {
                *x = updated;
            }
            assert_eq!(list.pop_front(), Some(updated), "the write through iter_mut is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_linked_list_into_iter_yields_owned_values_in_order",
            claim: VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `.into_iter()` consumes the list, yielding its owned
        /// elements in front-to-back order.
        #[kani::proof]
        fn verify_linked_list_into_iter_yields_owned_values_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut list = LinkedList::new();
            list.push_back(a);
            list.push_back(b);
            let mut it = list.into_iter();
            assert_eq!(it.next(), Some(a));
            assert_eq!(it.next(), Some(b));
            assert_eq!(it.next(), None);
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
            harness: "verify_linked_list_extract_if_partitions_by_the_predicate",
            claim: VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC,
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
        /// here.
        #[kani::proof]
        fn verify_linked_list_extract_if_partitions_by_the_predicate() {
            fn is_even(x: &mut i32) -> bool {
                *x % 2 == 0
            }
            let mut list: LinkedList<i32> = LinkedList::from([1, 2, 3, 4]);
            let extracted: Vec<i32> = list
                .extract_if(is_even as fn(&mut i32) -> bool)
                .collect();
            assert_eq!(extracted, vec![2, 4], "extract_if removes exactly the matching elements");
            let remaining: Vec<i32> = list.into_iter().collect();
            assert_eq!(remaining, vec![1, 3], "extract_if leaves the non-matching elements, in order");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_drain_removes_and_yields_in_order",
            claim: VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC,
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
        /// and leaves the deque empty.
        #[kani::proof]
        fn verify_vec_deque_drain_removes_and_yields_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let drained: Vec<i32> = dq.drain(..).collect();
            assert_eq!(drained, vec![a, b], "drain yields every element in order");
            assert!(dq.is_empty(), "drain leaves the deque empty");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_iter_yields_references_in_order",
            claim: VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC,
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
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_iter_mut_writes_through",
            claim: VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC,
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
        /// through one is visible when the deque is read back.
        #[kani::proof]
        fn verify_vec_deque_iter_mut_writes_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(value);
            for x in dq.iter_mut() {
                *x = updated;
            }
            assert_eq!(dq.pop_front(), Some(updated), "the write through iter_mut is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_deque_into_iter_yields_owned_values_in_order",
            claim: VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `.into_iter()` consumes the deque, yielding its owned
        /// elements in front-to-back order.
        #[kani::proof]
        fn verify_vec_deque_into_iter_yields_owned_values_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            let mut dq = VecDeque::new();
            dq.push_back(a);
            dq.push_back(b);
            let mut it = dq.into_iter();
            assert_eq!(it.next(), Some(a));
            assert_eq!(it.next(), Some(b));
            assert_eq!(it.next(), None);
        }
    }
}
