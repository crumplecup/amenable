//! The `std::path` types.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::process_and_atomic_tail::VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC;
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::path::Ancestors<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ancestors_model_yields_self_then_each_parent_up_to_root".to_owned(),
            VERIFY_ANCESTORS_MODEL_YIELDS_SELF_THEN_EACH_PARENT_UP_TO_ROOT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Ancestors<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Ancestors<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Ancestors<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMPONENT_MODEL_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Component<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_component_model_distinguishes_root_from_normal_segments".to_owned(),
            VERIFY_COMPONENT_MODEL_DISTINGUISHES_ROOT_FROM_NORMAL_SEGMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Component<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Component<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Component<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_COMPONENTS_MODEL_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Components<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_components_model_yields_root_then_named_segments_in_order".to_owned(),
            VERIFY_COMPONENTS_MODEL_YIELDS_ROOT_THEN_NAMED_SEGMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Components<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Components<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Components<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ITER_MODEL_YIELDS_THE_NAMED_SEGMENTS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_components_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Iter<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_iter_model_yields_the_named_segments".to_owned(),
            VERIFY_ITER_MODEL_YIELDS_THE_NAMED_SEGMENTS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Iter<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Iter<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract: `.iter()` over the fixed example `"/a/b"` always
// yields exactly 3 segments.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::path::Iter<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::path::Iter<'static>>",
    "path_iter_yields_three_segments"
);

const VERIFY_DISPLAY_MODEL_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_display_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Display<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_display_model_renders_a_valid_utf8_path_verbatim".to_owned(),
            VERIFY_DISPLAY_MODEL_RENDERS_A_VALID_UTF8_PATH_VERBATIM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Display<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Display<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Display<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PATH_MODEL_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Path> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_path_model_derives_extension_file_name_and_parent".to_owned(),
            VERIFY_PATH_MODEL_DERIVES_EXTENSION_FILE_NAME_AND_PARENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Path>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Path>",
        "verus",
        || {
            <RustStdStandard<std::path::Path> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PATH_BUF_MODEL_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_buf_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::PathBuf> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_path_buf_model_push_pop_and_join_build_the_expected_path".to_owned(),
            VERIFY_PATH_BUF_MODEL_PUSH_POP_AND_JOIN_BUILD_THE_EXPECTED_PATH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::PathBuf>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::PathBuf>",
        "verus",
        || {
            <RustStdStandard<std::path::PathBuf> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PREFIX_MODEL_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::Prefix<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_prefix_model_disk_identifies_the_drive_letter".to_owned(),
            VERIFY_PREFIX_MODEL_DISK_IDENTIFIES_THE_DRIVE_LETTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::Prefix<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::Prefix<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::Prefix<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_PREFIX_COMPONENT_MODEL_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::PrefixComponent<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_prefix_component_model_pairs_raw_text_with_parsed_prefix".to_owned(),
            VERIFY_PREFIX_COMPONENT_MODEL_PAIRS_RAW_TEXT_WITH_PARSED_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::PrefixComponent<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::PrefixComponent<'static>>",
        "verus",
        || {
            <RustStdStandard<std::path::PrefixComponent<'static>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STRIP_PREFIX_ERROR_MODEL_REPORTS_A_NON_MATCHING_PREFIX_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/path_strip_prefix_carrier.rs");

impl VerusWitness for RustStdStandard<std::path::StripPrefixError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_strip_prefix_error_model_reports_a_non_matching_prefix".to_owned(),
            VERIFY_STRIP_PREFIX_ERROR_MODEL_REPORTS_A_NON_MATCHING_PREFIX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::path::StripPrefixError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::path::StripPrefixError>",
        "verus",
        || {
            <RustStdStandard<std::path::StripPrefixError> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/fs_path_carrier.rs");

pub(super) const DIR_BUILDER_MODEL_CREATES_NESTED_DIRECTORIES_RECURSIVELY_VERUS_FRAGMENT: &str = r#"pub open spec fn dir_builder_model_creates_nested_directories_recursively(
    a: char,
    b: char,
    c: char,
    result: DirBuilderResult,
) -> bool {
    &&& result.0 == (a,)
    &&& result.1 == (a, b)
    &&& result.2 == (a, b, c)
}"#;
