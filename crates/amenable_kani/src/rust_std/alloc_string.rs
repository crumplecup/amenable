//! `KaniWitness` impls for `alloc::string`.

use std::string::{FromUtf8Error, FromUtf16Error};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

// Written as the fully-qualified `std::string::Drain<'static>` throughout:
// its bare name collides with `alloc::vec::Drain` and the
// `alloc::collections::{binary_heap,vec_deque}::Drain` carriers, and this
// qualification is what lets `amenable_std`'s matching evidence string
// disambiguate them for tooling reading the registry (e.g. `elicit_doc`'s
// coverage report).
impl KaniWitness for RustStdStandard<std::string::Drain<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_drain_removes_and_yields_the_content",
            claim: VERIFY_STRING_DRAIN_REMOVES_AND_YIELDS_THE_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::string::Drain<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::string::Drain<'static>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_STRING_DRAIN_REMOVES_AND_YIELDS_THE_CONTENT_SRC, {
        /// `.drain(..)` yields the String's content and leaves it
        /// empty afterward, for any (symbolic) single-character string.
        #[kani::proof]
        fn verify_string_drain_removes_and_yields_the_content() {
            let byte: u8 = kani::any();
            kani::assume(byte < 128);
            let c = byte as char;
            let mut s = c.to_string();
            let drained: String = s.drain(..).collect();
            assert_eq!(drained, c.to_string(), "drain yields the string's content");
            assert!(s.is_empty(), "drain leaves the string empty");
        }
    }
}

impl KaniWitness for RustStdStandard<FromUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_utf16_rejects_a_lone_surrogate",
            claim: VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromUtf16Error>",
        verifier: "kani",
        describe: || <RustStdStandard<FromUtf16Error> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC, {
        /// `String::from_utf16` accepts a valid UTF-16 code unit and
        /// rejects a lone surrogate half (a high surrogate with no
        /// paired low surrogate) — the specific failure mode
        /// `FromUtf16Error` reports.
        #[kani::proof]
        fn verify_from_utf16_rejects_a_lone_surrogate() {
            let valid: [u16; 1] = [0x61];
            assert!(String::from_utf16(&valid).is_ok(), "a valid code unit is accepted");

            let lone_surrogate: [u16; 1] = [0xD800];
            assert!(
                String::from_utf16(&lone_surrogate).is_err(),
                "a lone surrogate half is rejected"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<FromUtf8Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_utf8_error_recovers_the_original_bytes",
            claim: VERIFY_FROM_UTF8_ERROR_RECOVERS_THE_ORIGINAL_BYTES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<FromUtf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FromUtf8Error>",
        verifier: "kani",
        describe: || <RustStdStandard<FromUtf8Error> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FROM_UTF8_ERROR_RECOVERS_THE_ORIGINAL_BYTES_SRC, {
        /// `String::from_utf8`'s error doesn't discard the invalid
        /// bytes: `.as_bytes()`/`.into_bytes()` both recover exactly
        /// the original vector that failed to convert.
        #[kani::proof]
        fn verify_from_utf8_error_recovers_the_original_bytes() {
            let byte: u8 = kani::any();
            let bytes = vec![byte, 0xFFu8];
            let err = String::from_utf8(bytes.clone()).unwrap_err();
            assert_eq!(err.as_bytes(), &bytes[..], "as_bytes recovers the original bytes");
            assert_eq!(err.into_bytes(), bytes, "into_bytes recovers the original bytes");
        }
    }
}
