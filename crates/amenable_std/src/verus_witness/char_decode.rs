//! `char` decoding/case-conversion/escaping iterators: `DecodeUtf16`,
//! `DecodeUtf16Error`, `ToLowercase`/`ToUppercase`, and
//! `EscapeDebug`/`EscapeDefault`/`EscapeUnicode`.

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
