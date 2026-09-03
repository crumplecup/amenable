//! `VerusWitness` impls for the shared gallery marker types the
//! primitive-shapes and iter-adapter carriers reuse: `IncrementHeadroom`,
//! `ValueUnchanged`, `ObservedValueMatchesInput`,
//! `ObservedOptionMatchesInput`, and `ObservedPairMatchesInput`. Each
//! reuses an existing harness's verbatim source as its own `claim` rather
//! than adding a new Verus proof.

use super::cell::VERIFY_REF_CELL_MODEL_DYNAMIC_BORROW_RULES_SRC;
use super::iter_adapters_a::VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC;
use super::iter_adapters_b::VERIFY_ONCE_MODEL_YIELDS_EXACTLY_ONE_VALUE_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use super::panic_ops_time_future::VERIFY_SHARED_REFERENCE_MODEL_DEREFERENCES_TO_THE_REFERENT_SRC;
use super::sync_atomic::VERIFY_ATOMIC_BOOL_MODEL_LOAD_STORE_SRC;
use crate::{
    IncrementHeadroom, ObservedOptionMatchesInput, ObservedPairMatchesInput,
    ObservedValueMatchesInput, ValueUnchanged,
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
