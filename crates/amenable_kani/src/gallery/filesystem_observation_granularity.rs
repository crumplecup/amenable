//! Gallery cases isolating the filesystem accommodation granularity boundary.
//!
//! `replace_recommendations::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario`
//! already preserves the direct OS-backed `std::fs` false trail. This module
//! records the next lesson from the filesystem migration:
//!
//! - moving off the OS boundary is necessary, but not sufficient
//! - a generic mutable filesystem state machine can still be too expensive
//! - the tractable proof surface is the narrow observable law itself
//!
//! In July 2026, `OpenOptions::create_new` was first migrated onto the shared
//! `KaniFileSystem` state model below. Even with fixed symbolic path labels and
//! no real OS calls, Kani still timed out on the combined "existing file /
//! existing directory / fresh path" scenario. Replacing that with a tiny
//! single-path observation (`KaniCreateNewObservation`) preserved the actual
//! Rust-facing law and verified quickly.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::filesystem_observation_granularity::generic_filesystem_state_machine_times_out_for_create_new".to_owned(),
            harness: "gallery::filesystem_observation_granularity::generic_filesystem_state_machine_times_out_for_create_new".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a generic mutable filesystem state machine still times out for create_new".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, GENERIC_FILESYSTEM_STATE_MACHINE_TIMES_OUT_FOR_CREATE_NEW_SRC, {
        /// This is the "second filesystem false trail" after the real
        /// tempdir baseline: a fully Amenable-owned mutable filesystem model
        /// with fixed-depth paths and no OS calls. If this still times out,
        /// the issue is not foreign boundaries anymore; it is the cost of
        /// asking Kani to reason about a generic mutable filesystem state
        /// machine when the proof only needs one `create_new` law.
        #[kani::proof]
        fn generic_filesystem_state_machine_times_out_for_create_new() {
            let directory = crate::KaniFsPath::root().join(crate::KaniFsLabel::new('d'));
            let path = directory.join(crate::KaniFsLabel::new('f'));

            let mut existing_file = crate::KaniFileSystem::new();
            existing_file.create_dir_all(&directory);
            existing_file.create_file(&path);

            let mut existing_directory = crate::KaniFileSystem::new();
            existing_directory.create_dir_all(&path);

            let mut fresh = crate::KaniFileSystem::new();
            fresh.create_dir_all(&directory);

            assert!(existing_file.create_new_file(&path).is_err());
            assert!(existing_directory.create_new_file(&path).is_err());
            assert!(fresh.create_new_file(&path).is_ok());
            assert!(fresh.is_file(&path));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::filesystem_observation_granularity::single_path_create_new_observation_passes".to_owned(),
            harness: "gallery::filesystem_observation_granularity::single_path_create_new_observation_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a single-path create_new observation verifies the same law quickly".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::BestPractice,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, SINGLE_PATH_CREATE_NEW_OBSERVATION_PASSES_SRC, {
        /// Same Rust-facing law as the timeout case above, but phrased as the
        /// exact observable boundary `OpenOptions::create_new` exposes:
        /// reject existing file and directory paths, accept a missing path,
        /// and leave the successful path in a file state.
        #[kani::proof]
        fn single_path_create_new_observation_passes() {
            let mut existing_file = crate::KaniCreateNewObservation::existing_file();
            let mut existing_directory = crate::KaniCreateNewObservation::existing_directory();
            let mut fresh = crate::KaniCreateNewObservation::missing();

            assert!(existing_file.create_new().is_err());
            assert!(existing_directory.create_new().is_err());
            assert!(fresh.create_new().is_ok());
            assert!(fresh.is_file());
        }
    }
}
