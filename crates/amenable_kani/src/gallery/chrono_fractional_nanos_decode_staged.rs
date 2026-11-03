//! The decode-direction and full-round-trip isolation hypotheses:
//! once encode alone was cheap, does decoding the result -- and the
//! encode+decode round trip together -- stay cheap too, staged the
//! same way (closed-form vs prefix-carrying decode). Split out of
//! `chrono_fractional_nanos_round_trip` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]
}

#[cfg(kani)]
use crate::gallery::chrono_fractional_nanos_contract_cost::plain_full_encode;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_staged::assumed_digits_round_trip_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_staged::check_assumed_digits_round_trip".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 9: skip calling any encode function at all. Directly assume each byte equals its closed-form digit value (9 independent assumptions, no function call, no recursion, no leaf chain), then run the real, unstubbed decode loop and assert the round trip. Isolates whether running *any* encode computation (recursive, leaf-based, or looped) was itself the cost, versus the relational claim between encode and decode. FLAGGED, NOT PASSING (2026-11-01): a real re-run via the protected `amenable gallery run` CLI times out at the 3m harness timeout, both before and after this file's split out of chrono_fractional_nanos_round_trip (confirmed via the ledger's own earlier-timestamped entry for the pre-split case name). Deliberately not re-investigated now -- flagged and deferred until the open cordial worklist clears.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_ASSUMED_DIGITS_ROUND_TRIP_SRC, {
        /// Hypothesis 9: no encode call at all, just direct `assume`
        /// per byte, then real decode.
        #[kani::proof]
        fn check_assumed_digits_round_trip() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let digits: [u8; 9] = kani::any();
            kani::assume(digits[0] == b'0' + u8::try_from((nanos / 100_000_000) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0));
            kani::assume(digits[3] == b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0));
            kani::assume(digits[4] == b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0));
            kani::assume(digits[5] == b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0));
            kani::assume(digits[6] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0));
            kani::assume(digits[7] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0));
            kani::assume(digits[8] == b'0' + u8::try_from(nanos % 10).unwrap_or(0));
            let decoded = amenable_ext::fractional_digit_bytes_to_nanos(&digits);
            assert_eq!(decoded, nanos, "decode recovers the assumed digits exactly");
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_staged::probe_plain_full_encode_alone_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_staged::check_probe_plain_full_encode_alone".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 12: contract-free twin of full_encode_param_passing -- identical leaf-based, non-recursive arithmetic, but the leaves carry no #[kani::requires]/#[kani::ensures] at all. Isolates whether contract attributes themselves add cost to a plainly-called (not stubbed) function, since recursion was ruled out as the variable (full_encode_param_passing has none either, yet was expensive).".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_PROBE_PLAIN_FULL_ENCODE_ALONE_SRC, {
        /// Isolation probe only -- see the registration above.
        #[kani::proof]
        fn check_probe_plain_full_encode_alone() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = plain_full_encode(nanos);
        }
    }
}

