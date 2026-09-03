//! The `Successors`/`FromFn` iterator adapters, and the `std::fmt` formatting
//! types (`Alignment`, `Formatter`, `Arguments`, the `Debug*` builders).

use super::iter_adapters_d::VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_successors_model_generates_from_the_previous_item".to_owned(),
            VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Successors<i32, fn(&i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FROM_FN_MODEL_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_fn_model_yields_until_the_closure_returns_none".to_owned(),
            VERIFY_FROM_FN_MODEL_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ALIGNMENT_MODEL_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::Alignment> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_alignment_model_reaches_the_formatter_from_the_format_spec".to_owned(),
            VERIFY_ALIGNMENT_MODEL_REACHES_THE_FORMATTER_FROM_THE_FORMAT_SPEC_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Alignment>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Alignment>",
        "verus",
        || {
            <RustStdStandard<std::fmt::Alignment> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_FORMATTER_MODEL_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::Formatter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_formatter_model_exposes_the_parsed_width_and_precision".to_owned(),
            VERIFY_FORMATTER_MODEL_EXPOSES_THE_PARSED_WIDTH_AND_PRECISION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Formatter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Formatter<'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::Formatter<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ARGUMENTS_MODEL_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");
const FMT_ARGUMENTS_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_arguments_result_matches_display_token(display_token: i32, result: i32) -> bool {
    result == display_token
}"#;
const FMT_FROM_FN_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_from_fn_result_matches_display_token(display_token: i32, result: i32) -> bool {
    result == display_token
}"#;
const FMT_DEBUG_STRUCT_RESULT_MATCHES_NAMED_FIELDS_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_struct_result_matches_named_fields(
    type_label: u8,
    field_label: u8,
    value_token: i32,
    result: (u8, u8, i32),
) -> bool {
    result == (type_label, field_label, value_token)
}"#;
const FMT_DEBUG_TUPLE_RESULT_MATCHES_POSITIONAL_FIELDS_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_tuple_result_matches_positional_fields(
    type_label: u8,
    value_token: i32,
    result: (u8, i32),
) -> bool {
    result == (type_label, value_token)
}"#;
const FMT_DEBUG_LIST_RESULT_MATCHES_ENTRIES_IN_BRACKETS_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_list_result_matches_entries_in_brackets(
    first_token: i32,
    second_token: i32,
    result: (i32, i32),
) -> bool {
    result == (first_token, second_token)
}"#;
const FMT_DEBUG_SET_RESULT_MATCHES_ENTRIES_IN_BRACES_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_set_result_matches_entries_in_braces(
    first_token: i32,
    second_token: i32,
    result: (i32, i32),
) -> bool {
    result == (first_token, second_token)
}"#;
const FMT_DEBUG_MAP_RESULT_MATCHES_KEY_VALUE_PAIR_VERUS_FRAGMENT: &str = r#"pub open spec fn fmt_debug_map_result_matches_key_value_pair(
    key_label: u8,
    value_token: i32,
    result: (u8, i32),
) -> bool {
    result == (key_label, value_token)
}"#;

impl VerusWitness for RustStdStandard<std::fmt::Arguments<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_arguments_model_renders_the_same_as_the_value_itself".to_owned(),
            VERIFY_ARGUMENTS_MODEL_RENDERS_THE_SAME_AS_THE_VALUE_ITSELF_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::Arguments<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Arguments<'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::Arguments<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::Arguments<'static>>",
        "verus",
        "ensures",
        || FMT_ARGUMENTS_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT,
    )
}

const VERIFY_FROM_FN_MODEL_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_fn_model_forwards_display_to_the_supplied_closure".to_owned(),
            VERIFY_FROM_FN_MODEL_FORWARDS_DISPLAY_TO_THE_SUPPLIED_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::FromFn<fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result>>",
        "verus",
        "ensures",
        || FMT_FROM_FN_RESULT_MATCHES_DISPLAY_TOKEN_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_STRUCT_MODEL_RENDERS_NAMED_FIELDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugStruct<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_struct_model_renders_named_fields".to_owned(),
            VERIFY_DEBUG_STRUCT_MODEL_RENDERS_NAMED_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugStruct<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugStruct<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugStruct<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugStruct<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_STRUCT_RESULT_MATCHES_NAMED_FIELDS_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_TUPLE_MODEL_RENDERS_POSITIONAL_FIELDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugTuple<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_tuple_model_renders_positional_fields".to_owned(),
            VERIFY_DEBUG_TUPLE_MODEL_RENDERS_POSITIONAL_FIELDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugTuple<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugTuple<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugTuple<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugTuple<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_TUPLE_RESULT_MATCHES_POSITIONAL_FIELDS_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_LIST_MODEL_RENDERS_ENTRIES_IN_BRACKETS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugList<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_list_model_renders_entries_in_brackets".to_owned(),
            VERIFY_DEBUG_LIST_MODEL_RENDERS_ENTRIES_IN_BRACKETS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugList<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugList<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugList<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugList<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_LIST_RESULT_MATCHES_ENTRIES_IN_BRACKETS_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_SET_MODEL_RENDERS_ENTRIES_IN_BRACES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugSet<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_set_model_renders_entries_in_braces".to_owned(),
            VERIFY_DEBUG_SET_MODEL_RENDERS_ENTRIES_IN_BRACES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugSet<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugSet<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugSet<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugSet<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_SET_RESULT_MATCHES_ENTRIES_IN_BRACES_VERUS_FRAGMENT,
    )
}

const VERIFY_DEBUG_MAP_MODEL_RENDERS_KEY_VALUE_PAIRS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/fmt_carrier.rs");

impl VerusWitness for RustStdStandard<std::fmt::DebugMap<'static, 'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_debug_map_model_renders_key_value_pairs".to_owned(),
            VERIFY_DEBUG_MAP_MODEL_RENDERS_KEY_VALUE_PAIRS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::fmt::DebugMap<'static, 'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugMap<'static, 'static>>",
        "verus",
        || {
            <RustStdStandard<std::fmt::DebugMap<'static, 'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::fmt::DebugMap<'static, 'static>>",
        "verus",
        "ensures",
        || FMT_DEBUG_MAP_RESULT_MATCHES_KEY_VALUE_PAIR_VERUS_FRAGMENT,
    )
}

pub(super) const VERIFY_DISCRIMINANT_MODEL_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/discriminant_carrier.rs");
