//! Gallery cases for `String::drain(..)` observation strategy under Kani.
//!
//! The production `Drain<'static>` witness needs a verifier-friendly way to
//! observe that draining yields the string's content and leaves the source
//! empty. The direct `collect::<String>()` observation timed out even after we
//! bounded the symbolic string to a single ASCII character, so we preserve that
//! failure mode here and keep the lighter iterator-step observation beside it.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::string_drain::single_char_collect_times_out".to_owned(),
            harness: "gallery::string_drain::single_char_collect_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "collecting a drained single-character string still times out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::string_drain::single_char_incremental_next_passes".to_owned(),
            harness: "gallery::string_drain::single_char_incremental_next_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "stepwise observation makes single-character drain verifier-friendly".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
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
