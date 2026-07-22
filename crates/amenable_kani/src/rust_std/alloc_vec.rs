//! `KaniWitness` impls for `alloc::vec`.

use std::vec::Vec;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Vec<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_push_pop_round_trips".to_owned(),
            claim: VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Vec<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Vec<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC, {
        /// `push` appends and is indexable, and `pop` removes and
        /// returns the last element, leaving the Vec empty. Also
        /// checked with a drop-instrumented, non-`Copy` witness type:
        /// `pop()` transfers ownership out *without* dropping the
        /// value, and dropping the Vec drops every remaining element
        /// exactly once — `i32` alone has no drop glue to distinguish
        /// "moved out cleanly" from "dropped early" or "leaked".
        #[kani::proof]
        fn verify_vec_push_pop_round_trips() {
            let value = <i32 as crate::KaniCompose>::kani_any();
            let mut v = <Vec<i32> as crate::KaniCompose>::kani_depth0();
            v.push(value);
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], value, "the pushed value is indexable");
            assert_eq!(v.pop(), Some(value), "pop returns the last pushed value");
            assert!(v.is_empty(), "pop leaves the Vec empty");
            assert_eq!(v.pop(), None, "popping an exhausted Vec returns None");

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witnesses = Vec::new();
            witnesses.push(DropWitness { drop_count: drop_count.clone() });
            witnesses.push(DropWitness { drop_count: drop_count.clone() });
            let popped = witnesses.pop().unwrap();
            assert_eq!(drop_count.get(), 0, "pop does not drop the returned value");
            drop(popped);
            assert_eq!(drop_count.get(), 1, "the popped value drops once its owner drops it");
            drop(witnesses);
            assert_eq!(drop_count.get(), 2, "dropping the Vec drops the remaining element");
        }
    }
}

impl KaniWitness for RustStdStandard<std::vec::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_drain_removes_and_yields_in_order".to_owned(),
            claim: VERIFY_VEC_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::vec::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::Drain<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::vec::Drain<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC, {
        /// `.drain(..)` yields every element in order and leaves the
        /// Vec empty afterward.
        #[kani::proof]
        fn verify_vec_drain_removes_and_yields_in_order() {
            let mut v = <Vec<i32> as crate::KaniCompose>::kani_depth2();
            let expected = v.clone();
            let drained: Vec<i32> = v.drain(..).collect();
            assert_eq!(drained, expected, "drain yields every element in order");
            assert!(v.is_empty(), "drain leaves the Vec empty");

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witnesses = vec![
                DropWitness { drop_count: drop_count.clone() },
                DropWitness { drop_count: drop_count.clone() },
                DropWitness { drop_count: drop_count.clone() },
            ];
            let mut drain = witnesses.drain(..);
            let first = drain.next().unwrap();
            assert_eq!(drop_count.get(), 0, "drain transfers a yielded value without dropping it");
            drop(first);
            assert_eq!(drop_count.get(), 1, "the caller drops the yielded value exactly once");
            drop(drain);
            assert_eq!(drop_count.get(), 3, "dropping an unfinished drain drops every remaining value");
            assert!(witnesses.is_empty(), "dropping an unfinished drain leaves the Vec empty");
        }
    }
}

impl KaniWitness for RustStdStandard<std::vec::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_into_iter_yields_owned_values_in_order".to_owned(),
            claim: VERIFY_VEC_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::vec::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::vec::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `.into_iter()` consumes the Vec, yielding its owned elements
        /// in order.
        #[kani::proof]
        fn verify_vec_into_iter_yields_owned_values_in_order() {
            let expected = <Vec<i32> as crate::KaniCompose>::kani_depth2();
            let mut it = expected.clone().into_iter();
            assert_eq!(it.next(), Some(expected[0]));
            assert_eq!(it.next(), Some(expected[1]));
            assert_eq!(it.next(), None);

            struct DropWitness {
                drop_count: std::rc::Rc<std::cell::Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(std::cell::Cell::new(0));
            let mut witness_iter = vec![
                DropWitness { drop_count: drop_count.clone() },
                DropWitness { drop_count: drop_count.clone() },
                DropWitness { drop_count: drop_count.clone() },
            ]
            .into_iter();
            let first = witness_iter.next().unwrap();
            assert_eq!(drop_count.get(), 0, "IntoIter transfers a yielded value without dropping it");
            drop(first);
            assert_eq!(drop_count.get(), 1, "the caller drops the yielded value exactly once");
            drop(witness_iter);
            assert_eq!(drop_count.get(), 3, "dropping IntoIter drops every remaining value");
        }
    }
}

impl KaniWitness for RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_extract_if_partitions_by_the_predicate".to_owned(),
            claim: VERIFY_VEC_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC, {
        /// `.extract_if(.., predicate)` removes and yields exactly the
        /// elements matching the predicate, leaving the rest — in
        /// order — behind in the Vec.
        #[kani::proof]
        fn verify_vec_extract_if_partitions_by_the_predicate() {
            fn is_even(x: &mut i32) -> bool {
                *x % 2 == 0
            }
            let mut v = vec![1, 2, 3, 4];
            let extracted: Vec<i32> = v.extract_if(.., is_even as fn(&mut i32) -> bool).collect();
            assert_eq!(extracted, vec![2, 4], "extract_if removes exactly the matching elements");
            assert_eq!(v, vec![1, 3], "extract_if leaves the non-matching elements, in order");

            let mut early_drop = vec![1, 2, 3, 4];
            let mut extractor = early_drop.extract_if(.., is_even as fn(&mut i32) -> bool);
            assert_eq!(extractor.next(), Some(2), "extract_if yields the first matching element");
            drop(extractor);
            assert_eq!(
                early_drop,
                vec![1, 3, 4],
                "dropping ExtractIf preserves the unvisited elements in order"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_splice_replaces_a_range_and_yields_what_it_removed".to_owned(),
            claim: VERIFY_SPLICE_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SPLICE_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC, {
        /// `.splice(range, replacement)` replaces the given range with
        /// the replacement iterator's elements, and yields exactly the
        /// elements it removed.
        #[kani::proof]
        fn verify_splice_replaces_a_range_and_yields_what_it_removed() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let c: i32 = kani::any();
            let mut v = vec![a, b, c];
            let removed: Vec<i32> = v.splice(1..2, vec![9, 8]).collect();
            assert_eq!(removed, vec![b], "splice yields exactly the elements it removed");
            assert_eq!(v, vec![a, 9, 8, c], "splice replaces the range with the given elements");

            let mut early_drop = vec![a, b, c];
            let mut splice = early_drop.splice(1..2, vec![9, 8]);
            assert_eq!(splice.next(), Some(b), "splice first yields the removed element");
            drop(splice);
            assert_eq!(
                early_drop,
                vec![a, 9, 8, c],
                "dropping an unfinished splice still completes replacement"
            );
        }
    }
}
