//! `KaniWitness` impls for `core::ffi`.
//!
//! `c_void` is only ever used behind a raw pointer and has no safe
//! constructor or accessor — nothing a harness can build to check a
//! property of. It stays at the trusted disposition.

use std::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_std::{NonNulByte, RustStdStandard};

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted, kani_requires};

impl KaniWitness for RustStdStandard<CStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            claim: VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<CStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CStr>",
        verifier: "kani",
        describe: || <RustStdStandard<CStr> as KaniWitness>::proof().to_string(),
    }
}

/// [`NonNulByte`] reuses the same harness rather than adding a new Kani
/// proof — it names the precondition every `CStr`/`CString`-family proof
/// in this crate already assumes, it doesn't prove anything new.
impl KaniWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            claim: VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
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
            kani::assume(NonNulByte::requires(byte));
            let bytes = [byte, 0];
            let cstr = CStr::from_bytes_with_nul(&bytes).unwrap();
            assert_eq!(
                cstr.to_bytes(),
                &[byte],
                "to_bytes excludes the terminating nul"
            );
            assert_eq!(
                cstr.to_bytes_with_nul(),
                &bytes,
                "the retained representation includes the original terminator"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere".to_owned(),
            claim: VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromBytesUntilNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromBytesUntilNulError>",
        verifier: "kani",
        describe: || <RustStdStandard<FromBytesUntilNulError> as KaniWitness>::proof()
            .to_string(),
    }
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
            kani::assume(NonNulByte::requires(byte));
            let with_nul = [byte, 0, byte];
            assert!(
                CStr::from_bytes_until_nul(&with_nul).is_ok(),
                "a nul byte anywhere in the input is accepted"
            );

            let without_nul = [byte, byte, byte];
            assert!(
                CStr::from_bytes_until_nul(&without_nul).is_err(),
                "no nul byte at all is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromBytesWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            claim: VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromBytesWithNulError>",
        verifier: "kani",
        describe: || <RustStdStandard<FromBytesWithNulError> as KaniWitness>::proof().to_string(),
    }
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
            kani::assume(NonNulByte::requires(byte));
            assert!(
                CStr::from_bytes_with_nul(&[byte, 0]).is_ok(),
                "a nul as the last byte is accepted"
            );
            assert!(
                CStr::from_bytes_with_nul(&[byte, byte]).is_err(),
                "no nul byte at all is rejected"
            );
            assert!(
                CStr::from_bytes_with_nul(&[0, byte]).is_err(),
                "an interior nul with trailing data is rejected"
            );
        }
    }
}

impl_kani_witness_trusted!(core::ffi::c_void);
