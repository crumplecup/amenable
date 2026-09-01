use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};
    pub(super) use std::cell::Cell;
    pub(super) use std::collections::VecDeque;

    pub(super) use crate::CollectedSequenceMatchesExpected;
    pub(super) use crate::EmptiedContainerReportsEmpty;
    pub(super) use crate::IteratorYieldsAReferenceToTheStoredValue;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
}
#[cfg(kani)]
use mirror::{
    Cell, CollectedSequenceMatchesExpected, EmptiedContainerReportsEmpty, Ensures,
    IteratorYieldsAReferenceToTheStoredValue, IteratorYieldsNoneWhenExhausted, Requires, VecDeque,
};

impl KaniWitness for RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_drain_removes_and_yields_in_order".to_owned(),
            VERIFY_VEC_DEQUE_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Drain<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::vec_deque::Drain<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(
                CollectedSequenceMatchesExpected::ensures((drained, vec![a, b])),
                "drain yields every element in order"
            );
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
        CheckedProof::new(
            "verify_vec_deque_iter_yields_references_in_order".to_owned(),
            VERIFY_VEC_DEQUE_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&a))));
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&b))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            drop(it);
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_front(), Some(a))),
                "iteration leaves the first value in place"
            );
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_front(), Some(b))),
                "iteration leaves the second value in place"
            );
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
        CheckedProof::new(
            "verify_vec_deque_iter_mut_writes_through".to_owned(),
            VERIFY_VEC_DEQUE_ITER_MUT_WRITES_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_front(), Some(updated_first))),
                "the first write is visible at the front"
            );
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_front(), Some(updated_second))),
                "the second write preserves deque order"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::vec_deque::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_into_iter_yields_owned_values_in_order".to_owned(),
            VERIFY_VEC_DEQUE_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::vec_deque::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<std::collections::vec_deque::IntoIter<i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
