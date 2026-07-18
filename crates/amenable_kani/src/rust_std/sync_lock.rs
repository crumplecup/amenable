//! `KaniWitness` impls for `std::sync`'s locking primitives.
//!
//! Poisoning is triggered via `std::panic::catch_unwind` around a closure
//! that locks and then panics while the guard is held — confirmed
//! empirically to poison the lock even in a single-threaded harness
//! (poisoning tracks "a panic occurred while a guard was live," not
//! specifically a *cross-thread* panic).

use std::sync::{
    Barrier, BarrierWaitResult, LazyLock, MutexGuard, OnceLock, OnceState, PoisonError,
    WaitTimeoutResult,
};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<std::sync::Mutex<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_mutex_excludes_a_second_lock_while_held",
            claim: VERIFY_MUTEX_EXCLUDES_A_SECOND_LOCK_WHILE_HELD_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_MUTEX_EXCLUDES_A_SECOND_LOCK_WHILE_HELD_SRC, {
        /// `.lock()` derefs to the wrapped value, and `.try_lock()`
        /// fails while a guard is already held, succeeding again once
        /// it's dropped.
        #[kani::proof]
        fn verify_mutex_excludes_a_second_lock_while_held() {
            let value: i32 = kani::any();
            let mutex = std::sync::Mutex::new(value);
            {
                let guard = mutex.lock().unwrap();
                assert_eq!(*guard, value, "lock derefs to the wrapped value");
                assert!(
                    mutex.try_lock().is_err(),
                    "try_lock fails while a guard is already held"
                );
            }
            assert!(
                mutex.try_lock().is_ok(),
                "try_lock succeeds once the guard is dropped"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::MutexGuard<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_mutex_guard_writes_through",
            claim: VERIFY_MUTEX_GUARD_WRITES_THROUGH_SRC,
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
            harness: "verify_rwlock_allows_concurrent_reads_but_not_a_write",
            claim: VERIFY_RWLOCK_ALLOWS_CONCURRENT_READS_BUT_NOT_A_WRITE_SRC,
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
            harness: "verify_rwlock_read_guard_derefs_to_the_value",
            claim: VERIFY_RWLOCK_READ_GUARD_DEREFS_TO_THE_VALUE_SRC,
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
            harness: "verify_rwlock_write_guard_writes_through",
            claim: VERIFY_RWLOCK_WRITE_GUARD_WRITES_THROUGH_SRC,
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
            harness: "verify_once_runs_its_closure_exactly_once",
            claim: VERIFY_ONCE_RUNS_ITS_CLOSURE_EXACTLY_ONCE_SRC,
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
            harness: "verify_once_state_reports_not_poisoned_on_a_clean_run",
            claim: VERIFY_ONCE_STATE_REPORTS_NOT_POISONED_ON_A_CLEAN_RUN_SRC,
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
            harness: "verify_once_lock_initializes_exactly_once",
            claim: VERIFY_ONCE_LOCK_INITIALIZES_EXACTLY_ONCE_SRC,
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
            harness: "verify_lazy_lock_caches_its_initializer_result",
            claim: VERIFY_LAZY_LOCK_CACHES_ITS_INITIALIZER_RESULT_SRC,
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
            assert_eq!(
                first, second,
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
            harness: "verify_barrier_of_one_is_its_own_leader",
            claim: VERIFY_BARRIER_OF_ONE_IS_ITS_OWN_LEADER_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_BARRIER_OF_ONE_IS_ITS_OWN_LEADER_SRC, {
        /// A `Barrier` built for exactly one participant returns
        /// immediately from `.wait()`, and that lone participant is
        /// always the leader.
        #[kani::proof]
        fn verify_barrier_of_one_is_its_own_leader() {
            let barrier = Barrier::new(1);
            let result = barrier.wait();
            assert!(result.is_leader(), "the sole participant is the leader");
        }
    }
}

impl KaniWitness for RustStdStandard<BarrierWaitResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_barrier_wait_result_reports_the_sole_participant_as_leader",
            claim: VERIFY_BARRIER_WAIT_RESULT_REPORTS_THE_SOLE_PARTICIPANT_AS_LEADER_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_BARRIER_WAIT_RESULT_REPORTS_THE_SOLE_PARTICIPANT_AS_LEADER_SRC, {
        /// Same claim as `Barrier` itself, checked directly on the
        /// `.is_leader()` accessor this carrier exists to expose.
        #[kani::proof]
        fn verify_barrier_wait_result_reports_the_sole_participant_as_leader() {
            let barrier = Barrier::new(1);
            assert!(barrier.wait().is_leader());
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::Condvar> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_condvar_wait_timeout_reports_timing_out",
            claim: VERIFY_CONDVAR_WAIT_TIMEOUT_REPORTS_TIMING_OUT_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_CONDVAR_WAIT_TIMEOUT_REPORTS_TIMING_OUT_SRC, {
        /// `.wait_timeout()` on a `Condvar` nobody ever notifies times
        /// out (checked with a zero-duration timeout, so the harness
        /// doesn't actually wait) and reports that through its
        /// `WaitTimeoutResult`.
        #[kani::proof]
        fn verify_condvar_wait_timeout_reports_timing_out() {
            use std::time::Duration;

            let mutex = std::sync::Mutex::new(false);
            let condvar = std::sync::Condvar::new();
            let guard = mutex.lock().unwrap();
            let (_guard, timeout_result) = condvar
                .wait_timeout(guard, Duration::from_millis(0))
                .unwrap();
            assert!(
                timeout_result.timed_out(),
                "a never-notified wait times out"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<PoisonError<MutexGuard<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_poison_error_still_recovers_the_guards_value",
            claim: VERIFY_POISON_ERROR_STILL_RECOVERS_THE_GUARDS_VALUE_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_POISON_ERROR_STILL_RECOVERS_THE_GUARDS_VALUE_SRC, {
        /// A panic while a `Mutex` guard is held poisons it (confirmed
        /// via `catch_unwind` — poisoning tracks "a panic occurred
        /// while a guard was live," not specifically a cross-thread
        /// panic, so this works in a single-threaded harness). The
        /// resulting `PoisonError` doesn't discard the data:
        /// `.into_inner()` still recovers the guard.
        #[kani::proof]
        fn verify_poison_error_still_recovers_the_guards_value() {
            let mutex = std::sync::Mutex::new(0i32);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _guard = mutex.lock().unwrap();
                panic!("poison it");
            }));
            assert!(result.is_err());

            match mutex.lock() {
                Ok(_) => panic!("expected the mutex to be poisoned"),
                Err(poison_err) => {
                    let guard = poison_err.into_inner();
                    assert_eq!(*guard, 0, "into_inner still recovers the guard's value");
                }
            }
        }
    }
}

impl KaniWitness for RustStdStandard<std::sync::TryLockError<MutexGuard<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_try_lock_error_distinguishes_poisoned_from_would_block",
            claim: VERIFY_TRY_LOCK_ERROR_DISTINGUISHES_POISONED_FROM_WOULD_BLOCK_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_TRY_LOCK_ERROR_DISTINGUISHES_POISONED_FROM_WOULD_BLOCK_SRC, {
        /// `try_lock`'s two failure modes are distinct: `Poisoned` when
        /// a prior panic poisoned the mutex, `WouldBlock` when it's
        /// simply already held.
        #[kani::proof]
        fn verify_try_lock_error_distinguishes_poisoned_from_would_block() {
            let poisoned = std::sync::Mutex::new(0i32);
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _guard = poisoned.lock().unwrap();
                panic!("poison it");
            }));
            match poisoned.try_lock() {
                Err(std::sync::TryLockError::Poisoned(_)) => {}
                _ => panic!("expected Poisoned"),
            }

            let value: i32 = kani::any();
            let held = std::sync::Mutex::new(value);
            let _guard = held.lock().unwrap();
            match held.try_lock() {
                Err(std::sync::TryLockError::WouldBlock) => {}
                _ => panic!("expected WouldBlock"),
            }
        }
    }
}

impl KaniWitness for RustStdStandard<WaitTimeoutResult> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_wait_timeout_result_reports_timed_out",
            claim: VERIFY_WAIT_TIMEOUT_RESULT_REPORTS_TIMED_OUT_SRC,
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

amenable_derive::harness! {
    kani, VERIFY_WAIT_TIMEOUT_RESULT_REPORTS_TIMED_OUT_SRC, {
        /// Same claim as `Condvar`'s own proof, checked directly on
        /// the `.timed_out()` accessor this carrier exists to expose.
        #[kani::proof]
        fn verify_wait_timeout_result_reports_timed_out() {
            use std::time::Duration;

            let mutex = std::sync::Mutex::new(false);
            let condvar = std::sync::Condvar::new();
            let guard = mutex.lock().unwrap();
            let (_guard, timeout_result) = condvar
                .wait_timeout(guard, Duration::from_millis(0))
                .unwrap();
            assert!(timeout_result.timed_out());
        }
    }
}
