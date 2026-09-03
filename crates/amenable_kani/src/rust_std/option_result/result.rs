//! `KaniWitness` impls for `core::result`.

use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::RustStdStandard;

use super::super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

#[cfg(kani)]
use crate::{
    AccessorRecoversTheExpectedValue, DerefReflectsTheStoredValue,
    IteratorYieldsAReferenceToTheStoredValue, IteratorYieldsNoneWhenExhausted,
};
#[cfg(kani)]
use amenable_core::Ensures;

/// A fallible operation's outcome, once computed, known to report failure.
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_owned(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
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
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FallibleOperationReportsFailure",
        "kani",
        || <FallibleOperationReportsFailure as KaniWitness>::proof().to_string(),
    )
}

/// The positive counterpart to [`FallibleOperationReportsFailure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct FallibleOperationReportsSuccess;

impl KaniWitness for FallibleOperationReportsSuccess {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_owned(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(FallibleOperationReportsSuccess);

kani_ensures!(
    FallibleOperationReportsSuccess,
    "amenable_kani::FallibleOperationReportsSuccess",
    bool,
    |is_ok| is_ok
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::FallibleOperationReportsSuccess",
        "kani",
        || <FallibleOperationReportsSuccess as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_ok_and_err_are_disjoint".to_owned(),
            VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        "kani",
        || <RustStdStandard<Result<i32, i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Result<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
    (Result<i32, i32>, Result<i32, i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RESULT_OK_AND_ERR_ARE_DISJOINT_SRC, {
        /// `Ok` round-trips its value through `unwrap`, and `Err`
        /// round-trips its value through `unwrap_err`.
        #[kani::proof]
        fn verify_result_ok_and_err_are_disjoint() {
            let value: i32 = kani::any();
            let ok: Result<i32, i32> = Ok(value);
            assert!(FallibleOperationReportsSuccess::ensures(ok.is_ok()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((ok.unwrap(), value)),
                "Ok round-trips its value"
            );

            let err_value: i32 = kani::any();
            let err: Result<i32, i32> = Err(err_value);
            assert!(FallibleOperationReportsFailure::ensures(err.is_err()));
            assert!(
                AccessorRecoversTheExpectedValue::ensures((err.unwrap_err(), err_value)),
                "Err round-trips its value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_into_iter_yields_the_ok_value_only".to_owned(),
            VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::result::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::IntoIter<i32>>",
        "kani",
        || <RustStdStandard<core::result::IntoIter<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::result::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<core::result::IntoIter<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RESULT_INTO_ITER_YIELDS_THE_OK_VALUE_ONLY_SRC, {
        /// `Ok(value).into_iter()` yields the owned value once, then stops;
        /// `Err(_).into_iter()` yields nothing.
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_iter_yields_a_reference_to_the_ok_value".to_owned(),
            VERIFY_RESULT_ITER_YIELDS_A_REFERENCE_TO_THE_OK_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::result::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<core::result::Iter<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&value))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(it.next()));
        }
    }
}

impl KaniWitness for RustStdStandard<core::result::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_result_iter_mut_writes_through_to_the_result".to_owned(),
            VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::result::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::result::IterMut<'static, i32>>",
        "kani",
        || <RustStdStandard<core::result::IterMut<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RESULT_ITER_MUT_WRITES_THROUGH_TO_THE_RESULT_SRC, {
        /// `.iter_mut()` yields a mutable reference to the `Ok` value, and a
        /// write through it is visible in the `Result` afterward.
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
            assert!(
                RustStdStandard::<Result<i32, i32>>::ensures((res, Ok(updated))),
                "the write through iter_mut is visible"
            );
        }
    }
}
