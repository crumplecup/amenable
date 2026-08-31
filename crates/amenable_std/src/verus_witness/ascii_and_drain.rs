//! UTF-8/ASCII error and escape types, `CStr`, and the draining/extracting
//! iterators over Vec/VecDeque/LinkedList/String.

use super::cell::{
    FROM_UTF8_ERROR_MODEL_AS_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    FROM_UTF8_ERROR_MODEL_INTO_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    FROM_UTF8_ERROR_MODEL_NEW_PRESERVES_BYTES_VERUS_FRAGMENT,
    VERIFY_FROM_UTF8_ERROR_MODEL_RECOVERS_THE_ORIGINAL_BYTES_SRC,
};
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::{NonNulByte, RustStdStandard};
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::string::FromUtf8Error> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_from_utf8_error_model_recovers_the_original_bytes".to_owned(),
            VERIFY_FROM_UTF8_ERROR_MODEL_RECOVERS_THE_ORIGINAL_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::string::FromUtf8Error>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        || {
            <RustStdStandard<std::string::FromUtf8Error> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        "ensures",
        || FROM_UTF8_ERROR_MODEL_NEW_PRESERVES_BYTES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        "ensures",
        || FROM_UTF8_ERROR_MODEL_AS_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::FromUtf8Error>",
        "verus",
        "ensures",
        || FROM_UTF8_ERROR_MODEL_INTO_BYTES_PRESERVES_BYTES_VERUS_FRAGMENT,
    )
}

const VERIFY_ESCAPE_DEFAULT_MODEL_ESCAPES_A_CONTROL_BYTE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ascii_escape_carrier.rs");

impl VerusWitness for RustStdStandard<core::ascii::EscapeDefault> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_escape_default_model_escapes_a_control_byte".to_owned(),
            VERIFY_ESCAPE_DEFAULT_MODEL_ESCAPES_A_CONTROL_BYTE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ascii::EscapeDefault>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ascii::EscapeDefault>",
        "verus",
        || {
            <RustStdStandard<core::ascii::EscapeDefault> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cstr_carrier.rs");

const CSTR_TO_BYTES_MATCHES_MODEL_VERUS_FRAGMENT: &str = r#"pub open spec fn cstr_to_bytes_matches_model(cstr: &CStr, result: &[u8]) -> bool {
    result@ == cstr_bytes_spec(cstr)
}"#;

impl VerusWitness for RustStdStandard<core::ffi::CStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<core::ffi::CStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::CStr>",
        "verus",
        || {
            <RustStdStandard<core::ffi::CStr> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<core::ffi::CStr>",
        "verus",
        "ensures",
        || CSTR_TO_BYTES_MATCHES_MODEL_VERUS_FRAGMENT,
    )
}

/// [`NonNulByte`] reuses the same harness rather than adding a new Verus
/// proof — it names the precondition every `CStr`/`CString`-family proof
/// in this crate already requires, it doesn't prove anything new.
impl VerusWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_owned(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(NonNulByte);

amenable_derive::verus_requires_witness!(
    NonNulByte,
    "amenable_std::NonNulByte",
    "verify_cstr_excludes_the_terminating_nul_from_to_bytes"
);

const VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/ordered_pair_into_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::Drain<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::vec::Drain<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

// Singleton contract, registered once per real type this carrier backs
// (matching the harness registration above): a freshly-constructed
// model always starts positioned before the first element.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::vec::Drain<'static, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::vec::Drain<'static, i32>>",
    [
        "ordered_pair_into_iter_model_starts_at_position_zero",
        "ordered_pair_into_iter_advance_result_matches",
    ]
);

impl VerusWitness for RustStdStandard<std::collections::vec_deque::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::vec_deque::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IntoIter<i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::collections::linked_list::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::linked_list::IntoIter<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IntoIter<i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::collections::linked_list::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::linked_list::Iter<'static, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::Iter<'static, i32>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

impl VerusWitness for RustStdStandard<std::string::Drain<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_into_iter_model_yields_owned_values_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_INTO_ITER_MODEL_YIELDS_OWNED_VALUES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::string::Drain<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
        "verus",
        || {
            <RustStdStandard<std::string::Drain<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::string::Drain<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::string::Drain<'static>>",
    "ordered_pair_into_iter_model_starts_at_position_zero"
);

const VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/vec_extract_if_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_extract_if_model_partitions_by_the_predicate".to_owned(),
            VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<std::vec::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
    "partition_result_matches"
);

impl VerusWitness
    for RustStdStandard<
        std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_extract_if_model_partitions_by_the_predicate".to_owned(),
            VERIFY_VEC_EXTRACT_IF_MODEL_PARTITIONS_BY_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::ExtractIf<'static, i32, fn(&mut i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SPLICE_MODEL_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/vec_splice_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_splice_model_replaces_a_range_and_yields_what_it_removed".to_owned(),
            VERIFY_SPLICE_MODEL_REPLACES_A_RANGE_AND_YIELDS_WHAT_IT_REMOVED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>);

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>",
    "splice_result_matches"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::vec::Splice<'static, std::vec::IntoIter<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

pub(super) const VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter_transform_carrier.rs");
