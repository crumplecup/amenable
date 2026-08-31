use super::CheckedProof;

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

use crate::{
    CreusotVerifier, CreusotWitness, HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC,
    HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC,
    VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC,
    VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC,
    VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC,
    VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC,
};
use amenable_core::{Ensures, Evidence, Provenance, Witness};

use amenable_std::{RustStdProvenance, RustStdStandard};

#[expect(
    deprecated,
    reason = "SipHasher itself is stable, only deprecated as a recommendation to use DefaultHasher instead; covering it is a coverage-completeness question, not a call to use it"
)]
type SipHasherAlias = std::hash::SipHasher;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<SipHasherAlias> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<SipHasherAlias>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SipHasher>",
        "creusot",
        || <RustStdStandard<SipHasherAlias> as CreusotWitness>::proof().report().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_default_hasher_is_deterministic_across_fresh_instances".to_string(),
            VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<DefaultHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
        "creusot",
        || <RustStdStandard<DefaultHasher> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<RandomState> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_random_state_gives_the_same_hasher_seed_across_calls".to_string(),
            VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<RandomState>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RandomState>",
        "creusot",
        || <RustStdStandard<RandomState> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<HashMap<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_hash_map_insert_then_get_recovers_the_value".to_string(),
            VERIFY_HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<HashMap<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        "creusot",
        || <RustStdStandard<HashMap<i32, i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn hash_map_insert_then_get_recovers_the_value` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<HashMap<i32, i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        HASH_MAP_INSERT_THEN_GET_RECOVERS_THE_VALUE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashMap<i32, i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<HashMap<i32, i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<HashSet<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_hash_set_insert_then_contains_reports_membership".to_string(),
            VERIFY_HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<HashSet<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        "creusot",
        || <RustStdStandard<HashSet<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn hash_set_insert_then_contains_reports_membership`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<HashSet<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        HASH_SET_INSERT_THEN_CONTAINS_REPORTS_MEMBERSHIP_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<HashSet<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<HashSet<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
