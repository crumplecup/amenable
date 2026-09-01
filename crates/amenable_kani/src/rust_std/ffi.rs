//! `KaniWitness` impls for `core::ffi`.
//!
//! `c_void` is only ever used behind a raw pointer and has no safe
//! constructor or accessor — nothing a harness can build to check a
//! property of. It stays at the trusted disposition.

use std::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError};

use amenable_core::Evidence;
use amenable_std::{NonNulByte, RustStdStandard};

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_requires};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};

    pub(super) use crate::FallibleOperationReportsFailure;
    pub(super) use crate::FallibleOperationReportsSuccess;
    pub(super) use crate::IndexRecoversTheStoredElement;
}
#[cfg(kani)]
use mirror::{
    Ensures, FallibleOperationReportsFailure, FallibleOperationReportsSuccess,
    IndexRecoversTheStoredElement, Requires,
};

impl KaniWitness for RustStdStandard<CStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CStr>",
        "kani",
        || <RustStdStandard<CStr> as KaniWitness>::proof().to_string(),
    )
}

/// [`NonNulByte`] reuses the same harness rather than adding a new Kani
/// proof — it names the precondition every `CStr`/`CString`-family proof
/// in this crate already assumes, it doesn't prove anything new.
impl KaniWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(NonNulByte);

kani_requires!(NonNulByte, "amenable_std::NonNulByte", u8, |byte| byte != 0);

amenable_derive::harness! {
    kani, VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC, {
        /// `CStr::from_bytes_with_nul` accepts a nul-terminated byte
        /// sequence, and `.to_bytes()` reports its content without the
        /// terminator itself.
        #[kani::proof]
        fn verify_cstr_excludes_the_terminating_nul_from_to_bytes() {
            let byte: u8 = kani::any();
            kani::assume(<NonNulByte as Requires<crate::KaniVerifier>>::requires(byte));
            let bytes = [byte, 0];
            let cstr = CStr::from_bytes_with_nul(&bytes).unwrap();
            // Decomposed into a length check plus per-index byte checks
            // rather than a whole-slice equality comparison: coercing a
            // `&[u8]` and a fixed array to the same slice type for a
            // generic `(T, T)` equality forces CBMC into a
            // symbolic-length memcmp, timing out where the original
            // `PartialEq<[u8; N]>`-specialized `assert_eq!` did not --
            // the documented "symbolic-length memcmp" CBMC failure
            // pattern (see `alloc_ffi.rs`'s identical decomposition).
            assert!(RustStdStandard::<usize>::ensures((cstr.to_bytes().len(), 1)));
            assert!(
                IndexRecoversTheStoredElement::ensures((cstr.to_bytes()[0], byte)),
                "to_bytes excludes the terminating nul"
            );
            assert!(RustStdStandard::<usize>::ensures((
                cstr.to_bytes_with_nul().len(),
                2
            )));
            assert!(
                IndexRecoversTheStoredElement::ensures((cstr.to_bytes_with_nul()[0], byte)),
                "the retained representation includes the original terminator"
            );
            assert!(IndexRecoversTheStoredElement::ensures((
                cstr.to_bytes_with_nul()[1],
                0u8
            )));
        }
    }
}

impl KaniWitness for RustStdStandard<FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere".to_owned(),
            VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FromBytesUntilNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromBytesUntilNulError>",
        "kani",
        || <RustStdStandard<FromBytesUntilNulError> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC, {
        /// `CStr::from_bytes_until_nul` succeeds when a nul byte
        /// appears anywhere in the input (using everything up to it),
        /// and fails — producing this error — only when none is
        /// present at all.
        #[kani::proof]
        fn verify_from_bytes_until_nul_requires_a_nul_byte_somewhere() {
            let byte: u8 = kani::any();
            kani::assume(<NonNulByte as Requires<crate::KaniVerifier>>::requires(byte));
            let with_nul = [byte, 0, byte];
            assert!(
                FallibleOperationReportsSuccess::ensures(
                    CStr::from_bytes_until_nul(&with_nul).is_ok()
                ),
                "a nul byte anywhere in the input is accepted"
            );

            let without_nul = [byte, byte, byte];
            assert!(
                FallibleOperationReportsFailure::ensures(
                    CStr::from_bytes_until_nul(&without_nul).is_err()
                ),
                "no nul byte at all is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromBytesWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromBytesWithNulError>",
        "kani",
        || <RustStdStandard<FromBytesWithNulError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC, {
        /// Unlike `from_bytes_until_nul`, `CStr::from_bytes_with_nul`
        /// requires the nul to be exactly the last byte: no nul at all
        /// is rejected, and so is an interior nul with trailing data
        /// after it.
        #[kani::proof]
        fn verify_from_bytes_with_nul_requires_the_nul_only_at_the_end() {
            let byte: u8 = kani::any();
            kani::assume(<NonNulByte as Requires<crate::KaniVerifier>>::requires(byte));
            assert!(
                FallibleOperationReportsSuccess::ensures(
                    CStr::from_bytes_with_nul(&[byte, 0]).is_ok()
                ),
                "a nul as the last byte is accepted"
            );
            assert!(
                FallibleOperationReportsFailure::ensures(
                    CStr::from_bytes_with_nul(&[byte, byte]).is_err()
                ),
                "no nul byte at all is rejected"
            );
            assert!(
                FallibleOperationReportsFailure::ensures(
                    CStr::from_bytes_with_nul(&[0, byte]).is_err()
                ),
                "an interior nul with trailing data is rejected"
            );
        }
    }
}

impl_kani_witness_trusted!(core::ffi::c_void);
