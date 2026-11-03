//! The deep half (depth 5 through 9) of `staged_encode_K`, split out
//! of `chrono_fractional_nanos_encode_staged` once that file grew
//! large enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]

    use amenable_core::{Ensures, Requires};
    use crate::{IndexRecoversTheStoredElement, ValueIsWithinInclusiveRange};
    use crate::gallery::chrono_fractional_nanos_encode_staged::staged_encode_4;



    #[kani::requires(nanos >= 0 && nanos < 100_000)]
    #[kani::ensures(|result: &[u8; 5]| {
        result[4] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
            && result[3] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
            && result[2] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0)
            && result[1] == b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0)
            && result[0] == b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0)
    })]
    pub(super) fn staged_encode_5(nanos: i64) -> [u8; 5] {
        let upper = staged_encode_4(nanos / 10);
        [
            upper[0],
            upper[1],
            upper[2],
            upper[3],
            b'0' + u8::try_from(nanos % 10).unwrap_or(0),
        ]
    }



    #[kani::requires(nanos >= 0 && nanos < 1_000_000)]
    #[kani::ensures(|result: &[u8; 6]| {
        result[5] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
            && result[4] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
            && result[3] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0)
            && result[2] == b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0)
            && result[1] == b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0)
            && result[0] == b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0)
    })]
    pub(super) fn staged_encode_6(nanos: i64) -> [u8; 6] {
        let upper = staged_encode_5(nanos / 10);
        [
            upper[0],
            upper[1],
            upper[2],
            upper[3],
            upper[4],
            b'0' + u8::try_from(nanos % 10).unwrap_or(0),
        ]
    }



    #[kani::requires(ValueIsWithinInclusiveRange::requires((nanos, 0, 9_999_999)))]
    #[kani::ensures(|result: &[u8; 7]| {
        let upper = staged_encode_6(nanos / 10);
        let new_digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
        IndexRecoversTheStoredElement::ensures((result[6], new_digit))
            && IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], upper[5]))
    })]
    pub(super) fn staged_encode_7(nanos: i64) -> [u8; 7] {
        let upper = staged_encode_6(nanos / 10);
        [
            upper[0],
            upper[1],
            upper[2],
            upper[3],
            upper[4],
            upper[5],
            b'0' + u8::try_from(nanos % 10).unwrap_or(0),
        ]
    }



    #[kani::requires(ValueIsWithinInclusiveRange::requires((nanos, 0, 99_999_999)))]
    #[kani::ensures(|result: &[u8; 8]| {
        let upper = staged_encode_7(nanos / 10);
        let new_digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
        IndexRecoversTheStoredElement::ensures((result[7], new_digit))
            && IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], upper[5]))
            && IndexRecoversTheStoredElement::ensures((result[6], upper[6]))
    })]
    pub(super) fn staged_encode_8(nanos: i64) -> [u8; 8] {
        let upper = staged_encode_7(nanos / 10);
        [
            upper[0],
            upper[1],
            upper[2],
            upper[3],
            upper[4],
            upper[5],
            upper[6],
            b'0' + u8::try_from(nanos % 10).unwrap_or(0),
        ]
    }



    #[kani::requires(ValueIsWithinInclusiveRange::requires((nanos, 0, 999_999_999)))]
    #[kani::ensures(|result: &[u8; 9]| {
        let upper = staged_encode_8(nanos / 10);
        let new_digit = b'0' + u8::try_from(nanos % 10).unwrap_or(0);
        IndexRecoversTheStoredElement::ensures((result[8], new_digit))
            && IndexRecoversTheStoredElement::ensures((result[0], upper[0]))
            && IndexRecoversTheStoredElement::ensures((result[1], upper[1]))
            && IndexRecoversTheStoredElement::ensures((result[2], upper[2]))
            && IndexRecoversTheStoredElement::ensures((result[3], upper[3]))
            && IndexRecoversTheStoredElement::ensures((result[4], upper[4]))
            && IndexRecoversTheStoredElement::ensures((result[5], upper[5]))
            && IndexRecoversTheStoredElement::ensures((result[6], upper[6]))
            && IndexRecoversTheStoredElement::ensures((result[7], upper[7]))
    })]
    pub(super) fn staged_encode_9(nanos: i64) -> [u8; 9] {
        let upper = staged_encode_8(nanos / 10);
        [
            upper[0],
            upper[1],
            upper[2],
            upper[3],
            upper[4],
            upper[5],
            upper[6],
            upper[7],
            b'0' + u8::try_from(nanos % 10).unwrap_or(0),
        ]
    }
}

#[cfg(kani)]
use kani_only::{staged_encode_5, staged_encode_6, staged_encode_7, staged_encode_8, staged_encode_9};
#[cfg(kani)]
use amenable_core::Requires;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_staged_encode_deep::staged_encode_5_passes".to_owned(),
            "gallery::chrono_fractional_nanos_staged_encode_deep::check_staged_encode_5".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, stage 5: stubs staged_encode_4's proven four-fact conjunction, adds one more position.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_5_SRC, {
        /// `staged_encode_5`'s own contract: stubs `staged_encode_4`.
        #[kani::proof_for_contract(staged_encode_5)]
        #[kani::stub_verified(crate::gallery::chrono_fractional_nanos_encode_staged::staged_encode_4)]
        fn check_staged_encode_5() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 100_000);
            let _ = staged_encode_5(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_staged_encode_deep::staged_encode_6_passes".to_owned(),
            "gallery::chrono_fractional_nanos_staged_encode_deep::check_staged_encode_6".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, stage 6: the critical test -- this is the exact stage count where the old decode_2..9 chain (ensures calling the previous stage's function a second time) triggered the duplicate-call blowup that crashed the host. Here each stage's ensures restates every position directly from nanos instead, never calling the previous stage.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_6_SRC, {
        /// `staged_encode_6`'s own contract: stubs `staged_encode_5`.
        #[kani::proof_for_contract(staged_encode_6)]
        #[kani::stub_verified(staged_encode_5)]
        fn check_staged_encode_6() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 1_000_000);
            let _ = staged_encode_6(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_staged_encode_deep::staged_encode_7_passes".to_owned(),
            "gallery::chrono_fractional_nanos_staged_encode_deep::check_staged_encode_7".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, stage 7: stubs staged_encode_6's proven six-fact conjunction, adds one more position.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_7_SRC, {
        /// `staged_encode_7`'s own contract: stubs `staged_encode_6`.
        #[kani::proof_for_contract(staged_encode_7)]
        #[kani::stub_verified(staged_encode_6)]
        fn check_staged_encode_7() {
            let nanos: i64 = kani::any();
            kani::assume(ValueIsWithinInclusiveRange::requires((nanos, 0, 9_999_999)));
            let _ = staged_encode_7(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_staged_encode_deep::staged_encode_8_passes".to_owned(),
            "gallery::chrono_fractional_nanos_staged_encode_deep::check_staged_encode_8".to_owned(),
            "amenable_kani".to_owned(),
            "redesign: the first version of this stage restated all 8 positions fresh via division from nanos directly -- 8 divisions by growing, non-power-of-2 constants in one ensures, redundantly re-deriving the 7 facts stage 7 already proved. Confirmed to time out. This version instead compares result[0..7] directly against upper[i] via a fresh call to staged_encode_7 -- but that call appears in both the body and the ensures, so the harness must also stub every stage staged_encode_7's own ensures transitively calls (staged_encode_6) or it still times out; this pattern does not scale past 8 (see staged_encode_9_passes).".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_8_SRC, {
        /// `staged_encode_8`'s own contract: stubs `staged_encode_7`.
        /// `staged_encode_7`'s own `ensures` calls `staged_encode_6`
        /// (to reference `upper[i]` directly rather than re-deriving
        /// it via division) -- when `staged_encode_7` is stubbed here,
        /// that call inside its `ensures` is not itself stubbed, so it
        /// would run for real unless `staged_encode_6` is stubbed too.
        #[kani::proof_for_contract(staged_encode_8)]
        #[kani::stub_verified(staged_encode_7)]
        #[kani::stub_verified(staged_encode_6)]
        fn check_staged_encode_8() {
            let nanos: i64 = kani::any();
            kani::assume(ValueIsWithinInclusiveRange::requires((nanos, 0, 99_999_999)));
            let _ = staged_encode_8(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_staged_encode_deep::staged_encode_9_passes".to_owned(),
            "gallery::chrono_fractional_nanos_staged_encode_deep::check_staged_encode_9".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: even with every transitively-referenced stage stubbed (8, 7, 6), this still times out. Root cause: each stage's own body AND ensures both call the previous stage (to get `upper` for the direct comparison) -- the exact duplicate-call shape that caused the original crash, just reintroduced one level up. Stage 9's two calls to stage 8 each independently re-evaluate stage 8's own ensures, which itself double-calls stage 7, which double-calls stage 6 -- multiplicative compounding across the chain, not linear. Confirmed safe to re-run (memory-bounded).".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_9_SRC, {
        /// `staged_encode_9`'s own contract: stubs `staged_encode_8`,
        /// plus every stage its own `ensures` transitively calls
        /// (`staged_encode_7`, then `staged_encode_6`) -- the same
        /// transitive-stub fix `staged_encode_8`'s own harness needed.
        #[kani::proof_for_contract(staged_encode_9)]
        #[kani::stub_verified(staged_encode_8)]
        #[kani::stub_verified(staged_encode_7)]
        #[kani::stub_verified(staged_encode_6)]
        fn check_staged_encode_9() {
            let nanos: i64 = kani::any();
            kani::assume(ValueIsWithinInclusiveRange::requires((nanos, 0, 999_999_999)));
            let _ = staged_encode_9(nanos);
        }
    }
}
