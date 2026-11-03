//! The encode-direction isolation hypotheses: `encode_flat_loop_value_
//! exact` (hypothesis 2's loop rewrite, now with a value-exact ensures
//! instead of just discarding the result) and `staged_encode_K`
//! (recursive staging, value-exact at each depth). The remaining
//! encode-direction variants -- combine chains, prefix-carrying
//! chains, truncation stages -- moved to their own files
//! (`chrono_fractional_nanos_combine_digit`, `..._combine_prefix`,
//! `..._truncate_stage`, `..._full_prefix`) once this one grew large
//! enough to be unnavigable as one unit.

mod kani_only {
    #![cfg(kani)]



    /// Hypothesis 3: the same loop body, but with a value-exact
    /// `#[kani::ensures]` -- all 9 positions, each a direct closed-
    /// form digit extraction from `nanos` -- no decode call, no
    /// recursion, no loop inside the ensures itself.
    #[kani::requires(nanos >= 0 && nanos <= 999_999_999)]
    #[kani::ensures(|result: &[u8; 9]| {
        result[8] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
            && result[7] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
            && result[6] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0)
            && result[5] == b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0)
            && result[4] == b'0' + u8::try_from((nanos / 10_000) % 10).unwrap_or(0)
            && result[3] == b'0' + u8::try_from((nanos / 100_000) % 10).unwrap_or(0)
            && result[2] == b'0' + u8::try_from((nanos / 1_000_000) % 10).unwrap_or(0)
            && result[1] == b'0' + u8::try_from((nanos / 10_000_000) % 10).unwrap_or(0)
            && result[0] == b'0' + u8::try_from((nanos / 100_000_000) % 10).unwrap_or(0)
    })]
    pub(super) fn encode_flat_loop_value_exact(nanos: i64) -> [u8; 9] {
        let mut digits = [b'0'; 9];
        let mut remaining = nanos;
        for digit in digits.iter_mut().rev() {
            *digit = b'0' + u8::try_from(remaining % 10).unwrap_or(0);
            remaining /= 10;
        }
        digits
    }


    // ── Hypothesis 4: stage the *comparison itself*, growing the fact
    // count one position at a time -- the exact shape that resolved
    // `offset_parts_2 -> offset_parts_3 -> offset_parts_4`'s own
    // fact-count wall. Each stage's own `ensures` restates every
    // position *directly from `nanos`*, never by calling the previous
    // stage's function (that was decode_2..9's mistake). Only the
    // *body* stubs the previous stage, for efficiency computing the
    // result -- the ensures and the body are independent at every
    // stage, so there is no duplicate-substitution compounding.
    // Hypothesis 3 tried to prove all 9 facts in one unstaged shot;
    // this builds them up 1, 2, 3, ... the way the offset fix did.

    #[kani::requires(nanos >= 0 && nanos < 10)]
    #[kani::ensures(|result: &[u8; 1]| {
        result[0] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
    })]
    pub(super) fn staged_encode_1(nanos: i64) -> [u8; 1] {
        [b'0' + u8::try_from(nanos % 10).unwrap_or(0)]
    }


    #[kani::requires(nanos >= 0 && nanos < 100)]
    #[kani::ensures(|result: &[u8; 2]| {
        result[1] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
            && result[0] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
    })]
    pub(super) fn staged_encode_2(nanos: i64) -> [u8; 2] {
        let upper = staged_encode_1(nanos / 10);
        [upper[0], b'0' + u8::try_from(nanos % 10).unwrap_or(0)]
    }


    #[kani::requires(nanos >= 0 && nanos < 1_000)]
    #[kani::ensures(|result: &[u8; 3]| {
        result[2] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
            && result[1] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
            && result[0] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0)
    })]
    pub(super) fn staged_encode_3(nanos: i64) -> [u8; 3] {
        let upper = staged_encode_2(nanos / 10);
        [upper[0], upper[1], b'0' + u8::try_from(nanos % 10).unwrap_or(0)]
    }


    #[kani::requires(nanos >= 0 && nanos < 10_000)]
    #[kani::ensures(|result: &[u8; 4]| {
        result[3] == b'0' + u8::try_from(nanos % 10).unwrap_or(0)
            && result[2] == b'0' + u8::try_from((nanos / 10) % 10).unwrap_or(0)
            && result[1] == b'0' + u8::try_from((nanos / 100) % 10).unwrap_or(0)
            && result[0] == b'0' + u8::try_from((nanos / 1_000) % 10).unwrap_or(0)
    })]
    pub(crate) fn staged_encode_4(nanos: i64) -> [u8; 4] {
        let upper = staged_encode_3(nanos / 10);
        [
            upper[0],
            upper[1],
            upper[2],
            b'0' + u8::try_from(nanos % 10).unwrap_or(0),
        ]
    }}

#[cfg(kani)]
pub(crate) use kani_only::staged_encode_4;
#[cfg(kani)]
use kani_only::{encode_flat_loop_value_exact, staged_encode_1, staged_encode_2, staged_encode_3};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_encode_staged::encode_flat_loop_value_exact_passes".to_owned(),
            "gallery::chrono_fractional_nanos_encode_staged::check_encode_flat_loop_value_exact".to_owned(),
            "amenable_kani".to_owned(),
            "false trail: a loop-shaped body does not rescue the 9-independent-facts ensures shape -- proof_for_contract against a value-exact, decode-free, closed-form 9-position ensures still times out on the loop-rewritten encode, just as it did on the old recursive depth-chain. Confirms the independent-fact-count cost is inherent to proof_for_contract's own DFCC checking of a multi-position array result, not specific to recursion. Confirmed safe to re-run.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_ENCODE_FLAT_LOOP_VALUE_EXACT_SRC, {
        /// `encode_flat_loop_value_exact`'s own contract: the base
        /// case, no stubbing.
        #[kani::proof_for_contract(encode_flat_loop_value_exact)]
        fn check_encode_flat_loop_value_exact() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos <= 999_999_999);
            let _ = encode_flat_loop_value_exact(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_encode_staged::staged_encode_1_passes".to_owned(),
            "gallery::chrono_fractional_nanos_encode_staged::check_staged_encode_1".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, base case: the one-digit base case, no stubbing. Required so staged_encode_2's own #[kani::stub_verified] target has a corresponding #[kani::proof_for_contract] harness.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_1_SRC, {
        /// `staged_encode_1`'s own contract: the base case.
        #[kani::proof_for_contract(staged_encode_1)]
        fn check_staged_encode_1() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 10);
            let _ = staged_encode_1(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_encode_staged::staged_encode_2_passes".to_owned(),
            "gallery::chrono_fractional_nanos_encode_staged::check_staged_encode_2".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, stage 2 only: staging the comparison's own fact count the way offset_parts_2->3->4 did -- this stage's ensures restates both positions directly from nanos (never calling staged_encode_1), while only the body stubs staged_encode_1. Testing the smallest real stage before building further.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_2_SRC, {
        /// `staged_encode_2`'s own contract: stubs `staged_encode_1`.
        #[kani::proof_for_contract(staged_encode_2)]
        #[kani::stub_verified(staged_encode_1)]
        fn check_staged_encode_2() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 100);
            let _ = staged_encode_2(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_encode_staged::staged_encode_3_passes".to_owned(),
            "gallery::chrono_fractional_nanos_encode_staged::check_staged_encode_3".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, stage 3: stubs staged_encode_2's proven two-fact conjunction, adds one more position. Ensures restates all three positions directly from nanos, never calling staged_encode_2.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_3_SRC, {
        /// `staged_encode_3`'s own contract: stubs `staged_encode_2`.
        #[kani::proof_for_contract(staged_encode_3)]
        #[kani::stub_verified(staged_encode_2)]
        fn check_staged_encode_3() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 1_000);
            let _ = staged_encode_3(nanos);
        }
    }
}







::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_fractional_nanos_encode_staged::staged_encode_4_passes".to_owned(),
            "gallery::chrono_fractional_nanos_encode_staged::check_staged_encode_4".to_owned(),
            "amenable_kani".to_owned(),
            "hypothesis 4, stage 4: stubs staged_encode_3's proven three-fact conjunction, adds one more position.".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_STAGED_ENCODE_4_SRC, {
        /// `staged_encode_4`'s own contract: stubs `staged_encode_3`.
        #[kani::proof_for_contract(staged_encode_4)]
        #[kani::stub_verified(staged_encode_3)]
        fn check_staged_encode_4() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 10_000);
            let _ = staged_encode_4(nanos);
        }
    }
}


