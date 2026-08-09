//! `KaniWitness` impls for `std::sync`'s locking primitives.
//!
//! Direct guard, `RwLock`, `Once`, `OnceLock`, and `LazyLock` laws still verify
//! against the real standard-library types. The `Mutex` exclusion, `Barrier`,
//! `Condvar`, poisoning, and timeout-result laws instead use Amenable-owned
//! observations: their direct std paths either hit unsupported Kani
//! boundaries (`futex_wait`, `clock_gettime`, `catch_unwind`) or rely on
//! mutual-exclusion behavior that Kani's no-concurrency environment model does
//! not enforce. The false trails remain preserved in the gallery.

use std::sync::{
    Barrier, BarrierWaitResult, LazyLock, MutexGuard, OnceLock, OnceState, PoisonError,
    WaitTimeoutResult,
};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::{Establish, Evidence, ProofToken};
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
use crate::{
    KaniBarrierLeaderObservation, KaniMutexExclusionObservation, KaniMutexFailureObservation,
    KaniVerifier, KaniWaitTimeoutObservation, KaniWitness,
};

impl KaniWitness for RustStdStandard<std::sync::Mutex<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_mutex_excludes_a_second_lock_while_held".to_owned(),
            claim: VERIFY_MUTEX_EXCLUDES_A_SECOND_LOCK_WHILE_HELD_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Mutex<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Mutex<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::Mutex<i32>> as KaniWitness>::proof().to_string(),
    }
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
        CheckedProof {
            harness: "verify_mutex_guard_writes_through".to_owned(),
            claim: VERIFY_MUTEX_GUARD_WRITES_THROUGH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::MutexGuard<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::MutexGuard<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::MutexGuard<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
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
                assert_eq!(*guard, value);
                *guard = updated;
            }
            assert_eq!(
                *mutex.lock().unwrap(),
                updated,
                "a write through the guard is visible after it's dropped"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::RwLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rwlock_allows_concurrent_reads_but_not_a_write".to_owned(),
            claim: VERIFY_RWLOCK_ALLOWS_CONCURRENT_READS_BUT_NOT_A_WRITE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::RwLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::RwLock<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::RwLock<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RWLOCK_ALLOWS_CONCURRENT_READS_BUT_NOT_A_WRITE_SRC, {
        /// Unlike `Mutex`, two read guards can be held at once — but a
        /// write is still exclusive against them, the defining
        /// difference between the two lock types.
        #[kani::proof]
        fn verify_rwlock_allows_concurrent_reads_but_not_a_write() {
            let value: i32 = kani::any();
            let lock = std::sync::RwLock::new(value);
            {
                let r1 = lock.read().unwrap();
                let r2 = lock.read().unwrap();
                assert_eq!(*r1, value);
                assert_eq!(*r2, value, "two read guards can be held concurrently");
                assert!(
                    lock.try_write().is_err(),
                    "a write is rejected while readers are held"
                );
            }
            assert!(
                lock.try_write().is_ok(),
                "a write succeeds once the readers are dropped"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::RwLockReadGuard<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rwlock_read_guard_derefs_to_the_value".to_owned(),
            claim: VERIFY_RWLOCK_READ_GUARD_DEREFS_TO_THE_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::RwLockReadGuard<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::RwLockReadGuard<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::RwLockReadGuard<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RWLOCK_READ_GUARD_DEREFS_TO_THE_VALUE_SRC, {
        /// `.read()` derefs to the locked value.
        #[kani::proof]
        fn verify_rwlock_read_guard_derefs_to_the_value() {
            let value: i32 = kani::any();
            let lock = std::sync::RwLock::new(value);
            let guard = lock.read().unwrap();
            assert_eq!(*guard, value);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rwlock_write_guard_writes_through".to_owned(),
            claim: VERIFY_RWLOCK_WRITE_GUARD_WRITES_THROUGH_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::RwLockWriteGuard<'static, i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RWLOCK_WRITE_GUARD_WRITES_THROUGH_SRC, {
        /// A write through `.write()`'s guard is visible on a later
        /// read, once the write guard is dropped.
        #[kani::proof]
        fn verify_rwlock_write_guard_writes_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let lock = std::sync::RwLock::new(value);
            {
                let mut guard = lock.write().unwrap();
                assert_eq!(*guard, value);
                *guard = updated;
            }
            assert_eq!(*lock.read().unwrap(), updated);
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::Once> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_once_runs_its_closure_exactly_once".to_owned(),
            claim: VERIFY_ONCE_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Once>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Once>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::Once> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC, {
        /// `.call_once()` runs its closure the first time; a second
        /// call is a no-op, observed through a shared counter.
        #[kani::proof]
        fn verify_once_runs_its_closure_exactly_once() {
            static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let once = std::sync::Once::new();
            once.call_once(|| {
                CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
            once.call_once(|| {
                CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
            assert_eq!(
                CALLS.load(std::sync::atomic::Ordering::SeqCst),
                1,
                "call_once runs its closure exactly once"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<OnceState> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_once_state_reports_not_poisoned_on_a_clean_run".to_owned(),
            claim: VERIFY_ONCE_STATE_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OnceState>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OnceState>",
        verifier: "kani",
        describe: || <RustStdStandard<OnceState> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_STATE_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC, {
        /// `.call_once_force()` hands its closure an `OnceState`
        /// reporting `is_poisoned() == false` on a clean (never-
        /// panicked) `Once`.
        #[kani::proof]
        fn verify_once_state_reports_not_poisoned_on_a_clean_run() {
            let once = std::sync::Once::new();
            once.call_once_force(|state| {
                assert!(!state.is_poisoned(), "a clean Once reports not poisoned");
            });
        }
    }
}

impl KaniWitness for RustStdStandard<OnceLock<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_once_lock_initializes_exactly_once".to_owned(),
            claim: VERIFY_ONCE_LOCK_INITIALIZES_EXACTLY_ONCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OnceLock<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OnceLock<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<OnceLock<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_LOCK_INITIALIZES_EXACTLY_ONCE_SRC, {
        /// Same exactly-once contract as `core::cell::OnceCell`, for
        /// the thread-safe carrier: empty, first `set` succeeds, a
        /// second `set` is rejected without disturbing the value.
        #[kani::proof]
        fn verify_once_lock_initializes_exactly_once() {
            let cell: OnceLock<i32> = OnceLock::new();
            assert!(cell.get().is_none());

            let value: i32 = kani::any();
            assert!(cell.set(value).is_ok(), "the first set succeeds");
            assert_eq!(cell.get(), Some(&value));

            let other: i32 = kani::any();
            assert!(cell.set(other).is_err(), "a second set is rejected");
            assert_eq!(
                cell.get(),
                Some(&value),
                "the original value survives a rejected second set"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<LazyLock<i32, fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_lazy_lock_caches_its_initializer_result".to_owned(),
            claim: VERIFY_LAZY_LOCK_CACHES_ITS_INITIALIZER_RESULT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<LazyLock<i32, fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LazyLock<i32, fn() -> i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<LazyLock<i32, fn() -> i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<LazyLock<i32, fn() -> i32>>,
    "amenable_std::rust_std::RustStdStandard<LazyLock<i32, fn() -> i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<LazyLock<i32, fn() -> i32>>",
        verifier: "kani",
        kind: "ensures",
        fragment: || "RustStdStandard::<LazyLock<i32, fn() -> i32>>::ensures((first, second))",
        harnesses: &["verify_lazy_lock_caches_its_initializer_result"],
    }
}

amenable_derive::harness! {
    kani, VERIFY_LAZY_LOCK_CACHES_ITS_INITIALIZER_RESULT_SRC, {
        /// Same caching proof technique as `core::cell::LazyCell`:
        /// `kani::any()` inside the initializer means a re-invoked
        /// closure would force two independently arbitrary results, so
        /// the two derefs being forced equal is exactly what "ran once,
        /// cached" means.
        #[kani::proof]
        fn verify_lazy_lock_caches_its_initializer_result() {
            fn init() -> i32 {
                kani::any()
            }
            let lazy: LazyLock<i32, fn() -> i32> = LazyLock::new(init);
            let first = *lazy;
            let second = *lazy;
            assert!(
                RustStdStandard::<LazyLock<i32, fn() -> i32>>::ensures((first, second)),
                "LazyLock caches its initializer's result across derefs"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Barrier> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_barrier_of_one_is_its_own_leader".to_owned(),
            claim: VERIFY_BARRIER_OF_ONE_IS_ITS_OWN_LEADER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Barrier>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Barrier>",
        verifier: "kani",
        describe: || <RustStdStandard<Barrier> as KaniWitness>::proof().to_string(),
    }
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
        CheckedProof {
            harness: "verify_barrier_wait_result_reports_the_sole_participant_as_leader".to_owned(),
            claim: VERIFY_BARRIER_WAIT_RESULT_REPORTS_THE_SOLE_PARTICIPANT_AS_LEADER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BarrierWaitResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BarrierWaitResult>",
        verifier: "kani",
        describe: || <RustStdStandard<BarrierWaitResult> as KaniWitness>::proof().to_string(),
    }
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

impl KaniWitness for RustStdStandard<std::sync::Condvar> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_condvar_wait_timeout_reports_timing_out".to_owned(),
            claim: VERIFY_CONDVAR_WAIT_TIMEOUT_REPORTS_TIMING_OUT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::Condvar>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::Condvar>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::Condvar> as KaniWitness>::proof().to_string(),
    }
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
    /// [`KaniMutexExclusionObservation::demonstrate_exclusion`] does.
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

impl KaniWitness for RustStdStandard<PoisonError<MutexGuard<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_poison_error_still_recovers_the_guards_value".to_owned(),
            claim: VERIFY_POISON_ERROR_STILL_RECOVERS_THE_GUARDS_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<PoisonError<MutexGuard<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<PoisonError<MutexGuard<'static, i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<PoisonError<MutexGuard<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    }
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
    /// [`KaniMutexExclusionObservation::demonstrate_exclusion`] does.
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_lock_error_distinguishes_poisoned_from_would_block".to_owned(),
            claim: VERIFY_TRY_LOCK_ERROR_DISTINGUISHES_POISONED_FROM_WOULD_BLOCK_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    }
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
    /// [`KaniMutexExclusionObservation::demonstrate_exclusion`] does.
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

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_wait_timeout_result_reports_timed_out".to_owned(),
            claim: VERIFY_WAIT_TIMEOUT_RESULT_REPORTS_TIMED_OUT_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<WaitTimeoutResult>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<WaitTimeoutResult>",
        verifier: "kani",
        describe: || <RustStdStandard<WaitTimeoutResult> as KaniWitness>::proof().to_string(),
    }
}

/// Lawful token minted once `RustStdStandard<WaitTimeoutResult>`'s timeout
/// accessor claim has been established from a `KaniWaitTimeoutObservation`.
pub struct RustStdWaitTimeoutResultToken(());

impl ProofToken for RustStdWaitTimeoutResultToken {
    type Proposition = RustStdStandard<WaitTimeoutResult>;
}

impl Establish<KaniWaitTimeoutWitnessToken, KaniVerifier> for RustStdStandard<WaitTimeoutResult> {
    type Token = RustStdWaitTimeoutResultToken;

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
