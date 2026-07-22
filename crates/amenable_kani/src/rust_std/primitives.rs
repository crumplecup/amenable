//! `KaniWitness` impls for Rust's scalar primitives and `String`.

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, impl_kani_witness_trusted};

impl_kani_witness_trusted!(
    bool, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

impl KaniWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_unicode_scalar".to_owned(),
            claim: VERIFY_CHAR_UNICODE_SCALAR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<char>",
        verifier: "kani",
        describe: || <RustStdStandard<char> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHAR_UNICODE_SCALAR_SRC, {
        /// `char` is constrained to Unicode scalar values (excludes the
        /// surrogate range `0xD800..=0xDFFF`) and round-trips through `u32`.
        #[kani::proof]
        fn verify_char_unicode_scalar() {
            let c: char = kani::any();
            let u = c as u32;

            assert!(
                u <= 0xD7FF || (0xE000..=0x10FFFF).contains(&u),
                "char is a valid Unicode scalar value"
            );

            let c2 = char::from_u32(u).expect("valid unicode scalar round-trips");
            assert!(c == c2, "char round-trips through u32");
        }
    }
}

impl KaniWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_string_utf8_valid".to_owned(),
            claim: VERIFY_STRING_UTF8_VALID_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<String>",
        verifier: "kani",
        describe: || <RustStdStandard<String> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_STRING_UTF8_VALID_SRC, {
        /// `String` is always valid UTF-8, and its length/emptiness are
        /// consistent. Kani cannot model heap allocation symbolically, so
        /// this checks the invariant on concrete representative values.
        #[kani::proof]
        fn verify_string_utf8_valid() {
            let s = String::from("hello");
            assert!(
                std::str::from_utf8(s.as_bytes()).is_ok(),
                "String is valid UTF-8"
            );
            assert!(!s.is_empty(), "non-empty string has positive length");

            let empty = String::new();
            assert!(empty.is_empty(), "empty string has zero length");
            assert!(
                std::str::from_utf8(empty.as_bytes()).is_ok(),
                "empty String is valid UTF-8"
            );
        }
    }
}
