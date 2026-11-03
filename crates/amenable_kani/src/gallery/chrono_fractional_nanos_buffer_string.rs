//! Building a real `String` for Kani testing purposes without ever
//! calling `kani::any::<String>()` (which `String` cannot implement):
//! a `KaniUtf8Buffer` staged the same way as the digit chain, then an
//! unconditional, fixed-length `.push()` sequence to build the real
//! `String` from it, plus the `KaniFractionalSecondDescriptor` wrapper
//! that composes this into a real `amenable_time::FractionalSecondDescriptor`.
//! Split out of `chrono_fractional_nanos_round_trip` once that file
//! grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::KaniUtf8Buffer;


    // ── Hypothesis 18: peel over KaniUtf8Buffer, lean on the proven
    // refinement (gallery::string_buffer_correspondence) to carry the
    // result to String, rather than trying to get Kani to verify a
    // real String's construction directly. `String` itself can never
    // be `stub_verified` (confirmed: it does not implement `kani::
    // Arbitrary`), so content-exactness is proven here, on
    // `KaniUtf8Buffer<9>` (which is Arbitrary-clean), one byte at a
    // time -- exactly `gallery::string_depth_induction`'s own
    // extend_by_one_byte/extend_by_two_bytes shape, but for content-
    // exactness (each byte matches its source digit) instead of
    // no_embedded_nul. Built up incrementally, one depth at a time.

    const DIGIT_BUFFER_MAX_LEN: usize = 9;

    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 1 && r.as_bytes()[0] == digits[0]
    })]
    pub(super) fn digit_buffer_depth_1(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let buffer: KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> = kani::any();
        kani::assume(buffer.len() == 0);
        buffer
            .push_byte(digits[0])
            .expect("empty buffer, within MAX_LEN==9 capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 2 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1]
    })]
    pub(super) fn digit_buffer_depth_2(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_1(digits);
        prior
            .push_byte(digits[1])
            .expect("len==1 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 3 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2]
    })]
    pub(super) fn digit_buffer_depth_3(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_2(digits);
        prior
            .push_byte(digits[2])
            .expect("len==2 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 4 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2] && r.as_bytes()[3] == digits[3]
    })]
    pub(super) fn digit_buffer_depth_4(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_3(digits);
        prior
            .push_byte(digits[3])
            .expect("len==3 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 5 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2] && r.as_bytes()[3] == digits[3] && r.as_bytes()[4] == digits[4]
    })]
    pub(super) fn digit_buffer_depth_5(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_4(digits);
        prior
            .push_byte(digits[4])
            .expect("len==4 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 6 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2] && r.as_bytes()[3] == digits[3] && r.as_bytes()[4] == digits[4] && r.as_bytes()[5] == digits[5]
    })]
    pub(super) fn digit_buffer_depth_6(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_5(digits);
        prior
            .push_byte(digits[5])
            .expect("len==5 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 7 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2] && r.as_bytes()[3] == digits[3] && r.as_bytes()[4] == digits[4] && r.as_bytes()[5] == digits[5] && r.as_bytes()[6] == digits[6]
    })]
    pub(super) fn digit_buffer_depth_7(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_6(digits);
        prior
            .push_byte(digits[6])
            .expect("len==6 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 8 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2] && r.as_bytes()[3] == digits[3] && r.as_bytes()[4] == digits[4] && r.as_bytes()[5] == digits[5] && r.as_bytes()[6] == digits[6] && r.as_bytes()[7] == digits[7]
    })]
    pub(super) fn digit_buffer_depth_8(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_7(digits);
        prior
            .push_byte(digits[7])
            .expect("len==7 < MAX_LEN==9, within capacity")
    }


    #[kani::ensures(|r: &KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN>| {
        r.len() == 9 && r.as_bytes()[0] == digits[0] && r.as_bytes()[1] == digits[1] && r.as_bytes()[2] == digits[2] && r.as_bytes()[3] == digits[3] && r.as_bytes()[4] == digits[4] && r.as_bytes()[5] == digits[5] && r.as_bytes()[6] == digits[6] && r.as_bytes()[7] == digits[7] && r.as_bytes()[8] == digits[8]
    })]
    pub(crate) fn digit_buffer_depth_9(digits: [u8; 9]) -> KaniUtf8Buffer<DIGIT_BUFFER_MAX_LEN> {
        let prior = digit_buffer_depth_8(digits);
        prior
            .push_byte(digits[8])
            .expect("len==8 < MAX_LEN==9, within capacity")
    }}

#[cfg(kani)]
pub(crate) use kani_only::digit_buffer_depth_9;
#[cfg(kani)]
use kani_only::{
    digit_buffer_depth_1, digit_buffer_depth_2, digit_buffer_depth_3, digit_buffer_depth_4,
    digit_buffer_depth_5, digit_buffer_depth_6, digit_buffer_depth_7, digit_buffer_depth_8,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_1".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_1".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, base case: a KaniUtf8Buffer<9> holding exactly digits[0], nothing to stub.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_1_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_1)]
        fn check_digit_buffer_depth_1() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_1(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_2".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_1, pushes one more byte. Content-exactness, not no_embedded_nul.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_2_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_2)]
        #[kani::stub_verified(digit_buffer_depth_1)]
        fn check_digit_buffer_depth_2() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_2(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_3".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_2, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_3_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_3)]
        #[kani::stub_verified(digit_buffer_depth_2)]
        fn check_digit_buffer_depth_3() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_3(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_4".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_3, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_4_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_4)]
        #[kani::stub_verified(digit_buffer_depth_3)]
        fn check_digit_buffer_depth_4() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_4(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_5".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_4, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_5_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_5)]
        #[kani::stub_verified(digit_buffer_depth_4)]
        fn check_digit_buffer_depth_5() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_5(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_6".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_5, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_6_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_6)]
        #[kani::stub_verified(digit_buffer_depth_5)]
        fn check_digit_buffer_depth_6() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_6(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_7".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_6, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_7_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_7)]
        #[kani::stub_verified(digit_buffer_depth_6)]
        fn check_digit_buffer_depth_7() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_7(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_8".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_8".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_7, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_8_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_8)]
        #[kani::stub_verified(digit_buffer_depth_7)]
        fn check_digit_buffer_depth_8() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_8(digits);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_buffer_string::digit_buffer_depth_9".to_owned(),
            "gallery::chrono_fractional_nanos_buffer_string::check_digit_buffer_depth_9".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 18, step: stubs digit_buffer_depth_8, pushes one more byte.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_BUFFER_DEPTH_9_SRC, {
        #[kani::proof_for_contract(digit_buffer_depth_9)]
        #[kani::stub_verified(digit_buffer_depth_8)]
        fn check_digit_buffer_depth_9() {
            let digits: [u8; 9] = kani::any();
            let _ = digit_buffer_depth_9(digits);
        }
    }
}

