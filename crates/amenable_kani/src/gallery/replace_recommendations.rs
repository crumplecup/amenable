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
//!   native multi-minute harness timeout (`hash`, `fmt`, `BTree*`,
//!   `LinkedList::extract_if`, `String::from_utf8`, and similar cases)
//! - OS entropy-source boundaries reached by process-randomized seeding
//!   (`RandomState::new()`)
//! - thread-local-storage boundaries reached by `std::thread::current()`
//!   (`pthread_key_create`)
//! - real futex/clock syscall boundaries reached by `Barrier`/`Condvar`
//!   (`futex_wait`, `clock_gettime`)
//! - Kani's no-concurrency-support environment not enforcing real mutual
//!   exclusion for `Mutex::try_lock`
//! - `std::process::Command`/`Child` construction and spawning reaching
//!   several distinct unsupported foreign constructs (`strlen` via
//!   `CString`, `gnu_get_libc_version`, C string literals in `Stdio`)
//! - any `std::net` socket construction (`TcpListener`/`TcpStream`/
//!   `UdpSocket`) reaching an unsupported `socket` syscall

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector".to_owned(),
            harness: "gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct String::from_utf8 error recovery can still time out for a tiny fixed invalid vector".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, FROM_UTF8_ERROR_TIMES_OUT_EVEN_FOR_A_FIXED_TWO_BYTE_INVALID_VECTOR_SRC, {
        /// This is the reduced `FromUtf8Error` representative: no symbolic
        /// prefix, no cloning, and only a fixed two-byte invalid vector. If
        /// this still times out, the issue is the direct owned
        /// `String::from_utf8` / `FromUtf8Error` path rather than a richer
        /// proof-side setup.
        #[kani::proof]
        fn from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector() {
            let err = String::from_utf8(vec![b'x', 0xFFu8]).unwrap_err();

            assert_eq!(
                err.as_bytes(),
                &[b'x', 0xFFu8],
                "as_bytes should recover the original invalid vector"
            );
            assert_eq!(
                err.into_bytes(),
                vec![b'x', 0xFFu8],
                "into_bytes should recover the original invalid vector"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::btree_map_symbolic_iteration_times_out_in_the_direct_std_path".to_owned(),
            harness: "gallery::replace_recommendations::btree_map_symbolic_iteration_times_out_in_the_direct_std_path".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "direct BTreeMap symbolic iteration can still time out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, BTREE_MAP_SYMBOLIC_ITERATION_TIMES_OUT_IN_THE_DIRECT_STD_PATH_SRC, {
        /// This is the reduced `BTreeMap` representative: two symbolic keys,
        /// reverse insertion order, and one observed iteration sequence. If
        /// this still times out, the issue is the direct std B-tree path
        /// itself rather than a richer production-specific assertion.
        #[kani::proof]
        fn btree_map_symbolic_iteration_times_out_in_the_direct_std_path() {
            let k1: i32 = kani::any();
            let k2: i32 = kani::any();
            kani::assume(k1 < k2);
            let v1: i32 = kani::any();
            let v2: i32 = kani::any();

            let mut map = std::collections::BTreeMap::new();
            map.insert(k2, v2);
            map.insert(k1, v1);

            let entries: Vec<(&i32, &i32)> = map.iter().collect();
            assert_eq!(
                entries,
                vec![(&k1, &v1), (&k2, &v2)],
                "BTreeMap iteration should respect ascending key order"
            );
        }
    }
}

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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::random_state_construction_reaches_an_unsupported_entropy_source_boundary".to_owned(),
            harness: "gallery::replace_recommendations::random_state_construction_reaches_an_unsupported_entropy_source_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "RandomState::new() reaches an unsupported OS entropy-source boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, RANDOM_STATE_CONSTRUCTION_REACHES_AN_UNSUPPORTED_ENTROPY_SOURCE_BOUNDARY_SRC, {
        /// This is the reduced form behind the `RandomState` review: the
        /// per-instance determinism claim itself is reasonable (two
        /// hashers built from the *same* instance should agree), but
        /// `RandomState::new()` reaches foreign functions Kani reports as
        /// unsupported (`strlen`, `OwnedFd::drop`'s `close`) before the
        /// claim can be established -- consistent with reading a process
        /// entropy source (e.g. `/dev/urandom` or `getrandom`) to pick its
        /// random per-instance seed, the same class of OS-backed boundary
        /// as the pipe/fd cases above, not a proof-side deficiency.
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::thread_current_reaches_an_unsupported_thread_local_storage_boundary".to_owned(),
            harness: "gallery::replace_recommendations::thread_current_reaches_an_unsupported_thread_local_storage_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "std::thread::current() reaches an unsupported pthread_key_create boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::barrier_wait_reaches_an_unsupported_futex_boundary".to_owned(),
            harness: "gallery::replace_recommendations::barrier_wait_reaches_an_unsupported_futex_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Barrier::wait() reaches an unsupported futex syscall boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::condvar_wait_timeout_reaches_an_unsupported_clock_boundary".to_owned(),
            harness: "gallery::replace_recommendations::condvar_wait_timeout_reaches_an_unsupported_clock_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Condvar::wait_timeout reaches an unsupported clock_gettime boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::try_lock_succeeds_under_kanis_no_concurrency_environment_model".to_owned(),
            harness: "gallery::replace_recommendations::try_lock_succeeds_under_kanis_no_concurrency_environment_model".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Mutex::try_lock succeeds while a guard is held, under Kani's no-concurrency-support environment model".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::command_construction_reaches_an_unsupported_cstring_boundary".to_owned(),
            harness: "gallery::replace_recommendations::command_construction_reaches_an_unsupported_cstring_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Command::new(...).arg(...) reaches an unsupported CString strlen boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, COMMAND_CONSTRUCTION_REACHES_AN_UNSUPPORTED_CSTRING_BOUNDARY_SRC, {
        /// This is the reduced form behind the `Command`-args review: pure
        /// builder introspection with no spawning at all still reaches
        /// `strlen` (via `CString::from_raw`), since `Command`'s Unix
        /// representation converts the program path and arguments to
        /// `CString` unconditionally at construction time. An OS-backed
        /// boundary reached before any spawn-specific claim is even in
        /// play, not a proof-side deficiency.
        #[kani::proof]
        fn command_construction_reaches_an_unsupported_cstring_boundary() {
            let mut command = std::process::Command::new("prog");
            command.arg("a");
            let _: Vec<&std::ffi::OsStr> = command.get_args().collect();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::command_spawn_reaches_an_unsupported_glibc_version_boundary".to_owned(),
            harness: "gallery::replace_recommendations::command_spawn_reaches_an_unsupported_glibc_version_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Command::spawn() reaches an unsupported gnu_get_libc_version boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, COMMAND_SPAWN_REACHES_AN_UNSUPPORTED_GLIBC_VERSION_BOUNDARY_SRC, {
        /// This is the reduced form behind both the `Child`-process-id and
        /// `ExitStatus` reviews: spawning any process at all, regardless of
        /// what it does, reaches `gnu_get_libc_version` (glibc version
        /// detection used to pick a `posix_spawn` vs. `fork`/`exec`
        /// strategy) before any spawn-specific claim can be checked. An
        /// OS/libc-backed boundary, not a proof-side deficiency.
        #[kani::proof]
        fn command_spawn_reaches_an_unsupported_glibc_version_boundary() {
            let _ = std::process::Command::new("true").spawn();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::stdio_conversion_reaches_an_unsupported_c_string_literal_boundary".to_owned(),
            harness: "gallery::replace_recommendations::stdio_conversion_reaches_an_unsupported_c_string_literal_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Stdio::to_child_stdio reaches an unsupported C string literal construct".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, STDIO_CONVERSION_REACHES_AN_UNSUPPORTED_C_STRING_LITERAL_BOUNDARY_SRC, {
        /// This is the reduced form behind the `Output`/`Stdio` reviews:
        /// configuring a piped/null standard stream and spawning reaches a
        /// C string literal construct in `Stdio::to_child_stdio` Kani
        /// reports unsupported, before any output-capture or
        /// handle-discarding claim can be checked.
        #[kani::proof]
        fn stdio_conversion_reaches_an_unsupported_c_string_literal_boundary() {
            let _ = std::process::Command::new("true")
                .stdout(std::process::Stdio::null())
                .spawn();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::replace_recommendations::socket_construction_reaches_an_unsupported_socket_syscall_boundary".to_owned(),
            harness: "gallery::replace_recommendations::socket_construction_reaches_an_unsupported_socket_syscall_boundary".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "TcpListener::bind reaches an unsupported socket() syscall boundary".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
    kani, SOCKET_CONSTRUCTION_REACHES_AN_UNSUPPORTED_SOCKET_SYSCALL_BOUNDARY_SRC, {
        /// This is the reduced form behind every `std::net` review
        /// (`TcpListener`, `TcpStream`, `UdpSocket`, `Incoming`,
        /// shutdown): whatever the specific claim, constructing any socket
        /// at all reaches the `socket()` syscall Kani reports unsupported,
        /// before any connect/accept/send/recv-specific claim can be
        /// checked. An OS-backed networking boundary, not a proof-side
        /// deficiency -- confirmed identical across all five production
        /// proofs in this review pass.
        #[kani::proof]
        fn socket_construction_reaches_an_unsupported_socket_syscall_boundary() {
            let _ = std::net::TcpListener::bind("127.0.0.1:0");
        }
    }
}
