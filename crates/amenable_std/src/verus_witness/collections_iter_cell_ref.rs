//! The remaining mutable/draining collection iterators, and `RefCell`'s
//! `Ref`/`RefMut` guards.

use super::collections::VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::mem_slice_net_non_zero::VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC;
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_iter_mut_model_writes_through_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ordered_pair_iter_mut_model_writes_through_in_order".to_owned(),
            VERIFY_ORDERED_PAIR_ITER_MUT_MODEL_WRITES_THROUGH_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::linked_list::IterMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::linked_list::IterMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/unordered_pair_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unordered_pair_model_yields_every_element_once".to_owned(),
            VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::Drain<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Drain<'static, i32>>",
    "drain_result_matches_order"
);

impl VerusWitness for RustStdStandard<std::collections::binary_heap::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unordered_pair_model_yields_every_element_once".to_owned(),
            VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::IntoIter<i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unordered_pair_model_yields_every_element_once".to_owned(),
            VERIFY_UNORDERED_PAIR_MODEL_YIELDS_EVERY_ELEMENT_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

impl VerusWitness for RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_max_heap_pair_pops_the_maximum_first".to_owned(),
            VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::binary_heap::PeekMut<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_REF_MODEL_DEREFS_TO_THE_BORROWED_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/ref_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::Ref<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_model_derefs_to_the_borrowed_value".to_owned(),
            VERIFY_REF_MODEL_DEREFS_TO_THE_BORROWED_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::Ref<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Ref<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::Ref<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REF_MUT_MODEL_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/ref_mut_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::RefMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_mut_model_derefs_and_writes_through_to_the_cell".to_owned(),
            VERIFY_REF_MUT_MODEL_DEREFS_AND_WRITES_THROUGH_TO_THE_CELL_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::RefMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::RefMut<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::RefMut<'static, i32>> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_DECODE_UTF16_MODEL_ROUND_TRIPS_AND_REPORTS_LONE_SURROGATES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/decode_utf16_carrier.rs");

pub(super) const DECODE_UTF16_UNIT_IS_NON_SURROGATE_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_unit_is_non_surrogate(unit: u16) -> bool {
    unit < 0xD800 || unit > 0xDFFF
}"#;
pub(super) const DECODE_UTF16_UNIT_IS_SURROGATE_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_unit_is_surrogate(unit: u16) -> bool {
    0xD800 <= unit <= 0xDFFF
}"#;
pub(super) const DECODE_UTF16_BMP_UNIT_DECODES_TO_SAME_SCALAR_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_bmp_unit_decodes_to_same_scalar(
    unit: u16,
    result: Option<u32>,
) -> bool {
    result == Some(unit as u32)
}"#;

pub(super) const DECODE_UTF16_LONE_SURROGATE_REPORTS_SAME_UNIT_VERUS_FRAGMENT: &str = r#"pub open spec fn decode_utf16_lone_surrogate_reports_same_unit(
    unit: u16,
    result: Result<u32, u16>,
) -> bool {
    result == Err(unit)
}"#;
