//! Building the real `String` for `KaniFractionalSecondDescriptor`
//! from a digit-buffer, via an unconditional, fixed-length `.push()`
//! sequence -- plus the `KaniFractionalSecondDescriptor` wrapper that
//! composes this into a real `amenable_time::FractionalSecondDescriptor`.
//! Split out of `chrono_fractional_nanos_buffer_string` once that
//! file grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::KaniUtf8Buffer;
    use crate::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_9;

    const DIGIT_BUFFER_MAX_LEN: usize = 9;



    /// Converts a `KaniUtf8Buffer<9>` known (by its own type) to hold
    /// exactly 9 bytes into a real `String`, via an *unconditional*
    /// sequence of exactly 9 `.push()` calls -- no branching, no
    /// symbolic trim length at all. Every prior OOM in this
    /// investigation involved a symbolic-length decision (a loop trip
    /// count, or a match over 10 possible lengths); this is the one
    /// shape never isolated on its own, confirmed cheap (21.3s).
    ///
    /// No contract: `proof_for_contract`'s own DFCC apparatus
    /// (modifies-clause/allocation tracking) was confirmed directly
    /// to explode on this body regardless of what the `ensures`
    /// actually checks (same ~146,000-step OOM with a trivial `|_|
    /// true` ensures as with the real byte-exactness one) -- the
    /// cost is `proof_for_contract` itself, on any function that
    /// performs real heap allocation, not this specific check. The
    /// same body, called plainly with no contract at all, is cheap
    /// (21.3s) -- see `check_buffer_to_string_unconditional`.
    pub(super) fn buffer_to_string_unconditional(
        buffer: KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>,
    ) -> String {
        let bytes = buffer.as_bytes();
        let mut s = String::new();
        s.push(bytes[0] as char);
        s.push(bytes[1] as char);
        s.push(bytes[2] as char);
        s.push(bytes[3] as char);
        s.push(bytes[4] as char);
        s.push(bytes[5] as char);
        s.push(bytes[6] as char);
        s.push(bytes[7] as char);
        s.push(bytes[8] as char);
        s
    }



    /// Diagnostic: identical body to `buffer_to_string_unconditional`,
    /// but with a trivial `ensures` -- isolates whether the
    /// `proof_for_contract`/DFCC apparatus itself (vs. the specific
    /// growing-conjunct check) is what explodes when checked against
    /// this body.
    #[kani::requires(buffer.len() == 9)]
    #[kani::ensures(|_: &String| true)]
    pub(super) fn buffer_to_string_unconditional_trivial_contract(
        buffer: KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>,
    ) -> String {
        let bytes = buffer.as_bytes();
        let mut s = String::new();
        s.push(bytes[0] as char);
        s.push(bytes[1] as char);
        s.push(bytes[2] as char);
        s.push(bytes[3] as char);
        s.push(bytes[4] as char);
        s.push(bytes[5] as char);
        s.push(bytes[6] as char);
        s.push(bytes[7] as char);
        s.push(bytes[8] as char);
        s
    }



    /// A local newtype wrapping the real, foreign `amenable_time::
    /// FractionalSecondDescriptor` -- lets us hand-write `kani::
    /// Arbitrary` for it (the orphan rule blocks implementing
    /// Arbitrary for the foreign type directly, the same reason
    /// `RefinedFixedOffset` wraps `chrono::FixedOffset` instead of
    /// trying to impl Arbitrary on it directly). Its own `any()` goes
    /// through the now-confirmed-cheap chain: the already-proven
    /// digit core (stubbed by whichever harness constructs one of
    /// these), the already-proven buffer accumulation (stubbed), then
    /// the confirmed-cheap unconditional 9-push conversion -- never
    /// the real, symbolic-trim-length `nanos_to_fractional_seconds_
    /// digits`. This wrapper's own domain is narrower than the real
    /// type's (always exactly 9 digits, untrimmed, never shorter) --
    /// honest about that, and sufficient for any proof that only
    /// needs "a real FractionalSecondDescriptor with correct digit
    /// content for some nanos," not the exact trimmed form.
    #[derive(derive_new::new)]
    pub(super) struct KaniFractionalSecondDescriptor(amenable_time::FractionalSecondDescriptor);



    impl KaniFractionalSecondDescriptor {
        pub(super) fn descriptor(&self) -> &amenable_time::FractionalSecondDescriptor {
            &self.0
        }
    }



    impl kani::Arbitrary for KaniFractionalSecondDescriptor {
        fn any() -> Self {
            let nanos: i64 = kani::any();
            kani::assume(nanos > 0 && nanos <= 999_999_999);
            let digits = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let buffer = digit_buffer_depth_9(digits);
            let s = buffer_to_string_unconditional(buffer);
            Self::new(amenable_time::FractionalSecondDescriptor::new(s))
        }
    }



    /// The refinement claim, restated to avoid any second, independent
    /// stub substitution: takes `digits` as a parameter directly
    /// (matching `combine_digit_K`'s own parameter-passing shape),
    /// never recomputing it via a fresh call to `nanos_to_fractional_
    /// digit_bytes` inside the `ensures`. Composes transitively with
    /// that function's own, separately-proven contract (`digits` is
    /// already known, by the caller, to decode to some `nanos`) --
    /// never re-deriving that fact here, never calling `fractional_
    /// seconds_digits_to_nanos` at all.
    ///
    /// No contract: `proof_for_contract` explodes on any function
    /// returning a heap-allocating type, confirmed directly on
    /// `buffer_to_string_unconditional`'s own body regardless of the
    /// `ensures` content. This function's output wraps a real
    /// `String` too, so it inherits the same wall. Checked instead by
    /// a plain `#[kani::proof]` with a direct `assert!`, the same
    /// pattern `fractional_digit_round_trip` already uses for the
    /// same reason.
    pub(super) fn digit_bytes_to_kani_fractional_second_descriptor(
        digits: [u8; 9],
    ) -> KaniFractionalSecondDescriptor {
        let buffer = digit_buffer_depth_9(digits);
        let s = buffer_to_string_unconditional(buffer);
        KaniFractionalSecondDescriptor::new(amenable_time::FractionalSecondDescriptor::new(s))
    }
}

#[cfg(kani)]
use kani_only::{
    KaniFractionalSecondDescriptor, buffer_to_string_unconditional,
    buffer_to_string_unconditional_trivial_contract,
    digit_bytes_to_kani_fractional_second_descriptor,
};
#[cfg(kani)]
use crate::KaniUtf8Buffer;
#[cfg(kani)]
use crate::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_9;





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_descriptor::buffer_to_string_unconditional".to_owned(),
            "gallery::chrono_fractional_nanos_descriptor::check_buffer_to_string_unconditional".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 19: convert a known-9-byte KaniUtf8Buffer to a real String via an unconditional sequence of exactly 9 pushes -- no symbolic trim length, no branching. Isolates whether this specific shape (never tested before) is cheap.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_BUFFER_TO_STRING_UNCONDITIONAL_SRC, {
        #[kani::proof]
        #[kani::stub_verified(digit_buffer_depth_9)]
        fn check_buffer_to_string_unconditional() {
            let digits: [u8; 9] = kani::any();
            let buffer = digit_buffer_depth_9(digits);
            let _ = buffer_to_string_unconditional(buffer);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_descriptor::buffer_to_string_unconditional_trivial_contract".to_owned(),
            "gallery::chrono_fractional_nanos_descriptor::check_buffer_to_string_unconditional_trivial_contract".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 19 diagnostic: identical body to buffer_to_string_unconditional, trivial ensures. Isolates whether proof_for_contract's own DFCC apparatus (vs. the specific growing-conjunct check) is the cost.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_BUFFER_TO_STRING_UNCONDITIONAL_TRIVIAL_CONTRACT_SRC, {
        #[kani::proof_for_contract(buffer_to_string_unconditional_trivial_contract)]
        fn check_buffer_to_string_unconditional_trivial_contract() {
            let buffer: KaniUtf8Buffer<9> = kani::any();
            kani::assume(buffer.len() == 9);
            let _ = buffer_to_string_unconditional_trivial_contract(buffer);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_descriptor::buffer_to_string_unconditional_single_byte".to_owned(),
            "gallery::chrono_fractional_nanos_descriptor::check_buffer_to_string_unconditional_single_byte".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 19 diagnostic: compares exactly ONE byte (index 0 only) between the result String and the source buffer, no length check, no other indices. Isolates whether even a single comparison is enough to explode, before testing all 9.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_BUFFER_TO_STRING_UNCONDITIONAL_SINGLE_BYTE_SRC, {
        #[kani::proof]
        #[kani::stub_verified(digit_buffer_depth_9)]
        fn check_buffer_to_string_unconditional_single_byte() {
            let digits: [u8; 9] = kani::any();
            let buffer = digit_buffer_depth_9(digits);
            let bb = buffer.as_bytes();
            let b0 = bb[0];
            let s = buffer_to_string_unconditional(buffer);
            let rb = s.as_bytes();
            assert_eq!(rb[0], b0);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_descriptor::buffer_to_string_unconditional_byte_exact".to_owned(),
            "gallery::chrono_fractional_nanos_descriptor::check_buffer_to_string_unconditional_byte_exact".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 19: buffer_to_string_unconditional's byte-exactness, checked via a plain proof + direct assert! (not proof_for_contract, confirmed to explode on this body regardless of ensures content). Same pattern as fractional_digit_round_trip.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_BUFFER_TO_STRING_UNCONDITIONAL_BYTE_EXACT_SRC, {
        #[kani::proof]
        #[kani::stub_verified(digit_buffer_depth_9)]
        fn check_buffer_to_string_unconditional_byte_exact() {
            let digits: [u8; 9] = kani::any();
            let buffer = digit_buffer_depth_9(digits);
            let bb = buffer.as_bytes();
            let s = buffer_to_string_unconditional(buffer);
            let rb = s.as_bytes();
            assert_eq!(rb.len(), 9, "unconditional conversion always produces 9 bytes");
            assert_eq!(rb[0], bb[0]);
            assert_eq!(rb[1], bb[1]);
            assert_eq!(rb[2], bb[2]);
            assert_eq!(rb[3], bb[3]);
            assert_eq!(rb[4], bb[4]);
            assert_eq!(rb[5], bb[5]);
            assert_eq!(rb[6], bb[6]);
            assert_eq!(rb[7], bb[7]);
            assert_eq!(rb[8], bb[8]);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_descriptor::kani_fractional_second_descriptor_any_is_cheap".to_owned(),
            "gallery::chrono_fractional_nanos_descriptor::check_kani_fractional_second_descriptor_any".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 19: KaniFractionalSecondDescriptor's own Arbitrary::any() is cheap, stubbing both nanos_to_fractional_digit_bytes and digit_buffer_depth_9. If cheap, any function returning this wrapper can itself be stub_verified elsewhere.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_KANI_FRACTIONAL_SECOND_DESCRIPTOR_ANY_SRC, {
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        #[kani::stub_verified(digit_buffer_depth_9)]
        fn check_kani_fractional_second_descriptor_any() {
            let _: KaniFractionalSecondDescriptor = kani::any();
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_descriptor::digit_bytes_to_kani_fractional_second_descriptor_refines".to_owned(),
            "gallery::chrono_fractional_nanos_descriptor::check_digit_bytes_to_kani_fractional_second_descriptor_refines".to_owned(),
            "amenable_kani".to_owned(),
            "FLAGGED, NOT PASSING (2026-11-01): a real re-run (via the protected `amenable gallery run` CLI, not raw cargo kani) OOMs -- CBMC exits cleanly with status 6, not a crash, but a genuine resource exhaustion. This expectation previously said `Passed`; that was never re-verified and did not hold up. Likely the same root cause already confirmed elsewhere in this investigation: reading anything back from a push-built String (here, via `.descriptor().digits().as_bytes()` on the wrapped `FractionalSecondDescriptor`) is the expensive operation, independent of how the field is accessed. Deliberately not fixed now -- flagged and deferred so cordial cleanup work doesn't turn into open-ended proof-architecture work.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BYTES_TO_KANI_FRACTIONAL_SECOND_DESCRIPTOR_REFINES_SRC, {
        #[kani::proof]
        #[kani::stub_verified(digit_buffer_depth_9)]
        fn check_digit_bytes_to_kani_fractional_second_descriptor_refines() {
            let digits: [u8; 9] = kani::any();
            let wrapper = digit_bytes_to_kani_fractional_second_descriptor(digits);
            let rb = wrapper.descriptor().digits().as_bytes();
            assert_eq!(rb.len(), 9);
            assert_eq!(rb[0], digits[0]);
            assert_eq!(rb[1], digits[1]);
            assert_eq!(rb[2], digits[2]);
            assert_eq!(rb[3], digits[3]);
            assert_eq!(rb[4], digits[4]);
            assert_eq!(rb[5], digits[5]);
            assert_eq!(rb[6], digits[6]);
            assert_eq!(rb[7], digits[7]);
            assert_eq!(rb[8], digits[8]);
        }
    }
}
