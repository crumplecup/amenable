//! Isolates whether a growing *conjunct count* in a single
//! `#[kani::ensures]` -- independent of chaining separate functions --
//! is itself a cost driver. Split out of
//! `chrono_fractional_nanos_contract_cost` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]




    /// Hypothesis 15: isolate ensures-conjunct-COUNT alone. Same
    /// trivial identity body every time, same array type, called
    /// ONCE (no chaining). The only variable across
    /// growing_ensures_1..9 is the number of `result[i] == arr[i]`
    /// conjuncts in the ensures -- exactly combine_digit_K's own
    /// growth shape, with the real combine logic stripped out.
    #[kani::ensures(|result: &[u8; 9]| result[0] == arr[0])]
    pub(crate) fn growing_ensures_1(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| result[0] == arr[0] && result[1] == arr[1])]
    pub(crate) fn growing_ensures_2(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0] && result[1] == arr[1] && result[2] == arr[2]
    })]
    pub(crate) fn growing_ensures_3(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0] && result[1] == arr[1] && result[2] == arr[2] && result[3] == arr[3]
    })]
    pub(crate) fn growing_ensures_4(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
    })]
    pub(crate) fn growing_ensures_5(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
    })]
    pub(crate) fn growing_ensures_6(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
            && result[6] == arr[6]
    })]
    pub(crate) fn growing_ensures_7(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
            && result[6] == arr[6]
            && result[7] == arr[7]
    })]
    pub(crate) fn growing_ensures_8(arr: [u8; 9]) -> [u8; 9] {
        arr
    }



    #[kani::ensures(|result: &[u8; 9]| {
        result[0] == arr[0]
            && result[1] == arr[1]
            && result[2] == arr[2]
            && result[3] == arr[3]
            && result[4] == arr[4]
            && result[5] == arr[5]
            && result[6] == arr[6]
            && result[7] == arr[7]
            && result[8] == arr[8]
    })]
    pub(crate) fn growing_ensures_9(arr: [u8; 9]) -> [u8; 9] {
        arr
    }
}

#[cfg(kani)]
pub(crate) use kani_only::{
    growing_ensures_1, growing_ensures_2, growing_ensures_3, growing_ensures_4, growing_ensures_5,
    growing_ensures_6, growing_ensures_7, growing_ensures_8, growing_ensures_9,
};





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_1_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_1".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 1 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_1_SRC, {
        #[kani::proof]
        fn check_growing_ensures_1() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_1(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_2_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_2".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 2 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_2_SRC, {
        #[kani::proof]
        fn check_growing_ensures_2() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_2(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_3_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_3".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 3 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_3_SRC, {
        #[kani::proof]
        fn check_growing_ensures_3() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_3(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_4_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_4".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 4 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_4_SRC, {
        #[kani::proof]
        fn check_growing_ensures_4() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_4(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_5_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_5".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 5 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_5_SRC, {
        #[kani::proof]
        fn check_growing_ensures_5() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_5(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_6_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_6".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 6 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_6_SRC, {
        #[kani::proof]
        fn check_growing_ensures_6() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_6(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_7_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_7".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 7 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_7_SRC, {
        #[kani::proof]
        fn check_growing_ensures_7() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_7(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_8_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_8".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 8 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_8_SRC, {
        #[kani::proof]
        fn check_growing_ensures_8() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_8(arr);
        }
    }
}




::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_growing_ensures::growing_ensures_9_alone".to_owned(),
            "gallery::chrono_fractional_nanos_growing_ensures::check_growing_ensures_9".to_owned(),
            "amenable_kani".to_owned(), "hypothesis 15: single call, ensures with 9 conjunct(s), no chaining.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}
amenable_derive::gallery_harness! {
    kani, CHECK_GROWING_ENSURES_9_SRC, {
        #[kani::proof]
        fn check_growing_ensures_9() {
            let arr: [u8; 9] = kani::any();
            let _ = growing_ensures_9(arr);
        }
    }
}
