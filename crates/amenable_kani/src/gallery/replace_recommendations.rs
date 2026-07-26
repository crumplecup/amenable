//! Gallery cases for the main `recommendation = "replace"` failure patterns.
//!
//! These cases are intentionally smaller than the production proofs they stand
//! in for. The goal is not to re-prove each library contract here, but to keep
//! an executable record of why certain proof families are poor fits for direct
//! Kani verification in this repository.
//!
//! The current issue classes are:
//!
//! - unsupported foreign boundaries in reachable std implementations
//! - Unix file-descriptor duplication paths that bottom out in `fcntl`
//! - anonymous pipe creation paths that bottom out in `pipe2`
//! - unsupported `#[track_caller]` / `Location::caller()` boundaries
//! - unsupported panic-capture boundaries
//! - Kani environment-model mismatches against real-process invariants
//! - PATH-style helper expansion that still times out in direct std execution
//! - first-pass concrete `String` / `Vec` PATH models that still leak too much
//!   owned-string machinery into Kani
//! - OS-backed filesystem boundaries with real external state
//! - pure in-memory std implementation blow-up that still times out under the
//!   native multi-minute harness timeout (`hash`, `fmt`,
//!   `LinkedList::extract_if`, and similar cases)

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::linked_list_extract_if_times_out_even_with_incremental_observation".to_owned(),
            harness: "gallery::replace_recommendations::linked_list_extract_if_times_out_even_with_incremental_observation".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct LinkedList::extract_if can still time out even without materialization".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, LINKED_LIST_EXTRACT_IF_TIMES_OUT_EVEN_WITH_INCREMENTAL_OBSERVATION_SRC, {
        /// This is the reduced `LinkedList::extract_if` representative:
        /// incremental `next()` observation plus one early-drop remainder
        /// check, with no eager collection. If this still times out, the
        /// issue is the direct std linked-list path rather than proof-side
        /// materialization.
        #[kani::proof]
        fn linked_list_extract_if_times_out_even_with_incremental_observation() {
            fn is_even(x: &mut i32) -> bool {
                *x % 2 == 0
            }

            let mut list = std::collections::LinkedList::from([1, 2, 3, 4]);
            let mut extractor = list.extract_if(is_even as fn(&mut i32) -> bool);
            assert_eq!(extractor.next(), Some(2), "extract_if should yield the first matching element");
            drop(extractor);

            assert_eq!(list.pop_front(), Some(1), "the prefix element should remain in the list");
            assert_eq!(list.pop_front(), Some(3), "the first unvisited non-match should remain");
            assert_eq!(list.pop_front(), Some(4), "the unvisited matching suffix should remain");
            assert_eq!(list.pop_front(), None, "only the visited match should be removed");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::backtrace_force_capture_reaches_unsupported_foreign_boundary".to_owned(),
            harness: "gallery::replace_recommendations::backtrace_force_capture_reaches_unsupported_foreign_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct backtrace capture reaches an unsupported foreign boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, BACKTRACE_FORCE_CAPTURE_REACHES_UNSUPPORTED_FOREIGN_BOUNDARY_SRC, {
        /// This is the reduced form of the backtrace replace issue: the claim
        /// itself is straightforward, but Kani reaches `_Unwind_Backtrace`
        /// before it can establish the property.
        #[kani::proof]
        fn backtrace_force_capture_reaches_unsupported_foreign_boundary() {
            let backtrace = std::backtrace::Backtrace::force_capture();
            assert_eq!(
                backtrace.status(),
                std::backtrace::BacktraceStatus::Captured,
                "forced capture should report a captured backtrace"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::borrowed_fd_clone_reaches_unsupported_fcntl_boundary".to_owned(),
            harness: "gallery::replace_recommendations::borrowed_fd_clone_reaches_unsupported_fcntl_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "borrowed-fd cloning reaches an unsupported fcntl boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, BORROWED_FD_CLONE_REACHES_UNSUPPORTED_FCNTL_BOUNDARY_SRC, {
        /// This is the reduced form behind the refined Unix `OwnedFd`
        /// replacement review: the ownership-transfer claim itself is small,
        /// but seeding it from a live descriptor clone reaches `fcntl`
        /// through `BorrowedFd::try_clone_to_owned` before the property can
        /// be established.
        #[kani::proof]
        fn borrowed_fd_clone_reaches_unsupported_fcntl_boundary() {
            use std::os::unix::io::{AsFd, AsRawFd};

            let stdout = std::io::stdout();
            let owned = stdout.as_fd().try_clone_to_owned().unwrap();

            assert!(owned.as_raw_fd() >= 0, "cloned owned fd should stay live");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::anonymous_pipe_creation_reaches_unsupported_pipe2_boundary".to_owned(),
            harness: "gallery::replace_recommendations::anonymous_pipe_creation_reaches_unsupported_pipe2_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "anonymous pipe creation reaches an unsupported pipe2 boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, ANONYMOUS_PIPE_CREATION_REACHES_UNSUPPORTED_PIPE2_BOUNDARY_SRC, {
        /// This is the reduced form behind the refined `PipeReader` /
        /// `PipeWriter` replacement review: the delivery claim is reasonable,
        /// but the direct `std::io::pipe()` setup already reaches `pipe2`
        /// before any read/write property can be established.
        #[kani::proof]
        fn anonymous_pipe_creation_reaches_unsupported_pipe2_boundary() {
            use std::os::fd::AsRawFd;

            let (reader, writer) = std::io::pipe().unwrap();

            assert!(reader.as_raw_fd() >= 0, "reader end should stay live");
            assert!(writer.as_raw_fd() >= 0, "writer end should stay live");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::location_caller_reaches_unsupported_track_caller_boundary".to_owned(),
            harness: "gallery::replace_recommendations::location_caller_reaches_unsupported_track_caller_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Location::caller reaches an unsupported track_caller boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::catch_unwind_reaches_an_unsupported_panic_boundary".to_owned(),
            harness: "gallery::replace_recommendations::catch_unwind_reaches_an_unsupported_panic_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "catch_unwind reaches an unsupported panic boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::env_args_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            harness: "gallery::replace_recommendations::env_args_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "process-argument invariants can fail under Kani's synthetic model".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::env_args_os_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            harness: "gallery::replace_recommendations::env_args_os_process_invariant_fails_under_the_synthetic_kani_model".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "raw process-argument invariants can fail under Kani's synthetic model".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::join_paths_unjoinable_path_times_out_in_the_direct_std_path".to_owned(),
            harness: "gallery::replace_recommendations::join_paths_unjoinable_path_times_out_in_the_direct_std_path".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct join_paths can still time out even for one fixed unjoinable path".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_direct_std_path".to_owned(),
            harness: "gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_direct_std_path".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct split_paths round trips can still time out on a tiny fixed path list".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_first_concrete_string_model".to_owned(),
            harness: "gallery::replace_recommendations::split_paths_round_trip_times_out_in_the_first_concrete_string_model".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a first-pass concrete PATH string model can still time out under Kani".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::default_hasher_determinism_times_out_in_the_direct_std_path".to_owned(),
            harness: "gallery::replace_recommendations::default_hasher_determinism_times_out_in_the_direct_std_path".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct default-hasher determinism can still time out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario".to_owned(),
            harness: "gallery::replace_recommendations::real_filesystem_boundary_times_out_even_for_a_small_tempdir_scenario".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a small real-filesystem tempdir scenario can still time out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, REAL_FILESYSTEM_BOUNDARY_TIMES_OUT_EVEN_FOR_A_SMALL_TEMPDIR_SCENARIO_SRC, {
        /// This is the reduced filesystem-boundary representative: a single
        /// temp directory, one created file, one metadata observation, and
        /// cleanup. If this still times out, the issue is the real std::fs
        /// path itself rather than a larger production-specific assertion.
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::format_arguments_rendering_times_out_in_the_direct_std_path".to_owned(),
            harness: "gallery::replace_recommendations::format_arguments_rendering_times_out_in_the_direct_std_path".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct fmt::Arguments rendering can still time out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
