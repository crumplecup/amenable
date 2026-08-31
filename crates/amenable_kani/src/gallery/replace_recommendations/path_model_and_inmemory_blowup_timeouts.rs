::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_first_concrete_string_model".to_owned(),
            "gallery::replace_recommendations::path_model_and_inmemory_blowup_timeouts::split_paths_round_trip_times_out_in_the_first_concrete_string_model".to_owned(),
            "amenable_kani".to_owned(),
            "a first-pass concrete PATH string model can still time out under Kani".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SPLIT_PATHS_ROUND_TRIP_TIMES_OUT_IN_THE_FIRST_CONCRETE_STRING_MODEL_SRC, {
        /// This is the first accommodation false trail for `split_paths()`:
        /// direct std helpers are gone, but the proof still asks Kani to
        /// reason through owned `String`/`Vec` rendering and parsing. If this
        /// times out, the next step is a semantic wrapper, not more proof-side
        /// assertion trimming.
        #[kani::proof]
        fn split_paths_round_trip_times_out_in_the_first_concrete_string_model() {
            let paths = ::amenable_kani::KaniEnvPathList::from_strings(vec![
                "one".to_owned(),
                "two".to_owned(),
                "three".to_owned(),
            ])
            .expect("separator-free paths stay inside the modeled subset");
            let joined = ::amenable_kani::KaniEnvPaths::join(&paths);
            let split = ::amenable_kani::KaniEnvPaths::split(&joined);

            assert_eq!(split.len(), 3);
            assert_eq!(split.paths()[0].as_str(), "one");
            assert_eq!(split.paths()[1].as_str(), "two");
            assert_eq!(split.paths()[2].as_str(), "three");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::default_hasher_determinism_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::path_model_and_inmemory_blowup_timeouts::default_hasher_determinism_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct default-hasher determinism can still time out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, DEFAULT_HASHER_DETERMINISM_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is a reduced representative for pure in-memory std blow-up:
        /// there is no OS boundary and no panic recovery, only direct hashing
        /// work that still exceeds the verifier budget.
        #[kani::proof]
        fn default_hasher_determinism_times_out_in_the_direct_std_path() {
            use std::hash::{BuildHasher, Hasher};

            let value: u64 = kani::any();
            let builder = std::hash::BuildHasherDefault::<
                std::collections::hash_map::DefaultHasher,
            >::default();
            let mut first = builder.build_hasher();
            let mut second = builder.build_hasher();
            first.write_u64(value);
            second.write_u64(value);

            assert_eq!(
                first.finish(),
                second.finish(),
                "freshly built default hashers should agree on the same input"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::hash_map_insert_then_get_times_out_even_for_a_fixed_entry".to_owned(),
            "gallery::replace_recommendations::path_model_and_inmemory_blowup_timeouts::hash_map_insert_then_get_times_out_even_for_a_fixed_entry".to_owned(),
            "amenable_kani".to_owned(),
            "direct HashMap insert-then-get can still time out even for one fixed entry".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, HASH_MAP_INSERT_THEN_GET_TIMES_OUT_EVEN_FOR_A_FIXED_ENTRY_SRC, {
        /// Another representative for the pure in-memory std blow-up class
        /// (same bucket as `default_hasher_determinism_times_out_in_the_direct_std_path`):
        /// `HashMap::new()` defaults to `RandomState`, so every insert/get
        /// routes through the same hashing machinery, and the timeout
        /// shows up even for a single fixed (non-symbolic) key/value pair
        /// -- confirmed empirically before building
        /// `amenable_kani::hash_collections_model`'s accommodation model.
        /// `HashSet` shares this exact cause.
        #[kani::proof]
        fn hash_map_insert_then_get_times_out_even_for_a_fixed_entry() {
            let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
            map.insert(1, 2);
            assert_eq!(map.get(&1), Some(&2), "insert then get recovers the same value");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario".to_owned(),
            "gallery::replace_recommendations::path_model_and_inmemory_blowup_timeouts::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario".to_owned(),
            "amenable_kani".to_owned(),
            "a small real-filesystem tempdir scenario can still time out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, REAL_FILESYSTEM_BOUNDARY_TIMES_OUT_EVEN_FOR_A_SMALL_TEMPDIR_SCENARIO_SRC, {
        /// This is the reduced filesystem-boundary representative: a single
        /// temp directory, one created file, one metadata observation, and
        /// cleanup. If this still times out, the issue is the real std::fs
        /// path itself rather than a larger production-specific assertion.
        ///
        /// In July 2026 we also confirmed that two filesystem accommodations
        /// were still too heavy for Kani: first a heap-backed `Vec<String>`
        /// path model, then a more generic mutable filesystem state machine
        /// with fixed symbolic labels. Future filesystem proofs should target
        /// the narrow observable law directly rather than simulating a general
        /// filesystem.
        #[kani::proof]
        fn real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario() {
            let base =
                std::env::temp_dir().join(format!("amenable_kani_gallery_fs_{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&base);

            std::fs::create_dir_all(&base).unwrap();
            let file_path = base.join("data.txt");
            std::fs::File::create(&file_path).unwrap();

            assert!(std::fs::metadata(&file_path).unwrap().is_file());

            std::fs::remove_dir_all(&base).unwrap();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::format_arguments_rendering_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::path_model_and_inmemory_blowup_timeouts::format_arguments_rendering_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct fmt::Arguments rendering can still time out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, FORMAT_ARGUMENTS_RENDERING_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the reduced `fmt` representative: a single formatting
        /// argument rendered two ways. If this still times out, the issue is
        /// the direct formatting machinery rather than a richer proof shape.
        #[kani::proof]
        fn format_arguments_rendering_times_out_in_the_direct_std_path() {
            let value: i32 = kani::any();
            let args = format_args!("{}", value);

            assert_eq!(
                args.to_string(),
                value.to_string(),
                "Arguments should render the same as the value's own Display"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::from_fn_rendering_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::path_model_and_inmemory_blowup_timeouts::from_fn_rendering_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct fmt::from_fn rendering can still time out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, FROM_FN_RENDERING_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// Same formatting-machinery timeout class as
        /// `format_arguments_rendering_times_out_in_the_direct_std_path`,
        /// confirmed separately for `fmt::from_fn`.
        #[kani::proof]
        fn from_fn_rendering_times_out_in_the_direct_std_path() {
            let wrapped = core::fmt::from_fn(|f| write!(f, "hi"));
            assert_eq!(wrapped.to_string(), "hi", "from_fn's Display forwards to the closure");
        }
    }
}
