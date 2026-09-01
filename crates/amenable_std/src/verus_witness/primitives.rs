//! Core primitive wrapper types: char, String, Ordering, Option/Result, and
//! the small numeric wrapper types (Wrapping, Saturating, Reverse,
//! ManuallyDrop).

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::{RustStdStandard, ValidUnicodeScalar};
use amenable_core::Evidence;

const VERIFY_CHAR_ROUNDTRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/char_carrier.rs");

// `verify_char_roundtrip`'s real VerusCallShape (params/requires/ensures/
// imports) is no longer registered by hand here -- `verus_call_shape`
// derives it by parsing the real signature directly from
// crates/amenable_verus/src/rust_std/char_carrier.rs. Reused by
// RustStdStandard<char>, ValidUnicodeScalar, and the Verus derive-witness
// canary's CheckedVerusExportLeaf, all keyed by this one harness name.

impl VerusWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_roundtrip".to_owned(),
            VERIFY_CHAR_ROUNDTRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<char>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<char>,
    "amenable_std::rust_std::RustStdStandard<char>",
    "verify_char_roundtrip"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "verus",
        || <RustStdStandard<char> as VerusWitness>::proof().to_string(),
    )
}

/// The [`ValidUnicodeScalar`] contract type reuses `verify_char_roundtrip`
/// rather than adding a new Verus proof — it names the postcondition the
/// spec already checks, it doesn't prove anything new.
impl VerusWitness for ValidUnicodeScalar {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_char_roundtrip".to_owned(),
            VERIFY_CHAR_ROUNDTRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ValidUnicodeScalar);

// verify_char_roundtrip's second real clause only -- see
// RustStdStandard<char> just above for its first.
amenable_derive::verus_ensures_witness!(
    ValidUnicodeScalar,
    "amenable_std::ValidUnicodeScalar",
    "verify_char_roundtrip",
    [1]
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::ValidUnicodeScalar",
        "verus",
        || <ValidUnicodeScalar as VerusWitness>::proof().to_string(),
    )
}

const VERIFY_STRING_ROUNDTRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/string_carrier.rs");

impl VerusWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_string_roundtrip".to_owned(),
            VERIFY_STRING_ROUNDTRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<String>",
        "verus",
        || <RustStdStandard<String> as VerusWitness>::proof().to_string(),
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<String>,
    "amenable_std::rust_std::RustStdStandard<String>",
    "string_roundtrip_result_matches"
);

const VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/ordering_carrier.rs");

impl VerusWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordering_reverse_swaps_less_and_greater".to_owned(),
            VERIFY_ORDERING_REVERSE_SWAPS_LESS_AND_GREATER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        "verus",
        || {
            <RustStdStandard<std::cmp::Ordering> as VerusWitness>::proof().to_string()
        },
    )
}

// The real law `.reverse()` obeys -- named once, called from both the
// trusted `assume_specification` on the real method and its own
// re-derivation, instead of restated at each.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cmp::Ordering>,
    "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
    "ordering_reverse_swaps_less_and_greater"
);

const VERIFY_OPTION_UNWRAP_RETURNS_THE_WRAPPED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/option_carrier.rs");

impl VerusWitness for RustStdStandard<Option<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_option_unwrap_returns_the_wrapped_value".to_owned(),
            VERIFY_OPTION_UNWRAP_RETURNS_THE_WRAPPED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Option<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Option<i32>>",
        "verus",
        || <RustStdStandard<Option<i32>> as VerusWitness>::proof().to_string(),
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<Option<i32>>,
    "amenable_std::rust_std::RustStdStandard<Option<i32>>",
    ["option_wraps_the_given_value", "option_is_none"]
);

const VERIFY_RESULT_UNWRAP_RETURNS_THE_OK_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/result_carrier.rs");

impl VerusWitness for RustStdStandard<Result<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_result_unwrap_returns_the_ok_value".to_owned(),
            VERIFY_RESULT_UNWRAP_RETURNS_THE_OK_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<Result<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<Result<i32, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<Result<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<Result<i32, i32>>",
    [
        "result_wraps_the_given_ok_value",
        "result_wraps_the_given_err_value"
    ]
);

const VERIFY_WRAPPING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/wrapping_carrier.rs");

impl VerusWitness for RustStdStandard<std::num::Wrapping<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_wrapping_field_roundtrips_the_constructed_value".to_owned(),
            VERIFY_WRAPPING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::num::Wrapping<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::num::Wrapping<i32>>",
        "verus",
        || {
            <RustStdStandard<std::num::Wrapping<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SATURATING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/saturating_carrier.rs");

impl VerusWitness for RustStdStandard<std::num::Saturating<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_saturating_field_roundtrips_the_constructed_value".to_owned(),
            VERIFY_SATURATING_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::num::Saturating<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::num::Saturating<i32>>",
        "verus",
        || {
            <RustStdStandard<std::num::Saturating<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REVERSE_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/reverse_carrier.rs");

impl VerusWitness for RustStdStandard<std::cmp::Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_reverse_field_roundtrips_the_constructed_value".to_owned(),
            VERIFY_REVERSE_FIELD_ROUNDTRIPS_THE_CONSTRUCTED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cmp::Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Reverse<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cmp::Reverse<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/manually_drop_carrier.rs");

impl VerusWitness for RustStdStandard<std::mem::ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_manually_drop_derefs_and_into_inner_round_trip".to_owned(),
            VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::mem::ManuallyDrop<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::mem::ManuallyDrop<i32>>",
        "verus",
        || {
            <RustStdStandard<std::mem::ManuallyDrop<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_FP_CATEGORY_MATCHES_THE_VALUE_IT_CLASSIFIES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/num/fp_category_carrier.rs");
pub(super) const FP_CATEGORY_CLASSIFY_RESULT_MATCHES_SPECIAL_VALUE_CATEGORIES_VERUS_FRAGMENT: &str = r#"pub open spec fn fp_category_classify_result_matches_special_value_categories(
    value: f64,
    result: FpCategory,
) -> bool {
    (value.is_nan_spec() ==> result == FpCategory::Nan)
        && (value.is_infinite_spec() ==> result == FpCategory::Infinite)
}"#;
pub(super) const FP_CATEGORY_INPUTS_COVER_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn fp_category_inputs_cover_nan_and_infinite_cases(nan: f64, infinite: f64) -> bool {
    nan.is_nan_spec() && infinite.is_infinite_spec()
}"#;
pub(super) const FP_CATEGORY_RESULTS_MATCH_NAN_AND_INFINITE_CASES_VERUS_FRAGMENT: &str = r#"pub open spec fn fp_category_results_match_nan_and_infinite_cases(
    result: (FpCategory, FpCategory),
) -> bool {
    result.0 == FpCategory::Nan && result.1 == FpCategory::Infinite
}"#;
