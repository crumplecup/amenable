//! `KaniWitness` impls for `std::ffi` (`OsStr`/`OsString`).

use std::ffi::{OsStr, OsString};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_os_str_valid_utf8_content_round_trips_through_to_str",
            claim: VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsStr>",
        verifier: "kani",
        describe: || <RustStdStandard<OsStr> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC, {
        /// An `OsStr` built entirely of valid Unicode round-trips
        /// exactly through `.to_str()`, and `.len()` reports its byte
        /// length.
        #[kani::proof]
        fn verify_os_str_valid_utf8_content_round_trips_through_to_str() {
            let os_str = OsStr::new("hello");
            assert_eq!(os_str.to_str(), Some("hello"));
            assert_eq!(os_str.len(), 5);
        }
    }
}

impl KaniWitness for RustStdStandard<OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_os_string_push_appends_to_the_existing_content",
            claim: VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OsString>",
        verifier: "kani",
        describe: || <RustStdStandard<OsString> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC, {
        /// `.push()` appends to the existing content, without
        /// disturbing what was already there.
        #[kani::proof]
        fn verify_os_string_push_appends_to_the_existing_content() {
            let mut os_string = OsString::from("hello");
            os_string.push(", world");
            assert_eq!(os_string.as_os_str(), OsStr::new("hello, world"));
        }
    }
}
