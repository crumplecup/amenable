use std::collections::BinaryHeap;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

use crate::CheckedProof;
use crate::KaniWitness;
#[cfg(kani)]
use crate::PopRecoversTheStoredValue;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<BinaryHeap<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_pop_yields_the_maximum_first".to_owned(),
            VERIFY_BINARY_HEAP_POP_YIELDS_THE_MAXIMUM_FIRST_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BinaryHeap<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BinaryHeap<i32>>",
        "kani",
        || <RustStdStandard<BinaryHeap<i32>> as KaniWitness>::proof().to_string(),
    )
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
            assert!(
                PopRecoversTheStoredValue::ensures((heap.pop(), Some(a.max(b)))),
                "pop returns the greatest element first"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((heap.pop(), Some(a.min(b)))),
                "the second pop returns the remaining element"
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
