//! Decode-direction isolation: `decode_prefix_K`, the prefix-value-
//! carrying staged decode variant. Split out of
//! `chrono_fractional_nanos_decode_staged` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]




    // ── Hypothesis 11: decode's own accumulation, staged exactly
    // like truncate_stage_K -- one new fact per stage, never
    // restating earlier bytes, never calling the previous stage from
    // the ensures. `prefix_value` is a ghost parameter (the value
    // this K-byte prefix represents); each stage's own `requires`
    // states only the *newest* byte's own constraint -- the earlier
    // bytes' own correctness is pulled in transitively through the
    // stubbed call's own assumed precondition, exactly the way
    // truncate_stage_K never needed to restate earlier digits either.

    #[kani::requires(prefix_value >= 0 && prefix_value <= 9 && digits[0] == b'0' + u8::try_from(prefix_value).unwrap_or(0))]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_1(digits: &[u8; 1], prefix_value: i64) -> i64 {
        i64::from(digits[0]) - i64::from(b'0')
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 99
            && digits[0] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_2(digits: &[u8; 2], prefix_value: i64) -> i64 {
        let upper = decode_prefix_1(&[digits[0]], prefix_value / 10);
        upper * 10 + (i64::from(digits[1]) - i64::from(b'0'))
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 999
            && digits[0] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[2] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_3(digits: &[u8; 3], prefix_value: i64) -> i64 {
        let upper = decode_prefix_2(&[digits[0], digits[1]], prefix_value / 10);
        upper * 10 + (i64::from(digits[2]) - i64::from(b'0'))
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 9_999
            && digits[0] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0)
            && digits[2] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[3] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_4(digits: &[u8; 4], prefix_value: i64) -> i64 {
        let upper = decode_prefix_3(&[digits[0], digits[1], digits[2]], prefix_value / 10);
        upper * 10 + (i64::from(digits[3]) - i64::from(b'0'))
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 99_999
            && digits[0] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0)
            && digits[2] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0)
            && digits[3] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[4] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_5(digits: &[u8; 5], prefix_value: i64) -> i64 {
        let upper = decode_prefix_4(
            &[digits[0], digits[1], digits[2], digits[3]],
            prefix_value / 10,
        );
        upper * 10 + (i64::from(digits[4]) - i64::from(b'0'))
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 999_999
            && digits[0] == b'0' + u8::try_from((prefix_value / 100_000) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0)
            && digits[2] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0)
            && digits[3] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0)
            && digits[4] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[5] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_6(digits: &[u8; 6], prefix_value: i64) -> i64 {
        let upper = decode_prefix_5(
            &[digits[0], digits[1], digits[2], digits[3], digits[4]],
            prefix_value / 10,
        );
        upper * 10 + (i64::from(digits[5]) - i64::from(b'0'))
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 9_999_999
            && digits[0] == b'0' + u8::try_from((prefix_value / 1_000_000) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from((prefix_value / 100_000) % 10).unwrap_or(0)
            && digits[2] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0)
            && digits[3] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0)
            && digits[4] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0)
            && digits[5] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[6] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_7(digits: &[u8; 7], prefix_value: i64) -> i64 {
        let upper = decode_prefix_6(
            &[
                digits[0], digits[1], digits[2], digits[3], digits[4], digits[5],
            ],
            prefix_value / 10,
        );
        upper * 10 + (i64::from(digits[6]) - i64::from(b'0'))
    }



    #[kani::requires(
        prefix_value >= 0 && prefix_value <= 99_999_999
            && digits[0] == b'0' + u8::try_from((prefix_value / 10_000_000) % 10).unwrap_or(0)
            && digits[1] == b'0' + u8::try_from((prefix_value / 1_000_000) % 10).unwrap_or(0)
            && digits[2] == b'0' + u8::try_from((prefix_value / 100_000) % 10).unwrap_or(0)
            && digits[3] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0)
            && digits[4] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0)
            && digits[5] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0)
            && digits[6] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0)
            && digits[7] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0)
    )]
    #[kani::ensures(|result: &i64| *result == prefix_value)]
    pub(super) fn decode_prefix_8(digits: &[u8; 8], prefix_value: i64) -> i64 {
        let upper = decode_prefix_7(
            &[
                digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6],
            ],
            prefix_value / 10,
        );
        upper * 10 + (i64::from(digits[7]) - i64::from(b'0'))
    }
}

#[cfg(kani)]
use kani_only::{
    decode_prefix_1, decode_prefix_2, decode_prefix_3, decode_prefix_4, decode_prefix_5,
    decode_prefix_6, decode_prefix_7, decode_prefix_8,
};





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_1_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_1".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11, base case: decode's own one-digit accumulation, tied to a ghost prefix_value via one direct fact. No stubbing.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_1_SRC, {
        #[kani::proof_for_contract(decode_prefix_1)]
        fn check_decode_prefix_1() {
            let digits: [u8; 1] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 9);
            kani::assume(digits[0] == b'0' + u8::try_from(prefix_value).unwrap_or(0));
            let _ = decode_prefix_1(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11: stubs decode_prefix_1. False trail caught: stub_verified's own precondition is a real proof obligation checked against the caller's own context -- it is not assumed transitively. digits[0]'s own constraint must be restated explicitly, growing by one fact per stage. Testing whether this (unlike encode's growing byte-preservation sum) stays cheap.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_2_SRC, {
        #[kani::proof_for_contract(decode_prefix_2)]
        #[kani::stub_verified(decode_prefix_1)]
        fn check_decode_prefix_2() {
            let digits: [u8; 2] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 99);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_2(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11, stage 3: stubs decode_prefix_2.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_3_SRC, {
        #[kani::proof_for_contract(decode_prefix_3)]
        #[kani::stub_verified(decode_prefix_2)]
        fn check_decode_prefix_3() {
            let digits: [u8; 3] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 999);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_3(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11, stage 4: stubs decode_prefix_3.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_4_SRC, {
        #[kani::proof_for_contract(decode_prefix_4)]
        #[kani::stub_verified(decode_prefix_3)]
        fn check_decode_prefix_4() {
            let digits: [u8; 4] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 9_999);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[3] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_4(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11, stage 5: stubs decode_prefix_4.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_5_SRC, {
        #[kani::proof_for_contract(decode_prefix_5)]
        #[kani::stub_verified(decode_prefix_4)]
        fn check_decode_prefix_5() {
            let digits: [u8; 5] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 99_999);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0));
            kani::assume(digits[3] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[4] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_5(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11, stage 6: the critical test -- stubs decode_prefix_5, the exact stage count where the original decode_2..9 chain's duplicate-call bug crashed the host.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_6_SRC, {
        #[kani::proof_for_contract(decode_prefix_6)]
        #[kani::stub_verified(decode_prefix_5)]
        fn check_decode_prefix_6() {
            let digits: [u8; 6] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 999_999);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 100_000) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0));
            kani::assume(digits[3] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0));
            kani::assume(digits[4] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[5] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_6(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 11, stage 7: stubs decode_prefix_6.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_7_SRC, {
        #[kani::proof_for_contract(decode_prefix_7)]
        #[kani::stub_verified(decode_prefix_6)]
        fn check_decode_prefix_7() {
            let digits: [u8; 7] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 9_999_999);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 1_000_000) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((prefix_value / 100_000) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0));
            kani::assume(digits[3] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0));
            kani::assume(digits[4] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0));
            kani::assume(digits[5] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[6] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_7(&digits, prefix_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_prefix::decode_prefix_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_prefix::check_decode_prefix_8".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: times out, confirmed memory-bounded. Hypothesis 11's growing requires (one equality fact per earlier byte, restated in full at every stage) scaled further than the growing-ensures sum (hypothesis 10, which broke at stage 6) but still eventually breaks, at stage 8. Confirms the lesson generalizes once more: growing fact count breaks eventually whether it is a sum (ensures) or a conjunction of equalities (requires) -- the only shape that scaled cleanly to 9 (truncate_stage_K) never grows at all, exactly one new fact per stage, in both requires and ensures.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_PREFIX_8_SRC, {
        #[kani::proof_for_contract(decode_prefix_8)]
        #[kani::stub_verified(decode_prefix_7)]
        fn check_decode_prefix_8() {
            let digits: [u8; 8] = kani::any();
            let prefix_value: i64 = kani::any();
            kani::assume(prefix_value >= 0 && prefix_value <= 99_999_999);
            kani::assume(digits[0] == b'0' + u8::try_from((prefix_value / 10_000_000) % 10).unwrap_or(0));
            kani::assume(digits[1] == b'0' + u8::try_from((prefix_value / 1_000_000) % 10).unwrap_or(0));
            kani::assume(digits[2] == b'0' + u8::try_from((prefix_value / 100_000) % 10).unwrap_or(0));
            kani::assume(digits[3] == b'0' + u8::try_from((prefix_value / 10_000) % 10).unwrap_or(0));
            kani::assume(digits[4] == b'0' + u8::try_from((prefix_value / 1_000) % 10).unwrap_or(0));
            kani::assume(digits[5] == b'0' + u8::try_from((prefix_value / 100) % 10).unwrap_or(0));
            kani::assume(digits[6] == b'0' + u8::try_from((prefix_value / 10) % 10).unwrap_or(0));
            kani::assume(digits[7] == b'0' + u8::try_from(prefix_value % 10).unwrap_or(0));
            let _ = decode_prefix_8(&digits, prefix_value);
        }
    }
}
