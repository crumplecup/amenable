//! `KaniWitness` impls for `std::time`'s clock types.
//!
//! `Instant` is proved through an Amenable-owned monotonic-clock observation,
//! because the direct `Instant::now()` path reaches the gallery's unsupported
//! `clock_gettime` boundary. `SystemTime` and `SystemTimeError` are still
//! proved via arithmetic on `SystemTime::UNIX_EPOCH` rather than a real clock
//! read, so those claims stay fully deterministic and don't depend on when
//! the harness happens to run.

use std::time::{Instant, SystemTime, SystemTimeError};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::bridge_kani_witness;
use crate::{KaniInstantObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<Instant> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_instant_is_monotonically_nondecreasing".to_owned(),
            VERIFY_INSTANT_IS_MONOTONICALLY_NONDECREASING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Instant>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Instant>",
        "kani",
        || <RustStdStandard<Instant> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniInstantObservation` instance actually demonstrated
/// its later reading is not earlier, minted only by
/// [`KaniInstantObservation::demonstrate_monotonicity`].
pub struct KaniInstantWitnessToken(());

impl ProofToken for KaniInstantWitnessToken {
    type Proposition = KaniInstantObservation;
}

impl KaniInstantObservation {
    /// Assert the later monotonic clock reading is not earlier. Consumes
    /// `self`: the only way to obtain the token is to have run this check
    /// against a real observation instance, not to assert it
    /// independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_monotonicity(self) -> KaniInstantWitnessToken {
        assert!(
            self.later_is_not_earlier(),
            "a later monotonic clock reading should not be earlier"
        );
        KaniInstantWitnessToken(())
    }
}

/// Lawful token minted once `RustStdStandard<Instant>`'s monotonic-ordering
/// claim has been established from a `KaniInstantObservation` that has itself
/// demonstrated the later reading is not earlier.
pub struct RustStdInstantToken(());

impl ProofToken for RustStdInstantToken {
    type Proposition = RustStdStandard<Instant>;
}

impl Establish<KaniInstantWitnessToken, KaniVerifier> for RustStdStandard<Instant> {
    type Token = RustStdInstantToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniInstantWitnessToken) -> Self::Token {
        RustStdInstantToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_INSTANT_IS_MONOTONICALLY_NONDECREASING_SRC, {
        /// A later monotonic clock reading is never earlier than one taken
        /// before it.
        /// This proof uses the Amenable-owned monotonic-clock observation:
        /// the direct `Instant::now()` path reaches the gallery's
        /// unsupported `clock_gettime` boundary before the small ordering
        /// claim can be checked. The claim is established through
        /// `Establish<KaniInstantObservation, KaniVerifier> for
        /// RustStdStandard<Instant>` from the observation instance that
        /// actually demonstrated monotonicity, rather than asserted
        /// independently of it.
        #[kani::proof]
        fn verify_instant_is_monotonically_nondecreasing() {
            let observation = crate::KaniInstantObservation::monotonic();
            let demonstration = observation.demonstrate_monotonicity();

            let _token = RustStdStandard::<Instant>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<SystemTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_system_time_duration_since_computes_the_elapsed_span".to_owned(),
            VERIFY_SYSTEM_TIME_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SystemTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTime>",
        "kani",
        || <RustStdStandard<SystemTime> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SYSTEM_TIME_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC, {
        /// `.duration_since()` reports exactly the span between two
        /// `SystemTime`s built from `UNIX_EPOCH` arithmetic — fully
        /// deterministic, no real clock read involved.
        #[kani::proof]
        fn verify_system_time_duration_since_computes_the_elapsed_span() {
            use std::time::Duration;

            let later = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
            let elapsed = later.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            assert_eq!(elapsed, Duration::from_secs(100));
        }
    }
}

impl KaniWitness for RustStdStandard<SystemTimeError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_system_time_error_recovers_how_far_backward_it_went".to_owned(),
            VERIFY_SYSTEM_TIME_ERROR_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SystemTimeError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTimeError>",
        "kani",
        || <RustStdStandard<SystemTimeError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_SYSTEM_TIME_ERROR_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC, {
        /// `.duration_since()` fails when the argument is later than
        /// `self`, and the resulting error's `.duration()` reports
        /// exactly how far backward that gap is.
        #[kani::proof]
        fn verify_system_time_error_recovers_how_far_backward_it_went() {
            use std::time::Duration;

            let earlier = SystemTime::UNIX_EPOCH;
            let later = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
            match earlier.duration_since(later) {
                Err(err) => assert_eq!(err.duration(), Duration::from_secs(100)),
                Ok(_) => panic!("expected duration_since to fail going backward"),
            }
        }
    }
}
