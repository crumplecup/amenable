//! Core collection types (BTreeMap/BTreeSet/VecDeque/BinaryHeap/LinkedList)
//! and their basic iterators.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::rc_arc_hash::VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC;
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::collections::BTreeMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_btree_map_insert_get_remove_round_trips".to_owned(),
            VERIFY_BTREE_MAP_INSERT_GET_REMOVE_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BTreeMap<i32, i32>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::collections::BTreeMap<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
    "verify_btree_map_insert_get_remove_round_trips"
);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::collections::BTreeMap<i32, i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
    "verify_btree_map_insert_get_remove_round_trips"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BTreeMap<i32, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::BTreeMap<i32, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIPS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/btree_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::BTreeSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_btree_set_insert_contains_remove_round_trips".to_owned(),
            VERIFY_BTREE_SET_INSERT_CONTAINS_REMOVE_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::BTreeSet<i32>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::collections::BTreeSet<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
    "verify_btree_set_insert_contains_remove_round_trips"
);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::collections::BTreeSet<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
    "verify_btree_set_insert_contains_remove_round_trips"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BTreeSet<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::BTreeSet<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/vec_deque_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::VecDeque<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_deque_pushes_and_pops_from_both_ends".to_owned(),
            VERIFY_VEC_DEQUE_PUSHES_AND_POPS_FROM_BOTH_ENDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::VecDeque<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::VecDeque<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::VecDeque<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_TRY_RESERVE_PRESERVES_VEC_CONTENTS_REGARDLESS_OF_OUTCOME_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/try_reserve_error_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::TryReserveError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_try_reserve_preserves_vec_contents_regardless_of_outcome".to_owned(),
            VERIFY_TRY_RESERVE_PRESERVES_VEC_CONTENTS_REGARDLESS_OF_OUTCOME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::TryReserveError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::TryReserveError>",
        "verus",
        || {
            <RustStdStandard<std::collections::TryReserveError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_INTO_ITER_ROUND_TRIPS_VIA_COLLECT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/vec_into_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::vec::IntoIter<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_into_iter_round_trips_via_collect".to_owned(),
            VERIFY_VEC_INTO_ITER_ROUND_TRIPS_VIA_COLLECT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::vec::IntoIter<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::vec::IntoIter<i32>>",
        "verus",
        || {
            <RustStdStandard<std::vec::IntoIter<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_VEC_DEQUE_ITER_ROUND_TRIPS_VIA_COLLECT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/vec_deque_iter_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_vec_deque_iter_round_trips_via_collect".to_owned(),
            VERIFY_VEC_DEQUE_ITER_ROUND_TRIPS_VIA_COLLECT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::vec_deque::Iter<'static, i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::vec_deque::Iter<'static, i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CHARS_YIELDS_CHARACTERS_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/str_and_char/chars_carrier.rs");

impl VerusWitness for RustStdStandard<std::str::Chars<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_chars_yields_characters_in_order".to_owned(),
            VERIFY_CHARS_YIELDS_CHARACTERS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::str::Chars<'static>>);

amenable_derive::verus_requires_witness!(
    RustStdStandard<std::str::Chars<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
    "verify_chars_yields_characters_in_order"
);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::str::Chars<'static>>,
    "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
    "verify_chars_yields_characters_in_order"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::str::Chars<'static>>",
        "verus",
        || {
            <RustStdStandard<std::str::Chars<'static>> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_MAX_HEAP_PAIR_POPS_THE_MAXIMUM_FIRST_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/binary_heap_carrier.rs");

const BINARY_HEAP_MODEL_RECORDS_VALUES_IN_HEAP_ORDER_VERUS_FRAGMENT: &str = r#"pub open spec fn binary_heap_model_records_values_in_heap_order(
    observed_max: i32,
    observed_min: i32,
    a: i32,
    b: i32,
) -> bool {
    observed_max == if a >= b { a } else { b }
        && observed_min == if a >= b { b } else { a }
}"#;
const BINARY_HEAP_MODEL_POP_RETURNS_RECORDED_ORDER_VERUS_FRAGMENT: &str = r#"pub open spec fn binary_heap_model_pop_returns_recorded_order(
    first: i32,
    second: i32,
    max: i32,
    min: i32,
) -> bool {
    first == max && second == min
}"#;

impl VerusWitness for RustStdStandard<std::collections::BinaryHeap<i32>> {
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

bridge_verus_witness!(RustStdStandard<std::collections::BinaryHeap<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::collections::BinaryHeap<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
    "verify_max_heap_pair_pops_the_maximum_first"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::BinaryHeap<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        "verus",
        "ensures",
        || BINARY_HEAP_MODEL_RECORDS_VALUES_IN_HEAP_ORDER_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::BinaryHeap<i32>>",
        "verus",
        "ensures",
        || BINARY_HEAP_MODEL_POP_RETURNS_RECORDED_ORDER_VERUS_FRAGMENT,
    )
}

const VERIFY_FIFO_QUEUE_PAIR_POPS_IN_PUSH_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/collections/linked_list_carrier.rs");

impl VerusWitness for RustStdStandard<std::collections::LinkedList<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fifo_queue_pair_pops_in_push_order".to_owned(),
            VERIFY_FIFO_QUEUE_PAIR_POPS_IN_PUSH_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::collections::LinkedList<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::collections::LinkedList<i32>>",
        "verus",
        || {
            <RustStdStandard<std::collections::LinkedList<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/cell_carrier.rs");

pub(super) const CELL_MODEL_NEW_STORES_INITIAL_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_new_stores_initial_value(observed: int, initial: int) -> bool {
    observed == initial
}"#;
pub(super) const CELL_MODEL_GET_READS_CURRENT_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_get_reads_current_value(observed: int, current: int) -> bool {
    observed == current
}"#;
pub(super) const CELL_MODEL_REPLACE_RETURNS_PREVIOUS_VALUE_VERUS_FRAGMENT: &str = r#"pub open spec fn cell_model_replace_returns_previous_value(observed: int, previous: int) -> bool {
    observed == previous
}"#;
