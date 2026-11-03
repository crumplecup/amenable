//! Encode-direction isolation: the `full_prefix_K` staging variant
//! and `full_encode_value_tracking`, the value-tracking full-encode
//! probe. Split out of `chrono_fractional_nanos_encode_staged` once
//! that file grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use crate::gallery::chrono_fractional_nanos_combine_prefix::DigitPrefix;

    // ── Hypothesis 8: merge the byte-tracking chain with the
    // proven-cheap "relate directly to nanos" shape. combine_prefix_K
    // related value to the *previous stage's own field* (upper.value()
    // * 10 + digit) -- cheap in isolation, but the composition at the
    // top required chaining all 8 stub substitutions through one
    // relational claim, which timed out. Here each stage's own
    // ensures states `value == nanos / 10^(9-K)` directly (the exact
    // pattern truncate_stage_K already proved cheap, stage by stage),
    // plus one direct byte fact for the newest position. Earlier
    // bytes ride along, trusted structurally through the stub chain
    // -- not yet verified explicitly; testing incrementally before
    // deciding whether that is sound enough to feed into decode.

    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<1>| {
        result.value() == nanos / 100_000_000
            && result.bytes()[0] == b'0' + u8::try_from(nanos / 100_000_000).unwrap_or(0)
    })]
    pub(super) fn full_prefix_1(nanos: i64) -> DigitPrefix<1> {
        let v = nanos / 100_000_000;
        DigitPrefix::new([b'0' + u8::try_from(v).unwrap_or(0)], v)
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<2>| {
        result.value() == nanos / 10_000_000
            && result.bytes()[1] == b'0' + u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_2(nanos: i64) -> DigitPrefix<2> {
        let upper = full_prefix_1(nanos);
        let digit = (nanos / 10_000_000) % 10;
        DigitPrefix::new([upper.bytes()[0], b'0' + u8::try_from(digit).unwrap_or(0)], upper.value() * 10 + digit)
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<3>| {
        result.value() == nanos / 1_000_000
            && result.bytes()[2] == b'0' + u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_3(nanos: i64) -> DigitPrefix<3> {
        let upper = full_prefix_2(nanos);
        let digit = (nanos / 1_000_000) % 10;
        DigitPrefix::new([upper.bytes()[0], upper.bytes()[1], b'0' + u8::try_from(digit).unwrap_or(0)], upper.value() * 10 + digit)
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<4>| {
        result.value() == nanos / 100_000
            && result.bytes()[3] == b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0)
    })]
    pub(super) fn full_prefix_4(nanos: i64) -> DigitPrefix<4> {
        let upper = full_prefix_3(nanos);
        let digit = (nanos / 100_000) % 10;
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                b'0' + u8::try_from(digit).unwrap_or(0),
            ], upper.value() * 10 + digit)
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &DigitPrefix<5>| {
        result.value() == nanos / 10_000
            && result.bytes()[4] == b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0)
    })]
    pub(crate) fn full_prefix_5(nanos: i64) -> DigitPrefix<5> {
        let upper = full_prefix_4(nanos);
        let digit = (nanos / 10_000) % 10;
        DigitPrefix::new([
                upper.bytes()[0],
                upper.bytes()[1],
                upper.bytes()[2],
                upper.bytes()[3],
                b'0' + u8::try_from(digit).unwrap_or(0),
            ], upper.value() * 10 + digit)
    }}

#[cfg(kani)]
pub(crate) use kani_only::full_prefix_5;
#[cfg(kani)]
use kani_only::{full_prefix_1, full_prefix_2, full_prefix_3, full_prefix_4};
#[cfg(kani)]
use crate::gallery::chrono_fractional_nanos_full_prefix_deep::full_encode_value_tracking;

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix::full_encode_value_tracking_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix::check_full_encode_value_tracking".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: each combine_prefix leaf passes individually and fast (sub-second), but the composed top-level function, with all 8 stubbed, still times out -- even though the ensures is a single fact (result.value == nanos), not a growing conjunction. Hypothesis: proving the chained value == nanos still requires the solver to verify the base-10 digit-decomposition identity (9 divisions by different powers of 10, recombined via the value*10+digit recurrence) is exactly nanos -- a genuinely expensive multi-division relational claim for CBMC's bit-blasted arithmetic, independent of how the proof is staged or structured. Confirmed safe to re-run.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_ENCODE_VALUE_TRACKING_SRC, {
        /// `full_encode_value_tracking`'s own contract: stubs every
        /// leaf it calls.
        #[kani::proof_for_contract(full_encode_value_tracking)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_2)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_3)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_4)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_5)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_6)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_7)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_8)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_combine_prefix::combine_prefix_9)]
        fn check_full_encode_value_tracking() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_encode_value_tracking(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix::full_prefix_1_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix::check_full_prefix_1".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, base case: byte+value tracking, relating directly to nanos. No stubbing.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_1_SRC, {
        #[kani::proof_for_contract(full_prefix_1)]
        fn check_full_prefix_1() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_1(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix::full_prefix_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix::check_full_prefix_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8: stubs full_prefix_1. value relates directly to nanos (the proven-cheap pattern); only the newest byte is stated directly, the earlier byte rides along unverified -- testing whether this is cheap before deciding whether it is sound enough.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_2_SRC, {
        #[kani::proof_for_contract(full_prefix_2)]
        #[kani::stub_verified(full_prefix_1)]
        fn check_full_prefix_2() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_2(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix::full_prefix_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix::check_full_prefix_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 3: stubs full_prefix_2.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_3_SRC, {
        #[kani::proof_for_contract(full_prefix_3)]
        #[kani::stub_verified(full_prefix_2)]
        fn check_full_prefix_3() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_3(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix::full_prefix_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix::check_full_prefix_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 4: stubs full_prefix_3.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_4_SRC, {
        #[kani::proof_for_contract(full_prefix_4)]
        #[kani::stub_verified(full_prefix_3)]
        fn check_full_prefix_4() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_4(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_full_prefix::full_prefix_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_full_prefix::check_full_prefix_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 8, stage 5: stubs full_prefix_4.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_FULL_PREFIX_5_SRC, {
        #[kani::proof_for_contract(full_prefix_5)]
        #[kani::stub_verified(full_prefix_4)]
        fn check_full_prefix_5() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = full_prefix_5(nanos);
        }
    }
}

