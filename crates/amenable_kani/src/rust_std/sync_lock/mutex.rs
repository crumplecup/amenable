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
use crate::{KaniMutexExclusionObservation, KaniVerifier, KaniWitness};

impl KaniWitness for RustStdStandard<std::sync::Mutex<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mutex_excludes_a_second_lock_while_held".to_owned(),
            VERIFY_MUTEX_EXCLUDES_A_SECOND_LOCK_WHILE_HELD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Mutex<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::Mutex<i32>>",
        "kani",
        || <RustStdStandard<std::sync::Mutex<i32>> as KaniWitness>::proof().to_string(),
    )
}

/// Witness that a `KaniMutexExclusionObservation` instance actually
/// demonstrated the held-value and exclusion behavior, minted only by
/// [`KaniMutexExclusionObservation::demonstrate_exclusion`].
pub struct KaniMutexExclusionWitnessToken(());

impl ProofToken for KaniMutexExclusionWitnessToken {
    type Proposition = KaniMutexExclusionObservation;
}

impl KaniMutexExclusionObservation {
    /// Assert the held value derefs correctly and a second `try_lock`
    /// fails while held but succeeds after release. Consumes `self`: the
    /// only way to obtain the token is to have run this check against a
    /// real observation instance, not to assert it independently.
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(self)))]
    #[must_use]
    pub fn demonstrate_exclusion(self, value: i32) -> KaniMutexExclusionWitnessToken {
        assert_eq!(self.held_value(), value, "lock derefs to the wrapped value");
        assert!(
            self.try_lock_while_held_is_err(),
            "try_lock fails while a guard is already held"
        );
        assert!(
            self.try_lock_after_release_is_ok(),
            "try_lock succeeds once the guard is dropped"
        );
        KaniMutexExclusionWitnessToken(())
    }
}

/// Lawful token minted once
/// `RustStdStandard<std::sync::Mutex<i32>>`'s exclusion claim has been
/// established from a `KaniMutexExclusionObservation`.
pub struct RustStdMutexToken(());

impl ProofToken for RustStdMutexToken {
    type Proposition = RustStdStandard<std::sync::Mutex<i32>>;
}

impl Establish<KaniMutexExclusionWitnessToken, KaniVerifier>
    for RustStdStandard<std::sync::Mutex<i32>>
{
    type Token = RustStdMutexToken;

    fn establish(_credential: KaniMutexExclusionWitnessToken) -> Self::Token {
        RustStdMutexToken(())
    }
}

amenable_derive::harness! {
    kani, VERIFY_MUTEX_EXCLUDES_A_SECOND_LOCK_WHILE_HELD_SRC, {
        /// `.lock()` derefs to the wrapped value, and `.try_lock()` fails while
        /// a guard is already held, succeeding again once it's dropped.
        /// This proof uses the Amenable-owned bounded mutex-exclusion model:
        /// Kani's no-concurrency environment model does not enforce the real
        /// `Mutex` exclusion guarantee, so the claim is established through
        /// `Establish<KaniMutexExclusionObservation, KaniVerifier> for
        /// RustStdStandard<std::sync::Mutex<i32>>` from the observation that
        /// demonstrated the held-value and exclusion behavior.
        #[kani::proof]
        fn verify_mutex_excludes_a_second_lock_while_held() {
            let value: i32 = kani::any();
            let observation = KaniMutexExclusionObservation::new(value);
            let demonstration = observation.demonstrate_exclusion(value);

            let _token = RustStdStandard::<std::sync::Mutex<i32>>::establish(demonstration);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::MutexGuard<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mutex_guard_writes_through".to_owned(),
            VERIFY_MUTEX_GUARD_WRITES_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::MutexGuard<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::sync::MutexGuard<'static, i32>>",
        "kani",
        || <RustStdStandard<std::sync::MutexGuard<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MUTEX_GUARD_WRITES_THROUGH_SRC, {
        /// A write through the guard's `DerefMut` is visible on a
        /// later lock, once the guard is dropped.
        #[kani::proof]
        fn verify_mutex_guard_writes_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mutex = std::sync::Mutex::new(value);
            {
                let mut guard = mutex.lock().unwrap();
                assert!(DerefReflectsTheStoredValue::ensures((*guard, value)));
                *guard = updated;
            }
            assert!(
                DerefReflectsTheStoredValue::ensures((*mutex.lock().unwrap(), updated)),
                "a write through the guard is visible after it's dropped"
            );
        }
    }
}
