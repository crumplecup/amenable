//! `KaniWitness` impls for `core::option` and `core::result`.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_some_and_none_are_disjoint".to_owned(),
            claim: VERIFY_OPTION_SOME_AND_NONE_ARE_DISJOINT_SRC.to_owned(),
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

/// A fallible operation's outcome, once computed, known to report
/// failure: whatever real reason (a length mismatch, an out-of-range
/// conversion, a rejected alignment, ...) the operation's own body
/// decided on, `.is_err()` reflects it.
///
/// Independently hand-written as `assert!(result.is_err(), ...)` at 8
/// real sites across `TryFromSliceError`, `char::try_from`'s surrogate
/// rejection, `TryFromCharError`, `Layout::from_size_align`'s alignment
/// rejection, `TryFromIntError`, a `Result<i32, i32>` freshly
/// constructed as `Err`, and `HandleOrInvalid`'s sentinel-conversion
/// rejection -- the identical claim shape regardless of which fallible
/// operation or real rejection reason produced the `Err`. Same "trust
/// the body, name the flag" reasoning as `EmptiedContainerReportsEmpty`
/// (`rust_std::alloc_collections`): needs no type parameter, since
/// every real site already computes the `bool` before asserting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FallibleOperationReportsFailure;

impl KaniWitness for FallibleOperationReportsFailure {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_ok_and_err_are_disjoint".to_owned(),
            claim: VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(FallibleOperationReportsFailure);

kani_ensures!(
    FallibleOperationReportsFailure,
    "amenable_kani::FallibleOperationReportsFailure",
    bool,
    |is_err| is_err
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::FallibleOperationReportsFailure",
        verifier: "kani",
        describe: || <FallibleOperationReportsFailure as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_ok_and_err_are_disjoint".to_owned(),
            claim: VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
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
            assert!(FallibleOperationReportsFailure::ensures(err.is_err()));
            assert_eq!(err.unwrap_err(), err_value, "Err round-trips its value");
        }
    }
}

impl KaniWitness for RustStdStandard<core::option::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_into_iter_yields_zero_or_one_owned_value".to_owned(),
            claim: VERIFY_OPTION_INTO_ITER_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::option::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::option::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::option::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<core::option::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<core::option::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_OPTION_INTO_ITER_YIELDS_ZERO_OR_ONE_OWNED_VALUE_SRC, {
        /// `Some(value).into_iter()` yields the owned value once, then
        /// stops; `None.into_iter()` yields nothing at all.
        #[kani::proof]
        fn verify_option_into_iter_yields_zero_or_one_owned_value() {
            let value: i32 = kani::any();
            let mut it = Some(value).into_iter();
            assert!(RustStdStandard::<core::option::IntoIter<i32>>::ensures((it.next(), Some(value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));

            let mut empty_it = None::<i32>.into_iter();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(empty_it.next()),
                "None's into_iter yields nothing"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::option::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_yields_zero_or_one_reference".to_owned(),
            claim: VERIFY_OPTION_ITER_YIELDS_ZERO_OR_ONE_REFERENCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::option::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::option::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::option::Iter<'static, i32>> as KaniWitness>::proof()
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
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<core::option::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_option_iter_mut_writes_through_to_the_option".to_owned(),
            claim: VERIFY_OPTION_ITER_MUT_WRITES_THROUGH_TO_THE_OPTION_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::option::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::option::IterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::option::IterMut<'static, i32>> as KaniWitness>::proof()
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
                assert!(DerefReflectsTheStoredValue::ensures((*first, value)));
                *first = updated;
                assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            }
            assert_eq!(opt, Some(updated), "the write through iter_mut is visible");
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_into_iter_yields_the_ok_value_only".to_owned(),
            claim: VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::result::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::result::IntoIter<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::result::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<core::result::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<core::result::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC, {
        /// `Ok(value).into_iter()` yields the owned value once, then
        /// stops; `Err(_).into_iter()` yields nothing — the `Err`
        /// payload is never exposed through iteration.
        #[kani::proof]
        fn verify_result_into_iter_yields_the_ok_value_only() {
            let value: i32 = kani::any();
            let mut it = Ok::<i32, i32>(value).into_iter();
            assert!(RustStdStandard::<core::result::IntoIter<i32>>::ensures((it.next(), Some(value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));

            let err_value: i32 = kani::any();
            let mut empty_it = Err::<i32, i32>(err_value).into_iter();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(empty_it.next()),
                "Err's into_iter yields nothing"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_iter_yields_a_reference_to_the_ok_value".to_owned(),
            claim: VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::result::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::result::Iter<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::result::Iter<'static, i32>> as KaniWitness>::proof()
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
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_result_iter_mut_writes_through_to_the_result".to_owned(),
            claim: VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::result::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::result::IterMut<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<core::result::IterMut<'static, i32>> as KaniWitness>::proof()
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
                assert!(DerefReflectsTheStoredValue::ensures((*first, value)));
                *first = updated;
                assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
            }
            assert_eq!(res, Ok(updated), "the write through iter_mut is visible");
        }
    }
}
