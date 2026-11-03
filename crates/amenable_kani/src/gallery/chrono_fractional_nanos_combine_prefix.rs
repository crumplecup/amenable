//! Encode-direction isolation: `DigitPrefix` and its
//! `combine_prefix_K` chain -- carrying the already-proven decoded
//! `value` alongside `bytes` so each stage's own `ensures` is a direct
//! field comparison, never a walk over the array. Split out of
//! `chrono_fractional_nanos_encode_staged` once that file grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]




    // ── Hypothesis 6: carry `value` alongside `bytes`, updated by
    // pure arithmetic (`upper.value * 10 + digit`) at each stage --
    // never by division, never by a call to decode. The top-level
    // ensures then compares `result.value == nanos` directly: one
    // fact, no call, no division, no recalculation of any kind. This
    // is the actual "contract type to hold the complete bound" --
    // `value` *is* the bound, carried as data through the chain
    // rather than restated.

    #[derive(Clone, Copy, derive_getters::Getters, derive_new::new)]
    pub(crate) struct DigitPrefix<const K: usize> {
        #[getter(copy)]
        bytes: [u8; K],
        #[getter(copy)]
        value: i64,
    }



    impl<const K: usize> kani::Arbitrary for DigitPrefix<K> {
        fn any() -> Self {
            Self::new(kani::any(), kani::any())
        }
    }



    /// Diagnostic variant: takes the previous stage's fields flat
    /// (`upper_byte`, `upper_value`) rather than a `DigitPrefix<1>`
    /// parameter, so this signature mentions only one const-generic
    /// instantiation of `DigitPrefix` (the `<2>` return type) --
    /// testing whether Kani's `#[kani::proof_for_contract]` support
    /// specifically balks at a signature referencing two different
    /// instantiations of the same generic carrier type.
    #[kani::requires(upper_value >= 0 && upper_value <= 9 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<2>| {
        result.value() == upper_value * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper_byte
            && result.bytes()[1] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_2(upper_byte: u8, upper_value: i64, new_digit_value: u8) -> DigitPrefix<2> {
        DigitPrefix::new([upper_byte, b'0' + new_digit_value], upper_value * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 99 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<3>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_3(upper: DigitPrefix<2>, new_digit_value: u8) -> DigitPrefix<3> {
        DigitPrefix::new([upper.bytes()[0], upper.bytes()[1], b'0' + new_digit_value], upper.value() * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 999 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<4>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == upper.bytes()[2]
            && result.bytes()[3] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_4(upper: DigitPrefix<3>, new_digit_value: u8) -> DigitPrefix<4> {
        DigitPrefix::new([upper.bytes()[0], upper.bytes()[1], upper.bytes()[2], b'0' + new_digit_value], upper.value() * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 9_999 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<5>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == upper.bytes()[2]
            && result.bytes()[3] == upper.bytes()[3]
            && result.bytes()[4] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_5(upper: DigitPrefix<4>, new_digit_value: u8) -> DigitPrefix<5> {
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                b'0' + new_digit_value,
            ], upper.value() * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 99_999 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<6>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == upper.bytes()[2]
            && result.bytes()[3] == upper.bytes()[3]
            && result.bytes()[4] == upper.bytes()[4]
            && result.bytes()[5] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_6(upper: DigitPrefix<5>, new_digit_value: u8) -> DigitPrefix<6> {
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                b'0' + new_digit_value,
            ], upper.value() * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 999_999 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<7>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == upper.bytes()[2]
            && result.bytes()[3] == upper.bytes()[3]
            && result.bytes()[4] == upper.bytes()[4]
            && result.bytes()[5] == upper.bytes()[5]
            && result.bytes()[6] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_7(upper: DigitPrefix<6>, new_digit_value: u8) -> DigitPrefix<7> {
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                upper.bytes()[5],
                b'0' + new_digit_value,
            ], upper.value() * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 9_999_999 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<8>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == upper.bytes()[2]
            && result.bytes()[3] == upper.bytes()[3]
            && result.bytes()[4] == upper.bytes()[4]
            && result.bytes()[5] == upper.bytes()[5]
            && result.bytes()[6] == upper.bytes()[6]
            && result.bytes()[7] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_8(upper: DigitPrefix<7>, new_digit_value: u8) -> DigitPrefix<8> {
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                upper.bytes()[5],
                upper.bytes()[6],
                b'0' + new_digit_value,
            ], upper.value() * 10 + i64::from(new_digit_value))
    }



    #[kani::requires(upper.value() >= 0 && upper.value() <= 99_999_999 && new_digit_value <= 9)]
    #[kani::ensures(|result: &DigitPrefix<9>| {
        result.value() == upper.value() * 10 + i64::from(new_digit_value)
            && result.bytes()[0] == upper.bytes()[0]
            && result.bytes()[1] == upper.bytes()[1]
            && result.bytes()[2] == upper.bytes()[2]
            && result.bytes()[3] == upper.bytes()[3]
            && result.bytes()[4] == upper.bytes()[4]
            && result.bytes()[5] == upper.bytes()[5]
            && result.bytes()[6] == upper.bytes()[6]
            && result.bytes()[7] == upper.bytes()[7]
            && result.bytes()[8] == b'0' + new_digit_value
    })]
    pub(crate) fn combine_prefix_9(upper: DigitPrefix<8>, new_digit_value: u8) -> DigitPrefix<9> {
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                upper.bytes()[4],
                upper.bytes()[5],
                upper.bytes()[6],
                upper.bytes()[7],
                b'0' + new_digit_value,
            ], upper.value() * 10 + i64::from(new_digit_value))
    }
}

#[cfg(kani)]
pub(crate) use kani_only::{
    DigitPrefix, combine_prefix_2, combine_prefix_3, combine_prefix_4, combine_prefix_5,
    combine_prefix_6, combine_prefix_7, combine_prefix_8, combine_prefix_9,
};





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_prefix::check_combine_prefix_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 6, base: value is carried alongside bytes, updated by pure arithmetic (upper.value() * 10 + digit) -- no division, no decode call. A base case, nothing to stub.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_2_SRC, {
        /// `combine_prefix_2`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_prefix_2)]
        fn check_combine_prefix_2() {
            let upper_byte: u8 = kani::any();
            let upper_value: i64 = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_2(upper_byte, upper_value, new_digit_value);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_prefix::check_combine_prefix_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 6: upper is a DigitPrefix<2> parameter (not a call), carrying both bytes and value. A base case, nothing to stub.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_3_SRC, {
        /// `combine_prefix_3`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_prefix_3)]
        fn check_combine_prefix_3() {
            let upper: DigitPrefix<2> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_3(upper, new_digit_value);
        }
    }
}





amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_4_SRC, {
        #[kani::proof_for_contract(combine_prefix_4)]
        fn check_combine_prefix_4() {
            let upper: DigitPrefix<3> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_4(upper, new_digit_value);
        }
    }
}





amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_5_SRC, {
        #[kani::proof_for_contract(combine_prefix_5)]
        fn check_combine_prefix_5() {
            let upper: DigitPrefix<4> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_5(upper, new_digit_value);
        }
    }
}





amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_6_SRC, {
        #[kani::proof_for_contract(combine_prefix_6)]
        fn check_combine_prefix_6() {
            let upper: DigitPrefix<5> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_6(upper, new_digit_value);
        }
    }
}





amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_7_SRC, {
        #[kani::proof_for_contract(combine_prefix_7)]
        fn check_combine_prefix_7() {
            let upper: DigitPrefix<6> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_7(upper, new_digit_value);
        }
    }
}





amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_8_SRC, {
        #[kani::proof_for_contract(combine_prefix_8)]
        fn check_combine_prefix_8() {
            let upper: DigitPrefix<7> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_8(upper, new_digit_value);
        }
    }
}





amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_PREFIX_9_SRC, {
        #[kani::proof_for_contract(combine_prefix_9)]
        fn check_combine_prefix_9() {
            let upper: DigitPrefix<8> = kani::any();
            let new_digit_value: u8 = kani::any();
            let _ = combine_prefix_9(upper, new_digit_value);
        }
    }
}
