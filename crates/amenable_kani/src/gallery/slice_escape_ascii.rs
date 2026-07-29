//! Gallery cases for `std::slice::EscapeAscii` observation strategy under Kani.
//!
//! The production `rust_std::slice::verify_escape_ascii_leaves_printable_bytes_unescaped`
//! proof only needs a bounded claim over a two-byte slice: a printable byte
//! stays verbatim and `\n` expands to `\\n`. Materializing the whole iterator
//! with `collect::<Vec<u8>>()` still times out, and stepwise `next()`
//! observation alone does not rescue the real std iterator path either. We
//! keep both false trails here, then pair them with the bounded Amenable-owned
//! observation the production proof now uses.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::slice_escape_ascii::two_byte_collect_times_out".to_owned(),
            harness: "gallery::slice_escape_ascii::two_byte_collect_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "collecting a two-byte escape_ascii iterator still times out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, TWO_BYTE_COLLECT_TIMES_OUT_SRC, {
        /// Even a fixed `[printable, b'\n']` witness still times out when the
        /// full `EscapeAscii` iterator is eagerly materialized into a `Vec<u8>`.
        #[kani::proof]
        fn two_byte_collect_times_out() {
            let printable: u8 = kani::any();
            kani::assume((0x20..=0x7e).contains(&printable));
            let data = [printable, b'\n'];
            let escaped: Vec<u8> = data.escape_ascii().collect();

            assert_eq!(escaped, vec![printable, b'\\', b'n']);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::slice_escape_ascii::two_byte_incremental_next_still_times_out".to_owned(),
            harness: "gallery::slice_escape_ascii::two_byte_incremental_next_still_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "stepwise next() alone still times out on the same escape_ascii witness".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, TWO_BYTE_INCREMENTAL_NEXT_STILL_TIMES_OUT_SRC, {
        /// Same bounded semantic claim as the timed-out case above, but
        /// observed one yielded byte at a time instead of through eager
        /// collection. This still times out on the real `EscapeAscii`
        /// iterator, so observation style alone is not enough here.
        #[kani::proof]
        fn two_byte_incremental_next_still_times_out() {
            let printable: u8 = kani::any();
            kani::assume((0x20..=0x7e).contains(&printable));
            let data = [printable, b'\n'];
            let mut escaped = data.escape_ascii();

            assert_eq!(escaped.next(), Some(printable));
            assert_eq!(escaped.next(), Some(b'\\'));
            assert_eq!(escaped.next(), Some(b'n'));
            assert_eq!(escaped.next(), None);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::slice_escape_ascii::bounded_escape_ascii_observation_passes".to_owned(),
            harness: "gallery::slice_escape_ascii::bounded_escape_ascii_observation_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "a bounded escape_ascii observation states the same law without the std timeout".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::BestPractice,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, BOUNDED_ESCAPE_ASCII_OBSERVATION_PASSES_SRC, {
        /// Best-practice accommodation: keep the same `[printable, b'\n']`
        /// semantic law, but state it through the Amenable-owned bounded
        /// observation instead of the real `EscapeAscii` iterator path.
        #[kani::proof]
        fn bounded_escape_ascii_observation_passes() {
            let printable: u8 = kani::any();
            kani::assume((0x20..=0x7e).contains(&printable));

            let observation = ::amenable_kani::KaniEscapeAsciiObservation::new(printable);

            assert_eq!(observation.source(), [printable, b'\n']);
            assert_eq!(observation.escaped(), [printable, b'\\', b'n']);
        }
    }
}
