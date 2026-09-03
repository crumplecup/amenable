//! Numeric parse-error types and the core allocation primitives (Box, Layout,
//! LayoutError, Vec).

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::primitives::{
    FP_CATEGORY_CLASSIFY_RESULT_MATCHES_SPECIAL_VALUE_CATEGORIES_VERUS_FRAGMENT,
    FP_CATEGORY_INPUTS_COVER_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    FP_CATEGORY_RESULTS_MATCH_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC,
};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<core::num::FpCategory> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fp_category_matches_the_value_it_classifies".to_owned(),
            VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::FpCategory>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        || {
            <RustStdStandard<core::num::FpCategory> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        "ensures",
        || FP_CATEGORY_CLASSIFY_RESULT_MATCHES_SPECIAL_VALUE_CATEGORIES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        "requires",
        || FP_CATEGORY_INPUTS_COVER_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::FpCategory>",
        "verus",
        "ensures",
        || FP_CATEGORY_RESULTS_MATCH_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT,
    )
}

const VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/int_error_kind_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::IntErrorKind> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_int_error_kind_classifies_parse_failures".to_owned(),
            VERIFY_INT_ERROR_KIND_CLASSIFIES_PARSE_FAILURES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::IntErrorKind>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
        "verus",
        || {
            <RustStdStandard<core::num::IntErrorKind> as VerusWitness>::proof().to_string()
        },
    )
}

// The shared sequence-length precondition `amenable_std::verus_witness`
// registers for several accommodation models that need a specific,
// symbolic-independent element count.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<core::num::IntErrorKind>,
    "amenable_std::rust_std::RustStdStandard<core::num::IntErrorKind>",
    "has_length"
);

const VERIFY_PARSE_INT_ERROR_MODEL_REPORTS_THE_KIND_OF_THE_FAILURE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/int_error_kind_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::ParseIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_parse_int_error_model_reports_the_kind_of_the_failure".to_owned(),
            VERIFY_PARSE_INT_ERROR_MODEL_REPORTS_THE_KIND_OF_THE_FAILURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::ParseIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
        "verus",
        || {
            <RustStdStandard<core::num::ParseIntError> as VerusWitness>::proof().to_string()
        },
    )
}

// `ParseIntError::kind()`'s own postcondition, plus `i32::from_str`'s
// own `Empty`/lowercase-`InvalidDigit` conjuncts -- all three real
// claims this file's own `int_error_kind_carrier.rs` states about
// `ParseIntError`, named once each.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<core::num::ParseIntError>,
    "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
    [
        "parse_int_error_kind_matches",
        "from_str_empty_reports_empty_kind",
        "from_str_lowercase_reports_invalid_digit_kind",
    ]
);

// The shared "string starts with a lowercase ASCII letter" precondition
// `int_error_kind_carrier.rs` states once, reused by both its own
// `from_str` assume_specification and its own harness fn's `requires`.
amenable_derive::verus_requires_predicate!(
    RustStdStandard<core::num::ParseIntError>,
    "amenable_std::rust_std::RustStdStandard<core::num::ParseIntError>",
    "starts_with_lowercase_ascii_letter"
);

const VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/parse_float_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::ParseFloatError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_parse_float_error_occurs_only_for_unparseable_input".to_owned(),
            VERIFY_PARSE_FLOAT_ERROR_OCCURS_ONLY_FOR_UNPARSEABLE_INPUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::ParseFloatError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
        "verus",
        || {
            <RustStdStandard<core::num::ParseFloatError> as VerusWitness>::proof().to_string()
        },
    )
}

// The two fixed examples this file's whole claim rests on.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<core::num::ParseFloatError>,
    "amenable_std::rust_std::RustStdStandard<core::num::ParseFloatError>",
    "parse_float_examples_match_expected_outcome"
);

const VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/try_from_int_error_carrier.rs");

impl VerusWitness for RustStdStandard<core::num::TryFromIntError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_from_int_error_occurs_exactly_when_out_of_range".to_owned(),
            VERIFY_TRY_FROM_INT_ERROR_OCCURS_EXACTLY_WHEN_OUT_OF_RANGE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::num::TryFromIntError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::num::TryFromIntError>",
        "verus",
        || {
            <RustStdStandard<core::num::TryFromIntError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/box_carrier.rs");

impl VerusWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_box_derefs_and_writes_through".to_owned(),
            VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Box<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<Box<i32>>,
    "amenable_std::rust_std::RustStdStandard<Box<i32>>",
    "verify_box_derefs_and_writes_through"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        "verus",
        || <RustStdStandard<Box<i32>> as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/layout_carrier.rs");

impl VerusWitness for RustStdStandard<core::alloc::Layout> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment".to_owned(),
            VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::alloc::Layout>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::alloc::Layout>",
        "verus",
        || <RustStdStandard<core::alloc::Layout> as VerusWitness>::proof().to_string(),
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<core::alloc::Layout>,
    "amenable_std::rust_std::RustStdStandard<core::alloc::Layout>",
    "is_power_of_two_spec"
);

impl VerusWitness for RustStdStandard<core::alloc::LayoutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment".to_owned(),
            VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::alloc::LayoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::alloc::LayoutError>",
        "verus",
        || {
            <RustStdStandard<core::alloc::LayoutError> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<core::alloc::LayoutError>,
    "amenable_std::rust_std::RustStdStandard<core::alloc::LayoutError>",
    "from_size_align_rejects_a_non_power_of_two_alignment"
);

const VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/vec_carrier.rs");

impl VerusWitness for RustStdStandard<Vec<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_push_pop_round_trips".to_owned(),
            VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Vec<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
        "verus",
        || <RustStdStandard<Vec<i32>> as VerusWitness>::proof().to_string(),
    )
}

// A singleton contract: this len-after-one-push fact is never restated
// anywhere else, but still gets a real, named, callable predicate
// rather than staying an unnamed raw literal -- a named contract's
// whole point is giving an assumption an explicit, auditable source,
// not just deduplicating repeated text.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<Vec<i32>>,
    "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
    "vec_len_after_one_push_is_one"
);

pub(super) const VERIFY_CHAR_TRY_FROM_FAILS_EXACTLY_FOR_SURROGATES_AND_OUT_OF_RANGE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_try_from_carrier.rs");

pub(super) const U32_IS_VALID_UNICODE_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn u32_is_valid_unicode_scalar(value: u32) -> bool {
    value <= 0x0010_FFFF && !(0xD800 <= value && value <= 0xDFFF)
}"#;

pub(super) const CHAR_TRY_FROM_U32_SUCCEEDS_WITH_SAME_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn char_try_from_u32_succeeds_with_same_scalar(
    value: u32,
    result: Result<char, <char as core::convert::TryFrom<u32>>::Error>,
) -> bool {
    u32_is_valid_unicode_scalar(value) ==> (result is Ok && (result->Ok_0 as u32) == value)
}"#;

pub(super) const CHAR_TRY_FROM_U32_REJECTS_INVALID_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn char_try_from_u32_rejects_invalid_scalar(
    value: u32,
    result: Result<char, <char as core::convert::TryFrom<u32>>::Error>,
) -> bool {
    !u32_is_valid_unicode_scalar(value) ==> result is Err
}"#;
