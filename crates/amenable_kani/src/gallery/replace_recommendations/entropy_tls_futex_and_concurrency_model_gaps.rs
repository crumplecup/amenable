::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::random_state_construction_reaches_an_unsupported_entropy_source_boundary".to_owned(),
            "gallery::replace_recommendations::entropy_tls_futex_and_concurrency_model_gaps::random_state_construction_reaches_an_unsupported_entropy_source_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "RandomState::new() reaches an unsupported OS entropy-source boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, RANDOM_STATE_CONSTRUCTION_REACHES_AN_UNSUPPORTED_ENTROPY_SOURCE_BOUNDARY_SRC, {
        /// This is the reduced form behind the `RandomState` review: the
        /// per-instance determinism claim itself is reasonable (two
        /// hashers built from the *same* instance should agree), but
        /// `RandomState::new()` reaches Kani's unsupported OS
        /// entropy-source boundary before the claim can be established;
        /// the current reduction bottoms out in `getrandom` via a raw
        /// `syscall`, consistent with reading process entropy to pick the
        /// random per-instance seed. That is the same class of OS-backed
        /// boundary as the pipe/fd cases above, not a proof-side
        /// deficiency.
        #[kani::proof]
        fn random_state_construction_reaches_an_unsupported_entropy_source_boundary() {
            use std::hash::{BuildHasher, Hash, Hasher};

            let state = std::hash::RandomState::new();
            let mut hasher = state.build_hasher();
            "some value".hash(&mut hasher);
            let _ = hasher.finish();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::thread_current_reaches_an_unsupported_thread_local_storage_boundary".to_owned(),
            "gallery::replace_recommendations::entropy_tls_futex_and_concurrency_model_gaps::thread_current_reaches_an_unsupported_thread_local_storage_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "std::thread::current() reaches an unsupported pthread_key_create boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, THREAD_CURRENT_REACHES_AN_UNSUPPORTED_THREAD_LOCAL_STORAGE_BOUNDARY_SRC, {
        /// This is the reduced form behind both `thread::current()`- and
        /// `ThreadId`-stability reviews: the two-calls-agree claim itself
        /// is straightforward, but `std::thread::current()` reaches a
        /// `pthread_key_create` call (thread-local-storage key creation)
        /// Kani reports unsupported before the claim can be established --
        /// an OS-backed threading boundary, the same class as the other
        /// foreign-boundary cases above, not a proof-side deficiency.
        #[kani::proof]
        fn thread_current_reaches_an_unsupported_thread_local_storage_boundary() {
            let _ = std::thread::current();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::barrier_wait_reaches_an_unsupported_futex_boundary".to_owned(),
            "gallery::replace_recommendations::entropy_tls_futex_and_concurrency_model_gaps::barrier_wait_reaches_an_unsupported_futex_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Barrier::wait() reaches an unsupported futex syscall boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BARRIER_WAIT_REACHES_AN_UNSUPPORTED_FUTEX_BOUNDARY_SRC, {
        /// This is the reduced form behind both `Barrier` reviews: the
        /// sole-participant-is-leader claim is straightforward, but even a
        /// `Barrier::new(1)` still routes `.wait()` through the same
        /// generic futex-based wait/notify machinery as a multi-party
        /// barrier, which reaches a raw `syscall` (`futex_wait`) Kani
        /// reports unsupported before the claim can be checked -- an
        /// OS-backed threading boundary, not a proof-side deficiency.
        #[kani::proof]
        fn barrier_wait_reaches_an_unsupported_futex_boundary() {
            let barrier = std::sync::Barrier::new(1);
            let _ = barrier.wait();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::condvar_wait_timeout_reaches_an_unsupported_clock_boundary".to_owned(),
            "gallery::replace_recommendations::entropy_tls_futex_and_concurrency_model_gaps::condvar_wait_timeout_reaches_an_unsupported_clock_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Condvar::wait_timeout reaches an unsupported clock_gettime boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CONDVAR_WAIT_TIMEOUT_REACHES_AN_UNSUPPORTED_CLOCK_BOUNDARY_SRC, {
        /// This is the reduced form behind both `Condvar`/`WaitTimeoutResult`
        /// reviews: the never-notified-wait-times-out claim is
        /// straightforward, but computing the timeout deadline reaches
        /// `clock_gettime` (via `Timespec::now`) Kani reports unsupported,
        /// even for a zero-duration timeout -- an OS-backed clock boundary,
        /// not a proof-side deficiency.
        #[kani::proof]
        fn condvar_wait_timeout_reaches_an_unsupported_clock_boundary() {
            use std::time::Duration;

            let mutex = std::sync::Mutex::new(());
            let condvar = std::sync::Condvar::new();
            let guard = mutex.lock().unwrap();
            let _ = condvar.wait_timeout(guard, Duration::from_millis(0));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::mutex_poisoning_reaches_the_unsupported_catch_unwind_boundary".to_owned(),
            "gallery::replace_recommendations::entropy_tls_futex_and_concurrency_model_gaps::mutex_poisoning_reaches_the_unsupported_catch_unwind_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Mutex poisoning reaches an unsupported catch_unwind boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, MUTEX_POISONING_REACHES_THE_UNSUPPORTED_CATCH_UNWIND_BOUNDARY_SRC, {
        /// This is the reduced form behind the `PoisonError` review and
        /// the poisoned branch of the `TryLockError` review: the data-
        /// recovery claim is straightforward, but the direct poisoning
        /// setup must cross `std::panic::catch_unwind`, which Kani
        /// currently reports unsupported before the lock can be observed
        /// as poisoned. That is a verifier boundary for the direct std
        /// path, not a reason to drop the law.
        #[kani::proof]
        fn mutex_poisoning_reaches_the_unsupported_catch_unwind_boundary() {
            let mutex = std::sync::Mutex::new(0i32);
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _guard = mutex.lock().unwrap();
                panic!("poison it");
            }));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::try_lock_succeeds_under_kanis_no_concurrency_environment_model".to_owned(),
            "gallery::replace_recommendations::entropy_tls_futex_and_concurrency_model_gaps::try_lock_succeeds_under_kanis_no_concurrency_environment_model".to_owned(),
            "amenable_kani".to_owned(),
            "Mutex::try_lock succeeds while a guard is held, under Kani's no-concurrency-support environment model".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TRY_LOCK_SUCCEEDS_UNDER_KANIS_NO_CONCURRENCY_ENVIRONMENT_MODEL_SRC, {
        /// Unlike the other cases in this module, this is not an
        /// unsupported-construct failure -- it is a genuine assertion
        /// failure, and a genuine Kani-environment mismatch against a real
        /// invariant, the same class already documented for
        /// `env_args_process_invariant_fails_under_the_synthetic_kani_model`.
        /// Kani reports "Kani currently does not support concurrency" for
        /// every harness that reaches synchronization primitives; under
        /// that model, `Mutex::try_lock` was observed to succeed even
        /// while a `MutexGuard` from the same `.lock()` call is still
        /// alive and unreleased -- which never happens in real single- or
        /// multi-threaded Rust execution (`Mutex` is not reentrant).
        #[kani::proof]
        fn try_lock_succeeds_under_kanis_no_concurrency_environment_model() {
            let mutex = std::sync::Mutex::new(0i32);
            let _guard = mutex.lock().unwrap();
            assert!(
                mutex.try_lock().is_err(),
                "a real Mutex never grants a second lock while a guard is held"
            );
        }
    }
}
