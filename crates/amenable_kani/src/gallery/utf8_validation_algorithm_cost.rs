//! Gallery cases isolating why `utf8_model::is_valid_utf8` blows up on an
//! all-valid input, and why the fix is to stop running it under Kani at
//! all rather than to change its input representation.
//!
//! Attempting to migrate `rust_std::primitives::verify_string_utf8_valid`
//! and `rust_std::std_ffi::verify_os_str_valid_utf8_content_round_trips_through_to_str`
//! onto `utf8_model` surfaced a genuine timeout: `is_valid_utf8`'s full
//! multi-byte UTF-8 state machine (checking 1/2/3/4-byte lead-byte
//! patterns, continuation bytes, overlong encodings, and surrogates)
//! times out even for a fixed two-byte all-ASCII input, which must run the
//! loop to completion with no early `return`.
//!
//! The first hypothesis -- that this was the same "runtime-length `&[u8]`
//! slice vs. compile-time-bounded `[u8; N]` array" issue found in
//! `iter_materialization`/`slice_split_position` -- was tested directly and
//! **disproven**: a const-generic `for i in 0..N` rewrite of the exact same
//! full branch logic times out just as badly as the original `&[u8]`
//! version. A *simplified* single-branch ASCII-only check (`byte &
//! 0b1000_0000 != 0`) does resolve quickly in both forms, which is why the
//! first (retracted) version of this gallery module mistakenly attributed
//! the fix to array-vs-slice representation -- it had accidentally tested a
//! different, much cheaper function instead of reproducing the real one.
//!
//! The actual fix, informed by `~/repos/elicitation`'s
//! `verification::types::Utf8Bytes<MAX_LEN>` prior art: don't ask Kani to
//! run the validation state machine to completion at all. Treat "is this
//! valid UTF-8" as an assumed symbolic `bool` and verify only the
//! *wrapper's* bookkeeping (length, emptiness, byte recovery) conditional
//! on that assumption -- see `utf8_model::KaniUtf8Buffer`, which is what
//! the two production proofs above actually use.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::utf8_validation_algorithm_cost::full_branch_validation_over_two_valid_bytes_times_out".to_owned(),
            harness: "gallery::utf8_validation_algorithm_cost::full_branch_validation_over_two_valid_bytes_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "the full multi-byte UTF-8 validation state machine times out even for two fixed valid bytes".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, FULL_BRANCH_VALIDATION_OVER_TWO_VALID_BYTES_TIMES_OUT_SRC, {
        /// `utf8_model::KaniUtf8::is_valid` (the real 1/2/3/4-byte UTF-8
        /// state machine) over a fixed two-byte all-ASCII input -- must run
        /// its loop to completion since neither byte triggers an early
        /// `return false`.
        #[kani::proof]
        fn full_branch_validation_over_two_valid_bytes_times_out() {
            let a: u8 = kani::any();
            let b: u8 = kani::any();
            kani::assume(a < 0x80 && b < 0x80);
            let bytes: [u8; 2] = [a, b];
            assert!(crate::KaniUtf8::is_valid(&bytes));
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::utf8_validation_algorithm_cost::assumed_symbolic_validity_over_the_same_case_passes".to_owned(),
            harness: "gallery::utf8_validation_algorithm_cost::assumed_symbolic_validity_over_the_same_case_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "assuming validity symbolically instead of computing it resolves the same case immediately".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::BestPractice,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, ASSUMED_SYMBOLIC_VALIDITY_OVER_THE_SAME_CASE_PASSES_SRC, {
        /// Same two symbolic bytes as the timeout control above, but
        /// validity is assumed (`kani::any()`) rather than computed by
        /// running the state machine -- the `KaniUtf8Buffer` strategy.
        /// This does not re-verify the validation algorithm's own
        /// correctness (that's `is_valid_utf8`'s concern, exercised outside
        /// Kani); it verifies the wrapper's bookkeeping conditional on an
        /// assumed validity outcome. Compared index-by-index rather than
        /// via slice equality, which has its own symbolic-length `memcmp`
        /// cost (see `gallery::slice_split_position`).
        #[kani::proof]
        fn assumed_symbolic_validity_over_the_same_case_passes() {
            let bytes: [u8; 2] = kani::any();
            let len: usize = kani::any();
            kani::assume(len <= 2);

            if let Ok(buffer) = crate::KaniUtf8Buffer::<2>::new(bytes, len) {
                let recovered = buffer.as_bytes();
                assert_eq!(recovered.len(), len);
                if len >= 1 {
                    assert_eq!(recovered[0], bytes[0]);
                }
                if len >= 2 {
                    assert_eq!(recovered[1], bytes[1]);
                }
            }
        }
    }
}
