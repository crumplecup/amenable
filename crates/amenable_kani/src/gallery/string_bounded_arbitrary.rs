//! Tests Kani's own built-in `BoundedArbitrary for String`
//! (`bounded_any::<const N: usize>()`, shipped by Kani itself --
//! `library/kani/src/bounded_arbitrary.rs` in the installed toolchain,
//! not anything built in this codebase): generates a fixed-size
//! `[u8; N]` symbolic stack array, then takes the longest valid UTF-8
//! prefix via `bytes.utf8_chunks().next()`, truncating at the first
//! invalid byte rather than rejecting outright -- no `push`-loop
//! growth, no restriction to ASCII, genuinely arbitrary over every
//! possible byte value Kani can represent, for whatever `N` turns out
//! to be affordable.
//!
//! **That turned out to be only `N=1`.** `Utf8Chunks::next()`
//! (`core::str::lossy`) has its own internal loop handling multi-byte
//! continuation sequences, and it is not cheap over fully symbolic
//! bytes: confirmed directly that `N=2` times out identically to
//! `N=8` (same unwound-117+-iterations signature) -- the same root
//! cause as this gallery's earlier `run_utf8_validation` finding, now
//! confirmed inside Kani's own shipped code, not just this project's.
//! So `bounded_any::<N>()` is not a free, general "genuinely arbitrary
//! String of any small size" tool -- it is specifically a cheap
//! *one-byte* generator. That is still exactly what the string-length
//! induction needs: the induction's small representative doesn't
//! require a single `bounded_any::<N>()` call for `N>1` at all -- it
//! is built by stubbing `bounded_any::<1>()` (confirmed cheap)
//! together with one new symbolic char per step, the same composition
//! shape as the digit chain, never asking Kani to construct a
//! multi-byte bounded-arbitrary string directly -- not yet built as
//! its own gallery case.
//!
//! **What was actually confirmed:**

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_bounded_arbitrary::bounded_any_eight_bytes".to_owned(),
            "gallery::string_bounded_arbitrary::check_bounded_any_eight_bytes".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed TIMES OUT: Utf8Chunks::next()'s internal loop (core::str::lossy, handling multi-byte continuation sequences) unwound 118+ iterations over fully symbolic bytes before the harness timeout. Kani's own official BoundedArbitrary for String isn't free at N=8 -- same root cause as this gallery's earlier run_utf8_validation finding, now confirmed inside Kani's own shipped code.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_BOUNDED_ANY_EIGHT_BYTES_SRC, {
        /// `String::bounded_any::<8>()`: an 8-byte symbolic buffer,
        /// truncated to its longest valid UTF-8 prefix. Only checks
        /// the trivial shape fact (byte length within the declared
        /// bound) -- the point of this harness is cost, not a
        /// substantive claim.
        #[kani::proof]
        fn check_bounded_any_eight_bytes() {
            let s: String = kani::BoundedArbitrary::bounded_any::<8>();
            assert!(s.len() <= 8);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_bounded_arbitrary::bounded_any_one_byte".to_owned(),
            "gallery::string_bounded_arbitrary::check_bounded_any_one_byte".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed PASSES, 0.59s, 311/311 checks".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_BOUNDED_ANY_ONE_BYTE_SRC, {
        /// `String::bounded_any::<1>()`: the smallest possible
        /// nontrivial case, escalating from `bounded_any::<8>()`
        /// (confirmed timeout) to find the real cost boundary.
        #[kani::proof]
        fn check_bounded_any_one_byte() {
            let s: String = kani::BoundedArbitrary::bounded_any::<1>();
            assert!(s.len() <= 1);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::string_bounded_arbitrary::bounded_any_two_bytes".to_owned(),
            "gallery::string_bounded_arbitrary::check_bounded_any_two_bytes".to_owned(),
            "amenable_kani".to_owned(),
            "confirmed TIMES OUT, same Utf8Chunks::next() loop as bounded_any::<8>() (117-119 iterations before timeout). bounded_any::<N>() is only cheap at N=1 -- even N=2 hits the identical wall. Resolution: build the induction's small representative by stubbing bounded_any::<1>() (confirmed cheap) together with one new symbolic char per step, never calling bounded_any::<N> directly for N>1 -- not yet built as its own gallery case.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_BOUNDED_ANY_TWO_BYTES_SRC, {
        /// `String::bounded_any::<2>()`: the size a bounded
        /// representative `s` for the string-length induction's
        /// step would actually need (matching `KaniCompose::
        /// kani_depth2()`'s own shape).
        #[kani::proof]
        fn check_bounded_any_two_bytes() {
            let s: String = kani::BoundedArbitrary::bounded_any::<2>();
            assert!(s.len() <= 2);
        }
    }
}

// N=3 was planned as the next escalation rung but deliberately never
// run: N=2 already confirmed the identical Utf8Chunks::next() wall
// that broke N=8, so N=3 would almost certainly time out the same way
// and add no new information. The real resolution isn't a bigger N --
// it's composing small, cheap N=1 pieces via stub_verified instead of
// ever calling bounded_any::<N> directly for N>1 -- not yet built as
// its own gallery case.
