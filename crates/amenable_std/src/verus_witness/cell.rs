//! Interior-mutability cell types
//! (Cell/RefCell/OnceCell/UnsafeCell/LazyCell/LazyLock), `array::IntoIter` and
//! its gallery markers, and the Rc/Arc `Weak` handles.

use super::collections::{
    CELL_MODEL_GET_READS_CURRENT_VALUE_VERUS_FRAGMENT,
    CELL_MODEL_NEW_STORES_INITIAL_VALUE_VERUS_FRAGMENT,
    CELL_MODEL_REPLACE_RETURNS_PREVIOUS_VALUE_VERUS_FRAGMENT,
    VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC,
};
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::{
    ArrayIntoIterAdvanceMatchesPosition, ArrayIntoIterStartsAtFirstPosition, RustStdStandard,
    WriteStoresNewValue, YieldsThreeValuesInOrderThenEnds,
};
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::cell::Cell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cell_model_get_set_replace_round_trip".to_owned(),
            VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::Cell<i32>>);

amenable_derive::verus_ensures_witness!(
    RustStdStandard<std::cell::Cell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
    "verify_cell_model_get_set_replace_round_trip"
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::Cell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        "ensures",
        || CELL_MODEL_NEW_STORES_INITIAL_VALUE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        "ensures",
        || CELL_MODEL_GET_READS_CURRENT_VALUE_VERUS_FRAGMENT,
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::Cell<i32>>",
        "verus",
        "ensures",
        || CELL_MODEL_REPLACE_RETURNS_PREVIOUS_VALUE_VERUS_FRAGMENT,
    )
}

/// [`WriteStoresNewValue`] reuses `Cell`'s own round-trip harness rather
/// than adding a new Verus proof: it names the shared write-through law
/// the harness already establishes.
impl VerusWitness for WriteStoresNewValue {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cell_model_get_set_replace_round_trip".to_owned(),
            VERIFY_CELL_MODEL_GET_SET_REPLACE_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(WriteStoresNewValue);

// `write_stores_new_value` is shared across `cell_carrier`,
// `ref_cell_carrier`, `unsafe_cell_carrier`, and
// `ordered_pair_iter_mut_carrier` -- no single harness to derive a
// clause-index selector from, so this derives from the predicate's own
// real declaration (`observed == new_value`, in its own parameter
// names) rather than any one caller's argument-substituted instance of
// it (previously `final(self).value == new_value`, `cell_carrier`'s own
// call-site spelling -- also real, just a different, less general
// representation of the same shared law).
amenable_derive::verus_ensures_predicate!(
    WriteStoresNewValue,
    "amenable_std::WriteStoresNewValue",
    "write_stores_new_value"
);

const VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/array_into_iter_carrier.rs");

/// [`ArrayIntoIterStartsAtFirstPosition`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the model's
/// initial-state law.
impl VerusWitness for ArrayIntoIterStartsAtFirstPosition {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ArrayIntoIterStartsAtFirstPosition);

amenable_derive::verus_ensures_predicate!(
    ArrayIntoIterStartsAtFirstPosition,
    "amenable_std::ArrayIntoIterStartsAtFirstPosition",
    "array_into_iter_model_starts_at_first_position"
);

/// [`ArrayIntoIterAdvanceMatchesPosition`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the model's
/// one-step transition law.
impl VerusWitness for ArrayIntoIterAdvanceMatchesPosition {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ArrayIntoIterAdvanceMatchesPosition);

amenable_derive::verus_ensures_predicate!(
    ArrayIntoIterAdvanceMatchesPosition,
    "amenable_std::ArrayIntoIterAdvanceMatchesPosition",
    "array_into_iter_advance_matches_position"
);

/// [`YieldsThreeValuesInOrderThenEnds`] reuses the array `IntoIter`
/// harness rather than adding a new Verus proof: it names the
/// fixed-length consuming-iterator law the carrier already establishes.
impl VerusWitness for YieldsThreeValuesInOrderThenEnds {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(YieldsThreeValuesInOrderThenEnds);

amenable_derive::verus_ensures_witness!(
    YieldsThreeValuesInOrderThenEnds,
    "amenable_std::YieldsThreeValuesInOrderThenEnds",
    "verify_array_into_iter_model_yields_elements_in_order"
);

impl VerusWitness for RustStdStandard<std::array::IntoIter<i32, 3>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_array_into_iter_model_yields_elements_in_order".to_owned(),
            VERIFY_ARRAY_INTO_ITER_MODEL_YIELDS_ELEMENTS_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::array::IntoIter<i32, 3>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::array::IntoIter<i32, 3>>",
        "verus",
        || {
            <RustStdStandard<std::array::IntoIter<i32, 3>> as VerusWitness>::proof().to_string()
        },
    )
}

pub(super) const VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/ref_cell_carrier.rs");

// `verify_ref_cell_model_dynamic_borrow_rules`'s real VerusCallShape is
// no longer registered by hand here -- `verus_call_shape` derives it by
// parsing the real signature directly from
// crates/amenable_verus/src/rust_std/ref_cell_carrier.rs. Its own
// `&mut self`/`old`/`final` methods (`try_borrow`, `release_shared`,
// etc.) are purely internal to this one harness's body -- never
// independently registered or composed; this top-level harness is a
// plain value-returning function like any other. Its own `ensures` mixes
// raw tuple-field projections (some negated) with one named-predicate
// citation whose own argument is itself a projection-and-cast
// (`result.5 as int`) -- the reason `VerusCallShape.ensures`/`.requires`
// are plain `$placeholder` text templates rather than a structured
// predicate-call-only representation (a first design tried the latter
// and it didn't fit this harness at all), and derivation walks tokens
// directly rather than `Expr`'s own AST shape for the same reason.

impl VerusWitness for RustStdStandard<std::cell::RefCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_cell_model_dynamic_borrow_rules".to_owned(),
            VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::RefCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::RefCell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::RefCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
    [
        "write_stores_new_value",
        "try_borrow_result_matches",
        "try_borrow_mut_result_matches",
        "release_shared_decrements_borrow_state",
    ]
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::cell::RefCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::RefCell<i32>>",
    [
        "try_borrow_headroom_holds",
        "release_shared_requires_a_live_shared_borrow"
    ]
);

const VERIFY_ONCE_CELL_MODEL_INITIALIZES_EXACTLY_ONCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/once_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::OnceCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_cell_model_initializes_exactly_once".to_owned(),
            VERIFY_ONCE_CELL_MODEL_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::OnceCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::OnceCell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::OnceCell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

// `get()` reads back exactly the stored `Option<i32>` -- neither
// `observed_value_matches_input`/`observed_pair_matches_input`'s more
// specific typed shape fits an `Option<i32>`-vs-`Option<i32>` read-back,
// so this uses the generic positive-equality predicate instead.
// `empty()`'s/`set()`'s own postconditions ride along in the same
// registration -- this type's Ensures slot is already claimed by the
// `values_are_equal` call above, so every real postcondition this file
// states has to share it.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::OnceCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::OnceCell<i32>>",
    [
        "values_are_equal",
        "once_cell_empty_has_no_value",
        "once_cell_set_succeeds_when_empty",
        "once_cell_set_rejected_when_occupied",
    ]
);

const VERIFY_UNSAFE_CELL_MODEL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/unsafe_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::UnsafeCell<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_unsafe_cell_model_get_mut_and_into_inner_round_trip".to_owned(),
            VERIFY_UNSAFE_CELL_MODEL_GET_MUT_AND_INTO_INNER_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::UnsafeCell<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::UnsafeCell<i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::UnsafeCell<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::UnsafeCell<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::UnsafeCell<i32>>",
    "write_stores_new_value"
);

const VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/cell_and_ref/lazy_cell_carrier.rs");

impl VerusWitness for RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lazy_cell_model_caches_its_initializer_result".to_owned(),
            VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>,
    "amenable_std::rust_std::RustStdStandard<std::cell::LazyCell<i32, fn() -> i32>>",
    [
        "lazy_cell_uninitialized_has_no_cached_value",
        "force_caches_on_first_call",
        "force_returns_cached_value_on_later_calls",
    ]
);

impl VerusWitness for RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_lazy_cell_model_caches_its_initializer_result".to_owned(),
            VERIFY_LAZY_CELL_MODEL_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::LazyLock<i32, fn() -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/sync/weak_carrier.rs");

impl VerusWitness for RustStdStandard<std::rc::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::rc::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::rc::Weak<i32>>",
        "verus",
        || {
            <RustStdStandard<std::rc::Weak<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::rc::Weak<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::rc::Weak<i32>>",
    [
        "weak_upgrade_result_matches",
        "drop_strong_decrements_strong_count"
    ]
);

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::rc::Weak<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::rc::Weak<i32>>",
    "drop_strong_requires_a_live_strong_reference"
);

impl VerusWitness for RustStdStandard<std::sync::Weak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_weak_model_upgrade_fails_once_the_strong_count_hits_zero".to_owned(),
            VERIFY_WEAK_MODEL_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::sync::Weak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Weak<i32>>",
        "verus",
        || {
            <RustStdStandard<std::sync::Weak<i32>> as VerusWitness>::proof().to_string()
        },
    )
}
