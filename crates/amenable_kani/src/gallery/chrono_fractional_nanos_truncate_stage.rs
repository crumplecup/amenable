//! Encode-direction isolation: the `truncate_stage_K` staging variant.
//! Split out of `chrono_fractional_nanos_encode_staged` once that file
//! grew large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]




    /// The top-level consumer, value-tracking version: `value` is the
    /// "complete bound" -- carried as data through the chain,
    /// established by pure arithmetic at each stage, never
    /// recalculated via division or a decode call. The ensures is
    // ── Hypothesis 7: break the single 9-division identity into 8
    // smaller, *pairwise* 2-division identities. Each stage's own
    // claim relates directly to a truncated `nanos` (`nanos /
    // 10^(9-K)`), not to the previous stage's own fields -- so each
    // stage's own proof obligation, given the previous stage's own
    // truncated-quotient fact, reduces to the standard identity
    // `(x/10)*10 + x%10 == x` for one intermediate value `x = nanos /
    // 10^(9-K+1)`, not a relation tying multiple divisors of `nanos`
    // together at once.

    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 100_000_000)]
    pub(super) fn truncate_stage_1(nanos: i64) -> i64 {
        nanos / 100_000_000
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 10_000_000)]
    pub(super) fn truncate_stage_2(nanos: i64) -> i64 {
        let upper = truncate_stage_1(nanos);
        upper * 10 + (nanos / 10_000_000) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 1_000_000)]
    pub(super) fn truncate_stage_3(nanos: i64) -> i64 {
        let upper = truncate_stage_2(nanos);
        upper * 10 + (nanos / 1_000_000) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 100_000)]
    pub(super) fn truncate_stage_4(nanos: i64) -> i64 {
        let upper = truncate_stage_3(nanos);
        upper * 10 + (nanos / 100_000) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 10_000)]
    pub(super) fn truncate_stage_5(nanos: i64) -> i64 {
        let upper = truncate_stage_4(nanos);
        upper * 10 + (nanos / 10_000) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 1_000)]
    pub(super) fn truncate_stage_6(nanos: i64) -> i64 {
        let upper = truncate_stage_5(nanos);
        upper * 10 + (nanos / 1_000) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 100)]
    pub(super) fn truncate_stage_7(nanos: i64) -> i64 {
        let upper = truncate_stage_6(nanos);
        upper * 10 + (nanos / 100) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos / 10)]
    pub(super) fn truncate_stage_8(nanos: i64) -> i64 {
        let upper = truncate_stage_7(nanos);
        upper * 10 + (nanos / 10) % 10
    }



    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &i64| *result == nanos)]
    pub(super) fn truncate_stage_9(nanos: i64) -> i64 {
        let upper = truncate_stage_8(nanos);
        upper * 10 + nanos % 10
    }
}

#[cfg(kani)]
use kani_only::{
    truncate_stage_1, truncate_stage_2, truncate_stage_3, truncate_stage_4, truncate_stage_5,
    truncate_stage_6, truncate_stage_7, truncate_stage_8, truncate_stage_9,
};





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_1_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_1".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, base case: nanos / 10^8, no stubbing.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_1_SRC, {
        #[kani::proof_for_contract(truncate_stage_1)]
        fn check_truncate_stage_1() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_1(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7: stubs truncate_stage_1. Relates directly to nanos/10^7, not to the previous stage's own fields -- the proof obligation is the standard (x/10)*10+x%10==x identity for x = nanos/10^7, not a multi-divisor relation.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_2_SRC, {
        #[kani::proof_for_contract(truncate_stage_2)]
        #[kani::stub_verified(truncate_stage_1)]
        fn check_truncate_stage_2() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_2(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 3: stubs truncate_stage_2.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_3_SRC, {
        #[kani::proof_for_contract(truncate_stage_3)]
        #[kani::stub_verified(truncate_stage_2)]
        fn check_truncate_stage_3() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_3(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 4: stubs truncate_stage_3.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_4_SRC, {
        #[kani::proof_for_contract(truncate_stage_4)]
        #[kani::stub_verified(truncate_stage_3)]
        fn check_truncate_stage_4() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_4(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 5: stubs truncate_stage_4.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_5_SRC, {
        #[kani::proof_for_contract(truncate_stage_5)]
        #[kani::stub_verified(truncate_stage_4)]
        fn check_truncate_stage_5() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_5(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 6: stubs truncate_stage_5 -- the exact stage count where the old decode_2..9 chain's duplicate-call bug crashed the host.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_6_SRC, {
        #[kani::proof_for_contract(truncate_stage_6)]
        #[kani::stub_verified(truncate_stage_5)]
        fn check_truncate_stage_6() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_6(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 7: stubs truncate_stage_6.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_7_SRC, {
        #[kani::proof_for_contract(truncate_stage_7)]
        #[kani::stub_verified(truncate_stage_6)]
        fn check_truncate_stage_7() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_7(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_8".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 8: stubs truncate_stage_7.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_8_SRC, {
        #[kani::proof_for_contract(truncate_stage_8)]
        #[kani::stub_verified(truncate_stage_7)]
        fn check_truncate_stage_8() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_8(nanos);
        }
    }
}





::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_truncate_stage::truncate_stage_9_passes".to_owned(),
            "gallery::chrono_fractional_nanos_truncate_stage::check_truncate_stage_9".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 7, stage 9: the real target -- stubs truncate_stage_8. If this passes, the full round trip follows: this stage's own ensures is result == nanos exactly, and feeding its result into the already-proven-cheap decode loop recovers nanos by decode's own definition.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_TRUNCATE_STAGE_9_SRC, {
        #[kani::proof_for_contract(truncate_stage_9)]
        #[kani::stub_verified(truncate_stage_8)]
        fn check_truncate_stage_9() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = truncate_stage_9(nanos);
        }
    }
}
