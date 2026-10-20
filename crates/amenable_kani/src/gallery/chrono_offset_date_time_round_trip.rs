//! Gallery cases from the investigation into why `amenable_ext::ExtStandard<
//! chrono::DateTime<chrono::FixedOffset>>`'s full round-trip claim (every
//! year/month/day/hour/minute/second/offset field symbolic at once) times
//! out under Kani -- the production harness itself stays in `amenable_kani::
//! chrono::offset_date_time` (`check_offset_date_time_round_trips_through_
//! descriptor`), unresolved; this gallery records what the decomposition
//! attempts found along the way.
//!
//! Isolating by half, each other field fixed concrete:
//!
//! - naive (date+time) half alone, offset concrete: investigated, no clean
//!   confirmed verdict recorded.
//! - offset half alone, naive concrete, `offset_seconds` binned over a
//!   concrete `0..WINDOW` loop (never `kani::any()`) rather than left fully
//!   symbolic, the same bin-size-search discipline validated elsewhere in
//!   this workspace (`Tz`'s own `.parse()` investigation): **still times
//!   out even at `WINDOW = 2`**. Confirms the cost is about the offset
//!   being symbolic at all, not the size of its range -- binning, which
//!   rescues other dimensions in this codebase, does not rescue this one.
//!   Kept below as `offset_binning_times_out_even_at_the_smallest_window`.
//!
//! Isolating the pipeline shape itself, day symbolic, everything else
//! concrete:
//!
//! - a single `Exchange` call (reflect only, no realize, no equality):
//!   investigated as the minimal possible exercise of the trait, no
//!   confirmed verdict recorded separately from the two-call case below.
//! - both `Exchange` calls (reflect then realize), still with no equality
//!   check at the end: **confirmed expensive on its own** -- the two-hop
//!   call sequence is already the cost, before any comparison of the
//!   result against the original even happens. Kept below as
//!   `two_hop_exchange_without_equality_already_times_out`.
//! - the real `Exchange` call, fully concrete (no `kani::any()` at all): a
//!   sanity check confirming the trait dispatch itself is not inherently
//!   slow -- the cost only appears once a field is symbolic.
//!
//! Isolating the descriptor-building logic without `Exchange`/`Establish`
//! at all (just the real conversion code, called directly): investigated
//! as a separate layer from the trait-dispatch machinery, to rule out
//! whether `Exchange`/`Sidecar`/`Establish` themselves were the cost rather
//! than the conversion logic underneath. No confirmed verdict recorded
//! separately from the per-half findings above.
//!
//! Also investigated, as hypotheses about *why* the naive-half conversion
//! might be expensive (`naive_date_time_to_local_date_time_descriptor`'s
//! leap-second guard formats the whole symbolic value via `Display` on an
//! error path that this investigation's own construction never actually
//! reaches; whether `nanosecond()` on a symbolic date constant-folds to a
//! literal before CBMC has to reason about it) -- neither was carried to a
//! confirmed verdict before the investigation moved to the fractional-digit
//! layer specifically, below.
//!
//! **Digit decomposition, the deepest layer investigated.** The claim
//! `nanos_to_fractional_digit_bytes` / `fractional_digit_bytes_to_nanos`
//! round-trip exactly, proven directly against the pure, `Copy`, heap-free
//! `[u8; 9]` core both public string-based wrappers already reduce to
//! internally (per the project's own standing rule: strings are not
//! generally tractable for CBMC, so the round trip is proven below the
//! text-encoding layer, not through it). Restated as pure `i64` division/
//! modulo arithmetic with no byte arrays at all (`digit_i = (nanos /
//! 10^i) % 10` for `i` in `0..9`, `sum(digit_i * 10^i) == nanos`) to test
//! whether stripping the array plumbing was enough before trying anything
//! more elaborate:
//!
//! - one step of digit extraction (`remaining % 10`, `remaining / 10`),
//!   proven as its own contract relating the division only to its own
//!   input, never chained to another call: cheap, confirmed around 3.7s.
//!   Kept below as `peel_digit_contract_passes_cheaply` -- the one
//!   building block out of this whole investigation that stayed cheap at
//!   every scale tried.
//! - the full nine-digit identity, composed from nine `kani::stub_
//!   verified` calls to that one proven contract (so the composed proof
//!   only ever has to relate nine *linear* equations to each other, never
//!   nine real division circuits at once): shrank the front end from tens
//!   of thousands of symbolic-execution steps to under 5000, but did not
//!   remove the cost -- an `i128` accumulator was needed to rule out
//!   overflow as a separate confound in the composed proof itself (not in
//!   any caller), confirmed via a real, reachable overflow Kani reported
//!   inside the contract's own `ensures` closure before the widening.
//! - the combining step's own cost, isolated from `peel_digit`'s (already
//!   confirmed cheap): chaining and combining more stubbed calls in one
//!   flat proof obligation. Growing from 5 to 7 combined steps **jumped
//!   about 55x (1.74s to 96.3s)**, confirmed genuinely accelerating, not a
//!   one-off measurement. Kept below as `combining_seven_steps_passes_but_
//!   reveals_the_growth_curve` -- it still passes within the project's
//!   3-minute harness budget, but the trend it shows does not.
//! - hierarchical grouping (three digits peeled via three stubbed calls to
//!   `peel_digit`, then three of *that* group related at the top level,
//!   so no level ever relates more than three independent values) versus
//!   incremental chaining (each link stubs exactly one prior link plus one
//!   `peel_digit` call, so the combination at every step relates exactly
//!   two opaque values, no matter how long the chain grows): both tried,
//!   to see whether bounding how many values are related *at once* (not
//!   just how many steps exist overall) changes the curve. The incremental
//!   chain's own measured series: N=1 ~3s, N=2 ~3.5s, N=3 ~25-30s, N=4
//!   ~176s (extrapolated roughly 20 minutes for N=5; the series' memory
//!   use was non-monotonic across these runs -- 37, 658, 655, 70 MiB -- and
//!   not itself extrapolatable). Growth stayed super-linear regardless of
//!   topology: relating only two opaque values at a time slowed the curve
//!   but never flattened it.
//! - the two-hop round trip itself, day-only (31 possible values, not the
//!   full nine-digit `nanos` domain), staged via Kani contracts the same
//!   way the `NaiveWeek` gallery case (`chrono_naive_week_stub_
//!   composition`) staged its span claim: `reflect_step_day_only` and
//!   `realize_step_day_only` each independently contracted and proven,
//!   with the composed claim stubbing both so neither real body executes
//!   in the top-level proof. Unlike that gallery case's first (corrected)
//!   mistake, both contracts here state the exact relation, not a
//!   one-sided bound. **The contracted version of `reflect_step_day_only`
//!   exhausted memory** -- not merely timed out -- even restricted to this
//!   31-value domain. An uncontracted control with the identical body was
//!   written specifically to separate "DFCC's own instrumentation is the
//!   added cost" from "the `Exchange`-backed builder call underneath is
//!   itself the cost regardless of contracts"; a second control wrapped
//!   the same body in `requires`/`ensures` but with a trivial `|_| true`
//!   predicate, to separate "DFCC's structural instrumentation itself" from
//!   "evaluating this specific shape-predicate under CBMC." Kept below as
//!   `contracted_reflect_step_day_only_exhausts_memory`.
//!
//! **Conclusion.** No decomposition technique tried here -- byte-core
//! extraction, restating as pure arithmetic, flat grouping, hierarchical
//! grouping, incremental two-value chaining, or staged contract composition
//! with `stub_verified` -- removed the underlying cost; each only moved or
//! slowed it. The growth across the incremental chain is super-linear and,
//! by the trend these measurements show, continuing it out to the real
//! nine-digit claim implies a cost far beyond any practical CI budget, not
//! merely beyond the project's standing 3-minute harness limit. This
//! matches the same pattern the `chrono_day_count_staging` and
//! `chrono_naive_week_stub_composition` gallery cases already found for
//! other chrono claims: each piece is cheap in isolation, and it is
//! specifically the *relation* between multiple chrono-derived or
//! arithmetic values -- not any one piece alone -- that CBMC cannot finish
//! within reach. **Do not retry "decompose further" on this claim
//! expecting a finer split alone to remove the cost** -- every split tried
//! here, down to relating exactly two values at a time, still accelerated
//! with depth. The full round-trip claim remains open; the production
//! harness records that honestly as a known timeout rather than a verdict.

/// Every item here exists only for the Kani harnesses below, so the whole
/// module is gated once, rather than scattering `#[cfg(kani)]` across each
/// item individually (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    use amenable_core::{Establish, Exchange};
    use amenable_ext::{ChronoOffsetDateTime, ChronoTimeBackend, ChronoVerifier};
    use amenable_time::{
        OffsetDateTimeProof, OffsetDateTimeProofToken, OffsetDateTimeSemanticBundle,
        OffsetDateTimeSemanticBundleToken, ProvenOffsetDateTimeCarrier, TemporalInputToken,
    };

    pub(super) fn token() -> OffsetDateTimeSemanticBundleToken {
        let proof: OffsetDateTimeProofToken = <OffsetDateTimeProof as Establish<
            TemporalInputToken,
            ChronoVerifier,
        >>::establish(TemporalInputToken::new());
        <OffsetDateTimeSemanticBundle as Establish<OffsetDateTimeProofToken, ChronoVerifier>>::establish(proof)
    }

    /// Concrete, indexed window -- never `kani::any()` for
    /// `offset_seconds` itself -- the smallest useful bin size.
    pub(super) const WINDOW: i32 = 2;

    /// Everything but `offset_seconds` fully symbolic; `offset_seconds`
    /// itself ranges over a tiny concrete window. Confirmed to time out
    /// even at `WINDOW = 2`: the cost is about the offset being symbolic
    /// at all, not the size of its range.
    pub(super) fn reflect_with_binned_offset() {
        let year: i32 = kani::any();
        let month: u32 = kani::any();
        let day: u32 = kani::any();
        let hour: u32 = kani::any();
        let minute: u32 = kani::any();
        let second: u32 = kani::any();
        let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
            return;
        };
        let Some(naive) = date.and_hms_opt(hour, minute, second) else {
            return;
        };
        for offset_seconds in 0..WINDOW {
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                continue;
            };
            let Some(original) = naive.and_local_timezone(offset).single() else {
                continue;
            };

            let backend = ChronoTimeBackend;
            let carrier = ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
                ChronoOffsetDateTime::new(original),
                token(),
            );
            let Ok(reflected) = backend.exchange(carrier) else {
                continue;
            };
            let Ok(round_tripped) = backend.exchange(reflected) else {
                continue;
            };
            assert_eq!(**round_tripped.carrier(), original);
        }
    }

    /// Day symbolic only, everything else concrete: both `Exchange`
    /// calls (reflect then realize), no equality check on the result at
    /// all. Confirmed expensive on its own -- the two-hop call sequence
    /// is already the cost, before any comparison happens.
    pub(super) fn two_hop_exchange_no_equality() {
        let day: u32 = kani::any();
        let Some(date) = chrono::NaiveDate::from_ymd_opt(2024, 3, day) else {
            return;
        };
        let Some(naive) = date.and_hms_opt(12, 0, 0) else {
            return;
        };
        let offset = chrono::FixedOffset::east_opt(0).expect("0 is always a valid offset");
        let Some(original) = naive.and_local_timezone(offset).single() else {
            return;
        };

        let backend = ChronoTimeBackend;
        let carrier = ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
            ChronoOffsetDateTime::new(original),
            token(),
        );
        let Ok(reflected) = backend.exchange(carrier) else {
            return;
        };
        let _round_tripped = backend.exchange(reflected);
    }

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

    /// The day-only investigation's fixed shape (year=2024, month=3,
    /// time=12:00:00, offset=+00:00:00), reflected through the real
    /// `Exchange` pipeline, day left symbolic. The contracted version of
    /// this function exhausted memory under Kani -- not merely timed out
    /// -- even restricted to this 31-value domain.
    #[kani::requires(true)]
    #[kani::ensures(|_result: &Option<amenable_time::ReflectedOffsetDateTime>| true)]
    pub(super) fn reflect_step_day_only(day: u8) -> Option<amenable_time::ReflectedOffsetDateTime> {
        let date = chrono::NaiveDate::from_ymd_opt(2024, 3, u32::from(day))?;
        let naive = date.and_hms_opt(12, 0, 0)?;
        let offset = chrono::FixedOffset::east_opt(0).expect("0 is always a valid offset");
        let original = naive.and_local_timezone(offset).single()?;

        let backend = ChronoTimeBackend;
        let carrier = ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
            ChronoOffsetDateTime::new(original),
            token(),
        );
        backend.exchange(carrier).ok()
    }
}

#[cfg(kani)]
use kani_only::{
    combine_seven_steps, peel_digit, reflect_step_day_only, reflect_with_binned_offset,
    two_hop_exchange_no_equality,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::offset_binning_times_out_even_at_the_smallest_window".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::offset_binning_times_out_even_at_the_smallest_window".to_owned(),
            "amenable_kani".to_owned(),
            "binning offset_seconds over a concrete WINDOW=2 loop, rather than leaving it fully symbolic, still times out -- the cost is the offset being symbolic at all, not the size of its range".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, OFFSET_BINNING_TIMES_OUT_EVEN_AT_THE_SMALLEST_WINDOW_SRC, {
        /// `offset_seconds` binned over a concrete `WINDOW = 2` loop,
        /// every other field symbolic. Confirmed to time out even at
        /// this smallest useful window.
        #[kani::proof]
        fn offset_binning_times_out_even_at_the_smallest_window() {
            reflect_with_binned_offset();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::two_hop_exchange_without_equality_already_times_out".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::two_hop_exchange_without_equality_already_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "reflect then realize, day symbolic, with no equality check on the result at all, is already expensive -- the two-hop call sequence is the cost, before any comparison happens".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TWO_HOP_EXCHANGE_WITHOUT_EQUALITY_ALREADY_TIMES_OUT_SRC, {
        /// Reflect then realize, day symbolic, with no equality check
        /// on the result. Confirmed expensive on its own -- the two-hop
        /// call sequence is the cost, before any comparison happens.
        #[kani::proof]
        fn two_hop_exchange_without_equality_already_times_out() {
            two_hop_exchange_no_equality();
        }
    }
}

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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::contracted_reflect_step_day_only_exhausts_memory".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::contracted_reflect_step_day_only_exhausts_memory".to_owned(),
            "amenable_kani".to_owned(),
            "DFCC-contracting the real Exchange-backed reflect step, day-only (31 values), exhausts memory under Kani rather than merely timing out, even under a trivial ensures predicate".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CONTRACTED_REFLECT_STEP_DAY_ONLY_EXHAUSTS_MEMORY_SRC, {
        /// `reflect_step_day_only`'s own trivial-ensures contract,
        /// day-only (31 values). Confirmed to exhaust memory under
        /// Kani, not merely time out.
        #[kani::proof_for_contract(reflect_step_day_only)]
        fn contracted_reflect_step_day_only_exhausts_memory() {
            let day: u8 = kani::any();
            let _ = reflect_step_day_only(day);
        }
    }
}
