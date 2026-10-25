//! Digit decomposition, incremental digit-chain steps, for the CBMC
//! round-trip timeout investigation -- see
//! `chrono_offset_date_time_round_trip.rs`'s own module doc comment
//! for the full narrative this file is one chapter of. One step of
//! digit extraction (`peel_digit`, cheap on its own at every scale
//! tried); nine named, individually registered incremental steps
//! (`digit_chain_step_1`-`digit_chain_step_9`, each stubbing only the
//! immediately preceding step's already-proven contract) resolving
//! the full nine-digit fractional-digit identity cheaply, with the
//! cost staying flat at every step; and a flat-grouped combining
//! variant (`combine_seven_steps`) showing the super-linear growth
//! curve that incremental staging avoids. The two-hop day-only round
//! trip lives in the sibling
//! `chrono_offset_date_time_round_trip_digit_decomposition_day_only.rs`.

mod kani_only {
    #![cfg(kani)]

    /// One step of digit extraction: `remaining % 10` (the digit) and
    /// `remaining / 10` (what's left). The contract relates the single
    /// division back to its own input only, never to any other call --
    /// the one piece of this whole investigation that stayed cheap at
    /// every scale tried (confirmed around 3.7s).
    #[kani::requires(remaining >= 0)]
    #[kani::ensures(|result: &(i64, i64)| {
        // `i128`, not `i64`: when `stub_verified` substitutes this
        // contract elsewhere, it evaluates this expression against an
        // otherwise-unconstrained candidate `new_remaining` before the
        // equality filters it out -- an `i64` `new_remaining * 10` can
        // overflow for such a candidate (confirmed: a real, reachable
        // overflow inside this closure, not in any composed caller).
        let (digit, new_remaining) = *result;
        i128::from(remaining) == i128::from(new_remaining) * 10 + i128::from(digit)
            && digit >= 0
            && digit <= 9
    })]
    pub(super) fn peel_digit(remaining: i64) -> (i64, i64) {
        (remaining % 10, remaining / 10)
    }

    /// Chains and combines `STEPS` stubbed calls to `peel_digit` in one
    /// flat proof obligation -- isolating the *combining* step's own
    /// cost from `peel_digit`'s (already confirmed cheap alone). Growing
    /// this from 5 to 7 steps jumped about 55x (1.74s to 96.3s),
    /// confirmed genuinely accelerating, not a one-off measurement. Kept
    /// at 7 (96.3s), not higher: the next data point in this series
    /// (growing further) would exceed the project's 3-minute harness
    /// budget, matching the standing "no manual unwind/timeout bounds"
    /// discipline -- this case records the trend, not the limit.
    pub(super) fn combine_seven_steps() -> i64 {
        const STEPS: u32 = 7;
        let bound: i64 = 10i64.pow(STEPS);
        let nanos: i64 = kani::any();
        kani::assume(nanos >= 0 && nanos < bound);

        let mut remaining = nanos;
        let mut reconstructed: i64 = 0;
        let mut pow10: i64 = 1;
        for _ in 0..STEPS {
            let (digit, new_remaining) = peel_digit(remaining);
            reconstructed += digit * pow10;
            pow10 *= 10;
            remaining = new_remaining;
        }
        assert_eq!(
            reconstructed, nanos,
            "combined steps must reconstruct exactly"
        );
        reconstructed
    }

    /// Incremental digit-chain reconstruction, step 1: the lowest digit
    /// alone, `0 <= x < 10`. Proven directly against `peel_digit`'s own
    /// contract -- the base case the rest of the chain stubs.
    #[kani::requires(x >= 0 && x < 10)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_1(x: i64) -> i64 {
        let (digit, _remaining) = peel_digit(x);
        digit
    }

    /// Step 2: stubs step 1's own proven contract for the upper digit,
    /// peels one more digit itself. `0 <= x < 100`.
    #[kani::requires(x >= 0 && x < 100)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_2(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_1(remaining);
        upper * 10 + digit
    }

    /// Step 3: stubs step 2. `0 <= x < 1000`.
    #[kani::requires(x >= 0 && x < 1000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_3(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_2(remaining);
        upper * 10 + digit
    }

    /// Step 4: stubs step 3. `0 <= x < 10_000`.
    #[kani::requires(x >= 0 && x < 10_000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_4(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_3(remaining);
        upper * 10 + digit
    }

    /// Step 5: stubs step 4. `0 <= x < 100_000`.
    #[kani::requires(x >= 0 && x < 100_000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_5(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_4(remaining);
        upper * 10 + digit
    }

    /// Step 6: stubs step 5. `0 <= x < 1_000_000`.
    #[kani::requires(x >= 0 && x < 1_000_000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_6(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_5(remaining);
        upper * 10 + digit
    }

    /// Step 7: stubs step 6. `0 <= x < 10_000_000`.
    #[kani::requires(x >= 0 && x < 10_000_000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_7(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_6(remaining);
        upper * 10 + digit
    }

    /// Step 8: stubs step 7. `0 <= x < 100_000_000`.
    #[kani::requires(x >= 0 && x < 100_000_000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_8(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_7(remaining);
        upper * 10 + digit
    }

    /// Step 9: stubs step 8. `0 <= x < 1_000_000_000` -- the full
    /// nine-digit nanosecond domain, the real claim this whole
    /// investigation chased.
    #[kani::requires(x >= 0 && x < 1_000_000_000)]
    #[kani::ensures(|result: &i64| i128::from(*result) == i128::from(x))]
    pub(super) fn digit_chain_step_9(x: i64) -> i64 {
        let (digit, remaining) = peel_digit(x);
        let upper = digit_chain_step_8(remaining);
        upper * 10 + digit
    }
}

#[cfg(kani)]
use kani_only::{
    combine_seven_steps, digit_chain_step_1, digit_chain_step_2, digit_chain_step_3,
    digit_chain_step_4, digit_chain_step_5, digit_chain_step_6, digit_chain_step_7,
    digit_chain_step_8, digit_chain_step_9, peel_digit,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::peel_digit_contract_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_peel_digit_contract".to_owned(),
            "amenable_kani".to_owned(),
            "a single digit-extraction step, contracted against its own input only (never chained), passes cheaply -- the one building block in this investigation that stayed cheap at every scale tried".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_PEEL_DIGIT_CONTRACT_SRC, {
        /// `peel_digit`'s own contract, relating the division only to
        /// its own input, never chained. Passes cheaply.
        #[kani::proof_for_contract(peel_digit)]
        fn check_peel_digit_contract() {
            let remaining: i64 = kani::any();
            kani::assume(remaining >= 0);
            let _ = peel_digit(remaining);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_1_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_1".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 1 (the base case): reconstructs a single digit, 0 <= x < 10, directly against peel_digit's contract".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_1_SRC, {
        /// Step 1's own contract: a single digit, `0 <= x < 10`.
        #[kani::proof_for_contract(digit_chain_step_1)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_1() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 10);
            let _ = digit_chain_step_1(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_2_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_2".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 2: stubs step 1's proven contract for the upper digit, peels one more itself, 0 <= x < 100".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_2_SRC, {
        /// Step 2's own contract: stubs step 1, `0 <= x < 100`.
        #[kani::proof_for_contract(digit_chain_step_2)]
        #[kani::stub_verified(digit_chain_step_1)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_2() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 100);
            let _ = digit_chain_step_2(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_3_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_3".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 3: stubs step 2's proven contract, 0 <= x < 1000".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_3_SRC, {
        /// Step 3's own contract: stubs step 2, `0 <= x < 1000`.
        #[kani::proof_for_contract(digit_chain_step_3)]
        #[kani::stub_verified(digit_chain_step_2)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_3() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 1000);
            let _ = digit_chain_step_3(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_4_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_4".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 4: stubs step 3's proven contract, 0 <= x < 10_000".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_4_SRC, {
        /// Step 4's own contract: stubs step 3, `0 <= x < 10_000`.
        #[kani::proof_for_contract(digit_chain_step_4)]
        #[kani::stub_verified(digit_chain_step_3)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_4() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 10_000);
            let _ = digit_chain_step_4(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_5_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_5".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 5: stubs step 4's proven contract, 0 <= x < 100_000".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_5_SRC, {
        /// Step 5's own contract: stubs step 4, `0 <= x < 100_000`.
        #[kani::proof_for_contract(digit_chain_step_5)]
        #[kani::stub_verified(digit_chain_step_4)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_5() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 100_000);
            let _ = digit_chain_step_5(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_6_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_6".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 6: stubs step 5's proven contract, 0 <= x < 1_000_000".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_6_SRC, {
        /// Step 6's own contract: stubs step 5, `0 <= x < 1_000_000`.
        #[kani::proof_for_contract(digit_chain_step_6)]
        #[kani::stub_verified(digit_chain_step_5)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_6() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 1_000_000);
            let _ = digit_chain_step_6(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_7_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_7".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 7: stubs step 6's proven contract, 0 <= x < 10_000_000".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_7_SRC, {
        /// Step 7's own contract: stubs step 6, `0 <= x < 10_000_000`.
        #[kani::proof_for_contract(digit_chain_step_7)]
        #[kani::stub_verified(digit_chain_step_6)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_7() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 10_000_000);
            let _ = digit_chain_step_7(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_8_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_8".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 8: stubs step 7's proven contract, 0 <= x < 100_000_000".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_8_SRC, {
        /// Step 8's own contract: stubs step 7, `0 <= x < 100_000_000`.
        #[kani::proof_for_contract(digit_chain_step_8)]
        #[kani::stub_verified(digit_chain_step_7)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_8() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 100_000_000);
            let _ = digit_chain_step_8(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::digit_chain_step_9_passes_cheaply".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_digit_chain_step_9".to_owned(),
            "amenable_kani".to_owned(),
            "incremental digit-chain, step 9: stubs step 8's proven contract, 0 <= x < 1_000_000_000 -- the full nine-digit nanosecond domain".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_DIGIT_CHAIN_STEP_9_SRC, {
        /// Step 9's own contract: stubs step 8, `0 <= x < 1_000_000_000`
        /// -- the full nine-digit nanosecond domain.
        #[kani::proof_for_contract(digit_chain_step_9)]
        #[kani::stub_verified(digit_chain_step_8)]
        #[kani::stub_verified(peel_digit)]
        fn check_digit_chain_step_9() {
            let x: i64 = kani::any();
            kani::assume(x >= 0 && x < 1_000_000_000);
            let _ = digit_chain_step_9(x);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::combining_seven_steps_passes_but_reveals_the_growth_curve".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::combining_seven_steps_passes_but_reveals_the_growth_curve".to_owned(),
            "amenable_kani".to_owned(),
            "chaining and combining 7 stubbed peel_digit calls in one flat proof obligation passes (96.3s), but growing from 5 to 7 steps jumped about 55x (1.74s to 96.3s) -- the trend, not this one run, is the finding".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMBINING_SEVEN_STEPS_PASSES_BUT_REVEALS_THE_GROWTH_CURVE_SRC, {
        /// Seven stubbed `peel_digit` calls, chained and combined in
        /// one flat proof obligation. Passes (96.3s), but growing from
        /// 5 to 7 steps jumped about 55x over the same technique --
        /// the trend, not this one run, is the finding.
        #[kani::proof]
        #[kani::stub_verified(peel_digit)]
        fn combining_seven_steps_passes_but_reveals_the_growth_curve() {
            let _ = combine_seven_steps();
        }
    }
}
