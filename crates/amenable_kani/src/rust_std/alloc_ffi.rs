//! `KaniWitness` impls for `alloc::ffi`.

use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cstring_excludes_the_terminator_and_rejects_interior_nul",
            claim: VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<CString>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CString>",
        verifier: "kani",
        describe: || <RustStdStandard<CString> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC, {
        /// `CString::new` appends its own nul terminator (`.as_bytes()`
        /// excludes it), and rejects any input that already contains
        /// an interior nul byte.
        #[kani::proof]
        fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul() {
            let byte: u8 = kani::any();
            kani::assume(byte != 0);
            let cstring = CString::new(vec![byte]).unwrap();
            assert_eq!(cstring.as_bytes(), &[byte], "as_bytes excludes the terminator");

            let with_interior_nul = vec![byte, 0, byte];
            assert!(
                CString::new(with_interior_nul).is_err(),
                "an interior nul byte is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_vec_with_nul_requires_the_nul_only_at_the_end",
            claim: VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromVecWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromVecWithNulError>",
        verifier: "kani",
        describe: || <RustStdStandard<FromVecWithNulError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// `CString::from_vec_with_nul` requires the nul to be exactly
        /// the last byte, same rule as `CStr::from_bytes_with_nul` on
        /// the borrowed carrier.
        #[kani::proof]
        fn verify_from_vec_with_nul_requires_the_nul_only_at_the_end() {
            let byte: u8 = kani::any();
            kani::assume(byte != 0);
            assert!(
                CString::from_vec_with_nul(vec![byte, 0]).is_ok(),
                "a nul as the last byte is accepted"
            );
            assert!(
                CString::from_vec_with_nul(vec![byte, byte]).is_err(),
                "no nul byte at all is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_into_string_error_recovers_the_original_cstring",
            claim: VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<IntoStringError>",
        verifier: "kani",
        describe: || <RustStdStandard<IntoStringError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC, {
        /// `CString::into_string` fails when the bytes aren't valid
        /// UTF-8, and the error doesn't discard them: `.into_cstring()`
        /// recovers exactly the original `CString`.
        #[kani::proof]
        fn verify_into_string_error_recovers_the_original_cstring() {
            let invalid = CString::new(vec![0xFFu8]).unwrap();
            let original_bytes = invalid.as_bytes().to_vec();
            let err = invalid.into_string().unwrap_err();
            let recovered = err.into_cstring();
            assert_eq!(
                recovered.as_bytes(),
                &original_bytes[..],
                "into_cstring recovers the original CString"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_nul_error_reports_the_interior_nuls_position",
            claim: VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<NulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<NulError>",
        verifier: "kani",
        describe: || <RustStdStandard<NulError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC, {
        /// `NulError::nul_position` reports exactly where the
        /// rejecting nul byte was, not just that one was found.
        #[kani::proof]
        fn verify_nul_error_reports_the_interior_nuls_position() {
            let byte: u8 = kani::any();
            kani::assume(byte != 0);
            let err = CString::new(vec![byte, 0, byte]).unwrap_err();
            assert_eq!(err.nul_position(), 1, "nul_position reports the nul's index");
        }
    }
}
