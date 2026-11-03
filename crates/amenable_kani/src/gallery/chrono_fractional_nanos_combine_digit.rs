//! Encode-direction isolation: the combine-chain leaves
//! (`combine_digit_K`) that `nanos_to_fractional_digit_bytes` actually
//! stubs in production -- each a soundly stubbable leaf taking the
//! previous stage's bytes as a parameter rather than calling it
//! recursively. Split out of `chrono_fractional_nanos_encode_staged`
//! once that file grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use amenable_core::Ensures;
    use crate::IndexRecoversTheStoredElement;



    // ── Hypothesis 5: stop recalculating `upper` entirely. Every
    // earlier design computed `upper` *inside* each stage -- either
    // re-deriving it via division from `nanos` (expensive, growing),
    // or by calling the previous stage (duplicate-call compounding,
    // the exact shape that crashed the host the first time and
    // reappeared at stage 9). Here `upper` is a *parameter*: the
    // caller computes it once and passes it in, so each combine
    // step's own `ensures` compares against it directly -- no call,
    // no division, nothing to recalculate. Each step is a leaf, with
    // nothing to stub for its own proof, exactly like
    // `fractional_nanos_combine_one_digit`'s own shape (confirmed
    // cheap in isolation). A single top-level function threads `upper`
    // through by calling each leaf exactly once -- no stage calls
    // another stage, so there is no nesting to compound.

    /// False-trail reproduction only -- see `extract_digit_with_
    /// symbolic_divisor_times_out`'s own gallery registration below.
    /// Not used by the top-level encode function: fixed-constant
    /// divisions are inlined there instead of calling this.
    #[kani::requires(divisor >= 1)]
    #[kani::ensures(|result: &u8| {
        IndexRecoversTheStoredElement::ensures((*result, b'0' + u8::try_from((nanos / divisor) % 10).unwrap_or(0)))
    })]
    pub(super) fn extract_digit(nanos: i64, divisor: i64) -> u8 {
        b'0' + u8::try_from((nanos / divisor) % 10).unwrap_or(0)
    }



    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 2]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], new_digit))
    })]
    pub(crate) fn combine_digit_2(upper: [u8; 1], new_digit: u8) -> [u8; 2] {
        [upper[0], new_digit]
    }



    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 3]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], new_digit))
    })]
    pub(crate) fn combine_digit_3(upper: [u8; 2], new_digit: u8) -> [u8; 3] {
        [upper[0], upper[1], new_digit]
    }



    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 4]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], new_digit))
    })]
    pub(crate) fn combine_digit_4(upper: [u8; 3], new_digit: u8) -> [u8; 4] {
        [upper[0], upper[1], upper[2], new_digit]
    }



    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 5]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], new_digit))
    })]
    pub(crate) fn combine_digit_5(upper: [u8; 4], new_digit: u8) -> [u8; 5] {
        [upper[0], upper[1], upper[2], upper[3], new_digit]
    }



    #[kani::requires(true)]
    #[kani::ensures(|result: &[u8; 6]| {
        IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], new_digit))
    })]
    pub(crate) fn combine_digit_6(upper: [u8; 5], new_digit: u8) -> [u8; 6] {
        [upper[0], upper[1], upper[2], upper[3], upper[4], new_digit]
    }}

#[cfg(kani)]
pub(crate) use kani_only::{
    combine_digit_2, combine_digit_3, combine_digit_4, combine_digit_5, combine_digit_6,
};
#[cfg(kani)]
use kani_only::extract_digit;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit::extract_digit_with_symbolic_divisor_times_out".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit::check_extract_digit".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: a single digit-extraction lemma generalized to take the divisor as a symbolic i64 parameter (only constrained >= 1) times out, even though it is a leaf with nothing to stub. Every other working lemma in this investigation divided by a fixed, compile-time-constant divisor (nanos / 10, nanos / 100, ...); a variable/variable division forces CBMC to build a general division circuit instead of the cheap constant-divisor case. Removed in favor of inlining fixed-constant divisions at each call site. Confirmed safe to re-run.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_EXTRACT_DIGIT_SRC, {
        /// Reproduction only -- see the registration above. Not used
        /// by the top-level encode function; fixed-constant divisions
        /// are inlined there instead.
        #[kani::proof_for_contract(extract_digit)]
        fn check_extract_digit() {
            let nanos: i64 = kani::any();
            let divisor: i64 = kani::any();
            kani::assume(divisor >= 1);
            let _ = extract_digit(nanos, divisor);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit::combine_digit_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit::check_combine_digit_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: upper is a parameter, not recomputed via a call or division -- ensures compares against it directly. A base case, nothing to stub.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_2_SRC, {
        /// `combine_digit_2`'s own contract: the base case, no
        /// stubbing -- `upper` is just an arbitrary parameter.
        #[kani::proof_for_contract(combine_digit_2)]
        fn check_combine_digit_2() {
            let upper: [u8; 1] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_2(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit::combine_digit_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit::check_combine_digit_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: same shape as combine_digit_2, one position larger. A base case, nothing to stub.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_3_SRC, {
        /// `combine_digit_3`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_3)]
        fn check_combine_digit_3() {
            let upper: [u8; 2] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_3(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit::combine_digit_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit::check_combine_digit_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: same shape as combine_digit_2/3. A base case, nothing to stub. Required so full_encode_param_passing's own stub_verified target has a corresponding proof_for_contract harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_4_SRC, {
        /// `combine_digit_4`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_4)]
        fn check_combine_digit_4() {
            let upper: [u8; 3] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_4(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit::combine_digit_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit::check_combine_digit_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: same shape as combine_digit_2/3. A base case, nothing to stub. Required so full_encode_param_passing's own stub_verified target has a corresponding proof_for_contract harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_5_SRC, {
        /// `combine_digit_5`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_5)]
        fn check_combine_digit_5() {
            let upper: [u8; 4] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_5(upper, new_digit);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_combine_digit::combine_digit_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_combine_digit::check_combine_digit_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 5, leaf: same shape as combine_digit_2/3. A base case, nothing to stub. Required so full_encode_param_passing's own stub_verified target has a corresponding proof_for_contract harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_COMBINE_DIGIT_6_SRC, {
        /// `combine_digit_6`'s own contract: the base case, no
        /// stubbing.
        #[kani::proof_for_contract(combine_digit_6)]
        fn check_combine_digit_6() {
            let upper: [u8; 5] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = combine_digit_6(upper, new_digit);
        }
    }
}

