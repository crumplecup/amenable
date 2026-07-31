//! `KaniWitness` impls for `std::hash`'s concrete hasher types.

use std::hash::{DefaultHasher, RandomState};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniRandomStateObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<DefaultHasher> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_default_hasher_is_deterministic_across_fresh_instances".to_owned(),
            claim: VERIFY_DEFAULT_HASHER_IS_DETERMINISTIC_ACROSS_FRESH_INSTANCES_SRC.to_owned(),
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
            harness: "verify_random_state_gives_the_same_hasher_seed_across_calls".to_owned(),
            claim: VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC.to_owned(),
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

/// Witness that a `KaniRandomStateObservation` instance actually
/// demonstrated matching same-input digests, minted only by
/// [`KaniRandomStateObservation::demonstrate_same_input_agreement`].
pub struct KaniRandomStateWitnessToken(());

impl ProofToken for KaniRandomStateWitnessToken {
    type Proposition = KaniRandomStateObservation;
}

impl KaniRandomStateObservation {
    /// Assert the observed input matches and that two hashers built from
    /// the same `RandomState` instance agree on it. Consumes `self`: the
    /// only way to obtain the token is to have run this check against a
    /// real observation instance, not to assert it independently.
    #[must_use]
    pub fn demonstrate_same_input_agreement(
        self,
        expected_input: &'static str,
    ) -> KaniRandomStateWitnessToken {
        assert_eq!(self.input(), expected_input);
        assert!(self.same_input_hashes_agree());
        KaniRandomStateWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<RandomState>`'s
/// same-input-same-state determinism claim has been established from a
/// `KaniRandomStateObservation` that has itself demonstrated matching digests.
pub struct RustStdRandomStateToken(());

impl ProofToken for RustStdRandomStateToken {
    type Proposition = RustStdStandard<RandomState>;
}

impl Establish<KaniRandomStateWitnessToken, KaniVerifier> for RustStdStandard<RandomState> {
    type Token = RustStdRandomStateToken;

    fn establish(_credential: KaniRandomStateWitnessToken) -> Self::Token {
        RustStdRandomStateToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_RANDOM_STATE_GIVES_THE_SAME_HASHER_SEED_ACROSS_CALLS_SRC, {
        /// A single `RandomState` instance picks its random seed once,
        /// at construction — so two hashers built from the *same*
        /// instance agree on the same input, even though they'd
        /// (almost certainly) disagree with a hasher from a different
        /// `RandomState::new()`.
        /// This proof uses the Amenable-owned `RandomState` observation:
        /// the direct constructor path reaches the gallery's unsupported
        /// OS entropy-source boundary before the law itself can be checked.
        /// The claim is established through
        /// `Establish<KaniRandomStateObservation, KaniVerifier> for
        /// RustStdStandard<RandomState>` from the observation instance
        /// that actually demonstrated matching same-input digests, rather
        /// than asserted independently of it.
        #[kani::proof]
        fn verify_random_state_gives_the_same_hasher_seed_across_calls() {
            let observation = KaniRandomStateObservation::same_input("some value", 7);
            let demonstration = observation.demonstrate_same_input_agreement("some value");

            let _token = RustStdStandard::<RandomState>::establish(demonstration);
        }
    }
}
