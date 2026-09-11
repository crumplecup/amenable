//! The `std::time` canary backend for `amenable_time`'s temporal contract
//! interface.
//!
//! Lives in `amenable_std`, not `amenable_time`: `amenable_time` is the
//! trait/contract interface crate, kept dependency-light so it can be an
//! optional dep of `amenable_ext`'s jiff/chrono backends without dragging
//! in `amenable_std`'s whole std-lib registration surface (see
//! `docs/AMENABLE_EXT_PLAN.md`). Backend implementations instead live
//! alongside the type registrations they're built from — this one here,
//! future jiff/chrono ones in `amenable_ext`.
//!
//! A backend is a concrete type that performs temporal transitions as
//! [`Exchange`](amenable_core::Exchange)s and exposes its native carrier
//! types through the `Temporal*Props` families. The `Exchange` impls live
//! *here* rather than in `amenable_time` itself because every descriptor,
//! sidecar and token they touch is `amenable_time`'s own — the impl is
//! `impl ForeignTrait for LocalType`, which the orphan rule allows.
//!
//! This is a **canary**, not a usable backend: it implements only the
//! slice of the interface that `std::time::{Duration, SystemTime}` can
//! honestly back — durations, fixed instants, and endpoint ordering. Two
//! things make it a useful canary rather than a stub:
//!
//! - the trait-bound assertions in `tests/std_backend_test.rs`
//!   (`StdTimeBackend: TemporalIntervalFactory<CanaryVerifier>`, …) fail
//!   to compile if the interface drifts;
//! - its `Exchange` bodies *execute* the contracts — `order_offset_endpoints`
//!   resolves both endpoints through real Gregorian calendar arithmetic
//!   and returns `Err` for a reversed interval; the duration bridge
//!   round-trips a span through an actual `std::time::Duration` and
//!   rejects year/month components. It is a runtime oracle against the
//!   formal backends over the slice it covers.
//!
//! Real coverage of the rest (week/ordinal dates, time zones, ISO 8601 /
//! RFC 3339 parsing and formatting) needs a date-time library; a `jiff`
//! (or `chrono`) backend in `amenable_ext` would sit alongside this one.

use std::time::{Duration, SystemTime};

use amenable_core::{
    ClassifiedWitness, Establish, Exchange, Metadata, OwnedEntry, Provenance, Sidecar, Standard,
    Verifier, Witness, WitnessSupportSummary,
};
use amenable_time::{
    CompleteDateDescriptor, DurationDescriptor, DurationDescriptorBuilder,
    OffsetDateTimeDescriptor, OrderOffsetEndpointsEstablished, OrderOffsetEndpointsInput,
    OrderOffsetEndpointsOutput, OrderOffsetEndpointsPreconditionsToken, ParsedDuration,
    ParsedRecurringInterval, ParsedTimeInterval, ProvenDurationCarrier, RawInput,
    ReflectedDuration, SerializationProfile, TemporalDurationProps, TemporalError,
    TemporalErrorKind, TemporalInstantProps, TemporalProvenance, TemporalReporter, UtcOffsetSign,
};

// ── CanaryVerifier ──────────────────────────────────────────────────

/// Reporting surface for [`CanaryVerifier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CanaryVerifierMetadata;

impl Metadata for CanaryVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "canary"),
            OwnedEntry::new("kind", "runtime oracle over the std::time slice"),
            OwnedEntry::new("formal_tool", "none"),
        ]
    }
}

impl Provenance for CanaryVerifierMetadata {}

/// A [`Verifier`] marker for the `std::time` canary.
///
/// It runs no formal tool. What it *does* is let the canary's `Exchange`
/// bodies execute the same contract predicates the real backends prove
/// statically — `order_offset_endpoints` resolves both endpoints to epoch
/// seconds and refuses to establish `IntervalEndpointsOrdered` unless the
/// ordering genuinely holds; the duration bridge refuses to realize a
/// calendar-variable span as a fixed `std::time::Duration`. A violation is
/// an `Err` at runtime, not a proof obligation, so from this verifier's
/// point of view the machine-checkable contracts are trusted citations
/// (see `canary_trusts!`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CanaryVerifier;

impl Verifier for CanaryVerifier {
    type Metadata = CanaryVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "canary"
    }
}

// ── Trusted witnesses for the machine-checkable contracts ───────────
//
// The 23 range / ordering / arithmetic contracts carry real
// `Witness<KaniVerifier>` / `<CreusotVerifier>` / `<VerusVerifier>` proofs
// (`amenable_kani::time` etc.). `CanaryVerifier` is not one of those
// backends — it checks nothing — so from its point of view these are
// trusted citations exactly like the structural contracts. Without this
// block a `proof_composition` aggregate that reaches one of them (e.g.
// `IntervalEndpointsOrdered` → `IntervalStartPrecedesEnd`) would not be
// `Witness<CanaryVerifier>`, and the interval exchange surface would not
// type-check.

/// One `Witness<CanaryVerifier>` + `ClassifiedWitness<CanaryVerifier>` per
/// named contract, resting on the contract's own citation (the canary
/// verifies nothing).
macro_rules! canary_trusts {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Witness<CanaryVerifier> for $ty {
                type SupportingEvidence = Self;
                type ProofArtifact = TemporalProvenance;

                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn proof() -> Self::ProofArtifact {
                    <Self as Standard>::provenance(
                        &<Self as ::std::default::Default>::default(),
                    )
                }

                #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
                fn support() -> WitnessSupportSummary {
                    WitnessSupportSummary::trusted_leaf()
                }
            }

            impl ClassifiedWitness<CanaryVerifier> for $ty {}
        )+
    };
}

canary_trusts! {
    amenable_time::CalendarMonthInRangeOneToTwelve,
    amenable_time::HourInRangeZeroToTwentyFour,
    amenable_time::MinuteInRangeZeroToFiftyNine,
    amenable_time::SecondInRangeZeroToSixty,
    amenable_time::UtcOffsetHourInRangeZeroToTwentyThree,
    amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine,
    amenable_time::WeekdayInRangeOneToSeven,
    amenable_time::WeekNumberInRangeOneToFiftyThree,
    amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix,
    amenable_time::CenturyOrdinalInRangeZeroToNinetyNine,
    amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    amenable_time::IntervalStartPrecedesEnd,
    amenable_time::IntervalDurationIsNonNegative,
    amenable_time::UtcTimelineOrderingAppliesToFixedInstants,
    amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    amenable_time::CentennialYearDivisibleByOneHundred,
    amenable_time::LeapYearHasThreeHundredSixtySixCalendarDays,
    amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays,
    amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    amenable_time::CalendarDayWithinMonthBounds,
    amenable_time::LeapDayOccursOnlyInLeapYear,
}

/// `TemporalInputReceived` — "a caller handed us this text" — is a root
/// claim asserted by construction (see `exchange::markers`); its
/// `Witness<V>` is trivial and backend-provided, so the canary provides
/// the `CanaryVerifier` one here.
impl Witness<CanaryVerifier> for amenable_time::TemporalInputReceived {
    type SupportingEvidence = Self;
    type ProofArtifact = &'static str;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        "raw text received at the exchange boundary — a root claim, asserted by construction"
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trivial_leaf()
    }
}

impl ClassifiedWitness<CanaryVerifier> for amenable_time::TemporalInputReceived {}

// ── Native carriers ─────────────────────────────────────────────────

/// A [`std::time::Duration`] as a temporal duration carrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct StdDuration(
    /// The wrapped span.
    pub Duration,
);

/// A [`std::time::SystemTime`] as a fixed-instant carrier.
///
/// `SystemTime` is a point on the system clock's timeline measured from
/// the Unix epoch — an absolute instant with no civil-calendar or
/// time-zone structure of its own, which is exactly the slice of the
/// interface this backend claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct StdSystemTime(
    /// The wrapped instant.
    pub SystemTime,
);

impl Default for StdSystemTime {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self(SystemTime::UNIX_EPOCH)
    }
}

/// The only UTC offset `std::time` represents: `+00:00`.
///
/// `SystemTime` is epoch-relative, so a `std::time` value is always at
/// zero offset from UTC; this ZST records that.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct StdUtcOffset;

// ── The backend ─────────────────────────────────────────────────────

/// The `std::time` canary backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct StdTimeBackend;

impl TemporalDurationProps for StdTimeBackend {
    type Duration = StdDuration;
}

impl TemporalInstantProps for StdTimeBackend {
    type UtcOffset = StdUtcOffset;
    type OffsetDateTime = StdSystemTime;
    type Instant = StdSystemTime;
}

impl TemporalReporter for StdTimeBackend {
    /// None — `std::time` has no serializer.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supported_serialization_profiles(&self) -> Vec<SerializationProfile> {
        Vec::new()
    }

    /// `std::time::Duration` resolves to the nanosecond: nine digits.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn max_fractional_second_digits(&self) -> Option<u8> {
        Some(9)
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_leap_seconds(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_unknown_local_offset(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_named_zone_round_trip(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supports_end_of_day_twenty_four(&self) -> bool {
        false
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn current_tzdb_revision(&self) -> Option<String> {
        None
    }
}

// ── Real conversions to `std::time` ─────────────────────────────────

/// Days from the Unix epoch (1970-01-01) to a proleptic Gregorian date.
///
/// Howard Hinnant's `days_from_civil` — real calendar arithmetic, valid
/// for any `year`/`month`/`day` with `1 <= month <= 12`.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let year_of_era = y - era * 400;
    let month_offset = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * month_offset + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// Resolve an offset date-time descriptor to whole seconds from the Unix
/// epoch. Only complete *calendar* dates are supported — ordinal and week
/// dates need date-library machinery the canary does not carry.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn offset_datetime_to_epoch_seconds(
    descriptor: &OffsetDateTimeDescriptor,
) -> Result<i64, TemporalError> {
    let local = descriptor.local();
    let CompleteDateDescriptor::Calendar(date) = local.date() else {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            "std::time canary resolves complete calendar dates only, not ordinal or week dates"
                .to_owned(),
        )));
    };

    let days = days_from_civil(
        i64::from(date.year()),
        i64::from(date.month()),
        i64::from(date.day()),
    );
    let time = local.time();
    let seconds_of_day =
        i64::from(time.hour()) * 3600 + i64::from(time.minute()) * 60 + i64::from(time.second());

    let offset = descriptor.offset();
    let offset_seconds =
        i64::from(offset.hours()) * 3600 + i64::from(offset.minutes().unwrap_or(0)) * 60;
    let signed_offset = match offset.sign() {
        UtcOffsetSign::Positive => offset_seconds,
        UtcOffsetSign::Negative => -offset_seconds,
    };

    // A wall clock reading `local` at offset `+HH:MM` names the same
    // instant as `local - HH:MM` at UTC.
    Ok(days * 86_400 + seconds_of_day - signed_offset)
}

/// Resolve an ISO 8601 duration descriptor to a `std::time::Duration`.
///
/// Rejects the calendar-variable components (years, months) and fractional
/// suffixes: a `std::time::Duration` is a fixed span of whole nanoseconds,
/// so those simply do not convert.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn duration_descriptor_to_std(descriptor: &DurationDescriptor) -> Result<Duration, TemporalError> {
    if descriptor.years() != 0 || descriptor.months() != 0 {
        return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            "a duration with year or month components has no fixed std::time::Duration".to_owned(),
        )));
    }
    if descriptor.fractional_component().is_some() {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            "the std::time canary carries whole-second durations only".to_owned(),
        )));
    }

    let seconds = u64::from(descriptor.weeks()) * 604_800
        + u64::from(descriptor.days()) * 86_400
        + u64::from(descriptor.hours()) * 3600
        + u64::from(descriptor.minutes()) * 60
        + u64::from(descriptor.seconds());
    Ok(Duration::from_secs(seconds))
}

/// Decompose a `std::time::Duration` back into an ISO 8601 duration
/// descriptor of days / hours / minutes / seconds.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn std_to_duration_descriptor(span: Duration) -> Result<DurationDescriptor, TemporalError> {
    let mut remaining = span.as_secs();
    let days = remaining / 86_400;
    remaining %= 86_400;
    let hours = remaining / 3600;
    remaining %= 3600;
    let minutes = remaining / 60;
    let seconds = remaining % 60;

    let days = u32::try_from(days).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "duration's {days}-day span exceeds the descriptor's u32 day component: {err}"
        )))
    })?;

    DurationDescriptorBuilder::default()
        .days(days)
        .hours(u32::try_from(hours).unwrap_or_default())
        .minutes(u32::try_from(minutes).unwrap_or_default())
        .seconds(u32::try_from(seconds).unwrap_or_default())
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not build a duration descriptor: {err}"
            )))
        })
}

// ── Interval exchanges ──────────────────────────────────────────────
//
// `TemporalIntervalFactory<CanaryVerifier>` is a blanket-impl'd supertrait
// bundle over these four `Exchange`s. The three text-parse edges have no
// honest `std::time` implementation (there is no ISO 8601 parser in
// `std`), so they are lawful `Exchange` impls that report the operation
// unsupported. `order_offset_endpoints` is real: it resolves both
// endpoints to epoch seconds through actual Gregorian calendar arithmetic
// and only establishes the ordering proposition when the ordering
// genuinely holds — a runtime execution of the same `IntervalEndpointsOrdered`
// contract the formal backends prove statically.

/// Report the missing ISO 8601 parser for a text-parse edge.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn unsupported_parse(edge: &'static str) -> TemporalError {
    TemporalError::new(TemporalErrorKind::Unsupported(format!(
        "std::time cannot parse {edge}: no ISO 8601 / RFC 3339 parser in the standard library"
    )))
}

impl Exchange<RawInput, ParsedDuration, CanaryVerifier> for StdTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedDuration, TemporalError> {
        Err(unsupported_parse("an ISO 8601 duration"))
    }
}

impl Exchange<RawInput, ParsedRecurringInterval, CanaryVerifier> for StdTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedRecurringInterval, TemporalError> {
        Err(unsupported_parse("a recurring interval"))
    }
}

impl Exchange<RawInput, ParsedTimeInterval, CanaryVerifier> for StdTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedTimeInterval, TemporalError> {
        Err(unsupported_parse("a time interval"))
    }
}

impl Exchange<OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput, CanaryVerifier>
    for StdTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: OrderOffsetEndpointsInput,
    ) -> Result<OrderOffsetEndpointsOutput, TemporalError> {
        let request = input.request();
        let start = offset_datetime_to_epoch_seconds(request.start())?;
        let end = offset_datetime_to_epoch_seconds(request.end())?;

        // The whole point of the exchange: establish that the endpoints are
        // chronologically ordered. If they are not, there is no proof to
        // re-issue.
        if start > end {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                format!("interval start ({start}s) is after its end ({end}s)"),
            )));
        }

        let credential: OrderOffsetEndpointsPreconditionsToken =
            <OrderOffsetEndpointsInput as Sidecar<CanaryVerifier>>::sidecar(&input);
        let token = <OrderOffsetEndpointsEstablished as Establish<
            OrderOffsetEndpointsPreconditionsToken,
            CanaryVerifier,
        >>::establish(credential);

        Ok(OrderOffsetEndpointsOutput::new(
            OrderOffsetEndpointsEstablished::default(),
            token,
        ))
    }
}

// ── Duration native bridge ──────────────────────────────────────────
//
// `TemporalDurationNativeBridge<CanaryVerifier>` is the `realize_duration`
// / `reflect_duration` inverse pair. Both are real: they carry an ISO 8601
// duration descriptor through an actual `std::time::Duration` and back.
// The shared `DurationSemanticBundleToken` rides through unchanged — the
// two directions are genuine inverses over the whole-second span the
// carrier holds.

impl Exchange<ReflectedDuration, ProvenDurationCarrier<StdDuration>, CanaryVerifier>
    for StdTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedDuration,
    ) -> Result<ProvenDurationCarrier<StdDuration>, TemporalError> {
        let span = duration_descriptor_to_std(input.descriptor())?;
        let token = <ReflectedDuration as Sidecar<CanaryVerifier>>::sidecar(&input);
        Ok(ProvenDurationCarrier::<StdDuration>::new(
            StdDuration(span),
            token,
        ))
    }
}

impl Exchange<ProvenDurationCarrier<StdDuration>, ReflectedDuration, CanaryVerifier>
    for StdTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenDurationCarrier<StdDuration>,
    ) -> Result<ReflectedDuration, TemporalError> {
        let descriptor = std_to_duration_descriptor(input.carrier().0)?;
        let token =
            <ProvenDurationCarrier<StdDuration> as Sidecar<CanaryVerifier>>::sidecar(&input);
        Ok(ReflectedDuration::new(descriptor, token))
    }
}
