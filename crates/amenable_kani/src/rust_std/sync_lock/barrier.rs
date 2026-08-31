use std::sync::{Barrier, BarrierWaitResult};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
#[cfg(kani)]
use crate::FallibleOperationReportsSuccess;
#[cfg(kani)]
use crate::GetterRecoversTheStoredReference;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::rust_std::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniBarrierLeaderObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<Barrier> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_barrier_of_one_is_its_own_leader".to_owned(),
            VERIFY_BARRIER_OF_ONE_IS_ITS_OWN_LEADER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Barrier>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Barrier>",
        "kani",
        || <RustStdStandard<Barrier> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniBarrierLeaderObservation` instance actually
/// demonstrated the sole participant's leadership, minted only by
/// [`KaniBarrierLeaderObservation::demonstrate_leadership`] — shared by
/// every `Establish` impl claiming this exact one-party leader shape
/// (`Barrier` and `BarrierWaitResult` both reduce to the identical
/// `is_leader()` check).
pub struct KaniBarrierLeaderWitnessToken(());

impl ProofToken for KaniBarrierLeaderWitnessToken {
    type Proposition = KaniBarrierLeaderObservation;
}

impl KaniBarrierLeaderObservation {
    /// Assert the sole participant is the leader. Consumes `self` for
    /// the same reason
    /// [`KaniMutexExclusionObservation::demonstrate_exclusion`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_leadership(self) -> KaniBarrierLeaderWitnessToken {
        assert!(self.is_leader(), "the sole participant is the leader");
        KaniBarrierLeaderWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Barrier>`'s one-party leader
/// claim has been established from a `KaniBarrierLeaderObservation`.
pub struct RustStdBarrierToken(());

impl ProofToken for RustStdBarrierToken {
    type Proposition = RustStdStandard<Barrier>;
}

impl Establish<KaniBarrierLeaderWitnessToken, KaniVerifier> for RustStdStandard<Barrier> {
    type Token = RustStdBarrierToken;

    fn establish(_credential: KaniBarrierLeaderWitnessToken) -> Self::Token {
        RustStdBarrierToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_BARRIER_OF_ONE_IS_ITS_OWN_LEADER_SRC, {
        /// A `Barrier` built for exactly one participant returns immediately
        /// from `.wait()`, and that lone participant is always the leader.
        /// This proof uses the Amenable-owned one-party barrier model because
        /// the direct `Barrier::wait()` path reaches an unsupported futex
        /// boundary under Kani. The claim is established through
        /// `Establish<KaniBarrierLeaderObservation, KaniVerifier> for
        /// RustStdStandard<Barrier>` from the observation that demonstrated
        /// the sole participant's leadership.
        #[kani::proof]
        fn verify_barrier_of_one_is_its_own_leader() {
            let observation = KaniBarrierLeaderObservation::sole_participant();
            let demonstration = observation.demonstrate_leadership();

            let _token = RustStdStandard::<Barrier>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<BarrierWaitResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_barrier_wait_result_reports_the_sole_participant_as_leader".to_owned(),
            VERIFY_BARRIER_WAIT_RESULT_REPORTS_THE_SOLE_PARTICIPANT_AS_LEADER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<BarrierWaitResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BarrierWaitResult>",
        "kani",
        || <RustStdStandard<BarrierWaitResult> as KaniWitness>::proof().to_string(),
    )
}

/// Lawful token minted once `RustStdStandard<BarrierWaitResult>`'s leader
/// accessor claim has been established from a `KaniBarrierLeaderObservation`.
pub struct RustStdBarrierWaitResultToken(());

impl ProofToken for RustStdBarrierWaitResultToken {
    type Proposition = RustStdStandard<BarrierWaitResult>;
}

impl Establish<KaniBarrierLeaderWitnessToken, KaniVerifier> for RustStdStandard<BarrierWaitResult> {
    type Token = RustStdBarrierWaitResultToken;

    fn establish(_credential: KaniBarrierLeaderWitnessToken) -> Self::Token {
        RustStdBarrierWaitResultToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_BARRIER_WAIT_RESULT_REPORTS_THE_SOLE_PARTICIPANT_AS_LEADER_SRC, {
        /// Same claim as `Barrier` itself, checked directly on the
        /// `.is_leader()` accessor this carrier exists to expose.
        /// This proof uses the same Amenable-owned one-party barrier
        /// observation because the direct wait path reaches an unsupported
        /// futex boundary under Kani. The claim is established through
        /// `Establish<KaniBarrierLeaderObservation, KaniVerifier> for
        /// RustStdStandard<BarrierWaitResult>` from the observation that
        /// demonstrated the leader result.
        #[kani::proof]
        fn verify_barrier_wait_result_reports_the_sole_participant_as_leader() {
            let observation = KaniBarrierLeaderObservation::sole_participant();
            let demonstration = observation.demonstrate_leadership();

            let _token = RustStdStandard::<BarrierWaitResult>::establish(demonstration);
        }
    }
}
