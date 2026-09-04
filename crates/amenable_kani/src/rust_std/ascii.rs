//! `KaniWitness` impls for `core::ascii`.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<core::ascii::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_default_escapes_a_control_byte".to_owned(),
            VERIFY_ESCAPE_DEFAULT_ESCAPES_A_CONTROL_BYTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::ascii::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ascii::EscapeDefault>",
        "kani",
        || <RustStdStandard<core::ascii::EscapeDefault> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::ascii::EscapeDefault>,
    "amenable_std::rust_std::RustStdStandard<core::ascii::EscapeDefault>",
    (Vec<u8>, Vec<u8>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_DEFAULT_ESCAPES_A_CONTROL_BYTE_SRC, {
        /// `u8::escape_ascii()` renders the three named C-style control
        /// escapes the same way a Rust byte-string literal would (`\n`,
        /// `\t`, `\r`), and renders an unnamed control byte as a `\xNN`
        /// hex escape. The assertions call
        /// `RustStdStandard::<core::ascii::EscapeDefault>::ensures`
        /// directly rather than restating the comparison.
        #[kani::proof]
        fn verify_escape_default_escapes_a_control_byte() {
            let newline: Vec<u8> = b'\n'.escape_ascii().collect();
            assert!(
                RustStdStandard::<core::ascii::EscapeDefault>::ensures((
                    newline,
                    b"\\n".to_vec()
                )),
                "escape_ascii renders \\n as a two-byte escape"
            );

            let tab: Vec<u8> = b'\t'.escape_ascii().collect();
            assert!(
                RustStdStandard::<core::ascii::EscapeDefault>::ensures((tab, b"\\t".to_vec())),
                "escape_ascii renders \\t as a two-byte escape"
            );

            let carriage_return: Vec<u8> = b'\r'.escape_ascii().collect();
            assert!(
                RustStdStandard::<core::ascii::EscapeDefault>::ensures((
                    carriage_return,
                    b"\\r".to_vec()
                )),
                "escape_ascii renders \\r as a two-byte escape"
            );

            let bell: Vec<u8> = 0x07u8.escape_ascii().collect();
            assert!(
                RustStdStandard::<core::ascii::EscapeDefault>::ensures((bell, b"\\x07".to_vec())),
                "an unnamed control byte renders as a \\xNN hex escape"
            );
        }
    }
}
