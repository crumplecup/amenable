::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector".to_owned(),
            "gallery::replace_recommendations::collections_and_foreign_boundary_timeouts::from_utf8_error_times_out_even_for_a_fixed_two_byte_invalid_vector".to_owned(),
            "amenable_kani".to_owned(),
            "direct String::from_utf8 error recovery can still time out for a tiny fixed invalid vector".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
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
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::btree_map_symbolic_iteration_times_out_in_the_direct_std_path".to_owned(),
            "gallery::replace_recommendations::collections_and_foreign_boundary_timeouts::btree_map_symbolic_iteration_times_out_in_the_direct_std_path".to_owned(),
            "amenable_kani".to_owned(),
            "direct BTreeMap symbolic iteration can still time out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
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
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::linked_list_extract_if_times_out_even_with_incremental_observation".to_owned(),
            "gallery::replace_recommendations::collections_and_foreign_boundary_timeouts::linked_list_extract_if_times_out_even_with_incremental_observation".to_owned(),
            "amenable_kani".to_owned(),
            "direct LinkedList::extract_if can still time out even without materialization".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
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
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::backtrace_force_capture_reaches_unsupported_foreign_boundary".to_owned(),
            "gallery::replace_recommendations::collections_and_foreign_boundary_timeouts::backtrace_force_capture_reaches_unsupported_foreign_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "direct backtrace capture reaches an unsupported foreign boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
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
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::borrowed_fd_clone_reaches_unsupported_fcntl_boundary".to_owned(),
            "gallery::replace_recommendations::collections_and_foreign_boundary_timeouts::borrowed_fd_clone_reaches_unsupported_fcntl_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "borrowed-fd cloning reaches an unsupported fcntl boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BORROWED_FD_CLONE_REACHES_UNSUPPORTED_FCNTL_BOUNDARY_SRC, {
        /// This is the reduced form behind the refined Unix `OwnedFd`
        /// replacement review: the ownership-transfer claim itself is small,
        /// but seeding it from a live descriptor clone reaches `fcntl`
        /// through `BorrowedFd::try_clone_to_owned` before the property can
        /// be established.
        ///
        /// The final assertion calls `NonNegativeFd::ensures` directly
        /// (`fd_model.rs`) rather than restating the comparison.
        #[kani::proof]
        fn borrowed_fd_clone_reaches_unsupported_fcntl_boundary() {
            use std::os::unix::io::{AsFd, AsRawFd};

            let stdout = std::io::stdout();
            let owned = stdout.as_fd().try_clone_to_owned().unwrap();

            assert!(
                <crate::NonNegativeFd as amenable_core::Ensures<crate::KaniVerifier>>::ensures(
                    owned.as_raw_fd()
                ),
                "cloned owned fd should stay live"
            );
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::replace_recommendations::anonymous_pipe_creation_reaches_unsupported_pipe2_boundary".to_owned(),
            "gallery::replace_recommendations::collections_and_foreign_boundary_timeouts::anonymous_pipe_creation_reaches_unsupported_pipe2_boundary".to_owned(),
            "amenable_kani".to_owned(),
            "anonymous pipe creation reaches an unsupported pipe2 boundary".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ANONYMOUS_PIPE_CREATION_REACHES_UNSUPPORTED_PIPE2_BOUNDARY_SRC, {
        /// This is the reduced form behind the refined `PipeReader` /
        /// `PipeWriter` replacement review: the delivery claim is reasonable,
        /// but the direct `std::io::pipe()` setup already reaches `pipe2`
        /// before any read/write property can be established.
        ///
        /// Both assertions call `NonNegativeFd::ensures` directly
        /// (`fd_model.rs`) rather than restating the comparison.
        #[kani::proof]
        fn anonymous_pipe_creation_reaches_unsupported_pipe2_boundary() {
            use std::os::fd::AsRawFd;

            let (reader, writer) = std::io::pipe().unwrap();

            assert!(
                <crate::NonNegativeFd as amenable_core::Ensures<crate::KaniVerifier>>::ensures(
                    reader.as_raw_fd()
                ),
                "reader end should stay live"
            );
            assert!(
                <crate::NonNegativeFd as amenable_core::Ensures<crate::KaniVerifier>>::ensures(
                    writer.as_raw_fd()
                ),
                "writer end should stay live"
            );
        }
    }
}
