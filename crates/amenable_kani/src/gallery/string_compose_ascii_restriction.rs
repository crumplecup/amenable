//! `amenable_kani::compose::model`'s `KaniCompose for String` restricts
//! every symbolic char its `kani_depth1`/`kani_depth2`/`kani_any` build
//! to ASCII (`symbolic_ascii_char`: `byte < 128` then `byte as char`),
//! with no doc comment acknowledging the restriction. Any claim proven
//! using those methods has actually only been proven for the ASCII
//! subset -- every multi-byte UTF-8 codepoint (Latin-1 supplement, CJK,
//! emoji, anything above U+007F) is untouched, silently. Flagged
//! directly: grepping every real (non-self-test) use of `String`'s own
//! `KaniCompose` depth/any methods in this codebase found exactly one
//! (`crates/amenable_kani/src/ledger/mirror.rs`'s `Account` impl), and
//! it explicitly bypasses them in favor of `String::new()` for cost
//! reasons -- so no currently-accepted proof is unsound from this today,
//! but the restriction is silent, undocumented, and anyone building a
//! new proof on `kani_any()`/`kani_depth1/2()` for `String` would not
//! know their claim only covers ASCII.
//!
//! This isolates whether the restriction was ever load-bearing for
//! cost, or just an untested assumption: the symbolic `char` it builds
//! is only ever consumed via `.to_string()`/`.push()` (`char::
//! encode_utf8`, a fixed-bound branch on code-point range, not the
//! unbounded `run_utf8_validation` loop this gallery's
//! `string_drop_vacuity`/`boxed_error_variant_count` cases already
//! confirmed is expensive). A/B: the current ASCII-byte-cast
//! construction against `kani::any::<char>()` over the full Unicode
//! domain, same downstream use (`.to_string()`, length check).
//!
//! **What was actually confirmed:** the ASCII-byte-cast construction
//! passes in 0.83s (275 checks); `kani::any::<char>()` over the full
//! Unicode domain passes in 0.95s (338 checks) -- essentially the same
//! cost. The ASCII restriction was never load-bearing for performance.
//!
//! **Fixed.** `amenable_kani::compose::model`'s private
//! `symbolic_ascii_char()` helper (used only by `KaniCompose for
//! String`'s `kani_depth1`/`kani_depth2`/`kani_any`) was renamed to
//! `symbolic_char()` and simplified to `symbolic_any()` (i.e.
//! `kani::any::<char>()`), closing the soundness gap this file exists
//! to document.
//!
//! That surfaced a second, genuine bug the fix exposed rather than
//! caused: `compose/proofs.rs`'s self-tests asserted `String::
//! kani_depth1().len() == 1` -- true only because every ASCII char is
//! exactly one byte. A multi-byte char now makes that assertion false
//! in general (confirmed directly: CBMC found the counterexample).
//! The honest fix is not `.chars().count()` either -- confirmed
//! directly that `core::str::count::char_count_general_case` has its
//! own unbounded-looking loop, timing out even over 1-2 chars, the
//! same category of cost as this gallery's own `run_utf8_validation`
//! finding. `.len()` (byte count) stays cheap (O(1) stored field), so
//! the three affected self-tests
//! (`verify_kani_compose_string_depths`,
//! `verify_derive_kani_compose_struct_shapes`,
//! `verify_derive_kani_compose_enum_is_bounded`) were rewritten to
//! assert the true, cheaply-checkable bound instead of a now-false
//! exact match: at most 4 UTF-8 bytes per `char`
//! (`ComposeAnyLengthIsBounded`'s `<=`, not `ComposeArrayLengthIsFixed`'s
//! `==`). All three re-confirmed passing: 3.5s/429 checks, 1.5s/431
//! checks, 10.2s/509 checks.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_compose_ascii_restriction::ascii_restricted_one_char".to_owned(),
            "gallery::string_compose_ascii_restriction::check_ascii_restricted_one_char".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 0.83s, 275/275 checks -- the current in-production cost baseline for comparison against the full-Unicode alternative below".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_ASCII_RESTRICTED_ONE_CHAR_SRC, {
        /// The formerly-in-production construction (since fixed, see
        /// this module's doc comment):
        /// `amenable_kani::compose::model::symbolic_ascii_char`,
        /// inlined here rather than calling the private helper
        /// directly.
        #[kani::proof]
        fn check_ascii_restricted_one_char() {
            let byte: u8 = kani::any();
            kani::assume(byte < 0x80);
            let s = (byte as char).to_string();
            assert_eq!(s.len(), 1);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_compose_ascii_restriction::full_unicode_one_char".to_owned(),
            "gallery::string_compose_ascii_restriction::check_full_unicode_one_char".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 0.95s, 338/338 checks -- essentially the same cost as the ASCII-restricted version (~15% more time, more checks because char::encode_utf8's branch structure covers all four UTF-8 length cases instead of just one). The ASCII restriction in amenable_kani::compose::model::symbolic_ascii_char was not load-bearing for cost -- it can be lifted for free, closing a real soundness gap.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_UNICODE_ONE_CHAR_SRC, {
        /// The full-Unicode alternative: a genuinely arbitrary `char`
        /// (Kani's `Arbitrary` impl for `char` respects the type's
        /// own validity invariant -- no surrogates, nothing above
        /// U+10FFFF -- so this is a sound, not merely permissive,
        /// generator).
        #[kani::proof]
        fn check_full_unicode_one_char() {
            let c: char = kani::any();
            let s = c.to_string();
            assert!(!s.is_empty() && s.len() <= 4);
        }
    }
}
