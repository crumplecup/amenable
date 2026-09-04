use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniVerifier, KaniWaitTimeoutObservation, KaniWitness};

impl KaniWitness for RustStdStandard<std::sync::Condvar> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_condvar_wait_timeout_reports_timing_out".to_owned(),
            VERIFY_CONDVAR_WAIT_TIMEOUT_REPORTS_TIMING_OUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Condvar>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Condvar>",
        "kani",
        || <RustStdStandard<std::sync::Condvar> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniWaitTimeoutObservation` instance actually
/// demonstrated a never-notified wait timing out, minted only by
/// [`KaniWaitTimeoutObservation::demonstrate_timeout`] — shared by every
/// `Establish` impl claiming this exact timeout shape (`Condvar` and
/// `WaitTimeoutResult` both reduce to the identical `did_time_out()`
/// check).
pub struct KaniWaitTimeoutWitnessToken(());

impl ProofToken for KaniWaitTimeoutWitnessToken {
    type Proposition = KaniWaitTimeoutObservation;
}

impl KaniWaitTimeoutObservation {
    /// Assert a never-notified wait times out. Consumes `self` for the
    /// same reason
    /// [`crate::KaniMutexExclusionObservation::demonstrate_exclusion`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_timeout(self) -> KaniWaitTimeoutWitnessToken {
        assert!(self.did_time_out(), "a never-notified wait times out");
        KaniWaitTimeoutWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<std::sync::Condvar>`'s
/// timed-wait claim has been established from a `KaniWaitTimeoutObservation`.
pub struct RustStdCondvarToken(());

impl ProofToken for RustStdCondvarToken {
    type Proposition = RustStdStandard<std::sync::Condvar>;
}

impl Establish<KaniWaitTimeoutWitnessToken, KaniVerifier> for RustStdStandard<std::sync::Condvar> {
    type Token = RustStdCondvarToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniWaitTimeoutWitnessToken) -> Self::Token {
        RustStdCondvarToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_CONDVAR_WAIT_TIMEOUT_REPORTS_TIMING_OUT_SRC, {
        /// `.wait_timeout()` on a `Condvar` nobody ever notifies times out
        /// and reports that through its `WaitTimeoutResult`.
        /// This proof uses the Amenable-owned timeout observation because the
        /// direct `Condvar::wait_timeout()` path reaches an unsupported
        /// `clock_gettime` boundary under Kani. The claim is established
        /// through `Establish<KaniWaitTimeoutObservation, KaniVerifier> for
        /// RustStdStandard<std::sync::Condvar>` from the observation that
        /// demonstrated the timeout result.
        #[kani::proof]
        fn verify_condvar_wait_timeout_reports_timing_out() {
            let observation = KaniWaitTimeoutObservation::timed_out();
            let demonstration = observation.demonstrate_timeout();

            let _token = RustStdStandard::<std::sync::Condvar>::establish(demonstration);
        }
    }
}
