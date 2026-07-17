//! `KaniWitness` impls for `core::option` and `core::result`.

use std::option::{IntoIter as OptionIntoIter, Iter as OptionIter, IterMut as OptionIterMut};
use std::result::{IntoIter as ResultIntoIter, Iter as ResultIter, IterMut as ResultIterMut};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_some_and_none_are_disjoint",
            claim: VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Option<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC, {
        /// `Some` round-trips its value through `unwrap`, and `None`
        /// falls back to `unwrap_or`'s default.
        #[kani::proof]
        fn verify_option_some_and_none_are_disjoint() {
            let value: i32 = kani::any();
            let some: Option<i32> = Some(value);
            assert!(some.is_some());
            assert_eq!(some.unwrap(), value, "Some round-trips its value");

            let none: Option<i32> = None;
            assert!(none.is_none());
            assert_eq!(none.unwrap_or(0), 0, "None falls back to the default");
        }
    }
}

impl KaniWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_ok_and_err_are_disjoint",
            claim: VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Result<i32, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, {
        /// `Ok` round-trips its value through `unwrap`, and `Err`
        /// round-trips its value through `unwrap_err`.
        #[kani::proof]
        fn verify_result_ok_and_err_are_disjoint() {
            let value: i32 = kani::any();
            let ok: Result<i32, i32> = Ok(value);
            assert!(ok.is_ok());
            assert_eq!(ok.unwrap(), value, "Ok round-trips its value");

            let err_value: i32 = kani::any();
            let err: Result<i32, i32> = Err(err_value);
            assert!(err.is_err());
            assert_eq!(err.unwrap_err(), err_value, "Err round-trips its value");
        }
    }
}

impl KaniWitness for RustStdStandard<OptionIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_into_iter_yields_zero_or_one_owned_value",
            claim: VERIFY_OPTION_INTO_ITER_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OptionIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OptionIntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<OptionIntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_INTO_ITER_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC, {
        /// `Some(value).into_iter()` yields the owned value once, then
        /// stops; `None.into_iter()` yields nothing at all.
        #[kani::proof]
        fn verify_option_into_iter_yields_zero_or_one_owned_value() {
            let value: i32 = kani::any();
            let mut it = Some(value).into_iter();
            assert_eq!(it.next(), Some(value));
            assert_eq!(it.next(), None);

            let mut empty_it = None::<i32>.into_iter();
            assert_eq!(empty_it.next(), None, "None's into_iter yields nothing");
        }
    }
}

impl KaniWitness for RustStdStandard<OptionIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_yields_zero_or_one_reference",
            claim: VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OptionIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OptionIter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<OptionIter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC, {
        /// `.iter()` borrows instead of consuming: it yields a shared
        /// reference to the value, not the value itself.
        #[kani::proof]
        fn verify_option_iter_yields_zero_or_one_reference() {
            let value: i32 = kani::any();
            let opt = Some(value);
            let mut it = opt.iter();
            assert_eq!(it.next(), Some(&value));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<OptionIterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_mut_writes_through_to_the_option",
            claim: VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OptionIterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OptionIterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<OptionIterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC, {
        /// `.iter_mut()` yields a mutable reference to the contained
        /// value, and a write through it is visible in the `Option`
        /// afterward.
        #[kani::proof]
        fn verify_option_iter_mut_writes_through_to_the_option() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mut opt = Some(value);
            {
                let mut it = opt.iter_mut();
                let first = it.next().unwrap();
                assert_eq!(*first, value);
                *first = updated;
                assert_eq!(it.next(), None);
            }
            assert_eq!(opt, Some(updated), "the write through iter_mut is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<ResultIntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_into_iter_yields_the_ok_value_only",
            claim: VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ResultIntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ResultIntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ResultIntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC, {
        /// `Ok(value).into_iter()` yields the owned value once, then
        /// stops; `Err(_).into_iter()` yields nothing — the `Err`
        /// payload is never exposed through iteration.
        #[kani::proof]
        fn verify_result_into_iter_yields_the_ok_value_only() {
            let value: i32 = kani::any();
            let mut it = Ok::<i32, i32>(value).into_iter();
            assert_eq!(it.next(), Some(value));
            assert_eq!(it.next(), None);

            let err_value: i32 = kani::any();
            let mut empty_it = Err::<i32, i32>(err_value).into_iter();
            assert_eq!(empty_it.next(), None, "Err's into_iter yields nothing");
        }
    }
}

impl KaniWitness for RustStdStandard<ResultIter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_iter_yields_a_reference_to_the_ok_value",
            claim: VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ResultIter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ResultIter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ResultIter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC, {
        /// `.iter()` borrows instead of consuming: it yields a shared
        /// reference to the `Ok` value, not the value itself.
        #[kani::proof]
        fn verify_result_iter_yields_a_reference_to_the_ok_value() {
            let value: i32 = kani::any();
            let res: Result<i32, i32> = Ok(value);
            let mut it = res.iter();
            assert_eq!(it.next(), Some(&value));
            assert_eq!(it.next(), None);
        }
    }
}

impl KaniWitness for RustStdStandard<ResultIterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_iter_mut_writes_through_to_the_result",
            claim: VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ResultIterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ResultIterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ResultIterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC, {
        /// `.iter_mut()` yields a mutable reference to the `Ok` value,
        /// and a write through it is visible in the `Result` afterward.
        #[kani::proof]
        fn verify_result_iter_mut_writes_through_to_the_result() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mut res: Result<i32, i32> = Ok(value);
            {
                let mut it = res.iter_mut();
                let first = it.next().unwrap();
                assert_eq!(*first, value);
                *first = updated;
                assert_eq!(it.next(), None);
            }
            assert_eq!(res, Ok(updated), "the write through iter_mut is visible");
        }
    }
}
