//! Hypothesis tests for the full encode-decode round trip on
//! `amenable_ext`'s fractional-nanosecond digit core -- `nanos ->
//! [u8; 9] -> nanos` recovers the original value.
//!
//! A plain `#[kani::proof]` calling both real functions directly
//! (`amenable_kani::chrono::offset_date_time::
//! check_nanos_to_fractional_digit_bytes`) times out. Isolating each
//! half separately found why: decode (a flat `for` loop) is cheap as
//! a plain, fully real call, but encode alone -- `nanos_to_fractional_
//! digit_bytes`'s real, unstubbed 9-level *recursive* call chain
//! through `fractional_digit_bytes_depth_9 -> depth_8 -> ... ->
//! depth_1` -- already times out on its own, with no assertion at all
//! beyond discarding the result. Decode's own loop and encode's own
//! recursion are both bounded at 9, but Kani's symbolic execution
//! treats them very differently.
//!
//! Confirmed in isolation: `nanos_to_fractional_digit_bytes` alone
//! (its real, unstubbed recursive call into `fractional_digit_bytes_
//! depth_9`, with no decode and no assertion beyond discarding the
//! result) already times out as a plain `#[kani::proof]`. Contrast
//! with `encode_flat_loop_alone_is_cheap` below: the identical
//! arithmetic, rewritten as a flat loop instead of 9 recursive calls,
//! passes immediately. The recursion itself -- not the arithmetic, not
//! an assertion, not contract machinery -- is the cost.
//!
//! Two candidate fixes, tested here before touching production code:
//!
//! 1. **Peel and stub**: keep encode's existing staged, recursive
//!    structure (its own `depth_1..9` contracts are each already
//!    proven), but never let the real recursion run inside a plain
//!    proof -- `#[kani::stub_verified]` works on any harness, not
//!    only `#[kani::proof_for_contract]` ones, so a plain proof can
//!    stub `fractional_digit_bytes_depth_9` directly and still check
//!    a manual postcondition against decode's real loop.
//! 2. **Loop rewrite**: give encode a flat-loop body (mirroring
//!    decode's own shape) instead of 9 recursive function calls, on
//!    the hypothesis that the recursion itself -- not anything about
//!    the arithmetic -- is what Kani finds expensive.

mod kani_only {
    #![cfg(kani)]


    /// Hypothesis 2's reproduction: encode's identical arithmetic,
    /// rewritten as a flat loop instead of 9 nested recursive calls.
    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    pub(super) fn encode_flat_loop(nanos: i64) -> [u8; 9] {
        let mut digits = [b'0'; 9];
        let mut remaining = nanos;
        for digit in digits.iter_mut().rev() {
            *digit = b'0' + u8::try_from(remaining % 10).unwrap_or(0);
            remaining /= 10;
        }
        digits
    }
}

#[cfg(kani)]
use kani_only::encode_flat_loop;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_round_trip::peel_and_stub_round_trip".to_owned(),
            "gallery::chrono_fractional_nanos_round_trip::check_peel_and_stub_round_trip".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: #[kani::stub_verified] on depth_9 runs fast (no timeout), but genuinely fails -- depth_9's own ensures only constrains `value` and the newest byte (`bytes[8]`); positions 0..7 ride along unconstrained by the stub itself, so the stubbed array can hold non-digit garbage there, and decode's real arithmetic over that garbage does not recover `nanos`. A sound stub for this purpose would need depth_9's own ensures to be value-exact over every position, which reopens the array-position-check cost this whole investigation has been avoiding. Confirmed safe to re-run: fails fast, no timeout.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_PEEL_AND_STUB_ROUND_TRIP_SRC, {
        /// Hypothesis 1: stub encode's recursive core, run decode for
        /// real, check the round trip manually.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_9)]
        fn check_peel_and_stub_round_trip() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let bytes = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
            let decoded = amenable_ext::fractional_digit_bytes_to_nanos(&bytes);
            assert_eq!(decoded, nanos, "decode recovers the encoded value exactly");
        }
    }
}



::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_round_trip::encode_flat_loop_alone_is_cheap".to_owned(),
            "gallery::chrono_fractional_nanos_round_trip::check_encode_flat_loop_alone".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 2: encode's own arithmetic, rewritten as a flat loop instead of a 9-level recursive call chain, run alone as a plain proof with no assertion beyond discarding the result -- isolates whether the recursion itself (not the arithmetic) was the real cost.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_ENCODE_FLAT_LOOP_ALONE_SRC, {
        /// Hypothesis 2's isolation probe: the loop-rewritten encode,
        /// alone, no decode, no assertion.
        #[kani::proof]
        fn check_encode_flat_loop_alone() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = encode_flat_loop(nanos);
        }
    }
}



::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_round_trip::loop_based_round_trip".to_owned(),
            "gallery::chrono_fractional_nanos_round_trip::check_loop_based_round_trip".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: both halves real and unstubbed, both shaped as flat loops -- encode_flat_loop and fractional_digit_bytes_to_nanos's decode loop are each confirmed cheap alone, but chaining them for real in one proof times out anyway. Confirms the 'chaining two real calls compounds cost even when each is individually cheap' finding (already documented on offset_round_trips) generalizes to loop-shaped code, not just recursive/builder-shaped code. Confirmed safe to re-run: stays memory-bounded.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_LOOP_BASED_ROUND_TRIP_SRC, {
        /// The composed claim: both halves real, both loop-shaped, no
        /// stubbing at all.
        #[kani::proof]
        fn check_loop_based_round_trip() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let bytes = encode_flat_loop(nanos);
            let decoded = amenable_ext::fractional_digit_bytes_to_nanos(&bytes);
            assert_eq!(decoded, nanos, "decode recovers the loop-encoded value exactly");
        }
    }
}

