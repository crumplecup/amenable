//! Pipeline-shape isolation and offset binning, for the CBMC
//! round-trip timeout investigation -- see
//! `chrono_offset_date_time_round_trip.rs`'s own module doc comment
//! for the full narrative this file is one chapter of. Two separate
//! questions: whether *repeating* the real, already-cheap, bounded
//! `narrow_to_u8` compounds under DFCC the same way other repeated
//! calls in this investigation did (one/two/three inlined calls, then
//! the same three calls wrapped in their own trivially-contracted
//! caller -- all pass, ruling this out), and whether binning the
//! offset over a tiny concrete window rescues the full round trip the
//! way binning rescues other dimensions elsewhere in this workspace
//! (confirmed it does not -- still times out even at `WINDOW = 2`,
//! since the cost is about the offset being symbolic at all, not the
//! size of its range). The two-hop `Exchange` pipeline itself (reflect
//! then realize, no equality check) is also confirmed expensive on its
//! own here, before any comparison of the result ever happens.

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

    // ── Isolating repeated calls to the real narrow_to_u8 ───────────
    //
    // `fixed_offset_to_utc_offset_descriptor`'s own contract still
    // times out even with the arithmetic half stubbed and `narrow_to_u8`
    // (itself proven cheap alone, bounded by `requires(value <= 255)`)
    // left to inline. These isolate whether it's specifically *repeating*
    // the real, bounded `narrow_to_u8` -- not a symbolic/unbounded call,
    // not stubbing -- that compounds, the same pattern already confirmed
    // for the digit chain and the two-hop `Exchange` pipeline.

    /// One real, inlined call to `narrow_to_u8`, bounded input.
    pub(super) fn call_narrow_to_u8_once() {
        let v: u8 = kani::any();
        let _ = amenable_ext::narrow_to_u8(u32::from(v), amenable_time::TemporalComponent::Hour);
    }

    /// Two real, inlined calls in sequence.
    pub(super) fn call_narrow_to_u8_twice() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let _ = amenable_ext::narrow_to_u8(u32::from(v1), amenable_time::TemporalComponent::Hour);
        let _ = amenable_ext::narrow_to_u8(u32::from(v2), amenable_time::TemporalComponent::Minute);
    }

    /// Three real, inlined calls in sequence -- matches the real
    /// `fixed_offset_to_utc_offset_descriptor`'s own call count exactly.
    pub(super) fn call_narrow_to_u8_thrice() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = amenable_ext::narrow_to_u8(u32::from(v1), amenable_time::TemporalComponent::Hour);
        let _ = amenable_ext::narrow_to_u8(u32::from(v2), amenable_time::TemporalComponent::Minute);
        let _ = amenable_ext::narrow_to_u8(u32::from(v3), amenable_time::TemporalComponent::Second);
    }

    /// The exact same three calls, but now *itself* proven via
    /// `#[kani::proof_for_contract]` with a trivial ensures, rather
    /// than a plain `#[kani::proof]` -- isolates whether DFCC's own
    /// instrumentation, wrapping calls to an *already-contracted*
    /// function from *within* a function under contract analysis,
    /// reintroduces cost that plain repetition (no outer contract)
    /// didn't show at all.
    #[kani::requires(true)]
    #[kani::ensures(|_: &()| true)]
    pub(super) fn call_narrow_to_u8_thrice_contracted() {
        let v1: u8 = kani::any();
        let v2: u8 = kani::any();
        let v3: u8 = kani::any();
        let _ = amenable_ext::narrow_to_u8(u32::from(v1), amenable_time::TemporalComponent::Hour);
        let _ = amenable_ext::narrow_to_u8(u32::from(v2), amenable_time::TemporalComponent::Minute);
        let _ = amenable_ext::narrow_to_u8(u32::from(v3), amenable_time::TemporalComponent::Second);
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
}

#[cfg(kani)]
use kani_only::{
    call_narrow_to_u8_once, call_narrow_to_u8_thrice, call_narrow_to_u8_thrice_contracted,
    call_narrow_to_u8_twice, reflect_with_binned_offset, two_hop_exchange_no_equality,
};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_once_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_once_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "one real, inlined call to narrow_to_u8 -- the base case for isolating repetition cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CALL_NARROW_TO_U8_ONCE_ISOLATION_SRC, {
        #[kani::proof]
        fn call_narrow_to_u8_once_isolation() {
            call_narrow_to_u8_once();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_twice_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_twice_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "two real, inlined calls to narrow_to_u8 in sequence".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CALL_NARROW_TO_U8_TWICE_ISOLATION_SRC, {
        #[kani::proof]
        fn call_narrow_to_u8_twice_isolation() {
            call_narrow_to_u8_twice();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_thrice_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_thrice_isolation".to_owned(),
            "amenable_kani".to_owned(),
            "three real, inlined calls to narrow_to_u8 in sequence -- matches fixed_offset_to_utc_offset_descriptor's own call count exactly".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CALL_NARROW_TO_U8_THRICE_ISOLATION_SRC, {
        #[kani::proof]
        fn call_narrow_to_u8_thrice_isolation() {
            call_narrow_to_u8_thrice();
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_offset_date_time_round_trip::call_narrow_to_u8_thrice_contracted_isolation".to_owned(),
            "gallery::chrono_offset_date_time_round_trip::check_call_narrow_to_u8_thrice_contracted".to_owned(),
            "amenable_kani".to_owned(),
            "the exact same three narrow_to_u8 calls, but proven via proof_for_contract with a trivial ensures rather than a plain #[kani::proof] -- isolates DFCC's own instrumentation overhead on calls to an already-contracted function from within a contract-analyzed caller".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CHECK_CALL_NARROW_TO_U8_THRICE_CONTRACTED_SRC, {
        #[kani::proof_for_contract(call_narrow_to_u8_thrice_contracted)]
        fn check_call_narrow_to_u8_thrice_contracted() {
            call_narrow_to_u8_thrice_contracted();
        }
    }
}

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
