//! `KaniWitness` impls for `alloc::ffi`.

use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};
    pub(super) use amenable_std::NonNulByte;

    pub(super) use crate::FallibleOperationReportsFailure;
    pub(super) use crate::FallibleOperationReportsSuccess;
    pub(super) use crate::IndexRecoversTheStoredElement;
}
#[cfg(kani)]
use mirror::{
    Ensures, FallibleOperationReportsFailure, FallibleOperationReportsSuccess,
    IndexRecoversTheStoredElement, NonNulByte, Requires,
};

impl KaniWitness for RustStdStandard<CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul".to_owned(),
            VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CString>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CString>",
        "kani",
        || <RustStdStandard<CString> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC, {
        /// `CString::new` appends its own nul terminator (`.as_bytes()`
        /// excludes it), and rejects any input that already contains
        /// an interior nul byte.
        #[kani::proof]
        fn verify_cstring_excludes_the_terminator_and_rejects_interior_nul() {
            let byte: u8 = kani::any();
            kani::assume(<NonNulByte as Requires<crate::KaniVerifier>>::requires(byte));
            let cstring = CString::new(vec![byte]).unwrap();
            // Decomposed into a length check plus per-index byte checks
            // rather than a whole-slice `AccessorRecoversTheExpectedValue`
            // comparison: coercing `.as_bytes()` (a `&[u8]`) and a fixed
            // array literal to the same slice type for a generic `(T, T)`
            // equality forces CBMC into a symbolic-length memcmp, timing
            // out where the original `PartialEq<[u8; N]>`-specialized
            // `assert_eq!` (with the length known at compile time on one
            // side) did not -- confirmed empirically, the documented
            // "symbolic-length memcmp" CBMC failure pattern.
            assert!(RustStdStandard::<usize>::ensures((cstring.as_bytes().len(), 1)));
            assert!(
                IndexRecoversTheStoredElement::ensures((cstring.as_bytes()[0], byte)),
                "as_bytes excludes the terminator"
            );
            assert!(RustStdStandard::<usize>::ensures((
                cstring.as_bytes_with_nul().len(),
                2
            )));
            assert!(
                IndexRecoversTheStoredElement::ensures((cstring.as_bytes_with_nul()[0], byte)),
                "as_bytes_with_nul retains exactly the appended terminator"
            );
            assert!(IndexRecoversTheStoredElement::ensures((
                cstring.as_bytes_with_nul()[1],
                0u8
            )));

            let with_interior_nul = vec![byte, 0, byte];
            assert!(
                FallibleOperationReportsFailure::ensures(CString::new(with_interior_nul).is_err()),
                "an interior nul byte is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FromVecWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromVecWithNulError>",
        "kani",
        || <RustStdStandard<FromVecWithNulError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// `CString::from_vec_with_nul` requires the nul to be exactly
        /// the last byte, same rule as `CStr::from_bytes_with_nul` on
        /// the borrowed carrier.
        #[kani::proof]
        fn verify_from_vec_with_nul_requires_the_nul_only_at_the_end() {
            let byte: u8 = kani::any();
            kani::assume(<NonNulByte as Requires<crate::KaniVerifier>>::requires(byte));
            assert!(
                FallibleOperationReportsSuccess::ensures(
                    CString::from_vec_with_nul(vec![byte, 0]).is_ok()
                ),
                "a nul as the last byte is accepted"
            );
            assert!(
                FallibleOperationReportsFailure::ensures(
                    CString::from_vec_with_nul(vec![byte, byte]).is_err()
                ),
                "no nul byte at all is rejected"
            );
            assert!(
                FallibleOperationReportsFailure::ensures(
                    CString::from_vec_with_nul(vec![byte, 0, byte]).is_err()
                ),
                "a nul before the final byte is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_into_string_error_recovers_the_original_cstring".to_owned(),
            VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<IntoStringError>",
        "kani",
        || <RustStdStandard<IntoStringError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC, {
        /// `CString::into_string` fails when the bytes aren't valid
        /// UTF-8, and the error doesn't discard them: `.into_cstring()`
        /// recovers exactly the original `CString`.
        #[kani::proof]
        fn verify_into_string_error_recovers_the_original_cstring() {
            let invalid = CString::new(vec![0xFFu8, b'x']).unwrap();
            let err = invalid.into_string().unwrap_err();
            let recovered = err.into_cstring();
            // Decomposed the same way as `verify_cstring_excludes_the_
            // terminator_and_rejects_interior_nul` -- see that harness's
            // comment for why a whole-slice `AccessorRecoversTheExpectedValue`
            // comparison here times out under CBMC.
            assert!(RustStdStandard::<usize>::ensures((recovered.as_bytes().len(), 2)));
            assert!(
                IndexRecoversTheStoredElement::ensures((recovered.as_bytes()[0], 0xFFu8)),
                "into_cstring recovers the original CString"
            );
            assert!(IndexRecoversTheStoredElement::ensures((
                recovered.as_bytes()[1],
                b'x'
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nul_error_reports_the_interior_nuls_position".to_owned(),
            VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NulError>",
        "kani",
        || <RustStdStandard<NulError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC, {
        /// `NulError::nul_position` reports exactly where the
        /// rejecting nul byte was, not just that one was found.
        #[kani::proof]
        fn verify_nul_error_reports_the_interior_nuls_position() {
            let byte: u8 = kani::any();
            kani::assume(<NonNulByte as Requires<crate::KaniVerifier>>::requires(byte));
            let err = CString::new(vec![byte, 0, byte]).unwrap_err();
            assert!(
                RustStdStandard::<usize>::ensures((err.nul_position(), 1)),
                "nul_position reports the nul's index"
            );

            let first_of_two = CString::new(vec![byte, 0, 0, byte]).unwrap_err();
            assert!(
                RustStdStandard::<usize>::ensures((first_of_two.nul_position(), 1)),
                "nul_position reports the first interior nul"
            );
        }
    }
}
