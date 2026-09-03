//! `VerusWitness` impls for the stateful iterator adapters:
//! `Cycle`/`Fuse`/`Inspect`/`Peekable`/`Scan`/`FlatMap`/`Flatten`. The
//! count-limited/stateless adapters are in `iter_adapters_b`.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

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
