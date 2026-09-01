//! `char` decoding/case-conversion/escaping iterators, and every slice
//! chunking/splitting family (`chunks`/`chunk_by`/`split`, via their own
//! macros).

use super::collections_iter_cell_ref::{
    DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC,
};
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates".to_owned(),
            VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>,
    "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
    "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        || {
            <RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "ensures",
        || DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16<std::array::IntoIter<u16, 1>>>",
        "verus",
        "ensures",
        || DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    )
}

impl VerusWitness for RustStdStandard<std::char::DecodeUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates".to_owned(),
            VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::DecodeUtf16Error>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::char::DecodeUtf16Error>,
    "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
    "verify_decode_utf16_model_round_trips_and_reports_lone_surrogates"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        || {
            <RustStdStandard<std::char::DecodeUtf16Error> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "requires",
        || DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "ensures",
        || DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::DecodeUtf16Error>",
        "verus",
        "ensures",
        || DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT,
    )
}

const VERIFY_TO_LOWERCASE_MODEL_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::ToLowercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_to_lowercase_model_maps_an_uppercase_ascii_letter".to_owned(),
            VERIFY_TO_LOWERCASE_MODEL_MAPS_AN_UPPERCASE_ASCII_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::ToLowercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::ToLowercase>",
        "verus",
        || {
            <RustStdStandard<std::char::ToLowercase> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TO_UPPERCASE_MODEL_MAPS_A_LOWERCASE_ASCII_LETTER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::ToUppercase> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_to_uppercase_model_maps_a_lowercase_ascii_letter".to_owned(),
            VERIFY_TO_UPPERCASE_MODEL_MAPS_A_LOWERCASE_ASCII_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::ToUppercase>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::ToUppercase>",
        "verus",
        || {
            <RustStdStandard<std::char::ToUppercase> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAR_ESCAPE_DEBUG_MODEL_ESCAPES_A_NEWLINE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeDebug> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_escape_debug_model_escapes_a_newline".to_owned(),
            VERIFY_CHAR_ESCAPE_DEBUG_MODEL_ESCAPES_A_NEWLINE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeDebug>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::EscapeDebug>",
        "verus",
        || {
            <RustStdStandard<std::char::EscapeDebug> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAR_ESCAPE_DEFAULT_MODEL_ESCAPES_A_NEWLINE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_escape_default_model_escapes_a_newline".to_owned(),
            VERIFY_CHAR_ESCAPE_DEFAULT_MODEL_ESCAPES_A_NEWLINE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::EscapeDefault>",
        "verus",
        || {
            <RustStdStandard<std::char::EscapeDefault> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CHAR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::char::EscapeUnicode> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_escape_unicode_model_renders_the_codepoint_escape".to_owned(),
            VERIFY_CHAR_ESCAPE_UNICODE_MODEL_RENDERS_THE_CODEPOINT_ESCAPE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::char::EscapeUnicode>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::char::EscapeUnicode>",
        "verus",
        || {
            <RustStdStandard<std::char::EscapeUnicode> as VerusWitness>::proof().to_string()
        },
    )
}

macro_rules! impl_slice_chunks_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../../amenable_verus/src/rust_std/iter/slice_chunks_carrier.rs");

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

const TEN_INCREMENT_WRITE_THROUGH_VERUS_FRAGMENT: &str = r#"pub open spec fn ten_increment_write_through(before: int, after: int) -> bool {
    after == before + 10
}"#;

macro_rules! register_slice_chunks_increment_fragment {
    ($ty:ty) => {
        ::inventory::submit! {
            ::amenable_core::ContractRecord::new(
                concat!(
                    "amenable_std::rust_std::RustStdStandard<",
                    stringify!($ty),
                    ">"
                ),
                "verus",
                "ensures",
                || TEN_INCREMENT_WRITE_THROUGH_VERUS_FRAGMENT,
            )
        }
    };
}

impl_slice_chunks_verus_witness!(
    std::slice::Chunks<'static, i32>,
    "verify_chunks_model_yields_non_overlapping_groups_with_a_short_last_chunk",
    VERIFY_CHUNKS_MODEL_YIELDS_NON_OVERLAPPING_GROUPS_WITH_A_SHORT_LAST_CHUNK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::ChunksExact<'static, i32>,
    "verify_chunks_exact_model_discards_a_short_remainder",
    VERIFY_CHUNKS_EXACT_MODEL_DISCARDS_A_SHORT_REMAINDER_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::ChunksMut<'static, i32>,
    "verify_chunks_mut_model_writes_through_every_chunk",
    VERIFY_CHUNKS_MUT_MODEL_WRITES_THROUGH_EVERY_CHUNK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::ChunksExactMut<'static, i32>,
    "verify_chunks_exact_mut_model_leaves_the_remainder_untouched",
    VERIFY_CHUNKS_EXACT_MUT_MODEL_LEAVES_THE_REMAINDER_UNTOUCHED_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunks<'static, i32>,
    "verify_rchunks_model_groups_from_the_back",
    VERIFY_RCHUNKS_MODEL_GROUPS_FROM_THE_BACK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunksExact<'static, i32>,
    "verify_rchunks_exact_model_discards_a_short_remainder_at_the_front",
    VERIFY_RCHUNKS_EXACT_MODEL_DISCARDS_A_SHORT_REMAINDER_AT_THE_FRONT_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunksExactMut<'static, i32>,
    "verify_rchunks_exact_mut_model_leaves_the_front_remainder_untouched",
    VERIFY_RCHUNKS_EXACT_MUT_MODEL_LEAVES_THE_FRONT_REMAINDER_UNTOUCHED_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::RChunksMut<'static, i32>,
    "verify_rchunks_mut_model_writes_through_every_chunk",
    VERIFY_RCHUNKS_MUT_MODEL_WRITES_THROUGH_EVERY_CHUNK_SRC
);
impl_slice_chunks_verus_witness!(
    std::slice::Windows<'static, i32>,
    "verify_windows_model_yields_overlapping_slices",
    VERIFY_WINDOWS_MODEL_YIELDS_OVERLAPPING_SLICES_SRC
);

register_slice_chunks_increment_fragment!(std::slice::ChunksMut<'static, i32>);
register_slice_chunks_increment_fragment!(std::slice::ChunksExactMut<'static, i32>);
register_slice_chunks_increment_fragment!(std::slice::RChunksExactMut<'static, i32>);
register_slice_chunks_increment_fragment!(std::slice::RChunksMut<'static, i32>);

const VERIFY_CHUNK_BY_MODEL_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/slice_chunk_by_carrier.rs");

macro_rules! impl_chunk_by_verus_witness {
    ($ty:ty) => {
        impl VerusWitness for RustStdStandard<$ty> {
            type SupportingEvidence = Self;
            type ProofArtifact = VerusCheckedProof;

            fn proof() -> Self::ProofArtifact {
                VerusCheckedProof::new(
                    "verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate"
                        .to_owned(),
                    VERIFY_CHUNK_BY_MODEL_GROUPS_ADJACENT_ELEMENTS_MATCHING_THE_PREDICATE_SRC
                        .to_owned(),
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

impl_chunk_by_verus_witness!(std::slice::ChunkBy<'static, i32, fn(&i32, &i32) -> bool>);
impl_chunk_by_verus_witness!(std::slice::ChunkByMut<'static, i32, fn(&i32, &i32) -> bool>);

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::slice::ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<std::slice::ChunkBy<'static, i32, fn(&i32, &i32) -> bool>>",
    "chunk_by_result_matches_grouping"
);

macro_rules! impl_slice_split_verus_witness {
    ($ty:ty, $harness:literal, $const_name:ident) => {
        const $const_name: &str =
            include_str!("../../../amenable_verus/src/rust_std/iter/slice_split_carrier.rs");

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

impl_slice_split_verus_witness!(
    std::slice::Split<'static, i32, fn(&i32) -> bool>,
    "verify_split_model_yields_subslices_between_matches",
    VERIFY_SPLIT_MODEL_YIELDS_SUBSLICES_BETWEEN_MATCHES_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitMut<'static, i32, fn(&i32) -> bool>,
    "verify_split_mut_model_writes_through_the_first_piece",
    VERIFY_SPLIT_MUT_MODEL_WRITES_THROUGH_THE_FIRST_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitInclusive<'static, i32, fn(&i32) -> bool>,
    "verify_split_inclusive_model_keeps_the_match_at_the_end_of_each_piece",
    VERIFY_SPLIT_INCLUSIVE_MODEL_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitInclusiveMut<'static, i32, fn(&i32) -> bool>,
    "verify_split_inclusive_mut_model_keeps_the_match_at_the_end_of_each_piece",
    VERIFY_SPLIT_INCLUSIVE_MUT_MODEL_KEEPS_THE_MATCH_AT_THE_END_OF_EACH_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitN<'static, i32, fn(&i32) -> bool>,
    "verify_split_n_model_caps_the_number_of_pieces",
    VERIFY_SPLIT_N_MODEL_CAPS_THE_NUMBER_OF_PIECES_SRC
);
impl_slice_split_verus_witness!(
    std::slice::SplitNMut<'static, i32, fn(&i32) -> bool>,
    "verify_split_n_model_caps_the_number_of_pieces",
    VERIFY_SPLIT_N_MUT_MODEL_CAPS_THE_NUMBER_OF_PIECES_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplit<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_model_yields_subslices_from_the_back",
    VERIFY_RSPLIT_MODEL_YIELDS_SUBSLICES_FROM_THE_BACK_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplitMut<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_mut_model_writes_through_the_rearmost_piece",
    VERIFY_RSPLIT_MUT_MODEL_WRITES_THROUGH_THE_REARMOST_PIECE_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplitN<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back",
    VERIFY_RSPLIT_N_MODEL_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC
);
impl_slice_split_verus_witness!(
    std::slice::RSplitNMut<'static, i32, fn(&i32) -> bool>,
    "verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back",
    VERIFY_RSPLIT_N_MUT_MODEL_CAPS_THE_NUMBER_OF_PIECES_FROM_THE_BACK_SRC
);

const VERIFY_ESCAPE_ASCII_MODEL_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/escape_ascii_carrier.rs");
const ESCAPE_ASCII_INPUT_IS_PRINTABLE_ASCII_VERUS_FRAGMENT: &str = r#"pub open spec fn escape_ascii_input_is_printable_ascii(printable: u8) -> bool {
    32 <= printable && printable <= 126
}"#;
const ESCAPE_ASCII_RESULT_MATCHES_PRINTABLE_PLUS_NEWLINE_ESCAPE_VERUS_FRAGMENT: &str = r#"pub open spec fn escape_ascii_result_matches_printable_plus_newline_escape(
    printable: u8,
    result: (u8, u8, u8),
) -> bool {
    result.0 == printable && result.1 == 92 && result.2 == 110
}"#;

// `verify_escape_ascii_model_leaves_printable_bytes_unescaped`'s real
// VerusCallShape is no longer registered by hand here -- `verus_call_shape`
// derives it (including its `requires` clause -- the first harness with a
// real precondition, exercising the compositional renderer's
// requires-propagation) by parsing the real signature directly from
// crates/amenable_verus/src/rust_std/escape_ascii_carrier.rs.

impl VerusWitness for RustStdStandard<std::slice::EscapeAscii<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_escape_ascii_model_leaves_printable_bytes_unescaped".to_owned(),
            VERIFY_ESCAPE_ASCII_MODEL_LEAVES_PRINTABLE_BYTES_UNESCAPED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::EscapeAscii<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        "verus",
        || {
            <RustStdStandard<std::slice::EscapeAscii<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        "verus",
        "requires",
        || ESCAPE_ASCII_INPUT_IS_PRINTABLE_ASCII_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::EscapeAscii<'static>>",
        "verus",
        "ensures",
        || ESCAPE_ASCII_RESULT_MATCHES_PRINTABLE_PLUS_NEWLINE_ESCAPE_VERUS_FRAGMENT,
    )
}

const VERIFY_GET_DISJOINT_MUT_MODEL_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/get_disjoint_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::slice::GetDisjointMutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_get_disjoint_mut_model_rejects_overlap_and_out_of_bounds".to_owned(),
            VERIFY_GET_DISJOINT_MUT_MODEL_REJECTS_OVERLAP_AND_OUT_OF_BOUNDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::slice::GetDisjointMutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::GetDisjointMutError>",
        "verus",
        || {
            <RustStdStandard<std::slice::GetDisjointMutError> as VerusWitness>::proof().to_string()
        },
    )
}
