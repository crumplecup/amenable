//! The tail (`combine_digit_7` through `combine_digit_9`, plus
//! `full_encode_param_passing`) of `chrono_fractional_nanos_combine_digit`,
//! split out once that file grew large enough to be unnavigable as
//! one unit.

mod kani_only {
    #![cfg(kani)]

    use amenable_core::Ensures;
    use crate::IndexRecoversTheStoredElement;
    use crate::gallery::chrono_fractional_nanos_combine_digit::{
        combine_digit_2, combine_digit_3, combine_digit_4, combine_digit_5, combine_digit_6,
    };




    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 7]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], upper[5]))
            && IndexRecoversTheStoredElement::ensures((result[6], new_digit))
    })]
    pub(super) fn combine_digit_7(upper: [u8; 6], new_digit: u8) -> [u8; 7] {
        [
            upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], new_digit,
        ]
    }




    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 8]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], upper[5]))
            && IndexRecoversTheStoredElement::ensures((result[6], upper[6]))
            && IndexRecoversTheStoredElement::ensures((result[7], new_digit))
    })]
    pub(super) fn combine_digit_8(upper: [u8; 7], new_digit: u8) -> [u8; 8] {
        [
            upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], upper[6], new_digit,
        ]
    }




    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 9]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], upper[5]))
            && IndexRecoversTheStoredElement::ensures((result[6], upper[6]))
            && IndexRecoversTheStoredElement::ensures((result[7], upper[7]))
            && IndexRecoversTheStoredElement::ensures((result[8], new_digit))
    })]
    pub(super) fn combine_digit_9(upper: [u8; 8], new_digit: u8) -> [u8; 9] {
        [
            upper[0], upper[1], upper[2], upper[3], upper[4], upper[5], upper[6], upper[7],
            new_digit,
        ]
    }




    /// The top-level consumer: threads `upper` through by calling
    /// each leaf exactly once. No stage calls another stage -- this
    /// function is the only thing that chains them. Minimal `ensures`
    /// for now: only the final digit, direct from `nanos`, no call.
    /// Positions 0..7 ride along, trusted transitively through each
    /// leaf's own `IndexRecoversTheStoredElement::ensures((result[i],
    /// upper[i]))` fact -- unlike `fractional_digit_bytes_depth_9`'s
    /// own weak stub (which broke down when *that* function itself
    /// was stubbed directly), here every leaf in the chain is stubbed
    /// together in the same proof, so the transitive chain of facts
    /// is all present at once, not lost.
    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &[u8; 9]| {
        IndexRecoversTheStoredElement::ensures((amenable_ext::fractional_digit_bytes_to_nanos(result), nanos))
    })]
    pub(crate) fn full_encode_param_passing(nanos: i64) -> [u8; 9] {
        let d1 = [b'0' + u8::try_from((nanos / 100_000_000) % 10).unwrap_or(0)];
        let d2 = combine_digit_2(d1, b'0' + u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0));
        let d3 = combine_digit_3(d2, b'0' + u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0));
        let d4 = combine_digit_4(d3, b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0));
        let d5 = combine_digit_5(d4, b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0));
        let d6 = combine_digit_6(d5, b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0));
        let d7 = combine_digit_7(d6, b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0));
        let d8 = combine_digit_8(d7, b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0));
        combine_digit_9(d8, b'0' + u8::try_from(nanos % 10).unwrap_or(0))
    }
}

#[cfg(kani)]
pub(crate) use kani_only::full_encode_param_passing;
#[cfg(kani)]
use kani_only::{combine_digit_7, combine_digit_8, combine_digit_9};







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit_tail::combine_digit_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit_tail::check_combine_digit_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: same shape as combine_digit_2/3. A base case, nothing to stub. Required so full_encode_param_passing's own stub_verified target has a corresponding proof_for_contract harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_7_SRC, {
        /// `combine_digit_7`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_7)]
        fn check_combine_digit_7() {
            let upper: [u8; 6] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_7(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit_tail::combine_digit_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit_tail::check_combine_digit_8".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: same shape as combine_digit_2/3. A base case, nothing to stub. Required so full_encode_param_passing's own stub_verified target has a corresponding proof_for_contract harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_8_SRC, {
        /// `combine_digit_8`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_8)]
        fn check_combine_digit_8() {
            let upper: [u8; 7] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_8(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit_tail::combine_digit_9_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit_tail::check_combine_digit_9".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: the full nine-position combine step. Same shape as combine_digit_2/3, just larger -- upper is a parameter, nothing to stub, no call chain of any kind.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_9_SRC, {
        /// `combine_digit_9`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_9)]
        fn check_combine_digit_9() {
            let upper: [u8; 8] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_9(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit_tail::full_encode_param_passing_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit_tail::check_full_encode_param_passing".to_owned(),
            "amenable_kani".to_owned(),
            "the top-level consumer: stubs all 8 combine_digit leaves (none of which call each other, so no nesting/compounding is possible). Minimal ensures -- only the final digit, direct from nanos -- relying on the transitive chain of each leaf's own upper[i] fact for positions 0..7.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_ENCODE_PARAM_PASSING_SRC, {
        /// `full_encode_param_passing`'s own contract: stubs every
        /// leaf it calls.
        #[kani::proof_for_contract(full_encode_param_passing)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_digit::combine_digit_2)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_digit::combine_digit_3)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_digit::combine_digit_4)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_digit::combine_digit_5)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_digit::combine_digit_6)]
        #[kani::stub_verified(combine_digit_7)]
        #[kani::stub_verified(combine_digit_8)]
        #[kani::stub_verified(combine_digit_9)]
        fn check_full_encode_param_passing() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_encode_param_passing(nanos);
        }
    }
}
