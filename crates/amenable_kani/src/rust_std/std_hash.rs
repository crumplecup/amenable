//! `KaniWitness` impls for `std::hash`'s concrete hasher types.

use std::hash::{DefaultHasher, RandomState};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_default_hasher_is_deterministic_across_fresh_instances",
            claim: VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<DefaultHasher>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<DefaultHasher>",
        verifier: "kani",
        describe: || <RustStdStandard<DefaultHasher> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC, {
        /// `DefaultHasher::new()` always starts from the same fixed
        /// seed, so hashing the same value with two independent fresh
        /// instances gives the same result.
        #[kani::proof]
        fn verify_default_hasher_is_deterministic_across_fresh_instances() {
            use std::hash::{Hash, Hasher};

            let mut first = DefaultHasher::new();
            "some value".hash(&mut first);

            let mut second = DefaultHasher::new();
            "some value".hash(&mut second);

            assert_eq!(first.finish(), second.finish());
        }
    }
}

impl KaniWitness for RustStdStandard<RandomState> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_random_state_gives_the_same_hasher_seed_across_calls",
            claim: VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RandomState>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RandomState>",
        verifier: "kani",
        describe: || <RustStdStandard<RandomState> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC, {
        /// A single `RandomState` instance picks its random seed once,
        /// at construction — so two hashers built from the *same*
        /// instance agree on the same input, even though they'd
        /// (almost certainly) disagree with a hasher from a different
        /// `RandomState::new()`.
        #[kani::proof]
        fn verify_random_state_gives_the_same_hasher_seed_across_calls() {
            use std::hash::{BuildHasher, Hash, Hasher};

            let state = RandomState::new();

            let mut first = state.build_hasher();
            "some value".hash(&mut first);

            let mut second = state.build_hasher();
            "some value".hash(&mut second);

            assert_eq!(first.finish(), second.finish());
        }
    }
}
