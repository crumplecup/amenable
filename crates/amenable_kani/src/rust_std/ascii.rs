//! `KaniWitness` impls for `core::ascii`.

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<core::ascii::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_escape_default_escapes_a_control_byte".to_owned(),
            claim: VERIFY_ESCAPE_DEFAULT_ESCAPES_A_CONTROL_BYTE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::ascii::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::ascii::EscapeDefault>",
        verifier: "kani",
        describe: || <RustStdStandard<core::ascii::EscapeDefault> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_DEFAULT_ESCAPES_A_CONTROL_BYTE_SRC, {
        /// `u8::escape_ascii()` renders the three named C-style control
        /// escapes the same way a Rust byte-string literal would (`\n`,
        /// `\t`, `\r`), and renders an unnamed control byte as a `\xNN`
        /// hex escape.
        #[kani::proof]
        fn verify_escape_default_escapes_a_control_byte() {
            let newline: Vec<u8> = b'\n'.escape_ascii().collect();
            assert_eq!(newline, b"\\n", "escape_ascii renders \\n as a two-byte escape");

            let tab: Vec<u8> = b'\t'.escape_ascii().collect();
            assert_eq!(tab, b"\\t", "escape_ascii renders \\t as a two-byte escape");

            let carriage_return: Vec<u8> = b'\r'.escape_ascii().collect();
            assert_eq!(
                carriage_return, b"\\r",
                "escape_ascii renders \\r as a two-byte escape"
            );

            let bell: Vec<u8> = 0x07u8.escape_ascii().collect();
            assert_eq!(
                bell, b"\\x07",
                "an unnamed control byte renders as a \\xNN hex escape"
            );
        }
    }
}
