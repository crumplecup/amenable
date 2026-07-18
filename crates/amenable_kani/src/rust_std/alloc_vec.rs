//! `KaniWitness` impls for `alloc::vec`.

use std::vec::{
    Drain as VecDrain, ExtractIf as VecExtractIf, IntoIter as VecIntoIter, Splice, Vec,
};

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
            harness: "verify_vec_push_pop_round_trips",
            claim: VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC,
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
        /// returns the last element, leaving the Vec empty.
        #[kani::proof]
        fn verify_vec_push_pop_round_trips() {
            let value: i32 = kani::any();
            let mut v = Vec::new();
            v.push(value);
            assert_eq!(v.len(), 1);
            assert_eq!(v[0], value, "the pushed value is indexable");
            assert_eq!(v.pop(), Some(value), "pop returns the last pushed value");
            assert!(v.is_empty(), "pop leaves the Vec empty");
        }
    }
}

impl KaniWitness for RustStdStandard<VecDrain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_drain_removes_and_yields_in_order",
            claim: VERIFY_VEC_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<VecDrain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VecDrain<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<VecDrain<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_DRAIN_REMOVES_AND_YIELDS_IN_ORDER_SRC, {
        /// `.drain(..)` yields every element in order and leaves the
        /// Vec empty afterward.
        #[kani::proof]
        fn verify_vec_drain_removes_and_yields_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let mut v = vec![a, b];
            let drained: Vec<i32> = v.drain(..).collect();
            assert_eq!(drained, vec![a, b], "drain yields every element in order");
            assert!(v.is_empty(), "drain leaves the Vec empty");
        }
    }
}

impl KaniWitness for RustStdStandard<VecIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_into_iter_yields_owned_values_in_order",
            claim: VERIFY_VEC_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<VecIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VecIntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<VecIntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_VEC_INTO_ITER_YIELDS_OWNED_VALUES_IN_ORDER_SRC, {
        /// `.into_iter()` consumes the Vec, yielding its owned elements
        /// in order.
        #[kani::proof]
        fn verify_vec_into_iter_yields_owned_values_in_order() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let v = vec![a, b];
            let mut it = v.into_iter();
            assert_eq!(it.next(), Some(a));
            assert_eq!(it.next(), Some(b));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<VecExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_vec_extract_if_partitions_by_the_predicate",
            claim: VERIFY_VEC_EXTRACT_IF_PARTITIONS_BY_THE_PREDICATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<VecExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<VecExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<VecExtractIf<'static, i32, fn(&mut i32) -> bool>> as KaniWitness>::proof()
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
        }
    }
}

impl KaniWitness for RustStdStandard<Splice<'static, VecIntoIter<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_splice_replaces_a_range_and_yields_what_it_removed",
            claim: VERIFY_SPLICE_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Splice<'static, VecIntoIter<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Splice<'static, VecIntoIter<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Splice<'static, VecIntoIter<i32>>> as KaniWitness>::proof()
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
        }
    }
}
