//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::DateTime<
//! chrono::FixedOffset>>`.
//!
//! The claim: every real `chrono::DateTime<chrono::FixedOffset>`
//! round-trips through the instant bridge's own reflect-then-realize
//! pair (`ChronoTimeBackend`'s `Exchange<ProvenOffsetDateTimeCarrier<
//! ChronoOffsetDateTime>, ReflectedOffsetDateTime, ChronoVerifier>`
//! then its inverse) -- the same claim `amenable_ext`'s own
//! `offset_date_time_round_trips_through_real_chrono_types` integration
//! test checks on one fixed example, exercised here symbolically over
//! every real year/month/day/hour/minute/second/offset combination
//! instead.

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::{DateTime, FixedOffset};

use crate::ext_macros::ExtCheckedProof;
use crate::rust_std::bridge_kani_witness;

impl crate::KaniWitness for ExtStandard<DateTime<FixedOffset>> {
    type SupportingEvidence = Self;
    type ProofArtifact = ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        ExtCheckedProof::new(
            "check_offset_date_time_round_trips_through_descriptor".to_owned(),
            CHECK_OFFSET_DATE_TIME_ROUND_TRIPS_THROUGH_DESCRIPTOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<DateTime<FixedOffset>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::DateTime<chrono::FixedOffset>>",
        "kani",
        || <ExtStandard<DateTime<FixedOffset>> as crate::KaniWitness>::proof().to_string(),
    )
}

mod kani_only {
    #![cfg(kani)]

    pub(super) struct OffsetDateTimeRoundTripsThroughDescriptor;
    crate::rust_std::kani_local_claim!(OffsetDateTimeRoundTripsThroughDescriptor);

    impl amenable_core::Ensures<crate::KaniVerifier> for OffsetDateTimeRoundTripsThroughDescriptor {
        type Input = chrono::DateTime<chrono::FixedOffset>;
        type Bound = bool;

        fn ensures(original: chrono::DateTime<chrono::FixedOffset>) -> bool {
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

            let backend = ChronoTimeBackend;
            let carrier = ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
                ChronoOffsetDateTime::new(original),
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

            // Exact identity, not truncated: `UtcOffsetDescriptor` has
            // a `seconds` field (see its own doc comment) specifically
            // so a real backend offset with sub-minute precision --
            // confirmed real via chrono-tz's own historical data, not
            // hypothetical -- round-trips losslessly, the same as
            // every other component.
            **round_tripped.carrier() == original
        }
    }
}

#[cfg(kani)]
use kani_only::OffsetDateTimeRoundTripsThroughDescriptor;

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time::OffsetDateTimeRoundTripsThroughDescriptor",
        "kani",
        "ensures",
        || "backend.exchange(reflect(original)) realizes back to original",
    )
}

amenable_derive::harness! {
    kani, CHECK_OFFSET_DATE_TIME_ROUND_TRIPS_THROUGH_DESCRIPTOR_SRC, {
        /// Builds a real `chrono::DateTime<FixedOffset>` from symbolic
        /// fields, over every `i32` year and `u32` month/day/hour/
        /// minute/second plus a symbolic fixed offset -- skipping only
        /// the combinations chrono itself rejects as invalid (`None`
        /// from `from_ymd_opt`/`and_hms_opt`/`and_local_timezone`/
        /// `FixedOffset::east_opt`), the same discipline the civil
        /// witnesses in this crate already use.
        #[kani::proof]
        fn check_offset_date_time_round_trips_through_descriptor() {
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
            let Some(original) = naive.and_local_timezone(offset).single() else {
                return;
            };

            assert!(
                OffsetDateTimeRoundTripsThroughDescriptor::ensures(original),
                "a real offset date-time must round-trip through reflect-then-realize"
            );
        }
    }
}
