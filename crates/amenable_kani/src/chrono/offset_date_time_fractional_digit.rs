//! Verifies the real `amenable_ext` fractional-nanosecond digit core
//! contracts: `nanos -> [u8; 9] -> nanos` round trip, the staged
//! `fractional_digit_bytes_depth_K`/`combine_digit_K` decomposition
//! each direction's own contract composes from, and the calendar-half
//! integral-second civil descriptor contract. Split out of
//! `offset_date_time.rs` once that file grew large enough to be
//! unnavigable as one unit.

#[cfg(kani)]
use amenable_core::Requires;

amenable_derive::harness! {
    kani, CHECK_NAIVE_DATE_TIME_TO_LOCAL_DATE_TIME_DESCRIPTOR_NO_FRACTION_SRC, {
        /// `naive_date_time_to_local_date_time_descriptor_no_fraction`'s
        /// own contract -- first-ever standalone attempt at the civil
        /// (date/time) half of the round-trip investigation, restricted
        /// to the integral-second case. No stubbing yet: every call
        /// here (`narrow_to_u8_in_range`, both builders) is cheap and
        /// already proven or trivial on its own; this tests whether
        /// the civil bridge's own composition is tractable at all
        /// before composing it with anything else.
        #[kani::proof_for_contract(amenable_ext::naive_date_time_to_local_date_time_descriptor_no_fraction)]
        fn check_naive_date_time_to_local_date_time_descriptor_no_fraction() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return;
            };
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            let Some(time) = chrono::NaiveTime::from_hms_opt(hour, minute, second) else {
                return;
            };
            let datetime = chrono::NaiveDateTime::new(date, time);
            let _ = amenable_ext::naive_date_time_to_local_date_time_descriptor_no_fraction(datetime);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_TO_NANOS_SRC, {
        /// `fractional_digit_bytes_to_nanos`'s own real contract
        /// (declared on the function itself in `amenable_ext`),
        /// proven directly -- confirmed cheap (83s, 0 of 145 failed).
        ///
        /// A sibling gallery case
        /// (`gallery::chrono_fractional_nanos_decode_chain`) documents
        /// two false trails that genuinely do time out: a nested-
        /// wrapper chain with a duplicated stub call, and a flat loop
        /// calling a stubbed one-digit lemma once per iteration via
        /// `#[kani::stub_verified]`. Both rewrite the loop body to call
        /// a helper function so it can be stubbed; `#[kani::stub_
        /// verified]`'s own per-call-site precondition re-verification
        /// is what compounds across the 9 chained calls in either
        /// form. Neither false trail is this harness: the real
        /// function's loop body calls nothing (just inline `i64`
        /// arithmetic), so there is nothing to stub and no chaining
        /// for the contract machinery to compound over -- this is the
        /// direct, unstubbed form, never actually tried before despite
        /// an earlier, broader claim in this harness's own prior
        /// history that every form had been.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_to_nanos)]
        fn check_fractional_digit_bytes_to_nanos() {
            let digits: [u8; 9] = kani::any();
            let _ = amenable_ext::fractional_digit_bytes_to_nanos(&digits);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_1_SRC, {
        /// Stage 1's own contract: the base case, no stubbing.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_1)]
        fn check_fractional_digit_bytes_depth_1() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 10);
            let _ = amenable_ext::fractional_digit_bytes_depth_1(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_2_SRC, {
        /// Stage 2's own contract: stubs stage 1.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_2)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_1)]
        fn check_fractional_digit_bytes_depth_2() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 100);
            let _ = amenable_ext::fractional_digit_bytes_depth_2(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_3_SRC, {
        /// Stage 3's own contract: stubs stage 2.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_3)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_2)]
        fn check_fractional_digit_bytes_depth_3() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 1_000);
            let _ = amenable_ext::fractional_digit_bytes_depth_3(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_4_SRC, {
        /// Stage 4's own contract: stubs stage 3.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_4)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_3)]
        fn check_fractional_digit_bytes_depth_4() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 10_000);
            let _ = amenable_ext::fractional_digit_bytes_depth_4(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_5_SRC, {
        /// Stage 5's own contract: stubs stage 4.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_5)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_4)]
        fn check_fractional_digit_bytes_depth_5() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 100_000);
            let _ = amenable_ext::fractional_digit_bytes_depth_5(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_6_SRC, {
        /// Stage 6's own contract: stubs stage 5.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_6)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_5)]
        fn check_fractional_digit_bytes_depth_6() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 1_000_000);
            let _ = amenable_ext::fractional_digit_bytes_depth_6(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_7_SRC, {
        /// Stage 7's own contract: stubs stage 6.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_7)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_6)]
        fn check_fractional_digit_bytes_depth_7() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 10_000_000);
            let _ = amenable_ext::fractional_digit_bytes_depth_7(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_8_SRC, {
        /// Stage 8's own contract: stubs stage 7.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_8)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_7)]
        fn check_fractional_digit_bytes_depth_8() {
            let nanos: i64 = kani::any();
            kani::assume(nanos >= 0 && nanos < 100_000_000);
            let _ = amenable_ext::fractional_digit_bytes_depth_8(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_BYTES_DEPTH_9_SRC, {
        /// Stage 9's own contract: stubs stage 8 -- the full
        /// nine-digit nanosecond domain.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_bytes_depth_9)]
        #[kani::stub_verified(amenable_ext::fractional_digit_bytes_depth_8)]
        fn check_fractional_digit_bytes_depth_9() {
            let nanos: i64 = kani::any();
            kani::assume(crate::ValueIsWithinInclusiveRange::<i64>::requires((
                nanos,
                0,
                999_999_999,
            )));
            let _ = amenable_ext::fractional_digit_bytes_depth_9(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_FRACTIONAL_DIGIT_ROUND_TRIP_SRC, {
        /// FLAGGED, NOT YET PASSING (2026-11-01): this harness does not
        /// exist at any committed revision, and a real re-run (3m, then
        /// 6m `--harness-timeout`) times out both times with CBMC
        /// incrementally re-solving a growing SAT instance (1.49M
        /// variables, 85M+ clauses) -- an unbounded-loop-unwind
        /// signature, not a near miss. An earlier version of this doc
        /// comment claimed the opposite ("already confirmed cheap
        /// standalone"); that claim did not hold up under direct
        /// re-verification and should not be trusted. Likely cause:
        /// `ValidFractionalDigits::any()` calls `fractional_digit_
        /// bytes_depth_9`, which fully inlines all 9 `depth_1..8`
        /// stages with no stubbing -- the same "chaining distinct
        /// functions without stubbing compounds cost" shape this
        /// investigation already found and fixed elsewhere (`combine_
        /// digit_K`'s own peeling chain). Likely fix: give `Arbitrary::
        /// any()` the same stub_verified/peeling treatment. Deliberately
        /// not fixed now -- flagged and deferred so cordial cleanup work
        /// doesn't turn into open-ended proof-architecture work.
        #[kani::proof_for_contract(amenable_ext::fractional_digit_round_trip)]
        fn check_fractional_digit_round_trip() {
            let valid: amenable_ext::ValidFractionalDigits = kani::any();
            let _ = amenable_ext::fractional_digit_round_trip(valid);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_2_SRC, {
        /// `combine_digit_2`'s own contract: `upper` is a parameter,
        /// not recomputed via a call -- a base case, nothing to stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_2)]
        fn check_combine_digit_2() {
            let upper: [u8; 1] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_2(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_3_SRC, {
        /// `combine_digit_3`'s own contract: same shape as
        /// `combine_digit_2`, one position larger. A base case,
        /// nothing to stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_3)]
        fn check_combine_digit_3() {
            let upper: [u8; 2] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_3(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_4_SRC, {
        /// `combine_digit_4`'s own contract: a base case, nothing to
        /// stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_4)]
        fn check_combine_digit_4() {
            let upper: [u8; 3] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_4(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_5_SRC, {
        /// `combine_digit_5`'s own contract: a base case, nothing to
        /// stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_5)]
        fn check_combine_digit_5() {
            let upper: [u8; 4] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_5(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_6_SRC, {
        /// `combine_digit_6`'s own contract: a base case, nothing to
        /// stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_6)]
        fn check_combine_digit_6() {
            let upper: [u8; 5] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_6(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_7_SRC, {
        /// `combine_digit_7`'s own contract: a base case, nothing to
        /// stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_7)]
        fn check_combine_digit_7() {
            let upper: [u8; 6] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_7(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_8_SRC, {
        /// `combine_digit_8`'s own contract: a base case, nothing to
        /// stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_8)]
        fn check_combine_digit_8() {
            let upper: [u8; 7] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_8(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_COMBINE_DIGIT_9_SRC, {
        /// `combine_digit_9`'s own contract: the full nine-position
        /// combine step. A base case, nothing to stub.
        #[kani::proof_for_contract(amenable_ext::combine_digit_9)]
        fn check_combine_digit_9() {
            let upper: [u8; 8] = kani::any();
            let new_digit: u8 = kani::any();
            let _ = amenable_ext::combine_digit_9(upper, new_digit);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_NANOS_TO_FRACTIONAL_DIGIT_BYTES_CONTRACT_SRC, {
        /// `nanos_to_fractional_digit_bytes`'s own native
        /// `#[kani::requires]`/`#[kani::ensures]` contract, now backed
        /// by the `combine_digit_2..9` chain (each independently
        /// proven above, fully value-exact over every byte position --
        /// unlike `fractional_digit_bytes_depth_9`, which only pins the
        /// newest byte and so is unsound to stub for this purpose).
        /// Stubbing all 8 leaves (none of which call each other, so no
        /// nesting/compounding is possible -- confirmed separately that
        /// chaining distinct, growing-conjunct-ensures functions is
        /// exponential only when *inlined*, not when stubbed) closes
        /// the previously-documented open timeout: this now succeeds,
        /// establishing `fractional_digit_bytes_to_nanos(nanos_to_
        /// fractional_digit_bytes(nanos)) == nanos` as a real proof
        /// obligation, not a trusted assumption.
        #[kani::proof_for_contract(amenable_ext::nanos_to_fractional_digit_bytes)]
        #[kani::stub_verified(amenable_ext::combine_digit_2)]
        #[kani::stub_verified(amenable_ext::combine_digit_3)]
        #[kani::stub_verified(amenable_ext::combine_digit_4)]
        #[kani::stub_verified(amenable_ext::combine_digit_5)]
        #[kani::stub_verified(amenable_ext::combine_digit_6)]
        #[kani::stub_verified(amenable_ext::combine_digit_7)]
        #[kani::stub_verified(amenable_ext::combine_digit_8)]
        #[kani::stub_verified(amenable_ext::combine_digit_9)]
        fn check_nanos_to_fractional_digit_bytes_contract() {
            let nanos: i64 = kani::any();
            kani::assume(crate::ValueIsWithinInclusiveRange::<i64>::requires((
                nanos,
                0,
                999_999_999,
            )));
            let _ = amenable_ext::nanos_to_fractional_digit_bytes(nanos);
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_NANOS_TO_FRACTIONAL_SECONDS_DIGITS_CONTRACT_SRC, {
        /// `nanos_to_fractional_seconds_digits`'s own contract: stubs
        /// `nanos_to_fractional_digit_bytes` (already proven above),
        /// isolating the trim+render+re-parse claim
        /// (`fractional_seconds_digits_to_nanos(result) == Ok(nanos)`)
        /// from any larger caller's own state. This is the piece that
        /// `check_naive_date_time_to_local_date_time_descriptor_
        /// infallible` stubs instead of inlining, closing a real OOM
        /// (15,266 VCCs, 616,793-step program) caused by the
        /// trailing-zero scan and `char`-collecting `String` build
        /// running inline over stub-generated, symbolic bytes inside
        /// a much larger proof that also builds a real `chrono`
        /// date/time from symbolic fields.
        #[kani::proof_for_contract(amenable_ext::nanos_to_fractional_seconds_digits)]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        fn check_nanos_to_fractional_seconds_digits_contract() {
            let nanos: i64 = kani::any();
            kani::assume(crate::ValueIsWithinInclusiveRange::<i64>::requires((
                nanos,
                0,
                999_999_999,
            )));
            let _ = amenable_ext::nanos_to_fractional_seconds_digits(nanos);
        }
    }
}

