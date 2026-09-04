use std::collections::{LinkedList, TryReserveError, VecDeque};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

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

    pub(super) use crate::CollectedSequenceMatchesExpected;
    pub(super) use crate::EmptiedContainerReportsEmpty;
    pub(super) use crate::FallibleOperationReportsFailure;
}
#[cfg(kani)]
use mirror::{
    Cell, CollectedSequenceMatchesExpected, EmptiedContainerReportsEmpty, Ensures,
    FallibleOperationReportsFailure,
};

impl KaniWitness for RustStdStandard<LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_is_fifo_through_back_and_front".to_owned(),
            VERIFY_LINKED_LIST_IS_FIFO_THROUGH_BACK_AND_FRONT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinkedList<i32>>",
        "kani",
        || <RustStdStandard<LinkedList<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<LinkedList<i32>>,
    "amenable_std::rust_std::RustStdStandard<LinkedList<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

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
            assert!(
                RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), Some(a))),
                "the first-pushed element comes out first"
            );
            assert!(RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), Some(b))));
            assert!(
                RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), None)),
                "popping an exhausted FIFO returns None"
            );
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_deque_pushes_and_pops_from_both_ends".to_owned(),
            VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<VecDeque<i32>>",
        "kani",
        || <RustStdStandard<VecDeque<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<VecDeque<i32>>,
    "amenable_std::rust_std::RustStdStandard<VecDeque<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

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
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_front(), Some(b))),
                "pop_front returns the front-pushed element"
            );
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_back(), Some(a))),
                "pop_back returns the back-pushed element"
            );
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_front(), None)),
                "popping the exhausted front returns None"
            );
            assert!(
                RustStdStandard::<VecDeque<i32>>::ensures((dq.pop_back(), None)),
                "popping the exhausted back returns None"
            );
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_reserve_rejects_an_impossible_capacity".to_owned(),
            VERIFY_TRY_RESERVE_REJECTS_AN_IMPOSSIBLE_CAPACITY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TryReserveError>",
        "kani",
        || <RustStdStandard<TryReserveError> as KaniWitness>::proof().to_string(),
    )
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
                FallibleOperationReportsFailure::ensures(v.try_reserve(usize::MAX).is_err()),
                "an impossible reservation is rejected, not aborted"
            );
            assert!(
                CollectedSequenceMatchesExpected::ensures((v, vec![first, second])),
                "a failed reservation preserves existing values"
            );
        }
    }
}
