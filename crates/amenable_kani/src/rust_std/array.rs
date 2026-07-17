//! `KaniWitness` impls for `core::array`.

use std::array::TryFromSliceError;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<TryFromSliceError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_from_slice_rejects_a_length_mismatch",
            claim: VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TryFromSliceError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryFromSliceError>",
        verifier: "kani",
        describe: || <RustStdStandard<TryFromSliceError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC, {
        /// `<[T; N]>::try_from` succeeds only when the slice's length
        /// matches `N` exactly, and round-trips the elements when it
        /// does.
        #[kani::proof]
        fn verify_try_from_slice_rejects_a_length_mismatch() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            let matching: &[i32] = &[a, b];
            let arr: Result<[i32; 2], TryFromSliceError> = matching.try_into();
            assert_eq!(arr, Ok([a, b]), "a matching-length slice round-trips into the array");

            let mismatched: &[i32] = &[a, b, a];
            let bad: Result<[i32; 2], TryFromSliceError> = mismatched.try_into();
            assert!(bad.is_err(), "a mismatched-length slice is rejected");
        }
    }
}
