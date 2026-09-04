use std::char::{ParseCharError, ToLowercase, ToUppercase};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<ParseCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_parse_char_error_occurs_for_empty_or_multi_character_strings".to_owned(),
            VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ParseCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ParseCharError>",
        "kani",
        || <RustStdStandard<ParseCharError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC, {
        /// A string parses as `char` only when it holds exactly one
        /// character; an empty or multi-character string fails with
        /// `ParseCharError`, which has no accessor beyond success/failure.
        #[kani::proof]
        fn verify_parse_char_error_occurs_for_empty_or_multi_character_strings() {
            assert!(
                FallibleOperationReportsFailure::ensures("".parse::<char>().is_err()),
                "an empty string fails to parse as char"
            );
            assert!(
                FallibleOperationReportsFailure::ensures("ab".parse::<char>().is_err()),
                "a multi-character string fails to parse as char"
            );
            assert!(
                CollectedSequenceMatchesExpected::ensures(("a".parse::<char>(), Ok('a'))),
                "a single-character string parses as that char"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ToLowercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_to_lowercase_maps_an_uppercase_ascii_letter".to_owned(),
            VERIFY_TO_LOWERCASE_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ToLowercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ToLowercase>",
        "kani",
        || <RustStdStandard<ToLowercase> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<ToLowercase>,
    "amenable_std::rust_std::RustStdStandard<ToLowercase>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_TO_LOWERCASE_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC, {
        /// `.to_lowercase()` yields the lowercase mapping of an uppercase
        /// ASCII letter.
        #[kani::proof]
        fn verify_to_lowercase_maps_an_uppercase_ascii_letter() {
            let out: String = 'A'.to_lowercase().collect();
            assert!(
                RustStdStandard::<ToLowercase>::ensures((out, "a")),
                "to_lowercase maps 'A' to \"a\""
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ToUppercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_to_uppercase_maps_a_lowercase_ascii_letter".to_owned(),
            VERIFY_TO_UPPERCASE_MAPS_A_LOWERCASE_ASCII_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ToUppercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ToUppercase>",
        "kani",
        || <RustStdStandard<ToUppercase> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<ToUppercase>,
    "amenable_std::rust_std::RustStdStandard<ToUppercase>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_TO_UPPERCASE_MAPS_A_LOWERCASE_ASCII_LETTER_SRC, {
        /// `.to_uppercase()` yields the uppercase mapping of a lowercase
        /// ASCII letter.
        #[kani::proof]
        fn verify_to_uppercase_maps_a_lowercase_ascii_letter() {
            let out: String = 'a'.to_uppercase().collect();
            assert!(
                RustStdStandard::<ToUppercase>::ensures((out, "A")),
                "to_uppercase maps 'a' to \"A\""
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::char::EscapeDebug> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_escape_debug_escapes_a_newline".to_owned(),
            VERIFY_CHAR_ESCAPE_DEBUG_ESCAPES_A_NEWLINE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::char::EscapeDebug>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::EscapeDebug>",
        "kani",
        || <RustStdStandard<core::char::EscapeDebug> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::char::EscapeDebug>,
    "amenable_std::rust_std::RustStdStandard<core::char::EscapeDebug>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_ESCAPE_DEBUG_ESCAPES_A_NEWLINE_SRC, {
        /// `.escape_debug()` renders a newline as the two-character escape
        /// sequence `\n`, matching `Debug`'s formatting.
        #[kani::proof]
        fn verify_char_escape_debug_escapes_a_newline() {
            let out: String = '\n'.escape_debug().collect();
            assert!(
                RustStdStandard::<core::char::EscapeDebug>::ensures((out, "\\n")),
                "escape_debug renders \\n as a two-character escape"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::char::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_escape_default_escapes_a_newline".to_owned(),
            VERIFY_CHAR_ESCAPE_DEFAULT_ESCAPES_A_NEWLINE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::char::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::EscapeDefault>",
        "kani",
        || <RustStdStandard<core::char::EscapeDefault> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::char::EscapeDefault>,
    "amenable_std::rust_std::RustStdStandard<core::char::EscapeDefault>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_ESCAPE_DEFAULT_ESCAPES_A_NEWLINE_SRC, {
        /// `.escape_default()` renders a newline the same way a Rust
        /// string literal would: the two-character escape `\n`.
        #[kani::proof]
        fn verify_char_escape_default_escapes_a_newline() {
            let out: String = '\n'.escape_default().collect();
            assert!(
                RustStdStandard::<core::char::EscapeDefault>::ensures((out, "\\n")),
                "escape_default renders \\n as a two-character escape"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<core::char::EscapeUnicode> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_escape_unicode_renders_the_codepoint_escape".to_owned(),
            VERIFY_CHAR_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<core::char::EscapeUnicode>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::EscapeUnicode>",
        "kani",
        || <RustStdStandard<core::char::EscapeUnicode> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<core::char::EscapeUnicode>,
    "amenable_std::rust_std::RustStdStandard<core::char::EscapeUnicode>",
    (String, &'static str),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAR_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC, {
        /// `.escape_unicode()` renders a char as a `\u{...}` codepoint
        /// escape, even a plain ASCII letter.
        #[kani::proof]
        fn verify_char_escape_unicode_renders_the_codepoint_escape() {
            let out: String = 'a'.escape_unicode().collect();
            assert!(
                RustStdStandard::<core::char::EscapeUnicode>::ensures((out, "\\u{61}")),
                "escape_unicode renders a char as \\u{...}"
            );
        }
    }
}
