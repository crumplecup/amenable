//! `char`-conversion, type-identity, and slice-conversion error types.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::numeric_alloc::{
    CHAR_TRY_FROM_U32_REJECTS_INVALID_SCALAR_VERUS_FRAGMENT,
    CHAR_TRY_FROM_U32_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    U32_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT,
    VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC,
};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<core::char::CharTryFromError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_try_from_fails_exactly_for_surrogates_and_out_of_range".to_owned(),
            VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::char::CharTryFromError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        || {
            <RustStdStandard<core::char::CharTryFromError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        "ensures",
        || U32_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        "ensures",
        || CHAR_TRY_FROM_U32_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::CharTryFromError>",
        "verus",
        "ensures",
        || CHAR_TRY_FROM_U32_REJECTS_INVALID_SCALAR_VERUS_FRAGMENT,
    )
}

const VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_try_from_carrier.rs");

const CHAR_FITS_IN_U8_VERUS_FRAGMENT: &str = r#"pub open spec fn char_fits_in_u8(value: char) -> bool {
    (value as u32) <= 0xFF
}"#;

const U8_TRY_FROM_CHAR_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn u8_try_from_char_succeeds_with_same_scalar(
    value: char,
    result: Result<u8, <u8 as core::convert::TryFrom<char>>::Error>,
) -> bool {
    char_fits_in_u8(value) ==> (result is Ok && (result->Ok_0 as u32) == (value as u32))
}"#;

const U8_TRY_FROM_CHAR_REJECTS_OUT_OF_RANGE_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn u8_try_from_char_rejects_out_of_range_scalar(
    value: char,
    result: Result<u8, <u8 as core::convert::TryFrom<char>>::Error>,
) -> bool {
    !char_fits_in_u8(value) ==> result is Err
}"#;

impl VerusWitness for RustStdStandard<core::char::TryFromCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_from_char_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_CHAR_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::char::TryFromCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        || {
            <RustStdStandard<core::char::TryFromCharError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        "ensures",
        || CHAR_FITS_IN_U8_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        "ensures",
        || U8_TRY_FROM_CHAR_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::TryFromCharError>",
        "verus",
        "ensures",
        || U8_TRY_FROM_CHAR_REJECTS_OUT_OF_RANGE_SCALAR_VERUS_FRAGMENT,
    )
}

const VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/type_id_carrier.rs");

impl VerusWitness for RustStdStandard<core::any::TypeId> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_type_id_is_reflexive_and_distinguishes_distinct_types".to_owned(),
            VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::any::TypeId>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::any::TypeId>",
        "verus",
        || <RustStdStandard<core::any::TypeId> as VerusWitness>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<core::any::TypeId>,
    "amenable_std::rust_std::RustStdStandard<core::any::TypeId>",
    [
        "type_id_of_matches_spec",
        "type_id_eq_matches_identity",
        "i32_and_bool_type_ids_differ"
    ]
);

const VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/try_from_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::array::TryFromSliceError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_from_slice_rejects_a_length_mismatch".to_owned(),
            VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::array::TryFromSliceError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::array::TryFromSliceError>",
        "verus",
        || {
            <RustStdStandard<std::array::TryFromSliceError> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::array::TryFromSliceError>,
    "amenable_std::rust_std::RustStdStandard<std::array::TryFromSliceError>",
    "try_from_slice_result_matches"
);

// The negative counterpart to `has_length`, registered once here for
// all its real call sites.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::array::TryFromSliceError>,
    "amenable_std::rust_std::RustStdStandard<std::array::TryFromSliceError>",
    "does_not_have_length"
);

const VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/parse_char_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::char::ParseCharError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_parse_char_error_occurs_for_empty_or_multi_character_strings".to_owned(),
            VERIFY_PARSE_CHAR_ERROR_OCCURS_FOR_EMPTY_OR_MULTI_CHARACTER_STRINGS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::char::ParseCharError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::char::ParseCharError>",
        "verus",
        || {
            <RustStdStandard<core::char::ParseCharError> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<core::char::ParseCharError>,
    "amenable_std::rust_std::RustStdStandard<core::char::ParseCharError>",
    "char_from_str_result_matches"
);
