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
    pub(super) use amenable_core::Ensures;
    pub(super) use std::cell::Cell;
    pub(super) use std::collections::LinkedList;

    pub(super) use crate::CollectedSequenceMatchesExpected;
    pub(super) use crate::EmptiedContainerReportsEmpty;
    pub(super) use crate::IteratorYieldsAReferenceToTheStoredValue;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
}
#[cfg(kani)]
use mirror::{
    Cell, CollectedSequenceMatchesExpected, EmptiedContainerReportsEmpty, Ensures,
    IteratorYieldsAReferenceToTheStoredValue, IteratorYieldsNoneWhenExhausted, LinkedList,
};

impl KaniWitness for RustStdStandard<std::collections::linked_list::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_iter_yields_references_in_order".to_owned(),
            VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::linked_list::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::linked_list::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&a))));
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&b))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            drop(it);
            assert!(
                RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), Some(a))),
                "iteration leaves the first value in place"
            );
            assert!(
                RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), Some(b))),
                "iteration leaves the second value in place"
            );
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_iter_mut_writes_through".to_owned(),
            VERIFY_LINKED_LIST_ITER_MUT_WRITES_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        "kani",
        || <RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(
                RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), Some(updated_first))),
                "the first write is visible at the front"
            );
            assert!(
                RustStdStandard::<LinkedList<i32>>::ensures((list.pop_front(), Some(updated_second))),
                "the second write preserves list order"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::collections::linked_list::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_into_iter_yields_owned_values_in_order".to_owned(),
            VERIFY_LINKED_LIST_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::collections::linked_list::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<std::collections::linked_list::IntoIter<i32>> as KaniWitness>::proof()
            .to_string(),
    )
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_extract_if_partitions_by_the_predicate".to_owned(),
            VERIFY_LINKED_LIST_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(
    RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "kani",
        || <RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

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
            assert!(
                RustStdStandard::<
                    std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
                >::ensures((extractor.next(is_even as fn(&mut i32) -> bool), Some(2))),
                "extract_if yields the first matching element"
            );
            assert!(
                RustStdStandard::<
                    std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
                >::ensures((extractor.next(is_even as fn(&mut i32) -> bool), Some(4))),
                "extract_if continues yielding later matching elements in order"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(
                    extractor.next(is_even as fn(&mut i32) -> bool)
                ),
                "extract_if exhausts once every match has been yielded"
            );
            assert!(
                CollectedSequenceMatchesExpected::ensures((extractor.into_remaining(), vec![1, 3])),
                "extract_if leaves the non-matching elements in place and order"
            );

            let mut extractor = crate::KaniLinkedListExtractIf::new(vec![1, 2, 3, 4]);
            assert!(
                RustStdStandard::<
                    std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
                >::ensures((extractor.next(is_even as fn(&mut i32) -> bool), Some(2))),
                "extract_if yields the first matching element"
            );
            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    extractor.into_remaining(),
                    vec![1, 3, 4]
                )),
                "dropping extract_if retains the unvisited suffix and prior non-matches"
            );
        }
    }
}
