//! Gallery marker types for the primitive-shapes carrier, and the
//! `Rev`/`Skip`/`StepBy`/`Take`/`Once`/`Repeat*`/`Empty`/`Cycle`/`Fuse`/`Inspect`/`Peekable`
//! iterator adapters.

use super::cell::VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC;
use super::iter_adapters_a::VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::panic_ops_time_future::VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC;
use super::sync_atomic::VERIFY_ATOMIC_BOOL_MODEL_LOAD_STORE_SRC;
use crate::{
    IncrementHeadroom, ObservedOptionMatchesInput, ObservedPairMatchesInput,
    ObservedValueMatchesInput, RustStdStandard, ValueUnchanged,
};
use amenable_core::Evidence;

/// [`IncrementHeadroom`] reuses the same harness rather than adding a new
/// Verus proof — it names the precondition the harness already requires,
/// it doesn't prove anything new. Three supplementary fragments cover the
/// wider margin `slice_chunks_carrier`'s own models need (`a`/`b`/`c <=
/// i32::MAX - 10`, not registered through the `Requires` trait itself
/// since only one fragment can be the "canonical" one per type).
impl VerusWitness for IncrementHeadroom {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_enumerate_model_pairs_each_item_with_its_index".to_owned(),
            VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(IncrementHeadroom);

// Four sites need the tight, two-increment margin
// (verify_enumerate_model_pairs_each_item_with_its_index, verify_rev_
// model_reverses_iteration_order, verify_cycle_model_repeats_its_
// sequence_forever, verify_peekable_model_peek_does_not_consume) and call
// increment_headroom_holds directly; eight more need only the loosest,
// one-increment margin (verify_chain_model_sequences_two_iterators_end_
// to_end, verify_zip_model_pairs_items_from_two_iterators, verify_fuse_
// model_keeps_returning_none_once_exhausted, verify_inspect_model_calls_
// once_per_item_without_changing_values, verify_fn_pointer_model_calls_
// the_underlying_function, verify_map_model_applies_its_closure_to_each_
// item) and call single_increment_headroom_holds. The slice-chunk
// write-through models need a wider margin still and call
// ten_increment_headroom_holds. All three are real, shared
// `open spec fn`s in amenable_verus::rust_std::iter_sequence_carrier
// confirmed under real verus to give every call site genuine proof
// credit across carrier files -- see amenable_std::verus_gallery's
// cross_file_spec_fn_reuse_gets_real_proof_credit case.
amenable_derive::verus_requires_predicate!(
    IncrementHeadroom,
    "amenable_std::IncrementHeadroom",
    [
        "increment_headroom_holds",
        "single_increment_headroom_holds",
        "ten_increment_headroom_holds",
        "two_increment_headroom_holds",
        "four_increment_headroom_holds"
    ]
);

/// [`ValueUnchanged`] reuses `RefCell`'s own borrow-rules harness rather
/// than adding a new Verus proof — the harness's own `ensures` clauses
/// already establish this frame condition for `try_borrow`/
/// `try_borrow_mut`/`release_shared` (and `Weak::drop_strong` states the
/// identical claim) through one shared Verus `spec fn`, `value_unchanged`.
impl VerusWitness for ValueUnchanged {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_ref_cell_model_dynamic_borrow_rules".to_owned(),
            VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ValueUnchanged);

amenable_derive::verus_ensures_predicate!(
    ValueUnchanged,
    "amenable_std::ValueUnchanged",
    "value_unchanged"
);

/// [`ObservedValueMatchesInput`] reuses the shared-reference harness
/// rather than adding a new Verus proof — it names the direct identity
/// postcondition that many simple scalar-observation carriers now state
/// through one shared Verus `spec fn`.
impl VerusWitness for ObservedValueMatchesInput {
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

bridge_verus_witness!(ObservedValueMatchesInput);

// Registered under both "ensures" and "requires": `ref_cell_carrier.rs`'s
// `release_exclusive` states the identical direct-identity claim as a
// precondition too (`old(self).borrow_state == -1`), reusing the same
// real spec fn rather than adding a requires-only twin.
amenable_derive::verus_ensures_predicate!(
    ObservedValueMatchesInput,
    "amenable_std::ObservedValueMatchesInput",
    "observed_value_matches_input"
);

amenable_derive::verus_requires_predicate!(
    ObservedValueMatchesInput,
    "amenable_std::ObservedValueMatchesInput",
    "observed_value_matches_input"
);

/// [`ObservedOptionMatchesInput`] reuses the `Once` harness rather than
/// adding a new Verus proof — it names the direct `Option`-wrapped
/// identity postcondition that several `core::iter` generator carriers
/// now state through one shared Verus `spec fn`, the `Option`-wrapped
/// counterpart to [`ObservedValueMatchesInput`]'s bare-scalar version.
impl VerusWitness for ObservedOptionMatchesInput {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_yields_exactly_one_value".to_owned(),
            VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ObservedOptionMatchesInput);

amenable_derive::verus_ensures_predicate!(
    ObservedOptionMatchesInput,
    "amenable_std::ObservedOptionMatchesInput",
    "observed_option_matches_input"
);

/// [`ObservedPairMatchesInput`] reuses the `AtomicBool` load-store
/// harness rather than adding a new Verus proof — it names the direct
/// pair-identity postcondition that several accommodation models now
/// state through one shared, generic Verus `spec fn`.
impl VerusWitness for ObservedPairMatchesInput {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_atomic_bool_model_load_store".to_owned(),
            VERIFY_ATOMIC_BOOL_MODEL_LOAD_STORE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(ObservedPairMatchesInput);

amenable_derive::verus_ensures_predicate!(
    ObservedPairMatchesInput,
    "amenable_std::ObservedPairMatchesInput",
    "observed_pair_matches_input"
);

const VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_rev_model_reverses_iteration_order".to_owned(),
            VERIFY_REV_MODEL_REVERSES_ITERATION_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Rev<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Rev<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_skip_model_discards_the_first_n_items".to_owned(),
            VERIFY_SKIP_MODEL_DISCARDS_THE_FIRST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Skip<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Skip<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_skip_while_model_discards_items_while_the_predicate_holds".to_owned(),
            VERIFY_SKIP_WHILE_MODEL_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::SkipWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_step_by_model_yields_every_nth_item".to_owned(),
            VERIFY_STEP_BY_MODEL_YIELDS_EVERY_NTH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::StepBy<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Take<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_model_yields_at_most_n_items".to_owned(),
            VERIFY_TAKE_MODEL_YIELDS_AT_MOST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Take<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Take<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Take<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_window_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_take_while_model_yields_items_while_the_predicate_holds".to_owned(),
            VERIFY_TAKE_WHILE_MODEL_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::TakeWhile<std::ops::Range<i32>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Once<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_model_yields_exactly_one_value".to_owned(),
            VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Once<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Once<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::OnceWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_once_with_model_calls_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_WITH_MODEL_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::OnceWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::OnceWith<fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::OnceWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Repeat<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_model_yields_the_same_value_forever".to_owned(),
            VERIFY_REPEAT_MODEL_YIELDS_THE_SAME_VALUE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Repeat<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Repeat<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_with_model_calls_its_closure_once_per_item".to_owned(),
            VERIFY_REPEAT_WITH_MODEL_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::RepeatWith<fn() -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::RepeatWith<fn() -> i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::RepeatN<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_repeat_n_model_yields_the_value_exactly_n_times".to_owned(),
            VERIFY_REPEAT_N_MODEL_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::RepeatN<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::RepeatN<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::RepeatN<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_generator_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Empty<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_empty_model_yields_nothing".to_owned(),
            VERIFY_EMPTY_MODEL_YIELDS_NOTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Empty<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Empty<i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Empty<i32>> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_CYCLE_MODEL_REPEATS_ITS_SEQUENCE_FOREVER_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cycle_model_repeats_its_sequence_forever".to_owned(),
            VERIFY_CYCLE_MODEL_REPEATS_ITS_SEQUENCE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Cycle<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FUSE_MODEL_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_fuse_model_keeps_returning_none_once_exhausted".to_owned(),
            VERIFY_FUSE_MODEL_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Fuse<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_INSPECT_MODEL_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_inspect_model_calls_once_per_item_without_changing_values".to_owned(),
            VERIFY_INSPECT_MODEL_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Inspect<std::ops::Range<i32>, fn(&i32)>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_PEEKABLE_MODEL_PEEK_DOES_NOT_CONSUME_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_peekable_model_peek_does_not_consume".to_owned(),
            VERIFY_PEEKABLE_MODEL_PEEK_DOES_NOT_CONSUME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Peekable<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_SCAN_MODEL_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<
        std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_scan_model_threads_state_through_its_closure".to_owned(),
            VERIFY_SCAN_MODEL_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Scan<std::ops::Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
    "is_within_scan_sum_headroom"
);

const VERIFY_FLAT_MAP_MODEL_FLATTENS_EACH_GENERATED_ITERATOR_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<
        std::iter::FlatMap<
            std::array::IntoIter<i32, 1>,
            std::ops::Range<i32>,
            fn(i32) -> std::ops::Range<i32>,
        >,
    >
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_flat_map_model_flattens_each_generated_iterator".to_owned(),
            VERIFY_FLAT_MAP_MODEL_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<
        std::iter::FlatMap<
            std::array::IntoIter<i32, 1>,
            std::ops::Range<i32>,
            fn(i32) -> std::ops::Range<i32>,
        >,
    >
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FlatMap<std::array::IntoIter<i32, 1>, std::ops::Range<i32>, fn(i32) -> std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::FlatMap<std::array::IntoIter<i32, 1>, std::ops::Range<i32>, fn(i32) -> std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FLATTEN_MODEL_CONCATENATES_THE_INNER_ITERATORS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_flatten_model_concatenates_the_inner_iterators".to_owned(),
            VERIFY_FLATTEN_MODEL_CONCATENATES_THE_INNER_ITERATORS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i32>>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

pub(super) const VERIFY_SUCCESSORS_MODEL_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_stateful_carrier.rs");
