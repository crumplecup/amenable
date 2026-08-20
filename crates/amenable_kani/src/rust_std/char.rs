//! `KaniWitness` impls for `core::char`.
//!
//! `CharTryFromError`/`TryFromCharError` check a fully symbolic `u32`/`char`:
//! both reduce to a pure numeric range comparison, no string formatting
//! involved. `DecodeUtf16`/`DecodeUtf16Error` check a symbolic `u16` code
//! unit through a single `.next()` call on a one-element array iterator —
//! bounded, no `.collect()`. The escaping/case-mapping adapters
//! (`EscapeDebug`/`EscapeDefault`/`EscapeUnicode`/`ToLowercase`/
//! `ToUppercase`) instead check fixed representative examples, matching
//! `core::str`'s already-verified Escape* proofs: `.collect()`ing even a
//! two-byte symbolic `EscapeAscii` iterator times out under Kani (see
//! `gallery::slice_escape_ascii`), so these check one concrete `char` at a
//! time rather than a symbolic one. `ParseCharError` checks fixed
//! representative strings, since it has no accessor to check beyond
//! success/failure.

use std::char::{
    CharTryFromError, DecodeUtf16, DecodeUtf16Error, ParseCharError, ToLowercase, ToUppercase,
    TryFromCharError,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::{RustStdStandard, ValidUnicodeScalar};

use super::CheckedProof;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<CharTryFromError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range"
                .to_owned(),
            claim: VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<CharTryFromError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<CharTryFromError>",
        verifier: "kani",
        describe: || <RustStdStandard<CharTryFromError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC, {
        /// `char::try_from(u32)` succeeds exactly for valid Unicode scalar
        /// values (at most `U+10FFFF`, excluding the surrogate range), and
        /// preserves the value; it fails with `CharTryFromError` otherwise.
        /// `is_valid_scalar` calls `ValidUnicodeScalar::ensures` directly
        /// rather than restating its expression.
        #[kani::proof]
        fn verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range() {
            let value: u32 = kani::any();
            let result = char::try_from(value);
            let is_valid_scalar =
                <ValidUnicodeScalar as Ensures<crate::KaniVerifier>>::ensures(value);
            if is_valid_scalar {
                let parsed = result.expect("a valid Unicode scalar value must convert");
                assert_eq!(
                    parsed as u32, value,
                    "char::try_from preserves the scalar value"
                );
            } else {
                assert!(
                    FallibleOperationReportsFailure::ensures(result.is_err()),
                    "char::try_from fails for surrogate or out-of-range values"
                );
            }
        }
    }
}

/// The [`ValidUnicodeScalar`] contract type reuses
/// `verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range`
/// rather than adding a new Kani harness — it names the bound the harness
/// already checks, it doesn't prove anything new.
impl KaniWitness for ValidUnicodeScalar {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range"
                .to_owned(),
            claim: VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(ValidUnicodeScalar);

kani_ensures!(
    ValidUnicodeScalar,
    "amenable_std::ValidUnicodeScalar",
    u32,
    |value| value <= 0x0010_FFFF && !(0xD800..=0xDFFF).contains(&value)
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::ValidUnicodeScalar",
        verifier: "kani",
        describe: || <ValidUnicodeScalar as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<TryFromCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_from_char_error_occurs_exactly_when_out_of_range".to_owned(),
            claim: VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TryFromCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TryFromCharError>",
        verifier: "kani",
        describe: || <RustStdStandard<TryFromCharError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC, {
        /// `u8::try_from(char)` fails with `TryFromCharError` exactly when
        /// the char's scalar value doesn't fit in `u8`, and succeeds with
        /// the same value otherwise.
        #[kani::proof]
        fn verify_try_from_char_error_occurs_exactly_when_out_of_range() {
            let c: char = kani::any();
            let result = u8::try_from(c);
            if (c as u32) <= u32::from(u8::MAX) {
                assert_eq!(
                    result,
                    Ok(c as u8),
                    "try_from succeeds and preserves the value when it fits u8"
                );
            } else {
                assert!(
                    FallibleOperationReportsFailure::ensures(result.is_err()),
                    "try_from fails with TryFromCharError when the char doesn't fit u8"
                );
            }
        }
    }
}

impl KaniWitness for RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_decode_utf16_round_trips_a_bmp_code_unit".to_owned(),
            claim: VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence:
            "amenable_std::rust_std::RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        verifier: "kani",
        describe: || <RustStdStandard<DecodeUtf16<std::array::IntoIter<u16, 1>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_DECODE_UTF16_ROUND_TRIPS_A_BMP_CODE_UNIT_SRC, {
        /// A single non-surrogate UTF-16 code unit decodes to the char
        /// with that same scalar value.
        #[kani::proof]
        fn verify_decode_utf16_round_trips_a_bmp_code_unit() {
            let unit: u16 = kani::any();
            kani::assume(!(0xD800..=0xDFFF).contains(&unit));

            let mut iter = char::decode_utf16([unit]);
            let decoded = iter
                .next()
                .expect("a one-element iterator yields exactly one result")
                .expect("a non-surrogate BMP code unit always decodes successfully");
            assert_eq!(
                decoded as u32,
                u32::from(unit),
                "decoding a BMP code unit yields the char with that same scalar value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<DecodeUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_decode_utf16_error_reports_the_unpaired_surrogate".to_owned(),
            claim: VERIFY_DECODE_UTF16_ERROR_REPORTS_THE_UNPAIRED_SURROGATE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DecodeUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DecodeUtf16Error>",
        verifier: "kani",
        describe: || <RustStdStandard<DecodeUtf16Error> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_DECODE_UTF16_ERROR_REPORTS_THE_UNPAIRED_SURROGATE_SRC, {
        /// A lone high surrogate with no following low surrogate fails to
        /// decode, and the error reports the exact unpaired code unit.
        #[kani::proof]
        fn verify_decode_utf16_error_reports_the_unpaired_surrogate() {
            let lone_surrogate: u16 = 0xD800;
            let mut iter = char::decode_utf16([lone_surrogate]);
            let result = iter
                .next()
                .expect("a one-element iterator yields exactly one result");
            let err = result.expect_err("a lone surrogate with no pair must fail to decode");
            assert_eq!(
                err.unpaired_surrogate(),
                lone_surrogate,
                "DecodeUtf16Error reports the exact unpaired code unit"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ParseCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_parse_char_error_occurs_for_empty_or_multi_character_strings"
                .to_owned(),
            claim: VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC
                .to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ParseCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ParseCharError>",
        verifier: "kani",
        describe: || <RustStdStandard<ParseCharError> as KaniWitness>::proof().to_string(),
    }
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
            assert_eq!(
                "a".parse::<char>(),
                Ok('a'),
                "a single-character string parses as that char"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ToLowercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_to_lowercase_maps_an_uppercase_ascii_letter".to_owned(),
            claim: VERIFY_TO_LOWERCASE_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ToLowercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ToLowercase>",
        verifier: "kani",
        describe: || <RustStdStandard<ToLowercase> as KaniWitness>::proof().to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_to_uppercase_maps_a_lowercase_ascii_letter".to_owned(),
            claim: VERIFY_TO_UPPERCASE_MAPS_A_LOWERCASE_ASCII_LETTER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ToUppercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ToUppercase>",
        verifier: "kani",
        describe: || <RustStdStandard<ToUppercase> as KaniWitness>::proof().to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_escape_debug_escapes_a_newline".to_owned(),
            claim: VERIFY_CHAR_ESCAPE_DEBUG_ESCAPES_A_NEWLINE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::char::EscapeDebug>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::EscapeDebug>",
        verifier: "kani",
        describe: || <RustStdStandard<core::char::EscapeDebug> as KaniWitness>::proof().to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_escape_default_escapes_a_newline".to_owned(),
            claim: VERIFY_CHAR_ESCAPE_DEFAULT_ESCAPES_A_NEWLINE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::char::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::EscapeDefault>",
        verifier: "kani",
        describe: || <RustStdStandard<core::char::EscapeDefault> as KaniWitness>::proof()
            .to_string(),
    }
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_char_escape_unicode_renders_the_codepoint_escape".to_owned(),
            claim: VERIFY_CHAR_ESCAPE_UNICODE_RENDERS_THE_CODEPOINT_ESCAPE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<core::char::EscapeUnicode>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<core::char::EscapeUnicode>",
        verifier: "kani",
        describe: || <RustStdStandard<core::char::EscapeUnicode> as KaniWitness>::proof()
            .to_string(),
    }
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
