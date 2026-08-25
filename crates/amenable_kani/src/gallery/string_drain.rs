//! Gallery cases for `String::drain(..)` observation strategy under Kani.
//!
//! The production `Drain<'static>` witness needs a verifier-friendly way to
//! observe that draining yields the string's content and leaves the source
//! empty. The direct `collect::<String>()` observation timed out even after we
//! bounded the symbolic string to a single ASCII character, so we preserve that
//! failure mode here and keep the lighter iterator-step observation beside it.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_drain::single_char_collect_times_out".to_owned(),
            "gallery::string_drain::single_char_collect_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "collecting a drained single-character string still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::harness! {
    kani, SINGLE_CHAR_COLLECT_TIMES_OUT_SRC, {
        /// Even when the symbolic string is forced down to one ASCII code
        /// point, materializing the full drained iterator back into a `String`
        /// still pushes Kani into an intractable stdlib path.
        #[kani::proof]
        fn single_char_collect_times_out() {
            let mut s = <String as crate::KaniCompose>::kani_depth1();
            let expected = s.clone();
            let drained: String = s.drain(..).collect();
            assert_eq!(drained, expected, "drain yields the string's full content");
            assert!(s.is_empty(), "drain leaves the string empty");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_drain::single_char_incremental_next_passes".to_owned(),
            "gallery::string_drain::single_char_incremental_next_passes".to_owned(),
            "amenable_kani".to_owned(),
            "stepwise observation makes single-character drain verifier-friendly".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::harness! {
    kani, SINGLE_CHAR_INCREMENTAL_NEXT_PASSES_SRC, {
        /// The semantic claim is the same as the timed-out case, but we only
        /// observe the drained iterator one step at a time and then confirm the
        /// source string becomes empty after the iterator is dropped.
        #[kani::proof]
        fn single_char_incremental_next_passes() {
            let mut s = <String as crate::KaniCompose>::kani_depth1();
            let expected = s.chars().next().expect("kani_depth1 builds one character");

            let mut drained = s.drain(..);
            assert_eq!(
                drained.next(),
                Some(expected),
                "the first drained item matches the source character"
            );
            assert_eq!(drained.next(), None, "the single-character drain then exhausts");
            drop(drained);

            assert!(s.is_empty(), "drain leaves the source string empty");
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_drain::symbolic_utf8_observation_times_out".to_owned(),
            "gallery::string_drain::symbolic_utf8_observation_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "symbolic utf8 drain observation still times out after replacing the std iterator".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::harness! {
    kani, SYMBOLIC_UTF8_OBSERVATION_TIMES_OUT_SRC, {
        /// Replacing the native `String::drain(..)` iterator with an
        /// Amenable-owned observation is not sufficient on its own: asking
        /// Kani to quantify that observation over symbolic UTF-8 strings still
        /// expands into enough symbolic string machinery to time out. The
        /// production proof therefore works from the smaller
        /// `KaniUtf8Buffer<2>` witness instead.
        #[kani::proof]
        fn symbolic_utf8_observation_times_out() {
            fn assert_full_drain(source: crate::KaniUtf8String) {
                let expected = source.clone();
                let bytes = source.into_bytes();
                let len = bytes.len();
                let mut bounded = [0u8; 2];
                if len >= 1 {
                    bounded[0] = bytes[0];
                }
                if len >= 2 {
                    bounded[1] = bytes[1];
                }

                let buffer = crate::KaniUtf8Buffer::<2>::new(bounded, len)
                    .expect("symbolic utf8 compose always yields a bounded valid buffer");
                let observation = crate::KaniStringDrainObservation::new(buffer);
                let yielded = observation.yielded_bytes();

                assert_eq!(
                    yielded.len(),
                    len,
                    "drain yields a slice with the source byte length"
                );
                if len >= 1 {
                    assert_eq!(yielded[0], expected.as_bytes()[0], "drain preserves the first byte");
                }
                if len >= 2 {
                    assert_eq!(yielded[1], expected.as_bytes()[1], "drain preserves the second byte");
                }
                assert_eq!(
                    observation.source_len_after_drain(),
                    0,
                    "drain leaves the source length at zero"
                );
                assert!(observation.source_is_empty(), "drain leaves the source empty");
            }

            assert_full_drain(<crate::KaniUtf8String as crate::KaniCompose>::kani_depth0());
            assert_full_drain(<crate::KaniUtf8String as crate::KaniCompose>::kani_depth1());
            assert_full_drain(<crate::KaniUtf8String as crate::KaniCompose>::kani_depth2());
        }
    }
}
