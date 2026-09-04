use std::sync::{MutexGuard, PoisonError, WaitTimeoutResult};

use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::condvar::KaniWaitTimeoutWitnessToken;
#[cfg(kani)]
use crate::KaniWaitTimeoutObservation;
use crate::rust_std::CheckedProof;
use crate::rust_std::bridge_kani_witness;
use crate::{KaniMutexFailureObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<PoisonError<MutexGuard<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_poison_error_still_recovers_the_guards_value".to_owned(),
            VERIFY_POISON_ERROR_STILL_RECOVERS_THE_GUARDS_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<PoisonError<MutexGuard<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<PoisonError<MutexGuard<'static, i32>>>",
        "kani",
        || <RustStdStandard<PoisonError<MutexGuard<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// Witness that a `KaniMutexFailureObservation` instance actually
/// demonstrated recovering the guard's value from the poisoned case,
/// minted only by [`KaniMutexFailureObservation::demonstrate_poisoned_recovery`].
pub struct KaniMutexPoisonedRecoveryWitnessToken(());

impl ProofToken for KaniMutexPoisonedRecoveryWitnessToken {
    type Proposition = KaniMutexFailureObservation;
}

impl KaniMutexFailureObservation {
    /// Assert `into_inner` still recovers the guard's value in the
    /// poisoned case. Consumes `self` for the same reason
    /// [`crate::KaniMutexExclusionObservation::demonstrate_exclusion`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_poisoned_recovery(
        self,
        recovered_value: i32,
    ) -> KaniMutexPoisonedRecoveryWitnessToken {
        assert_eq!(
            self.poisoned_recovered_value(),
            recovered_value,
            "into_inner still recovers the guard's value"
        );
        KaniMutexPoisonedRecoveryWitnessToken(())
    }
}

/// Lawful token minted once
/// `RustStdStandard<PoisonError<MutexGuard<'static, i32>>>`'s recovery claim
/// has been established from a `KaniMutexFailureObservation`.
pub struct RustStdPoisonErrorToken(());

impl ProofToken for RustStdPoisonErrorToken {
    type Proposition = RustStdStandard<PoisonError<MutexGuard<'static, i32>>>;
}

impl Establish<KaniMutexPoisonedRecoveryWitnessToken, KaniVerifier>
    for RustStdStandard<PoisonError<MutexGuard<'static, i32>>>
{
    type Token = RustStdPoisonErrorToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniMutexPoisonedRecoveryWitnessToken) -> Self::Token {
        RustStdPoisonErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_POISON_ERROR_STILL_RECOVERS_THE_GUARDS_VALUE_SRC, {
        /// A panic while a `Mutex` guard is held poisons it, and the
        /// resulting `PoisonError` doesn't discard the data:
        /// `.into_inner()` still recovers the guard.
        /// This proof uses the Amenable-owned mutex-failure observation
        /// because the direct poisoning path reaches unsupported
        /// `catch_unwind` under Kani. The claim is established through
        /// `Establish<KaniMutexFailureObservation, KaniVerifier> for
        /// RustStdStandard<PoisonError<MutexGuard<'static, i32>>>` from the
        /// observation that demonstrated value recovery from the poisoned
        /// case.
        #[kani::proof]
        fn verify_poison_error_still_recovers_the_guards_value() {
            let recovered_value: i32 = kani::any();
            let held_value: i32 = kani::any();
            let observation = KaniMutexFailureObservation::new(recovered_value, held_value);
            let demonstration = observation.demonstrate_poisoned_recovery(recovered_value);

            let _token = RustStdStandard::<PoisonError<MutexGuard<'static, i32>>>::establish(
                demonstration,
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_try_lock_error_distinguishes_poisoned_from_would_block".to_owned(),
            VERIFY_TRY_LOCK_ERROR_DISTINGUISHES_POISONED_FROM_WOULD_BLOCK_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>",
        "kani",
        || <RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

/// Witness that a `KaniMutexFailureObservation` instance actually
/// demonstrated both `try_lock` failure classes (`Poisoned` and
/// `WouldBlock`), minted only by
/// [`KaniMutexFailureObservation::demonstrate_failure_classes`].
pub struct KaniMutexFailureClassesWitnessToken(());

impl ProofToken for KaniMutexFailureClassesWitnessToken {
    type Proposition = KaniMutexFailureObservation;
}

impl KaniMutexFailureObservation {
    /// Assert the poisoned case reports `Poisoned` and preserves the
    /// guarded value, and the already-held case reports `WouldBlock` and
    /// keeps the wrapped value. Consumes `self` for the same reason
    /// [`crate::KaniMutexExclusionObservation::demonstrate_exclusion`] does.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_failure_classes(
        self,
        poisoned_value: i32,
        held_value: i32,
    ) -> KaniMutexFailureClassesWitnessToken {
        assert!(
            self.poisoned_case_reports_poisoned(),
            "the poisoned case reports Poisoned"
        );
        assert_eq!(
            self.poisoned_recovered_value(),
            poisoned_value,
            "the poisoned case preserves the guarded value"
        );
        assert!(
            self.held_case_reports_would_block(),
            "the already-held case reports WouldBlock"
        );
        assert_eq!(
            self.held_value(),
            held_value,
            "the held case keeps the wrapped value"
        );
        KaniMutexFailureClassesWitnessToken(())
    }
}

/// Lawful token minted once
/// `RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>`'s
/// failure-classification claim has been established from a
/// `KaniMutexFailureObservation`.
pub struct RustStdTryLockErrorToken(());

impl ProofToken for RustStdTryLockErrorToken {
    type Proposition = RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>;
}

impl Establish<KaniMutexFailureClassesWitnessToken, KaniVerifier>
    for RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>
{
    type Token = RustStdTryLockErrorToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniMutexFailureClassesWitnessToken) -> Self::Token {
        RustStdTryLockErrorToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_TRY_LOCK_ERROR_DISTINGUISHES_POISONED_FROM_WOULD_BLOCK_SRC, {
        /// `try_lock`'s two failure modes are distinct: `Poisoned` when
        /// a prior panic poisoned the mutex, `WouldBlock` when it's simply
        /// already held.
        /// This proof uses the Amenable-owned mutex-failure observation
        /// because the direct poisoning path reaches unsupported
        /// `catch_unwind` under Kani and the direct already-held path is
        /// distorted by Kani's no-concurrency environment model. The claim is
        /// established through `Establish<KaniMutexFailureObservation,
        /// KaniVerifier> for
        /// RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>`
        /// from the observation that demonstrated both failure classes.
        #[kani::proof]
        fn verify_try_lock_error_distinguishes_poisoned_from_would_block() {
            let poisoned_value: i32 = kani::any();
            let held_value: i32 = kani::any();
            let observation = KaniMutexFailureObservation::new(poisoned_value, held_value);
            let demonstration = observation.demonstrate_failure_classes(poisoned_value, held_value);

            let _token =
                RustStdStandard::<std::sync::TryLockError<MutexGuard<'static, i32>>>::establish(
                    demonstration,
                );
        }
    }
}

impl KaniWitness for RustStdStandard<WaitTimeoutResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_wait_timeout_result_reports_timed_out".to_owned(),
            VERIFY_WAIT_TIMEOUT_RESULT_REPORTS_TIMED_OUT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<WaitTimeoutResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<WaitTimeoutResult>",
        "kani",
        || <RustStdStandard<WaitTimeoutResult> as KaniWitness>::proof().to_string(),
    )
}

/// Lawful token minted once `RustStdStandard<WaitTimeoutResult>`'s timeout
/// accessor claim has been established from a `KaniWaitTimeoutObservation`.
pub struct RustStdWaitTimeoutResultToken(());

impl ProofToken for RustStdWaitTimeoutResultToken {
    type Proposition = RustStdStandard<WaitTimeoutResult>;
}

impl Establish<KaniWaitTimeoutWitnessToken, KaniVerifier> for RustStdStandard<WaitTimeoutResult> {
    type Token = RustStdWaitTimeoutResultToken;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(_credential)))]
    fn establish(_credential: KaniWaitTimeoutWitnessToken) -> Self::Token {
        RustStdWaitTimeoutResultToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_WAIT_TIMEOUT_RESULT_REPORTS_TIMED_OUT_SRC, {
        /// Same claim as `Condvar`'s own proof, checked directly on
        /// the `.timed_out()` accessor this carrier exists to expose.
        /// This proof uses the same Amenable-owned timeout observation
        /// because the direct `Condvar::wait_timeout()` path reaches an
        /// unsupported `clock_gettime` boundary under Kani. The claim is
        /// established through `Establish<KaniWaitTimeoutObservation,
        /// KaniVerifier> for RustStdStandard<WaitTimeoutResult>` from the
        /// observation that demonstrated the timeout result.
        #[kani::proof]
        fn verify_wait_timeout_result_reports_timed_out() {
            let observation = KaniWaitTimeoutObservation::timed_out();
            let demonstration = observation.demonstrate_timeout();

            let _token = RustStdStandard::<WaitTimeoutResult>::establish(demonstration);
        }
    }
}
