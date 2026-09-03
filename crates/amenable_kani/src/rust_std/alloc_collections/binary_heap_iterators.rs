use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::shared_markers::VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::Ensures;
    pub(super) use std::cell::Cell;
    pub(super) use std::collections::BinaryHeap;

    pub(super) use crate::EmptiedContainerReportsEmpty;
    pub(super) use crate::PopRecoversTheStoredValue;
}
#[cfg(kani)]
use mirror::{BinaryHeap, Cell, EmptiedContainerReportsEmpty, Ensures, PopRecoversTheStoredValue};

impl KaniWitness for RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_drain_yields_every_pushed_element_once".to_owned(),
            VERIFY_BINARY_HEAP_DRAIN_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
        CheckedProof::new(
            "verify_binary_heap_into_iter_yields_every_pushed_element_once".to_owned(),
            VERIFY_BINARY_HEAP_INTO_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<std::collections::binary_heap::IntoIter<i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
        CheckedProof::new(
            "verify_binary_heap_iter_yields_every_pushed_element_once".to_owned(),
            VERIFY_BINARY_HEAP_ITER_YIELDS_EVERY_PUSHED_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
                <RustStdStandard<BinaryHeap<i32>> as Ensures<crate::KaniVerifier>>::ensures((
                    heap.len(),
                    2
                )),
                "iteration leaves every heap element in place"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((heap.pop(), Some(a.max(b)))),
                "iteration preserves the heap maximum"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((heap.pop(), Some(a.min(b)))),
                "iteration preserves the remaining element"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_peek_mut_exposes_the_maximum".to_owned(),
            VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// An `(actual, expected)` pair of `.peek()` results known to agree: a
/// container's peek accessor reveals a reference to the exact value
/// known to be there, without consuming it -- distinct from
/// `IteratorYieldsAReferenceToTheStoredValue` (`.next()`, consumes)
/// and `GetterRecoversTheStoredReference` (`OnceCell`/`OnceLock`'s
/// `.get()`) even though the `Ensures` impl body and the
/// lifetime-generic design are identical, same reasoning as keeping
/// those two separate from each other.
///
/// Independently hand-written as `assert_eq!(container.peek(),
/// Some(&value), ...)` at 3 real sites spanning `BinaryHeap::peek()`
/// and `Peekable::peek()`.
pub struct PeekRevealsTheStoredReference<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for PeekRevealsTheStoredReference<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for PeekRevealsTheStoredReference<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}
