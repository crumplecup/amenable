//! Decode-direction isolation: `decode_closed_K`, the closed-form
//! (multiplication, not division) staged decode. Split out of
//! `chrono_fractional_nanos_decode_staged` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    // ── Hypothesis 10: apply the same pairwise-staging fix to
    // decode's own side of the round trip. decode_closed_K's own
    // ensures states the closed-form value of `digits[0..K]` directly
    // -- multiplication by constants, not division -- stubbing only
    // the immediately preceding stage in the body, never restating
    // all K terms at once (hypothesis 3/4's wall, for encode's own
    // division-based version; untested whether decode's
    // multiplication-based version fares differently).

    #[kani::requires(digits[0].is_ascii_digit())]
    #[kani::ensures(|result: &i64| *result == i64::from(digits[0]) - i64::from(b'0'))]
    pub(super) fn decode_closed_1(digits: &[u8; 1]) -> i64 {
        i64::from(digits[0]) - i64::from(b'0')
    }



    #[kani::requires(digits[0].is_ascii_digit() && digits[1].is_ascii_digit())]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 10
                + (i64::from(digits[1]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_2(digits: &[u8; 2]) -> i64 {
        let upper = decode_closed_1(&[digits[0]]);
        upper * 10 + (i64::from(digits[1]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 100
                + (i64::from(digits[1]) - i64::from(b'0')) * 10
                + (i64::from(digits[2]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_3(digits: &[u8; 3]) -> i64 {
        let upper = decode_closed_2(&[digits[0], digits[1]]);
        upper * 10 + (i64::from(digits[2]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 1_000
                + (i64::from(digits[1]) - i64::from(b'0')) * 100
                + (i64::from(digits[2]) - i64::from(b'0')) * 10
                + (i64::from(digits[3]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_4(digits: &[u8; 4]) -> i64 {
        let upper = decode_closed_3(&[digits[0], digits[1], digits[2]]);
        upper * 10 + (i64::from(digits[3]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 10_000
                + (i64::from(digits[1]) - i64::from(b'0')) * 1_000
                + (i64::from(digits[2]) - i64::from(b'0')) * 100
                + (i64::from(digits[3]) - i64::from(b'0')) * 10
                + (i64::from(digits[4]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_5(digits: &[u8; 5]) -> i64 {
        let upper = decode_closed_4(&[digits[0], digits[1], digits[2], digits[3]]);
        upper * 10 + (i64::from(digits[4]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 100_000
                + (i64::from(digits[1]) - i64::from(b'0')) * 10_000
                + (i64::from(digits[2]) - i64::from(b'0')) * 1_000
                + (i64::from(digits[3]) - i64::from(b'0')) * 100
                + (i64::from(digits[4]) - i64::from(b'0')) * 10
                + (i64::from(digits[5]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_6(digits: &[u8; 6]) -> i64 {
        let upper = decode_closed_5(&[digits[0], digits[1], digits[2], digits[3], digits[4]]);
        upper * 10 + (i64::from(digits[5]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 1_000_000
                + (i64::from(digits[1]) - i64::from(b'0')) * 100_000
                + (i64::from(digits[2]) - i64::from(b'0')) * 10_000
                + (i64::from(digits[3]) - i64::from(b'0')) * 1_000
                + (i64::from(digits[4]) - i64::from(b'0')) * 100
                + (i64::from(digits[5]) - i64::from(b'0')) * 10
                + (i64::from(digits[6]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_7(digits: &[u8; 7]) -> i64 {
        let upper = decode_closed_6(&[
            digits[0], digits[1], digits[2], digits[3], digits[4], digits[5],
        ]);
        upper * 10 + (i64::from(digits[6]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 10_000_000
                + (i64::from(digits[1]) - i64::from(b'0')) * 1_000_000
                + (i64::from(digits[2]) - i64::from(b'0')) * 100_000
                + (i64::from(digits[3]) - i64::from(b'0')) * 10_000
                + (i64::from(digits[4]) - i64::from(b'0')) * 1_000
                + (i64::from(digits[5]) - i64::from(b'0')) * 100
                + (i64::from(digits[6]) - i64::from(b'0')) * 10
                + (i64::from(digits[7]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_8(digits: &[u8; 8]) -> i64 {
        let upper = decode_closed_7(&[
            digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6],
        ]);
        upper * 10 + (i64::from(digits[7]) - i64::from(b'0'))
    }



    #[kani::requires(digits.iter().all(|&b| b.is_ascii_digit()))]
    #[kani::ensures(|result: &i64| {
        *result
            == (i64::from(digits[0]) - i64::from(b'0')) * 100_000_000
                + (i64::from(digits[1]) - i64::from(b'0')) * 10_000_000
                + (i64::from(digits[2]) - i64::from(b'0')) * 1_000_000
                + (i64::from(digits[3]) - i64::from(b'0')) * 100_000
                + (i64::from(digits[4]) - i64::from(b'0')) * 10_000
                + (i64::from(digits[5]) - i64::from(b'0')) * 1_000
                + (i64::from(digits[6]) - i64::from(b'0')) * 100
                + (i64::from(digits[7]) - i64::from(b'0')) * 10
                + (i64::from(digits[8]) - i64::from(b'0'))
    })]
    pub(super) fn decode_closed_9(digits: &[u8; 9]) -> i64 {
        let upper = decode_closed_8(&[
            digits[0], digits[1], digits[2], digits[3], digits[4], digits[5], digits[6],
            digits[7],
        ]);
        upper * 10 + (i64::from(digits[8]) - i64::from(b'0'))
    }
}

#[cfg(kani)]
use kani_only::{
    decode_closed_1, decode_closed_2, decode_closed_3, decode_closed_4, decode_closed_5,
    decode_closed_6, decode_closed_7, decode_closed_8, decode_closed_9,
};





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_1_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_1".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, base case: decode's own closed-form value for one digit. No stubbing.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_1_SRC, {
        #[kani::proof_for_contract(decode_closed_1)]
        fn check_decode_closed_1() {
            let digits: [u8; 1] = kani::any();
            kani::assume(digits[0].is_ascii_digit());
            let _ = decode_closed_1(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10: stubs decode_closed_1. Closed form over digits directly, multiplication-based (not division) -- testing whether this scales where encode's division-based closed-form version did not.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_2_SRC, {
        #[kani::proof_for_contract(decode_closed_2)]
        #[kani::stub_verified(decode_closed_1)]
        fn check_decode_closed_2() {
            let digits: [u8; 2] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_2(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 3: stubs decode_closed_2.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_3_SRC, {
        #[kani::proof_for_contract(decode_closed_3)]
        #[kani::stub_verified(decode_closed_2)]
        fn check_decode_closed_3() {
            let digits: [u8; 3] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_3(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 4: stubs decode_closed_3.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_4_SRC, {
        #[kani::proof_for_contract(decode_closed_4)]
        #[kani::stub_verified(decode_closed_3)]
        fn check_decode_closed_4() {
            let digits: [u8; 4] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_4(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 5: stubs decode_closed_4.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_5_SRC, {
        #[kani::proof_for_contract(decode_closed_5)]
        #[kani::stub_verified(decode_closed_4)]
        fn check_decode_closed_5() {
            let digits: [u8; 5] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_5(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 6: stubs decode_closed_5 -- the exact stage count where the old decode_2..9 chain's duplicate-call bug crashed the host.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_6_SRC, {
        #[kani::proof_for_contract(decode_closed_6)]
        #[kani::stub_verified(decode_closed_5)]
        fn check_decode_closed_6() {
            let digits: [u8; 6] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_6(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 7: stubs decode_closed_6.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_7_SRC, {
        #[kani::proof_for_contract(decode_closed_7)]
        #[kani::stub_verified(decode_closed_6)]
        fn check_decode_closed_7() {
            let digits: [u8; 7] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_7(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_8".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 8: stubs decode_closed_7.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_8_SRC, {
        #[kani::proof_for_contract(decode_closed_8)]
        #[kani::stub_verified(decode_closed_7)]
        fn check_decode_closed_8() {
            let digits: [u8; 8] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_8(&digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_decode_closed::decode_closed_9_passes".to_owned(),
            "gallery::chrono_fractional_nanos_decode_closed::check_decode_closed_9".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 10, stage 9: the real target -- stubs decode_closed_8. If this passes, decode's own closed-form value over the full 9-digit array is proven, multiplication-based, matching the real decode loop's own definition exactly.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DECODE_CLOSED_9_SRC, {
        #[kani::proof_for_contract(decode_closed_9)]
        #[kani::stub_verified(decode_closed_8)]
        fn check_decode_closed_9() {
            let digits: [u8; 9] = kani::any();
            kani::assume(digits.iter().all(|&b| b.is_ascii_digit()));
            let _ = decode_closed_9(&digits);
        }
    }
}
