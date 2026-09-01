//! `char`-conversion and C-string FFI error types.

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
    ["type_id_of_matches_spec", "type_id_eq_matches_identity"]
);

const VERIFY_TRY_FROM_SLICE_REJECTS_A_LENGTH_MISMATCH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/try_from_slice_carrier.rs");

impl VerusWitness for RustStdStandard<std::array::TryFromSliceError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

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

const VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/from_utf16_error_carrier.rs");
const FROM_UTF16_RESULT_MATCHES_SINGLE_UNIT_EXAMPLES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf16_result_matches_single_unit_examples(
    units: &[u16],
    result: Result<String, FromUtf16Error>,
) -> bool {
    ((units@.len() == 1 && units@[0] == 0x61) ==> result is Ok)
        && ((units@.len() == 1 && units@[0] == 0xD800) ==> result is Err)
        && ((units@.len() == 1 && units@[0] == 0xDC00) ==> result is Err)
}"#;
const FROM_UTF16_INPUTS_COVER_VALID_AND_LONE_SURROGATE_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf16_inputs_cover_valid_and_lone_surrogate_cases(
    valid: &[u16],
    lone_surrogate: &[u16],
    lone_low_surrogate: &[u16],
) -> bool {
    valid@.len() == 1
        && valid@[0] == 0x61
        && lone_surrogate@.len() == 1
        && lone_surrogate@[0] == 0xD800
        && lone_low_surrogate@.len() == 1
        && lone_low_surrogate@[0] == 0xDC00
}"#;
const FROM_UTF16_CASE_RESULTS_MATCH_ACCEPT_REJECT_TRIPLE_VERUS_FRAGMENT: &str = r#"pub open spec fn from_utf16_case_results_match_accept_reject_triple(result: (bool, bool, bool)) -> bool {
    result.0 && result.1 && result.2
}"#;

impl VerusWitness for RustStdStandard<std::string::FromUtf16Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_utf16_rejects_a_lone_surrogate".to_owned(),
            VERIFY_FROM_UTF16_REJECTS_A_LONE_SURROGATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::string::FromUtf16Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        || {
            <RustStdStandard<std::string::FromUtf16Error> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        "ensures",
        || FROM_UTF16_RESULT_MATCHES_SINGLE_UNIT_EXAMPLES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        "requires",
        || FROM_UTF16_INPUTS_COVER_VALID_AND_LONE_SURROGATE_CASES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf16Error>",
        "verus",
        "ensures",
        || FROM_UTF16_CASE_RESULTS_MATCH_ACCEPT_REJECT_TRIPLE_VERUS_FRAGMENT,
    )
}

const VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/cstring_carrier.rs");

const INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT: &str = r#"pub open spec fn into_vec_u8_spec_matches_input_vec(v: Vec<u8>) -> bool {
    into_vec_u8_spec(v) == v@
}"#;

const CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT: &str = r#"pub open spec fn cstring_new_result_matches_input_bytes<T: Into<Vec<u8>>>(
    bytes: T,
    result: Result<CString, NulError>,
) -> bool {
    (cstring_input_has_no_preterminal_nul(bytes)
        ==> (result is Ok && cstring_bytes_spec(result->Ok_0) == into_vec_u8_spec(bytes)))
        && (cstring_input_has_a_preterminal_nul(bytes) ==> result is Err)
}"#;

impl VerusWitness for RustStdStandard<std::ffi::CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul".to_owned(),
            VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::CString>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::ffi::CString>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
    "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        "verus",
        || <RustStdStandard<std::ffi::CString> as VerusWitness>::proof().to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        "verus",
        "ensures",
        || INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::CString>",
        "verus",
        "ensures",
        || CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT,
    )
}

impl VerusWitness for RustStdStandard<std::ffi::NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul".to_owned(),
            VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::NulError>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::ffi::NulError>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
    "verify_cstring_excludes_the_terminator_and_rejects_interior_nul"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        "verus",
        || <RustStdStandard<std::ffi::NulError> as VerusWitness>::proof().to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        "verus",
        "ensures",
        || INTO_VEC_U8_SPEC_MATCHES_INPUT_VEC_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::NulError>",
        "verus",
        "ensures",
        || CSTRING_NEW_RESULT_MATCHES_INPUT_BYTES_VERUS_FRAGMENT,
    )
}

const VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ffi/from_vec_with_nul_carrier.rs");

const FROM_VEC_WITH_NUL_RESULT_MATCHES_NUL_PLACEMENT_VERUS_FRAGMENT: &str = r#"pub open spec fn from_vec_with_nul_result_matches_nul_placement(
    bytes: Vec<u8>,
    result: Result<CString, FromVecWithNulError>,
) -> bool {
    (bytes@.len() > 0 && bytes@[bytes@.len() - 1] == 0
        && !(exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0)
        ==> result is Ok)
        && (!exists|i: int| 0 <= i < bytes@.len() && bytes@[i] == 0 ==> result is Err)
        && ((exists|i: int| 0 <= i < bytes@.len() - 1 && bytes@[i] == 0) ==> result is Err)
}"#;

impl VerusWitness for RustStdStandard<std::ffi::FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end".to_owned(),
            VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::ffi::FromVecWithNulError>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::ffi::FromVecWithNulError>,
    "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
    "verify_from_vec_with_nul_requires_the_nul_only_at_the_end"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        "verus",
        || {
            <RustStdStandard<std::ffi::FromVecWithNulError> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::FromVecWithNulError>",
        "verus",
        "ensures",
        || FROM_VEC_WITH_NUL_RESULT_MATCHES_NUL_PLACEMENT_VERUS_FRAGMENT,
    )
}

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

pub(super) const VERIFY_RC_DEREFS_TO_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/rc_carrier.rs");
