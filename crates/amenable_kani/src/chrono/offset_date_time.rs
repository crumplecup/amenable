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
use amenable_core::{Ensures, Requires};
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

    /// Reflects then realizes a real `FixedOffset` through the bridge's
    /// own offset-descriptor pair, isolated from the naive-date-time
    /// half -- no fractional-second string logic, no `NaiveDate`
    /// construction, nothing else. Calls the real, now self-contracted
    /// `amenable_ext::fixed_offset_to_utc_offset_descriptor` directly
    /// (its own `#[kani::ensures]` states the real sign-and-magnitude
    /// relation) -- `#[kani::stub_verified(amenable_ext::
    /// fixed_offset_sign_and_magnitude)]` on this contract's own proof
    /// (below) substitutes that arithmetic step's *proven relation*
    /// wherever it's reached, including inside this inlined call,
    /// rather than inlining it too. Confirmed the direct cause of this
    /// claim's earlier timeout (see `gallery::
    /// chrono_offset_date_time_round_trip`'s isolation cases): the
    /// combined sign-and-magnitude conjunction exceeded the 3-minute
    /// budget as one unstubbed call, resolved once that arithmetic half
    /// -- `fixed_offset_sign_and_magnitude`, split out of `amenable_ext`
    /// itself for exactly this reason -- has its own contract and is
    /// stubbed instead of inlined.
    #[kani::requires(true)]
    #[kani::ensures(|result: &Result<chrono::FixedOffset, amenable_time::TemporalError>| {
        match result {
            Ok(round_tripped) => *round_tripped == offset,
            Err(_) => true,
        }
    })]
    pub(super) fn offset_round_trips(
        offset: chrono::FixedOffset,
    ) -> Result<chrono::FixedOffset, amenable_time::TemporalError> {
        let descriptor = amenable_ext::fixed_offset_to_utc_offset_descriptor_infallible(offset);
        Ok(amenable_ext::utc_offset_descriptor_to_fixed_offset_infallible(descriptor).into_inner())
    }

    pub(super) struct OffsetDateTimeRoundTripsThroughDescriptor;
    crate::rust_std::kani_local_claim!(OffsetDateTimeRoundTripsThroughDescriptor);

    /// The calendar half of
    /// `naive_date_time_to_local_date_time_descriptor_infallible`'s own
    /// correctness claim, named: a `CompleteDateDescriptor` matches a
    /// real `chrono::NaiveDate` when it's the `Calendar` variant and
    /// its year/month/day agree with the real date's own.
    pub(super) struct CalendarDescriptorMatchesTheRealDate;
    crate::rust_std::kani_local_claim!(CalendarDescriptorMatchesTheRealDate);

    impl amenable_core::Ensures<crate::KaniVerifier> for CalendarDescriptorMatchesTheRealDate {
        type Input = (amenable_time::CompleteDateDescriptor, chrono::NaiveDate);
        type Bound = bool;

        fn ensures((descriptor, real): Self::Input) -> bool {
            use chrono::Datelike;
            match descriptor {
                amenable_time::CompleteDateDescriptor::Calendar(cal) => {
                    cal.year() == real.year()
                        && u32::from(cal.month()) == real.month()
                        && u32::from(cal.day()) == real.day()
                }
                amenable_time::CompleteDateDescriptor::Ordinal(_)
                | amenable_time::CompleteDateDescriptor::Week(_) => false,
            }
        }
    }

    /// The hour/minute/second half of the same claim: a
    /// `LocalTimeDescriptor` matches a real `chrono::NaiveDateTime`
    /// when its hour/minute/second agree with the real datetime's own.
    pub(super) struct LocalTimeDescriptorMatchesTheRealHourMinuteSecond;
    crate::rust_std::kani_local_claim!(LocalTimeDescriptorMatchesTheRealHourMinuteSecond);

    impl amenable_core::Ensures<crate::KaniVerifier>
        for LocalTimeDescriptorMatchesTheRealHourMinuteSecond
    {
        type Input = (amenable_time::LocalTimeDescriptor, chrono::NaiveDateTime);
        type Bound = bool;

        fn ensures((descriptor, real): Self::Input) -> bool {
            use chrono::Timelike;
            u32::from(descriptor.hour()) == real.hour()
                && u32::from(descriptor.minute()) == real.minute()
                && u32::from(descriptor.second()) == real.second()
        }
    }

    /// The fractional-second-presence half of the same claim: a
    /// `LocalTimeDescriptor` carries a fractional second exactly when
    /// the real datetime's own nanosecond field is nonzero.
    pub(super) struct FractionalSecondPresenceMatchesNanos;
    crate::rust_std::kani_local_claim!(FractionalSecondPresenceMatchesNanos);

    impl amenable_core::Ensures<crate::KaniVerifier> for FractionalSecondPresenceMatchesNanos {
        type Input = (amenable_time::LocalTimeDescriptor, u32);
        type Bound = bool;

        fn ensures((descriptor, nanos): Self::Input) -> bool {
            descriptor.fractional_second().is_some() == (nanos != 0)
        }
    }

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
use kani_only::{
    CalendarDescriptorMatchesTheRealDate, FractionalSecondPresenceMatchesNanos,
    LocalTimeDescriptorMatchesTheRealHourMinuteSecond, OffsetDateTimeRoundTripsThroughDescriptor,
    offset_round_trips,
};

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time::CalendarDescriptorMatchesTheRealDate",
        "kani",
        "ensures",
        || "descriptor is the Calendar variant and its year/month/day equal the real date's own",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time::LocalTimeDescriptorMatchesTheRealHourMinuteSecond",
        "kani",
        "ensures",
        || "descriptor's hour/minute/second equal the real datetime's own",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time::FractionalSecondPresenceMatchesNanos",
        "kani",
        "ensures",
        || "descriptor carries a fractional second iff nanos != 0",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::offset_date_time::OffsetDateTimeRoundTripsThroughDescriptor",
        "kani",
        "ensures",
        || "backend.exchange(reflect(original)) realizes back to original",
    )
}

// Every harness below that symbolically generates a fractional-second
// nanosecond value and needs it within the domain
// `fractional_digit_bytes_to_nanos`/`nanos_to_fractional_digit_bytes`
// actually cover (`0..=999_999_999`) assumes this shape -- named once
// rather than left as a raw `&&`-combined inline condition at each of
// the many call sites that need it.
#[cfg(kani)]
amenable_derive::harness! {
    kani, CHECK_NAIVE_DATE_TIME_TO_LOCAL_DATE_TIME_DESCRIPTOR_INFALLIBLE_SRC, {
        /// `naive_date_time_to_local_date_time_descriptor_infallible`'s
        /// own contract -- the full civil reflect direction including
        /// the fractional-second path. Stubs `narrow_to_u8_in_range`
        /// (x5) and `nanos_to_fractional_digit_bytes` (the pure digit
        /// core, already proven). Under `#[cfg(kani)]`, this function's
        /// own body never calls the real, production `nanos_to_
        /// fractional_seconds_digits` at all -- that function returns
        /// `String`, which does not implement `kani::Arbitrary`
        /// (confirmed directly: a hard compiler wall, matching
        /// `gallery::string_depth_induction`'s own finding), so it can
        /// never be `stub_verified`, and reading back from its push-
        /// built result (even one byte) was separately confirmed to
        /// reintroduce the same OOM its symbolic trim length causes.
        /// Instead, the Kani-only body builds the fractional-second
        /// `String` via `digit_bytes_to_string_unconditional` (an
        /// unconditional 9-push sequence, no symbolic length decision
        /// at all, confirmed cheap -- 21.3s in isolation) and never
        /// reads it back.
        ///
        /// Checked by a plain proof + direct `assert!`, not `proof_
        /// for_contract`: confirmed directly that `proof_for_contract`
        /// explodes on any function returning a heap-allocating type
        /// (190,650-step OOM here), regardless of what the `ensures`
        /// checks -- this function returns a `LocalDateTimeDescriptor`
        /// containing a real `String` via `FractionalSecondDescriptor`,
        /// inheriting the same wall `buffer_to_string_unconditional`'s
        /// own isolated test hit. Every constituent operation here,
        /// called directly together in a plain proof, was separately
        /// confirmed cheap (30.9s) -- this harness asserts the same
        /// structural claim the function's own `#[kani::ensures]`
        /// states, the same pattern `fractional_digit_round_trip`
        /// already uses for the same reason.
        #[kani::proof]
        #[kani::stub_verified(amenable_ext::narrow_to_u8_in_range)]
        #[kani::stub_verified(amenable_ext::nanos_to_fractional_digit_bytes)]
        fn check_naive_date_time_to_local_date_time_descriptor_infallible() {
            use chrono::Timelike;
            use crate::ValueIsBelow;
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) else {
                return;
            };
            let hour: u32 = kani::any();
            let minute: u32 = kani::any();
            let second: u32 = kani::any();
            let nano: u32 = kani::any();
            let Some(time) = chrono::NaiveTime::from_hms_nano_opt(hour, minute, second, nano)
            else {
                return;
            };
            let datetime = chrono::NaiveDateTime::new(date, time);
            // Excludes the genuine leap-second case, matching the
            // real function's own `requires` -- `from_hms_nano_opt`
            // allows `nano` up to ~2 billion for leap-second
            // representation, which this function's own precondition
            // explicitly does not cover.
            kani::assume(ValueIsBelow::requires((
                datetime.nanosecond(),
                1_000_000_000,
            )));
            let nanos = datetime.nanosecond();
            let result =
                amenable_ext::naive_date_time_to_local_date_time_descriptor_infallible(datetime);

            assert!(
                CalendarDescriptorMatchesTheRealDate::ensures((result.date(), date)),
                "calendar fields match the real datetime"
            );
            assert!(
                LocalTimeDescriptorMatchesTheRealHourMinuteSecond::ensures((
                    result.time().clone(),
                    datetime
                )),
                "hour/minute/second match the real datetime"
            );
            assert!(
                FractionalSecondPresenceMatchesNanos::ensures((result.time().clone(), nanos)),
                "fractional second is present iff nanos != 0"
            );
        }
    }
}

amenable_derive::harness! {
    kani, CHECK_OFFSET_ROUND_TRIPS_SRC, {
        /// `offset_round_trips`'s own contract: every real `FixedOffset`
        /// round-trips through the bridge's offset-descriptor pair,
        /// isolated from the naive-date-time half. Its own body (in
        /// `amenable_kani`, test-support code, not real production
        /// API) calls the two whole-function infallible siblings
        /// directly -- `fixed_offset_to_utc_offset_descriptor_
        /// infallible` and `utc_offset_descriptor_to_fixed_offset_
        /// infallible` -- rather than the real, un-stubbable
        /// `Result<_, TemporalError>`-returning functions, so each
        /// direction can be stubbed as a *whole unit* here instead of
        /// inlining either one in full: chaining two real calls
        /// compounds cost even when each is individually cheap
        /// (confirmed directly -- stubbing only their internal
        /// arithmetic/build pieces while still inlining both composed
        /// functions in full still timed out at 5026 VCCs).
        #[kani::proof_for_contract(offset_round_trips)]
        #[kani::stub_verified(amenable_ext::fixed_offset_to_utc_offset_descriptor_infallible)]
        #[kani::stub_verified(amenable_ext::utc_offset_descriptor_to_fixed_offset_infallible)]
        fn check_offset_round_trips() {
            let offset_seconds: i32 = kani::any();
            let Some(offset) = chrono::FixedOffset::east_opt(offset_seconds) else {
                return;
            };
            let _ = offset_round_trips(offset);
        }
    }
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

