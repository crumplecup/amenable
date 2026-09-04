//! C-string FFI error types: UTF-16 conversion, `CString`/`NulError`, and
//! nul-placement validation.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
