//! The deep half (depth 6 through 9, plus the value-tracking
//! `full_encode_value_tracking` probe) of
//! `chrono_fractional_nanos_full_prefix`, split out once that file
//! grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::gallery::chrono_fractional_nanos_combine_prefix::{
        DigitPrefix, combine_prefix_2, combine_prefix_3, combine_prefix_4, combine_prefix_5,
        combine_prefix_6, combine_prefix_7, combine_prefix_8, combine_prefix_9,
    };
    use crate::gallery::chrono_fractional_nanos_full_prefix::full_prefix_5;


    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<6>| {
        result.value() == nanos / 1_000
            && result.bytes()[5] == b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_6(nanos: i64) -> DigitPrefix<6> {
        let upper = full_prefix_5(nanos);
        let digit = (nanos / 1_000) % 10;
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                b'0' + u8::try_from(digit).unwrap_or(0),
            ], upper.value() * 10 + digit)
    }




    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<7>| {
        result.value() == nanos / 100
            && result.bytes()[6] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_7(nanos: i64) -> DigitPrefix<7> {
        let upper = full_prefix_6(nanos);
        let digit = (nanos / 100) % 10;
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                upper.bytes()[5],
                b'0' + u8::try_from(digit).unwrap_or(0),
            ], upper.value() * 10 + digit)
    }




    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<8>| {
        result.value() == nanos / 10
            && result.bytes()[7] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_8(nanos: i64) -> DigitPrefix<8> {
        let upper = full_prefix_7(nanos);
        let digit = (nanos / 10) % 10;
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                upper.bytes()[5],
                upper.bytes()[6],
                b'0' + u8::try_from(digit).unwrap_or(0),
            ], upper.value() * 10 + digit)
    }




    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<9>| {
        result.value() == nanos
            && result.bytes()[8] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_9(nanos: i64) -> DigitPrefix<9> {
        let upper = full_prefix_8(nanos);
        let digit = nanos % 10;
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                upper.bytes()[5],
                upper.bytes()[6],
                upper.bytes()[7],
                b'0' + u8::try_from(digit).unwrap_or(0),
            ], upper.value() * 10 + digit)
    }




    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<9>| result.value() == nanos)]
    pub(crate) fn full_encode_value_tracking(nanos: i64) -> DigitPrefix<9> {
        let d1 = DigitPrefix::new([b'0' + u8::try_from((nanos / 100_000_000) % 10).unwrap_or(0)], (nanos / 100_000_000) % 10);
        let d2 = combine_prefix_2(
            d1.bytes()[0],
            d1.value(),
            u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0),
        );
        let d3 = combine_prefix_3(d2, u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0));
        let d4 = combine_prefix_4(d3, u8::try_from((nanos / 100_000) % 10).unwrap_or(0));
        let d5 = combine_prefix_5(d4, u8::try_from((nanos / 10_000) % 10).unwrap_or(0));
        let d6 = combine_prefix_6(d5, u8::try_from((nanos / 1_000) % 10).unwrap_or(0));
        let d7 = combine_prefix_7(d6, u8::try_from((nanos / 100) % 10).unwrap_or(0));
        let d8 = combine_prefix_8(d7, u8::try_from((nanos / 10) % 10).unwrap_or(0));
        combine_prefix_9(d8, u8::try_from(nanos % 10).unwrap_or(0))
    }
}

#[cfg(kani)]
pub(crate) use kani_only::full_encode_value_tracking;
#[cfg(kani)]
use kani_only::{full_prefix_6, full_prefix_7, full_prefix_8, full_prefix_9};
#[cfg(kani)]
use crate::gallery::chrono_fractional_nanos_combine_digit_tail::full_encode_param_passing;







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix_deep::full_prefix_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix_deep::check_full_prefix_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 6: stubs full_prefix_5 -- the exact stage count where the old decode_2..9 chain's duplicate-call bug crashed the host.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_6_SRC, {
        #[kani::proof_for_contract(full_prefix_6)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_full_prefix::full_prefix_5)]
        fn check_full_prefix_6() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_6(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix_deep::full_prefix_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix_deep::check_full_prefix_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 7: stubs full_prefix_6.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_7_SRC, {
        #[kani::proof_for_contract(full_prefix_7)]
        #[kani::stub_verified(full_prefix_6)]
        fn check_full_prefix_7() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_7(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix_deep::full_prefix_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix_deep::check_full_prefix_8".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 8: stubs full_prefix_7.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_8_SRC, {
        #[kani::proof_for_contract(full_prefix_8)]
        #[kani::stub_verified(full_prefix_7)]
        fn check_full_prefix_8() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_8(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix_deep::full_prefix_9_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix_deep::check_full_prefix_9".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 9: the real target -- stubs full_prefix_8. value is exact (nanos); the newest byte is direct; earlier bytes ride along unverified in this contract -- a known, deliberate soundness gap to resolve before using this to feed decode.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_9_SRC, {
        #[kani::proof_for_contract(full_prefix_9)]
        #[kani::stub_verified(full_prefix_8)]
        fn check_full_prefix_9() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_9(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix_deep::probe_full_prefix_9_alone_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix_deep::check_probe_full_prefix_9_alone".to_owned(),
            "amenable_kani".to_owned(),
            "isolation probe: full_prefix_9 run for real, no proof_for_contract, no stubbing anywhere in the chain, no assertion beyond discarding the result. The old depth_K recursive chain was confirmed expensive run this way; untested whether this new, value-tracking chain has the same problem.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_PROBE_FULL_PREFIX_9_ALONE_SRC, {
        /// Isolation probe only -- see the registration above.
        #[kani::proof]
        fn check_probe_full_prefix_9_alone() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_9(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix_deep::probe_full_encode_param_passing_alone_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix_deep::check_probe_full_encode_param_passing_alone".to_owned(),
            "amenable_kani".to_owned(),
            "isolation probe: full_encode_param_passing (the non-recursive, leaf-based combine_digit_K chain) run for real, no stubbing, no assertion beyond discarding the result. Unlike full_prefix_9 (recursive), this chain's leaves never call each other -- the caller threads upper through manually. Testing whether that structural difference makes it cheap to run for real, unlike the recursive depth_K/full_prefix_K shape. FLAGGED, NOT PASSING (2026-11-01): a real re-run via the protected `amenable gallery run` CLI times out at the 3m harness timeout, confirmed via the ledger's own earlier-timestamped entry for the pre-split case name -- this predates every file split made this session. Deliberately not re-investigated now -- flagged and deferred until the open cordial worklist clears.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_PROBE_FULL_ENCODE_PARAM_PASSING_ALONE_SRC, {
        /// Isolation probe only -- see the registration above.
        #[kani::proof]
        fn check_probe_full_encode_param_passing_alone() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_encode_param_passing(nanos);
        }
    }
}
