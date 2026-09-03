//! Every `str` iterator family with a shared shape, generated via its own
//! `impl_str_*_verus_witness!` macro: ASCII iteration, escaping, whitespace
//! splitting, UTF-8 chunking, and `Pattern`-based splitting/matching.
//!
//! `std::str::LinesAny`'s witness block below references a deprecated
//! item, and `#[expect(deprecated)]` attached to the individual impl/
//! macro-invocation/`inventory::submit!` sites didn't line up with where
//! the lint actually fires through macro expansion (confirmed: those
//! per-site attributes reported "unused attribute" while the warning
//! still fired elsewhere) — expecting it at the whole-module level
//! instead, the same fix this module's own `rc_arc_hash` (for
//! `SipHasher`) and `amenable_verus::rust_std::sip_hasher_carrier`
//! already use for the identical reason.
#![expect(
    deprecated,
    reason = "LinesAny itself is stable (only deprecated as a recommendation to use lines()/Lines instead); covering it is a coverage-completeness question, not a call to use it"
)]

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

macro_rules! impl_str_ascii_iter_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str = include_str!(
            "../../../amenable_verus/src/rust_std/str_and_char/str_ascii_iter_carrier.rs"
        );

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_ascii_iter_verus_witness!(
    std::str::Bytes<'static>,
    "verify_bytes_model_yields_the_utf8_encoding",
    VERIFY_BYTES_MODEL_YIELDS_THE_UTF8_ENCODING_SRC
);

// Shared by `.bytes()`'s (u8) and `.encode_utf16()`'s (u16) own claims,
// registered once here for all its real call sites.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::Bytes<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Bytes<'static>>",
    "numeric_cast_matches_char"
);
impl_str_ascii_iter_verus_witness!(
    std::str::CharIndices<'static>,
    "verify_char_indices_model_pairs_each_char_with_its_byte_offset",
    VERIFY_CHAR_INDICES_MODEL_PAIRS_EACH_CHAR_WITH_ITS_BYTE_OFFSET_SRC
);

// Singleton contract: the sole char's byte offset in a one-character
// str is always 0.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::CharIndices<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::CharIndices<'static>>",
    "char_indices_first_offset_is_zero"
);
impl_str_ascii_iter_verus_witness!(
    std::str::EncodeUtf16<'static>,
    "verify_encode_utf16_model_yields_utf16_code_units",
    VERIFY_ENCODE_UTF16_MODEL_YIELDS_UTF16_CODE_UNITS_SRC
);

macro_rules! impl_str_escape_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../../amenable_verus/src/rust_std/str_and_char/str_escape_carrier.rs");

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_escape_verus_witness!(
    std::str::EscapeDebug<'static>,
    "verify_str_escape_debug_model_escapes_control_characters",
    VERIFY_STR_ESCAPE_DEBUG_MODEL_ESCAPES_CONTROL_CHARACTERS_SRC
);
impl_str_escape_verus_witness!(
    std::str::EscapeDefault<'static>,
    "verify_str_escape_default_model_escapes_control_characters",
    VERIFY_STR_ESCAPE_DEFAULT_MODEL_ESCAPES_CONTROL_CHARACTERS_SRC
);
impl_str_escape_verus_witness!(
    std::str::EscapeUnicode<'static>,
    "verify_str_escape_unicode_model_renders_the_codepoint_escape",
    VERIFY_STR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC
);

const VERIFY_LINES_MODEL_SPLITS_ON_LINE_ENDINGS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/str_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::Lines<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lines_model_splits_on_line_endings".to_owned(),
            VERIFY_LINES_MODEL_SPLITS_ON_LINE_ENDINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::Lines<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Lines<'static>>",
        "verus",
        || {
            <RustStdStandard<std::str::Lines<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_LINES_ANY_MODEL_SPLITS_ON_ANY_LINE_ENDING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/str_lines_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::LinesAny<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lines_any_model_splits_on_any_line_ending".to_owned(),
            VERIFY_LINES_ANY_MODEL_SPLITS_ON_ANY_LINE_ENDING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::LinesAny<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::LinesAny<'static>>",
        "verus",
        || {
            <RustStdStandard<std::str::LinesAny<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_str_whitespace_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str = include_str!(
            "../../../amenable_verus/src/rust_std/str_and_char/str_whitespace_carrier.rs"
        );

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_whitespace_verus_witness!(
    std::str::SplitAsciiWhitespace<'static>,
    "verify_split_ascii_whitespace_model_collapses_runs_of_whitespace",
    VERIFY_SPLIT_ASCII_WHITESPACE_MODEL_COLLAPSES_RUNS_OF_WHITESPACE_SRC
);
impl_str_whitespace_verus_witness!(
    std::str::SplitWhitespace<'static>,
    "verify_split_whitespace_model_collapses_runs_of_whitespace",
    VERIFY_SPLIT_WHITESPACE_MODEL_COLLAPSES_RUNS_OF_WHITESPACE_SRC
);

macro_rules! impl_str_utf8_chunks_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str = include_str!(
            "../../../amenable_verus/src/rust_std/str_and_char/str_utf8_chunks_carrier.rs"
        );

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Chunks<'static>,
    "verify_utf8_chunks_model_yields_one_chunk_for_wholly_valid_input",
    VERIFY_UTF8_CHUNKS_MODEL_YIELDS_ONE_CHUNK_FOR_WHOLLY_VALID_INPUT_SRC
);
impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Chunk<'static>,
    "verify_utf8_chunk_model_separates_the_valid_prefix_from_invalid_bytes",
    VERIFY_UTF8_CHUNK_MODEL_SEPARATES_THE_VALID_PREFIX_FROM_INVALID_BYTES_SRC
);

// Singleton contract: the one bad byte is always exactly 0xFF in this
// fixed example.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::Utf8Chunk<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Utf8Chunk<'static>>",
    "utf8_chunk_invalid_byte_is_0xff"
);

impl_str_utf8_chunks_verus_witness!(
    std::str::Utf8Error,
    "verify_utf8_error_model_reports_the_valid_prefix_length_and_error_span",
    VERIFY_UTF8_ERROR_MODEL_REPORTS_THE_VALID_PREFIX_LENGTH_AND_ERROR_SPAN_SRC
);

// Singleton contract: the fixed example's valid-prefix length (2) and
// error span (1).
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::str::Utf8Error>,
    "amenable_std::rust_std::RustStdStandard<std::str::Utf8Error>",
    "utf8_error_reports_length_and_span"
);

// Singleton contract: `invalid`'s lower bound.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::Utf8Error>,
    "amenable_std::rust_std::RustStdStandard<std::str::Utf8Error>",
    "invalid_byte_is_never_a_valid_utf8_lead_byte"
);

macro_rules! impl_str_pattern_split_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str = include_str!(
            "../../../amenable_verus/src/rust_std/str_and_char/str_pattern_split_carrier.rs"
        );

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_pattern_split_verus_witness!(
    std::str::Split<'static, char>,
    "verify_str_split_model_yields_substrings_between_pattern_matches",
    VERIFY_STR_SPLIT_MODEL_YIELDS_SUBSTRINGS_BETWEEN_PATTERN_MATCHES_SRC
);
impl_str_pattern_split_verus_witness!(
    std::str::SplitN<'static, char>,
    "verify_str_splitn_model_limits_to_n_substrings",
    VERIFY_STR_SPLITN_MODEL_LIMITS_TO_N_SUBSTRINGS_SRC
);
impl_str_pattern_split_verus_witness!(
    std::str::SplitInclusive<'static, char>,
    "verify_str_split_inclusive_model_keeps_the_delimiter_attached",
    VERIFY_STR_SPLIT_INCLUSIVE_MODEL_KEEPS_THE_DELIMITER_ATTACHED_SRC
);

macro_rules! impl_str_pattern_reverse_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str = include_str!(
            "../../../amenable_verus/src/rust_std/str_and_char/str_pattern_reverse_carrier.rs"
        );

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_pattern_reverse_verus_witness!(
    std::str::RSplit<'static, char>,
    "verify_str_rsplit_model_yields_substrings_from_the_back",
    VERIFY_STR_RSPLIT_MODEL_YIELDS_SUBSTRINGS_FROM_THE_BACK_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::RSplit<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::RSplit<'static, char>>",
    "values_are_distinct"
);

impl_str_pattern_reverse_verus_witness!(
    std::str::RSplitN<'static, char>,
    "verify_str_rsplitn_model_limits_to_n_substrings_from_the_back",
    VERIFY_STR_RSPLITN_MODEL_LIMITS_TO_N_SUBSTRINGS_FROM_THE_BACK_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::RSplitN<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::RSplitN<'static, char>>",
    "values_are_distinct"
);

macro_rules! impl_str_pattern_terminator_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str = include_str!(
            "../../../amenable_verus/src/rust_std/str_and_char/str_pattern_terminator_carrier.rs"
        );

        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    $harness.to_owned(),
                    $const_name.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_pattern_terminator_verus_witness!(
    std::str::SplitTerminator<'static, char>,
    "verify_str_split_terminator_model_suppresses_a_trailing_empty_substring",
    VERIFY_STR_SPLIT_TERMINATOR_MODEL_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::SplitTerminator<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::SplitTerminator<'static, char>>",
    "values_are_distinct"
);

impl_str_pattern_terminator_verus_witness!(
    std::str::RSplitTerminator<'static, char>,
    "verify_str_rsplit_terminator_model_suppresses_a_trailing_empty_substring_from_the_back",
    VERIFY_STR_RSPLIT_TERMINATOR_MODEL_SUPPRESSES_A_TRAILING_EMPTY_SUBSTRING_FROM_THE_BACK_SRC
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::str::RSplitTerminator<'static, char>>,
    "amenable_std::rust_std::RustStdStandard<std::str::RSplitTerminator<'static, char>>",
    "values_are_distinct"
);

// Reused by AsciiByte in the next file, str_more_and_io_a.rs -- see that
// impl's own doc comment.
pub(super) const VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/str_pattern_match_carrier.rs");

macro_rules! impl_str_matches_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_str_matches_model_yields_every_non_overlapping_occurrence".to_owned(),
                    VERIFY_STR_MATCHES_MODEL_YIELDS_EVERY_NON_OVERLAPPING_OCCURRENCE_SRC.to_owned(),
                    <Self::SupportingEvidence as Evidence>::basis().audit(),
                )
            }
        }

        bridge_verus_witness!(RustStdStandard<$ty>);

        ::inventory::submit! {
            ::amenable_core::ProofRecord::new(
                concat!("amenable_std::rust_std::RustStdStandard<", stringify!($ty), ">"),
                "verus",
                || <RustStdStandard<$ty> as VerusWitness>::proof().to_string(),
            )
        }
    };
}

impl_str_matches_verus_witness!(std::str::Matches<'static, char>);
impl_str_matches_verus_witness!(std::str::RMatches<'static, char>);
