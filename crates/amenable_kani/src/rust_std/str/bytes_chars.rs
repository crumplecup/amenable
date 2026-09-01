use std::str::{CharIndices, Chars, EncodeUtf16};

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
#[cfg(kani)]
use amenable_std::AsciiByte;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use super::lines_and_markers::CollectedSequenceMatchesExpected;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<std::str::Bytes<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_bytes_yields_the_utf8_encoding".to_owned(),
            VERIFY_BYTES_YIELDS_THE_UTF8_ENCODING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::Bytes<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Bytes<'static>>",
        "kani",
        || <RustStdStandard<std::str::Bytes<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::Bytes<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Bytes<'static>>",
    (Option<u8>, Option<u8>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_BYTES_YIELDS_THE_UTF8_ENCODING_SRC, {
        /// `.bytes()` yields the UTF-8 encoding of the str, checked for
        /// any single-byte (ASCII) character.
        #[kani::proof]
        fn verify_bytes_yields_the_utf8_encoding() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let s = (byte as char).to_string();
            let mut it = s.bytes();
            assert!(
                RustStdStandard::<std::str::Bytes<'static>>::ensures((it.next(), Some(byte))),
                "bytes yields the str's UTF-8 encoding"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<CharIndices<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_indices_pairs_each_char_with_its_byte_offset".to_owned(),
            VERIFY_CHAR_INDICES_PAIRS_EACH_CHAR_WITH_ITS_BYTE_OFFSET_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<CharIndices<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CharIndices<'static>>",
        "kani",
        || <RustStdStandard<CharIndices<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<CharIndices<'static>>,
    "amenable_std::rust_std::RustStdStandard<CharIndices<'static>>",
    (Option<(usize, char)>, Option<(usize, char)>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_INDICES_PAIRS_EACH_CHAR_WITH_ITS_BYTE_OFFSET_SRC, {
        /// `.char_indices()` pairs the first char with byte offset 0.
        #[kani::proof]
        fn verify_char_indices_pairs_each_char_with_its_byte_offset() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let c = byte as char;
            let s = c.to_string();
            let mut it = s.char_indices();
            assert!(
                RustStdStandard::<CharIndices<'static>>::ensures((it.next(), Some((0, c)))),
                "the first char is paired with byte offset 0"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Chars<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chars_yields_the_str_characters".to_owned(),
            VERIFY_CHARS_YIELDS_THE_STR_CHARACTERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Chars<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Chars<'static>>",
        "kani",
        || <RustStdStandard<Chars<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Chars<'static>>,
    "amenable_std::rust_std::RustStdStandard<Chars<'static>>",
    (Option<char>, Option<char>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHARS_YIELDS_THE_STR_CHARACTERS_SRC, {
        /// `.chars()` yields the str's characters, for any (symbolic)
        /// single-character str.
        #[kani::proof]
        fn verify_chars_yields_the_str_characters() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let c = byte as char;
            let s = c.to_string();
            let mut it = s.chars();
            assert!(
                <RustStdStandard<Chars<'static>> as Ensures<crate::KaniVerifier>>::ensures((
                    it.next(),
                    Some(c)
                )),
                "chars yields the str's characters"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<EncodeUtf16<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_encode_utf16_yields_utf16_code_units".to_owned(),
            VERIFY_ENCODE_UTF16_YIELDS_UTF16_CODE_UNITS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<EncodeUtf16<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<EncodeUtf16<'static>>",
        "kani",
        || <RustStdStandard<EncodeUtf16<'static>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ENCODE_UTF16_YIELDS_UTF16_CODE_UNITS_SRC, {
        /// `.encode_utf16()` yields UTF-16 code units; for an ASCII
        /// character, the code unit numerically equals the byte.
        #[kani::proof]
        fn verify_encode_utf16_yields_utf16_code_units() {
            let byte: u8 = kani::any();
            // Canonical home: amenable_std::AsciiByte's Requires<KaniVerifier> impl
            // (rust_std::primitives) names this exact fragment.
            kani::assume(<AsciiByte as Requires<crate::KaniVerifier>>::requires(byte));
            let s = (byte as char).to_string();
            let mut it = s.encode_utf16();
            assert!(
                CollectedSequenceMatchesExpected::ensures((it.next(), Some(byte as u16))),
                "an ASCII character's UTF-16 code unit equals its byte value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::EscapeDebug<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_debug_escapes_control_characters".to_owned(),
            VERIFY_ESCAPE_DEBUG_ESCAPES_CONTROL_CHARACTERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::EscapeDebug<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::EscapeDebug<'static>>",
        "kani",
        || <RustStdStandard<std::str::EscapeDebug<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::EscapeDebug<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::EscapeDebug<'static>>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_DEBUG_ESCAPES_CONTROL_CHARACTERS_SRC, {
        /// `.escape_debug()` renders a newline as the two-character
        /// escape sequence `\n`, matching `Debug`'s formatting.
        #[kani::proof]
        fn verify_escape_debug_escapes_control_characters() {
            let s = "\n";
            let out: String = s.escape_debug().collect();
            assert!(
                RustStdStandard::<std::str::EscapeDebug<'static>>::ensures((out, "\\n")),
                "escape_debug renders \\n as a two-character escape"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::EscapeDefault<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_default_escapes_control_characters".to_owned(),
            VERIFY_ESCAPE_DEFAULT_ESCAPES_CONTROL_CHARACTERS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::EscapeDefault<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::EscapeDefault<'static>>",
        "kani",
        || <RustStdStandard<std::str::EscapeDefault<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::EscapeDefault<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::EscapeDefault<'static>>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_DEFAULT_ESCAPES_CONTROL_CHARACTERS_SRC, {
        /// `.escape_default()` renders a newline the same way a Rust
        /// string literal would: the two-character escape `\n`.
        #[kani::proof]
        fn verify_escape_default_escapes_control_characters() {
            let s = "\n";
            let out: String = s.escape_default().collect();
            assert!(
                RustStdStandard::<std::str::EscapeDefault<'static>>::ensures((out, "\\n")),
                "escape_default renders \\n as a two-character escape"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::str::EscapeUnicode<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_escape_unicode_renders_the_codepoint_escape".to_owned(),
            VERIFY_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::str::EscapeUnicode<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::EscapeUnicode<'static>>",
        "kani",
        || <RustStdStandard<std::str::EscapeUnicode<'static>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::str::EscapeUnicode<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::EscapeUnicode<'static>>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC, {
        /// `.escape_unicode()` renders every character as a
        /// `\u{...}` codepoint escape, even a plain ASCII letter.
        #[kani::proof]
        fn verify_escape_unicode_renders_the_codepoint_escape() {
            let s = "a";
            let out: String = s.escape_unicode().collect();
            assert!(
                RustStdStandard::<std::str::EscapeUnicode<'static>>::ensures((out, "\\u{61}")),
                "escape_unicode renders every char as \\u{...}"
            );
        }
    }
}
