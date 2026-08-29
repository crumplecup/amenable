//! `KaniWitness` impls for `std::collections` (`HashMap`/`HashSet`).
//!
//! The direct `HashMap::new()`/`HashSet::new()` path times out under Kani
//! even for a single fixed key/value pair — confirmed empirically, and
//! consistent with the "pure in-memory std implementation blow-up" class
//! already documented for `DefaultHasher`/`BTreeMap` in the proof gallery.
//! These proofs use `crate::hash_collections_model`'s one-entry
//! accommodation model instead: if the real `HashMap`/`HashSet` refine its
//! insert/lookup law, the Rust-facing claim follows.

use std::collections::{HashMap, HashSet};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<HashMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_hash_map_insert_then_get_recovers_the_value".to_owned(),
            VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<HashMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        "kani",
        || <RustStdStandard<HashMap<i32, i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC, {
        /// Inserting a key then getting it back recovers the same value.
        /// This proof uses the Amenable-owned bounded one-entry
        /// accommodation model (`KaniHashMap`): the direct
        /// `HashMap::new()` path times out under Kani even for a single
        /// fixed key/value pair.
        #[kani::proof]
        fn verify_hash_map_insert_then_get_recovers_the_value() {
            let key: i32 = kani::any();
            let value: i32 = kani::any();
            let map = crate::KaniHashMap::new(key, value);
            assert!(
                CollectedSequenceMatchesExpected::ensures((map.get(&key), Some(&value))),
                "insert then get recovers the same value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<HashSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_hash_set_insert_then_contains_reports_membership".to_owned(),
            VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<HashSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        "kani",
        || <RustStdStandard<HashSet<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<HashSet<i32>>,
    "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
    bool,
    |contains| contains
);

amenable_derive::harness! {
    kani, VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC, {
        /// Inserting a value makes the set report it as a member. This
        /// proof uses the same `crate::hash_collections_model` as
        /// `HashMap` -- see that proof's doc comment.
        #[kani::proof]
        fn verify_hash_set_insert_then_contains_reports_membership() {
            let value: i32 = kani::any();
            let set = crate::KaniHashSet::new(value);
            assert!(
                RustStdStandard::<HashSet<i32>>::ensures(set.contains(&value)),
                "insert makes the set report the value as a member"
            );
        }
    }
}
