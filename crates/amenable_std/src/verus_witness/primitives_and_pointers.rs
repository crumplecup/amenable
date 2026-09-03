//! The primitive-shape and pointer/reference carriers: fixed arrays and
//! slices, `str` itself, tuples, function pointers, raw pointers, and
//! shared/mutable references.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::panic_ops_time_future::{
    VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC,
    VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC,
};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<[i32; 3]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_model_indexing_and_length".to_owned(),
            VERIFY_ARRAY_MODEL_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<[i32; 3]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32; 3]>",
        "verus",
        || { <RustStdStandard<[i32; 3]> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<[i32]> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_slice_model_indexing_and_length".to_owned(),
            VERIFY_SLICE_MODEL_INDEXING_AND_LENGTH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<[i32]>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<[i32]>",
        "verus",
        || { <RustStdStandard<[i32]> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_str_model_byte_length_and_content".to_owned(),
            VERIFY_STR_MODEL_BYTE_LENGTH_AND_CONTENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<str>);

// The real Verus proof sites across the `str`, `path`, `process`, `env`,
// and panic carriers all call the shared `text_view_matches_expected`
// spec fn directly.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<str>,
    "amenable_std::rust_std::RustStdStandard<str>",
    "text_view_matches_expected"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "verus",
        || { <RustStdStandard<str> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<(i32, i32)> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_tuple_model_field_access".to_owned(),
            VERIFY_TUPLE_MODEL_FIELD_ACCESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<(i32, i32)>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<(i32, i32)>",
        "verus",
        || { <RustStdStandard<(i32, i32)> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<fn(i32) -> i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fn_pointer_model_calls_the_underlying_function".to_owned(),
            VERIFY_FN_POINTER_MODEL_CALLS_THE_UNDERLYING_FUNCTION_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<fn(i32) -> i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<fn(i32) -> i32>",
        "verus",
        || { <RustStdStandard<fn(i32) -> i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_const_pointer_model_cast_is_reproducible".to_owned(),
            VERIFY_CONST_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*const i32>",
        "verus",
        || { <RustStdStandard<*const i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_mut_pointer_model_cast_is_reproducible".to_owned(),
            VERIFY_MUT_POINTER_MODEL_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*mut i32>",
        "verus",
        || { <RustStdStandard<*mut i32> as VerusWitness>::proof().to_string() },
    )
}

impl VerusWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_shared_reference_model_dereferences_to_the_referent".to_owned(),
            VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "verus",
        || { <RustStdStandard<&'static i32> as VerusWitness>::proof().to_string() },
    )
}

const VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/primitive_shapes_carrier.rs");

impl VerusWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_mutable_reference_model_dereferences_to_and_updates_the_referent".to_owned(),
            VERIFY_MUTABLE_REFERENCE_MODEL_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "verus",
        || { <RustStdStandard<&'static mut i32> as VerusWitness>::proof().to_string() },
    )
}
