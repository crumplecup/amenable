::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::location_caller_reaches_unsupported_track_caller_boundary".to_owned(),
            "gallery::replace_recommendations::caller_panic_and_process_env_boundaries::location_caller_reaches_unsupported_track_caller_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "Location::caller reaches an unsupported track_caller boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, LOCATION_CALLER_REACHES_UNSUPPORTED_TRACK_CALLER_BOUNDARY_SRC, {
        /// This is the reduced form behind the `std::panic::Location`
        /// replacement review: the semantic claim is reasonable, but the
        /// direct call to `Location::caller()` already reaches a Kani
        /// unsupported boundary before any richer line/file relation can be
        /// established.
        #[kani::proof]
        fn location_caller_reaches_unsupported_track_caller_boundary() {
            #[track_caller]
            fn here() -> &'static std::panic::Location<'static> {
                std::panic::Location::caller()
            }

            let location = here();
            assert!(!location.file().is_empty(), "tracked caller should name a file");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::catch_unwind_reaches_an_unsupported_panic_boundary".to_owned(),
            "gallery::replace_recommendations::caller_panic_and_process_env_boundaries::catch_unwind_reaches_an_unsupported_panic_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "catch_unwind reaches an unsupported panic boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CATCH_UNWIND_REACHES_AN_UNSUPPORTED_PANIC_BOUNDARY_SRC, {
        /// This is the minimal panic-capture shape behind the writer-panicked
        /// replacement reviews: no external state is needed, only the
        /// `catch_unwind` boundary itself.
        #[kani::proof]
        fn catch_unwind_reaches_an_unsupported_panic_boundary() {
            let recovered = std::panic::catch_unwind(|| {
                panic!("gallery panic boundary");
            });
            assert!(
                recovered.is_err(),
                "catch_unwind should recover the panic payload"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::env_args_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            "gallery::replace_recommendations::caller_panic_and_process_env_boundaries::env_args_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            "amenable_kani".to_owned(),
            "process-argument invariants can fail under Kani's synthetic model".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENV_ARGS_PROCESS_INVARIANT_FAILS_UNDER_THE_SYNTHETIC_KANI_MODEL_SRC, {
        /// Real processes include their own program path in `args()`, but
        /// Kani's synthetic process model can violate that invariant. This is
        /// a model mismatch, not a counterexample to the Rust API contract.
        #[kani::proof]
        fn env_args_process_invariant_fails_under_the_synthetic_kani_model() {
            assert!(
                std::env::args().count() >= 1,
                "real processes should expose at least their own program path"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::env_args_os_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            "gallery::replace_recommendations::caller_panic_and_process_env_boundaries::env_args_os_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            "amenable_kani".to_owned(),
            "raw process-argument invariants can fail under Kani's synthetic model".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENV_ARGS_OS_PROCESS_INVARIANT_FAILS_UNDER_THE_SYNTHETIC_KANI_MODEL_SRC, {
        /// Real processes include their own program slot in `args_os()`, but
        /// Kani's synthetic process model can violate that invariant. This is
        /// a model mismatch, not a counterexample to the Rust API contract.
        #[kani::proof]
        fn env_args_os_process_invariant_fails_under_the_synthetic_kani_model() {
            assert!(
                std::env::args_os().count() >= 1,
                "real processes should expose at least their own program slot"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::join_paths_unjoinable_path_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::caller_panic_and_process_env_boundaries::join_paths_unjoinable_path_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct join_paths can still time out even for one fixed unjoinable path".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, JOIN_PATHS_UNJOINABLE_PATH_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the reduced `join_paths()` representative: one fixed
        /// unjoinable path and one `is_err()` assertion. If this still times
        /// out, the issue is the direct std helper path itself rather than a
        /// richer proof-side setup.
        #[kani::proof]
        fn join_paths_unjoinable_path_times_out_in_the_direct_std_path() {
            let bad_path = if cfg!(windows) { "a\"b" } else { "a:b" };
            assert!(std::env::join_paths([bad_path]).is_err());
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::caller_panic_and_process_env_boundaries::split_paths_round_trip_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct split_paths round trips can still time out on a tiny fixed path list".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SPLIT_PATHS_ROUND_TRIP_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the reduced `split_paths()` representative: a fixed
        /// three-path round trip with no symbolic input. If this still times
        /// out, the issue is the direct std helper path itself rather than a
        /// richer production assertion.
        #[kani::proof]
        fn split_paths_round_trip_times_out_in_the_direct_std_path() {
            let joined = std::env::join_paths(["one", "two", "three"]).unwrap();
            let split: Vec<std::path::PathBuf> = std::env::split_paths(&joined).collect();
            assert_eq!(
                split,
                vec![
                    std::path::PathBuf::from("one"),
                    std::path::PathBuf::from("two"),
                    std::path::PathBuf::from("three"),
                ]
            );
        }
    }
}
