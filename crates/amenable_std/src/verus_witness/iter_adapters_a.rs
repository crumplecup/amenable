//! The `Map`/`Cloned`/`Copied`/`Zip`/`Enumerate` iterator adapters.

use super::ascii_and_drain::VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC;
use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

impl VerusWitness for RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_map_model_applies_its_closure_to_each_item".to_owned(),
            VERIFY_MAP_MODEL_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Map<std::ops::Range<i32>, fn(i32) -> i32>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_FILTER_MODEL_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_filter_model_yields_only_items_matching_the_predicate".to_owned(),
            VERIFY_FILTER_MODEL_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

// `Filter`'s predicate and `FilterMap`'s closure land on the identical
// law -- named once, called from both.
amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
    "nonzero_item_survives_filtering"
);

const VERIFY_FILTER_MAP_MODEL_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_filter_map_model_applies_and_filters_in_one_step".to_owned(),
            VERIFY_FILTER_MAP_MODEL_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_MAP_WHILE_MODEL_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_transform_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_map_while_model_maps_items_while_the_closure_returns_some".to_owned(),
            VERIFY_MAP_WHILE_MODEL_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>",
    "is_within_map_while_doubling_headroom"
);

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::MapWhile<std::ops::Range<i32>, fn(i32) -> Option<i32>>>",
    "map_while_closure_result_matches"
);

const VERIFY_CLONED_MODEL_CLONES_EACH_REFERENCED_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_cloned_model_clones_each_referenced_item".to_owned(),
            VERIFY_CLONED_MODEL_CLONES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Cloned<std::slice::Iter<'static, i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_COPIED_MODEL_COPIES_EACH_REFERENCED_ITEM_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_transform_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_copied_model_copies_each_referenced_item".to_owned(),
            VERIFY_COPIED_MODEL_COPIES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Copied<std::slice::Iter<'static, i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_CHAIN_MODEL_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_sequence_carrier.rs");

impl VerusWitness
    for RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_chain_model_sequences_two_iterators_end_to_end".to_owned(),
            VERIFY_CHAIN_MODEL_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(
    RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Chain<std::ops::Range<i32>, std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

const VERIFY_ZIP_MODEL_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_zip_model_pairs_items_from_two_iterators".to_owned(),
            VERIFY_ZIP_MODEL_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Zip<std::ops::Range<i32>, std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}

// Reused by IncrementHeadroom in the next file, iter_adapters_b.rs -- see
// that impl's own doc comment.
pub(super) const VERIFY_ENUMERATE_MODEL_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/iter/iter_sequence_carrier.rs");

impl VerusWitness for RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> {
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

bridge_verus_witness!(RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>>",
        "verus",
        || {
            <RustStdStandard<std::iter::Enumerate<std::ops::Range<i32>>> as VerusWitness>::proof()
                .to_string()
        },
    )
}
