//! Digit decomposition, the two-hop day-only round trip, for the CBMC
//! round-trip timeout investigation -- see
//! `chrono_offset_date_time_round_trip.rs`'s own module doc comment
//! for the full narrative this file is one chapter of, and the
//! sibling
//! `chrono_offset_date_time_round_trip_digit_decomposition_steps.rs`
//! for the incremental digit-chain steps this builds on the same
//! staging technique from. `reflect_step_day_only`, even restricted
//! to a 31-value domain and fully contracted, exhausts memory
//! outright -- not merely times out.

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
use kani_only::reflect_step_day_only;

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
