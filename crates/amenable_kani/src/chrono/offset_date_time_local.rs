//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::DateTime<
//! chrono::Local>>`.
//!
//! The claim: every real `chrono::DateTime<chrono::Local>` round-trips
//! through the instant bridge's own reflect-then-realize pair, the
//! same claim `chrono::offset_date_time`'s own witness states for
//! `DateTime<FixedOffset>`. Once a `DateTime<Local>` value exists, it
//! already carries a concrete `FixedOffset` (`Local::Offset =
//! FixedOffset`, confirmed in chrono's own `TimeZone` impl for
//! `Local`), so this claim is about that stored (naive, offset)
//! pair's own round-trip fidelity -- not about whether it's the
//! offset the host's real timezone database would assign for this
//! moment. That second question is `Local`'s own boundary
//! (`amenable_kani::chrono::local`, docs/CHRONO_SUPPORT_PLAN.md
//! decision 13) and this claim never touches it: the symbolic witness
//! below builds `DateTime<Local>` via `DateTime::
//! from_naive_utc_and_offset` (chrono's own `TimeZone`-generic, raw
//! constructor -- no `iana-time-zone`/`winapi` lookup, confirmed
//! directly in chrono's own doctest for exactly this type: `DateTime::
//! <Local>::from_naive_utc_and_offset(naive_utc, offset)`), ranging
//! over every real offset rather than only ones a real host could
//! produce, which only makes the claim stronger, not narrower.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::{DateTime, Local};

use crate::ext_macros::ExtCheckedProof;
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<DateTime<Local>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "check_local_date_time_round_trips_through_descriptor".to_owned(),
            CHECK_LOCAL_DATE_TIME_ROUND_TRIPS_THROUGH_DESCRIPTOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<DateTime<Local>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::DateTime<chrono::Local>>",
        "kani",
        || <ExtStandard<DateTime<Local>> as crate::KaniWitness>::proof().to_string(),
    )
}

mod kani_only {
    #![cfg(kani)]

    /// `original.fixed_offset()` round-trips through the instant
    /// bridge's reflect-then-realize pair -- the `DateTime<Local>`
    /// version of `offset_date_time::OffsetDateTimeRoundTripsThrough
    /// Descriptor`, kept as its own small local claim rather than
    /// widening that one's visibility for a single extra use site
    /// (the same tradeoff `boxed_error_variant_count_first_pass_box_
    /// placement.rs`'s duplicated `Payload` already made).
    pub(super) struct LocalDateTimeRoundTripsThroughDescriptor;
    crate::rust_std::kani_local_claim!(LocalDateTimeRoundTripsThroughDescriptor);

    impl amenable_core::Ensures<crate::KaniVerifier> for LocalDateTimeRoundTripsThroughDescriptor {
        type Input = chrono::DateTime<chrono::Local>;
        type Bound = bool;

        fn ensures(original: chrono::DateTime<chrono::Local>) -> bool {
            use amenable_core::{Establish, Exchange};
            use amenable_ext::{ChronoOffsetDateTime, ChronoTimeBackend, ChronoVerifier};
            use amenable_time::{
                OffsetDateTimeProof, OffsetDateTimeProofToken, OffsetDateTimeSemanticBundle,
                OffsetDateTimeSemanticBundleToken, ProvenOffsetDateTimeCarrier,
                ReflectedOffsetDateTime, TemporalInputToken,
            };

            fn token() -> OffsetDateTimeSemanticBundleToken {
                let proof: OffsetDateTimeProofToken = <OffsetDateTimeProof as Establish<
                    TemporalInputToken,
                    ChronoVerifier,
                >>::establish(
                    TemporalInputToken::new()
                );
                <OffsetDateTimeSemanticBundle as Establish<
                    OffsetDateTimeProofToken,
                    ChronoVerifier,
                >>::establish(proof)
            }

            let fixed = original.fixed_offset();
            let backend = ChronoTimeBackend;
            let carrier = ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
                ChronoOffsetDateTime::new(fixed),
                token(),
            );
            let Ok(reflected) = backend.exchange(carrier) else {
                return false;
            };
            let reflected: ReflectedOffsetDateTime = reflected;
            let Ok(round_tripped) = backend.exchange(reflected) else {
                return false;
            };
            let round_tripped: ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime> = round_tripped;

            **round_tripped.carrier() == fixed
        }
    }
}

#[cfg(kani)]
use kani_only::LocalDateTimeRoundTripsThroughDescriptor;

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time_local::LocalDateTimeRoundTripsThroughDescriptor",
        "kani",
        "ensures",
        || "backend.exchange(reflect(original.fixed_offset())) realizes back to original.fixed_offset()",
    )
}

amenable_derive::harness! {
    kani, CHECK_LOCAL_DATE_TIME_ROUND_TRIPS_THROUGH_DESCRIPTOR_SRC, {
        /// Builds a real `chrono::DateTime<Local>` from symbolic
        /// fields, the same discipline `offset_date_time`'s own
        /// `DateTime<FixedOffset>` witness uses: every real `i32` year
        /// and `u32` month/day/hour/minute/second plus a symbolic
        /// fixed offset, skipping only the combinations chrono itself
        /// rejects as invalid. The `Local`-tagged value is built via
        /// `and_local_timezone(offset)` (identical to the
        /// `FixedOffset` witness, no host dependency -- `offset` is
        /// itself a plain `FixedOffset`, not `Local`) and then
        /// re-tagged with `from_naive_utc_and_offset`, so the value
        /// carries the exact same (naive, offset) pair either way.
        #[kani::proof]
        fn check_local_date_time_round_trips_through_descriptor() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            let offset_seconds: i32 = kani::any();

            let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return;
            };
            let Some(naive) = date.and_hms_opt(hour, minute, second) else {
                return;
            };
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let Some(fixed) = naive.and_local_timezone(offset).single() else {
                return;
            };
            let original =
                chrono::DateTime::<chrono::Local>::from_naive_utc_and_offset(fixed.naive_utc(), offset);

            assert!(
                LocalDateTimeRoundTripsThroughDescriptor::ensures(original),
                "a real local date-time must round-trip through reflect-then-realize"
            );
        }
    }
}
