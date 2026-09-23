//! A real jiff-backed `amenable_time` temporal backend.
//!
//! Lives here, not in `amenable_time` itself, for the same reason
//! `amenable_std::std_time_backend` does — see that module's own doc
//! comment: `amenable_time` is the trait/contract interface crate, kept
//! dependency-light; backend implementations live alongside the type
//! registrations they're built from. The `Exchange` impls are
//! `impl ForeignTrait for LocalType`, which the orphan rule allows here.
//!
//! Unlike `StdTimeBackend` (a deliberate **canary**, honestly scoped to
//! the slice `std::time` can back), `JiffTimeBackend` is a real, if
//! partial, backend: its `Exchange` bodies call jiff's actual calendar
//! arithmetic and parser code, not a hand-rolled stand-in. See
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md` for the full ~129-edge
//! surface map and the phased checklist this module works through.
//!
//! **Phase 1: `TemporalDurationProps` + `TemporalDurationNativeBridge`
//! over `jiff::Span`.** **Phase 2: `TemporalInstantProps` +
//! `TemporalInstantNativeBridge` over `jiff::Timestamp`/`jiff::tz::
//! Offset`/a small `JiffOffsetDateTime` composite.** **Phase 3 (this
//! file, so far, on top of Phases 1-2): `TemporalCivilProps` +
//! `TemporalCivilNativeBridge` over `jiff::civil::{Date,Time,DateTime,
//! ISOWeekDate}` — widens Phase 2's own calendar-date-only
//! `LocalDateTime` realize/reflect to real ordinal- and week-date
//! support too.** Every other `Temporal*Props`/`NativeBridge`/`Factory`
//! family named in the plan doc's checklist lands in later commits,
//! each widening this same `JiffTimeBackend` struct with its own real
//! `Exchange` impls.

use amenable_core::{
    ClassifiedWitness, Exchange, Metadata, OwnedEntry, Provenance, Sidecar, Standard, Verifier,
    Witness, WitnessSupportSummary,
};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, DurationDescriptor, DurationDescriptorBuilder,
    DurationFractionDescriptor, FractionalSecondDescriptor, LocalDateTimeDescriptor,
    LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder, OffsetDateTimeDescriptorBuilder,
    ProvenDurationCarrier, ProvenLocalDateTimeCarrier, ProvenOffsetDateTimeCarrier,
    ReflectedDuration, ReflectedLocalDateTime, ReflectedOffsetDateTime, TemporalCivilProps,
    TemporalComponent, TemporalDurationProps, TemporalError, TemporalErrorKind,
    TemporalInstantProps, TemporalProvenance, UtcOffsetDescriptor, UtcOffsetDescriptorBuilder,
    UtcOffsetRelationship, UtcOffsetSign,
};

// ── Phase 3: Civil ───────────────────────────────────────────────────

/// A [`jiff::civil::Date`] as a calendar (or ordinal) date carrier.
///
/// Reused for BOTH `TemporalCivilProps::CalendarDate` and `::OrdinalDate`
/// — the same real jiff type honestly backs both once resolved to a
/// concrete value, the same "one carrier, several associated-type
/// slots" pattern `amenable_std::StdSystemTime` already uses for its
/// own `Instant`/`OffsetDateTime`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffDate(
    /// The wrapped date.
    pub jiff::civil::Date,
);

/// A [`jiff::civil::Time`] as a local time-of-day carrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffTime(
    /// The wrapped time.
    pub jiff::civil::Time,
);

/// A [`jiff::civil::DateTime`] as a local date-time carrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffDateTime(
    /// The wrapped date-time.
    pub jiff::civil::DateTime,
);

/// A [`jiff::civil::ISOWeekDate`] as a week-date carrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffISOWeekDate(
    /// The wrapped ISO week date.
    pub jiff::civil::ISOWeekDate,
);

/// A reduced-precision calendar date carrier.
///
/// `jiff::civil::Date` has no partial (year-only / year-month-only)
/// representation of its own — this is a real, meaningful shape
/// (mirroring `amenable_time::ReducedCalendarDateDescriptor`'s own two
/// forms exactly, using jiff's own `i16`/`i8` component widths), but its
/// conversion logic isn't wired to any `Exchange` edge yet: no
/// `Temporal*NativeBridge` trait in Phases 1-3's scope needs a
/// `ReducedCalendarDate` realize/reflect pair — only `TemporalParser`'s
/// own `RawInput -> ParsedReducedCalendarDate` edge (Phase 9) will.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffReducedCalendarDate {
    /// Year-only calendar date form.
    Year {
        /// Signed calendar year.
        year: i16,
    },
    /// Year-month calendar date form.
    YearMonth {
        /// Signed calendar year.
        year: i16,
        /// Calendar month number.
        month: i8,
    },
}

impl Default for JiffReducedCalendarDate {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Year { year: 0 }
    }
}

/// A reduced-precision local time-of-day carrier.
///
/// Same "real shape, not yet wired to an `Exchange` edge" status as
/// [`JiffReducedCalendarDate`] — and a genuinely harder case besides:
/// `amenable_time::ReducedLocalTimeDescriptor` allows a fractional
/// component on its own hour/minute field, which `jiff::civil::Time`
/// cannot represent at all (its finest fractional unit is the second).
/// This carrier's own conversion logic, whenever Phase 9 needs it, will
/// have to reject that case the same way `duration_descriptor_to_jiff_span`
/// already rejects a duration fraction on a coarser-than-seconds unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffReducedLocalTime {
    /// Hour-only local time form.
    Hour {
        /// Hour of day.
        hour: i8,
    },
    /// Hour-minute local time form.
    HourMinute {
        /// Hour of day.
        hour: i8,
        /// Minute of hour.
        minute: i8,
    },
}

impl Default for JiffReducedLocalTime {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Hour { hour: 0 }
    }
}

impl TemporalCivilProps for JiffTimeBackend {
    type CalendarDate = JiffDate;
    type ReducedCalendarDate = JiffReducedCalendarDate;
    type OrdinalDate = JiffDate;
    type WeekDate = JiffISOWeekDate;
    type LocalTime = JiffTime;
    type ReducedLocalTime = JiffReducedLocalTime;
    type LocalDateTime = JiffDateTime;
}

// ── JiffVerifier ─────────────────────────────────────────────────────

/// Reporting surface for [`JiffVerifier`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct JiffVerifierMetadata;

impl Metadata for JiffVerifierMetadata {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn snapshot(&self) -> Vec<OwnedEntry> {
        vec![
            OwnedEntry::new("verifier_family", "jiff"),
            OwnedEntry::new(
                "kind",
                "runtime execution over jiff's real calendar/zone/parser API",
            ),
            OwnedEntry::new("formal_tool", "none"),
        ]
    }
}

impl Provenance for JiffVerifierMetadata {}

/// A [`Verifier`] marker for the real jiff-backed temporal backend.
///
/// Like [`CanaryVerifier`](amenable_std::CanaryVerifier), it runs no
/// formal tool — but unlike the canary, its `Exchange` bodies exercise
/// jiff's own real implementation (calendar arithmetic, ISO 8601
/// parsing, IANA tzdb lookups where later phases reach them), not a
/// hand-rolled stand-in scoped to what one small standard-library type
/// can honestly back. A violation is still an `Err` at runtime, not a
/// proof obligation, so from this verifier's point of view every
/// machine-checkable contract is a trusted citation exactly like the
/// structural ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct JiffVerifier;

impl Verifier for JiffVerifier {
    type Metadata = JiffVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "jiff"
    }
}

// ── Trusted witnesses for the machine-checkable contracts ───────────
//
// The 23 range / ordering / arithmetic contracts carry real
// `Witness<KaniVerifier>` / `<CreusotVerifier>` / `<VerusVerifier>` proofs
// (`amenable_kani::time` etc.). `JiffVerifier` is not one of those
// backends — it checks nothing — so from its point of view these are
// trusted citations exactly like the structural contracts (which get a
// verifier-generic `Witness<V>` for free from `amenable_time::
// structural_witness`'s own blanket impl — confirmed by compiling
// without this block first: the missing-impl errors named exactly this
// set, the same one `amenable_std::std_time_backend`'s own
// `canary_trusts!` block already covers for `CanaryVerifier`, since
// `BackendConversionSemanticBundle` — a shared sub-claim of every
// `*SemanticBundle` — reaches all of them). Without this block, no
// `Temporal*NativeBridge<JiffVerifier>` surface type-checks at all.

/// One `Witness<JiffVerifier>` + `ClassifiedWitness<JiffVerifier>` per
/// named contract, resting on the contract's own citation (this verifier
/// runs no formal tool).
macro_rules! jiff_backend_trusts {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Witness<JiffVerifier> for $ty {
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

            impl ClassifiedWitness<JiffVerifier> for $ty {}
        )+
    };
}

jiff_backend_trusts! {
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
/// claim asserted by construction (see `amenable_time::exchange::markers`);
/// its `Witness<V>` is trivial and backend-provided, so this backend
/// provides the `JiffVerifier` one here.
impl Witness<JiffVerifier> for amenable_time::TemporalInputReceived {
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

impl ClassifiedWitness<JiffVerifier> for amenable_time::TemporalInputReceived {}

// ── Native carriers ──────────────────────────────────────────────────

/// A [`jiff::Span`] as a temporal duration carrier.
///
/// `jiff::Span` derives only `Clone, Copy, Default` (a manual, non-derived
/// `Debug` impl, and deliberately no `PartialEq`/`Eq`/`Hash` — confirmed
/// by reading jiff's real source; see `amenable_ext::jiff`'s own
/// `SpanFieldwise` witness for why equal-elapsed spans can legitimately
/// compare unequal fieldwise). This wrapper mirrors that: no
/// `PartialEq`/`Eq`/`Hash` derive here either, and comparisons in tests
/// go through `jiff::Span`'s own getters, not `==` on the whole carrier.
#[derive(Debug, Clone, Copy, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffSpan(
    /// The wrapped span.
    pub jiff::Span,
);

// ── The backend ──────────────────────────────────────────────────────

/// The real jiff-backed temporal backend.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct JiffTimeBackend;

impl TemporalDurationProps for JiffTimeBackend {
    type Duration = JiffSpan;
}

// ── Real conversions to/from `jiff::Span` ───────────────────────────

/// Convert a lexical fractional-second digit string (ISO 8601's
/// decimal-fraction suffix, e.g. `"5"` for `.5`) into whole nanoseconds.
///
/// Canonically right-padded to 9 digits (nanosecond precision) and
/// truncated beyond that — real, but sufficient-not-exhaustive:
/// sub-nanosecond digits are documented precision loss, the same
/// "state the sufficient range, not the exact algorithm" convention
/// `amenable_ext`'s own jiff witnesses already use.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn fractional_seconds_digits_to_nanos(digits: &str) -> Result<i64, TemporalError> {
    let mut padded = digits.to_owned();
    if padded.len() > 9 {
        padded.truncate(9);
    } else {
        while padded.len() < 9 {
            padded.push('0');
        }
    }
    padded.parse::<i64>().map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "fractional-second digits {digits:?} are not a valid decimal: {err}"
        )))
    })
}

/// Convert whole nanoseconds (`0..=999_999_999`) back into a canonical
/// lexical fractional-second digit string, trimmed of trailing zeros
/// (e.g. `500_000_000` -> `"5"`).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn nanos_to_fractional_seconds_digits(nanos: i64) -> String {
    let padded = format!("{nanos:09}");
    let trimmed = padded.trim_end_matches('0');
    if trimmed.is_empty() {
        "0".to_owned()
    } else {
        trimmed.to_owned()
    }
}

/// Resolve an ISO 8601 duration descriptor to a real `jiff::Span`.
///
/// All seven whole-unit components round-trip exactly (`jiff::Span`
/// stores each independently, with no auto-carry between units —
/// confirmed by reading jiff's real source, the same "no cross-field
/// carry" finding this session's `SpanFieldwise` witness already
/// established). The fractional-second case is scoped honestly:
/// `jiff::Span` has no fractional representation for any COARSER unit
/// (years/months/weeks/days/hours/minutes are plain `i64` setters, no
/// fractional variant), so a descriptor fraction on any component other
/// than seconds is a real `Unsupported` error, not a silent truncation.
///
/// Uses jiff's fallible `try_*` setters throughout, never the panicking
/// `years()`/`months()`/etc — real jiff source confirms those panic
/// outright once a component exceeds jiff's own representable range
/// (e.g. years beyond ±19,998), and `DurationDescriptor`'s `u32` fields
/// can exceed that range trivially.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn duration_descriptor_to_jiff_span(
    descriptor: &DurationDescriptor,
) -> Result<jiff::Span, TemporalError> {
    let out_of_range = |err: jiff::Error| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "duration component out of jiff::Span's representable range: {err}"
        )))
    };

    let span = jiff::Span::new()
        .try_years(descriptor.years())
        .and_then(|s| s.try_months(descriptor.months()))
        .and_then(|s| s.try_weeks(descriptor.weeks()))
        .and_then(|s| s.try_days(descriptor.days()))
        .and_then(|s| s.try_hours(descriptor.hours()))
        .and_then(|s| s.try_minutes(descriptor.minutes()))
        .and_then(|s| s.try_seconds(descriptor.seconds()))
        .map_err(out_of_range)?;

    match descriptor.fractional_component() {
        None => Ok(span),
        Some(fraction) if fraction.component() == TemporalComponent::Second => {
            let nanos = fractional_seconds_digits_to_nanos(fraction.digits())?;
            span.try_nanoseconds(nanos).map_err(out_of_range)
        }
        Some(fraction) => Err(TemporalError::new(TemporalErrorKind::Unsupported(format!(
            "jiff backend supports a fractional-second component only, not a fraction on {}",
            fraction.component()
        )))),
    }
}

/// Decompose a real `jiff::Span` back into an ISO 8601 duration
/// descriptor — the exact inverse of
/// [`duration_descriptor_to_jiff_span`] for any span it could have
/// produced.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn jiff_span_to_duration_descriptor(span: jiff::Span) -> Result<DurationDescriptor, TemporalError> {
    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "span's {field} does not fit the descriptor's u32 component: {err}"
        )))
    };

    let years = u32::try_from(span.get_years()).map_err(|e| out_of_range("years", e))?;
    let months = u32::try_from(span.get_months()).map_err(|e| out_of_range("months", e))?;
    let weeks = u32::try_from(span.get_weeks()).map_err(|e| out_of_range("weeks", e))?;
    let days = u32::try_from(span.get_days()).map_err(|e| out_of_range("days", e))?;
    let hours = u32::try_from(span.get_hours()).map_err(|e| out_of_range("hours", e))?;
    let minutes = u32::try_from(span.get_minutes()).map_err(|e| out_of_range("minutes", e))?;
    let seconds = u32::try_from(span.get_seconds()).map_err(|e| out_of_range("seconds", e))?;

    let nanos = span.get_nanoseconds();
    let fractional_component = if nanos == 0 {
        None
    } else {
        let nanos_u32 = u32::try_from(nanos).map_err(|e| out_of_range("nanoseconds", e))?;
        Some(DurationFractionDescriptor::new(
            TemporalComponent::Second,
            nanos_to_fractional_seconds_digits(i64::from(nanos_u32)),
        ))
    };

    let mut builder = DurationDescriptorBuilder::default()
        .years(years)
        .months(months)
        .weeks(weeks)
        .days(days)
        .hours(hours)
        .minutes(minutes)
        .seconds(seconds);
    if let Some(fraction) = fractional_component {
        builder = builder.fractional_component(fraction);
    }
    builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "could not build a duration descriptor: {err}"
        )))
    })
}

// ── Duration native bridge ───────────────────────────────────────────
//
// `TemporalDurationNativeBridge<JiffVerifier>` is the `realize_duration`
// / `reflect_duration` inverse pair. Both are real: they carry an ISO
// 8601 duration descriptor through an actual `jiff::Span` and back,
// richer than `std::time::Duration`'s whole-seconds-only shape (years
// and months carry through natively, with no lossy conversion).

impl Exchange<ReflectedDuration, ProvenDurationCarrier<JiffSpan>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedDuration,
    ) -> Result<ProvenDurationCarrier<JiffSpan>, TemporalError> {
        let span = duration_descriptor_to_jiff_span(input.descriptor())?;
        let token = <ReflectedDuration as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenDurationCarrier::<JiffSpan>::new(
            JiffSpan(span),
            token,
        ))
    }
}

impl Exchange<ProvenDurationCarrier<JiffSpan>, ReflectedDuration, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenDurationCarrier<JiffSpan>,
    ) -> Result<ReflectedDuration, TemporalError> {
        let descriptor = jiff_span_to_duration_descriptor(input.carrier().0)?;
        let token = <ProvenDurationCarrier<JiffSpan> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedDuration::new(descriptor, token))
    }
}

// ── Phase 2: Instant ─────────────────────────────────────────────────

/// A [`jiff::Timestamp`] as a fixed-instant carrier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffTimestamp(
    /// The wrapped instant.
    pub jiff::Timestamp,
);

/// A [`jiff::tz::Offset`] as a UTC-offset carrier.
///
/// `jiff::tz::Offset` derives only `Clone, Copy, Eq, Hash, PartialEq,
/// PartialOrd, Ord` — confirmed by reading jiff's real source, there is
/// deliberately no `Default` (unlike `jiff::civil::DateTime`, which has
/// a manual one via `DateTime::ZERO`). `#[derive(amenable_derive::
/// Evidence)]`'s own `basis = "Self"` mode needs `Self: Default` UNLESS
/// a `basis_ctor` is given explicitly — confirmed by reading the real
/// macro expansion, which only adds the `Default` where-bound when
/// `basis_ctor` is absent — so this wrapper supplies `Offset::UTC` (a
/// real, concrete jiff constant) as its root basis instead of relying
/// on a `Default` the wrapped type doesn't have.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self", basis_ctor = "Self(jiff::tz::Offset::UTC)")]
pub struct JiffOffset(
    /// The wrapped offset.
    pub jiff::tz::Offset,
);

/// A real jiff-backed offset-aware date-time carrier: a local civil
/// date-time paired with the UTC offset it was recorded at.
///
/// `jiff` has no first-class "offset date-time" type of its own the way
/// the `time` crate does — `jiff::Zoned` always carries a full
/// `jiff::tz::TimeZone`, not a bare offset. This small composite is the
/// natural fit instead, built from two carriers jiff already supports
/// directly. Same `basis_ctor` need as `JiffOffset`'s own doc comment
/// documents (its `offset` field has no `Default`), supplied here as
/// `DateTime::ZERO` + `Offset::UTC`, both real jiff constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(
    basis = "Self",
    basis_ctor = "Self { local: jiff::civil::DateTime::ZERO, offset: jiff::tz::Offset::UTC }"
)]
pub struct JiffOffsetDateTime {
    /// The local (wall-clock) civil date-time.
    pub local: jiff::civil::DateTime,
    /// The UTC offset it was recorded at.
    pub offset: jiff::tz::Offset,
}

impl TemporalInstantProps for JiffTimeBackend {
    type UtcOffset = JiffOffset;
    type OffsetDateTime = JiffOffsetDateTime;
    type Instant = JiffTimestamp;
}

// ── Real conversions to/from jiff's civil/offset types ──────────────

/// Resolve a UTC-offset descriptor to a real `jiff::tz::Offset`.
///
/// Real, honest scoping: the RFC 9557-updated "unknown local offset"
/// relationship (`-00:00`) has no fixed-offset representation at all —
/// `jiff::tz::Offset` is always a concrete, known value — so that case
/// is a real `Unsupported` error, never silently coerced to `Offset::UTC`
/// or any other default.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn utc_offset_descriptor_to_jiff_offset(
    descriptor: UtcOffsetDescriptor,
) -> Result<jiff::tz::Offset, TemporalError> {
    if descriptor.relationship() == UtcOffsetRelationship::UnknownLocalOffset {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            "jiff::tz::Offset has no representation for the RFC 9557 unknown-local-offset case"
                .to_owned(),
        )));
    }

    let magnitude =
        i32::from(descriptor.hours()) * 3600 + i32::from(descriptor.minutes().unwrap_or(0)) * 60;
    let seconds = match descriptor.sign() {
        UtcOffsetSign::Positive => magnitude,
        UtcOffsetSign::Negative => -magnitude,
    };

    jiff::tz::Offset::from_seconds(seconds).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "offset out of jiff::tz::Offset's representable range: {err}"
        )))
    })
}

/// Decompose a real `jiff::tz::Offset` back into a UTC-offset
/// descriptor.
///
/// Canonicalizes to the integral-hour form (`minutes: None`) whenever
/// the offset is an exact whole number of hours — `jiff::tz::Offset`
/// itself carries no trace of whether the original descriptor stated
/// minutes explicitly as `0` or omitted them, so this is the one
/// faithful choice, matching `UtcOffsetDescriptor`'s own documented
/// convention ("`None` denotes the integral-hour form").
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn jiff_offset_to_utc_offset_descriptor(
    offset: jiff::tz::Offset,
) -> Result<UtcOffsetDescriptor, TemporalError> {
    let seconds = offset.seconds();
    let sign = if seconds < 0 {
        UtcOffsetSign::Negative
    } else {
        UtcOffsetSign::Positive
    };
    let magnitude = seconds.unsigned_abs();
    let hours = u8::try_from(magnitude / 3600).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "offset's hour component does not fit the descriptor's u8 component: {err}"
        )))
    })?;
    let remainder_minutes = (magnitude % 3600) / 60;

    let mut builder = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(hours);
    if remainder_minutes != 0 {
        let minutes = u8::try_from(remainder_minutes).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "offset's minute component does not fit the descriptor's u8 component: {err}"
            )))
        })?;
        builder = builder.minutes(minutes);
    }

    builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "could not build a UTC offset descriptor: {err}"
        )))
    })
}

/// Resolve a complete-date descriptor (calendar, ordinal, or week form)
/// to a real `jiff::civil::Date`.
///
/// All three forms are real, using jiff's own real cross-representation
/// support: `Date::new` directly for the calendar form;
/// `Date::new(year, 1, 1).with().day_of_year(n).build()` for the
/// ordinal form (confirmed via jiff's real source — this is the
/// documented, correct way to construct a date from its ordinal day,
/// not a hand-rolled day-count calculation); `Weekday::
/// from_monday_one_offset` (the same ISO weekday-number convention
/// already checked as a real witness for `civil::Weekday`) +
/// `ISOWeekDate::new(..).date()` for the week form.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn complete_date_descriptor_to_jiff_date(
    date: CompleteDateDescriptor,
) -> Result<jiff::civil::Date, TemporalError> {
    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "{field} does not fit jiff::civil::Date's component range: {err}"
        )))
    };
    let range_err = |err: jiff::Error| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "date out of jiff::civil::Date's representable range: {err}"
        )))
    };

    match date {
        CompleteDateDescriptor::Calendar(cal) => {
            let year = i16::try_from(cal.year()).map_err(|e| out_of_range("year", e))?;
            let month = i8::try_from(cal.month()).map_err(|e| out_of_range("month", e))?;
            let day = i8::try_from(cal.day()).map_err(|e| out_of_range("day", e))?;
            jiff::civil::Date::new(year, month, day).map_err(range_err)
        }
        CompleteDateDescriptor::Ordinal(ord) => {
            let year = i16::try_from(ord.year()).map_err(|e| out_of_range("year", e))?;
            let day_of_year =
                i16::try_from(ord.day_of_year()).map_err(|e| out_of_range("day_of_year", e))?;
            jiff::civil::Date::new(year, 1, 1)
                .map_err(range_err)?
                .with()
                .day_of_year(day_of_year)
                .build()
                .map_err(range_err)
        }
        CompleteDateDescriptor::Week(week) => {
            let week_year =
                i16::try_from(week.week_year()).map_err(|e| out_of_range("week_year", e))?;
            let week_number = i8::try_from(week.week()).map_err(|e| out_of_range("week", e))?;
            let weekday_number =
                i8::try_from(week.weekday()).map_err(|e| out_of_range("weekday", e))?;
            let weekday =
                jiff::civil::Weekday::from_monday_one_offset(weekday_number).map_err(range_err)?;
            jiff::civil::ISOWeekDate::new(week_year, week_number, weekday)
                .map(jiff::civil::ISOWeekDate::date)
                .map_err(range_err)
        }
    }
}

/// Resolve a local date-time descriptor to a real `jiff::civil::DateTime`.
///
/// Any of the three complete-date forms (calendar, ordinal, week) —
/// see [`complete_date_descriptor_to_jiff_date`].
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn local_date_time_descriptor_to_jiff_civil_datetime(
    descriptor: &LocalDateTimeDescriptor,
) -> Result<jiff::civil::DateTime, TemporalError> {
    let date = complete_date_descriptor_to_jiff_date(descriptor.date())?;
    let time = descriptor.time();

    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "{field} does not fit jiff::civil::Time's component range: {err}"
        )))
    };
    let hour = i8::try_from(time.hour()).map_err(|e| out_of_range("hour", e))?;
    let minute = i8::try_from(time.minute()).map_err(|e| out_of_range("minute", e))?;
    let second = i8::try_from(time.second()).map_err(|e| out_of_range("second", e))?;
    let subsec_nanosecond = match time.fractional_second() {
        None => 0,
        Some(fraction) => i32::try_from(fractional_seconds_digits_to_nanos(fraction.digits())?)
            .map_err(|e| out_of_range("fractional second", e))?,
    };
    let time = jiff::civil::Time::new(hour, minute, second, subsec_nanosecond).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "local time out of jiff::civil::Time's representable range: {err}"
        )))
    })?;

    Ok(jiff::civil::DateTime::from_parts(date, time))
}

/// Decompose a real `jiff::civil::DateTime` back into a local date-time
/// descriptor — always as a complete CALENDAR date, regardless of which
/// of the three forms [`local_date_time_descriptor_to_jiff_civil_datetime`]
/// originally produced it: `jiff::civil::Date` carries no trace of
/// having been constructed via its ordinal or week-date cross-
/// representation, so calendar form is the one canonical, always-
/// available choice (the same precision-is-not-preserved reasoning
/// `jiff_offset_to_utc_offset_descriptor`'s own doc comment documents
/// for offset minutes).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn jiff_civil_datetime_to_local_date_time_descriptor(
    datetime: jiff::civil::DateTime,
) -> Result<LocalDateTimeDescriptor, TemporalError> {
    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "datetime's {field} does not fit the descriptor's component range: {err}"
        )))
    };
    let year = i32::from(datetime.year());
    let month = u8::try_from(datetime.month()).map_err(|e| out_of_range("month", e))?;
    let day = u8::try_from(datetime.day()).map_err(|e| out_of_range("day", e))?;
    let hour = u8::try_from(datetime.hour()).map_err(|e| out_of_range("hour", e))?;
    let minute = u8::try_from(datetime.minute()).map_err(|e| out_of_range("minute", e))?;
    let second = u8::try_from(datetime.second()).map_err(|e| out_of_range("second", e))?;
    let nanos = datetime.subsec_nanosecond();

    let mut time_builder = LocalTimeDescriptorBuilder::default()
        .hour(hour)
        .minute(minute)
        .second(second);
    if nanos != 0 {
        time_builder = time_builder.fractional_second(FractionalSecondDescriptor::new(
            nanos_to_fractional_seconds_digits(i64::from(nanos)),
        ));
    }
    let time = time_builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "could not build a local time descriptor: {err}"
        )))
    })?;

    LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(time)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not build a local date-time descriptor: {err}"
            )))
        })
}

// ── Instant native bridge ────────────────────────────────────────────
//
// `TemporalInstantNativeBridge<JiffVerifier>` is the `realize_offset_
// date_time` / `reflect_offset_date_time` inverse pair. Both are real:
// they carry an offset-aware timestamp descriptor through an actual
// `jiff::civil::DateTime` + `jiff::tz::Offset` pair and back.

impl
    Exchange<ReflectedOffsetDateTime, ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedOffsetDateTime,
    ) -> Result<ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, TemporalError> {
        let descriptor = input.descriptor();
        let local = local_date_time_descriptor_to_jiff_civil_datetime(descriptor.local())?;
        let offset = utc_offset_descriptor_to_jiff_offset(descriptor.offset())?;
        let token = <ReflectedOffsetDateTime as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenOffsetDateTimeCarrier::<JiffOffsetDateTime>::new(
            JiffOffsetDateTime { local, offset },
            token,
        ))
    }
}

impl
    Exchange<ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, ReflectedOffsetDateTime, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
    ) -> Result<ReflectedOffsetDateTime, TemporalError> {
        let carrier = input.carrier();
        let local = jiff_civil_datetime_to_local_date_time_descriptor(carrier.local)?;
        let offset = jiff_offset_to_utc_offset_descriptor(carrier.offset)?;
        let descriptor = OffsetDateTimeDescriptorBuilder::default()
            .local(local)
            .offset(offset)
            .build()
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                    "could not build an offset date-time descriptor: {err}"
                )))
            })?;
        let token =
            <ProvenOffsetDateTimeCarrier<JiffOffsetDateTime> as Sidecar<JiffVerifier>>::sidecar(
                &input,
            );
        Ok(ReflectedOffsetDateTime::new(descriptor, token))
    }
}

// ── Civil native bridge ──────────────────────────────────────────────
//
// `TemporalCivilNativeBridge<JiffVerifier>` is the `realize_local_date_
// time` / `reflect_local_date_time` inverse pair. Both are real, and
// both now cover all three complete-date forms (calendar, ordinal,
// week) via `complete_date_descriptor_to_jiff_date` above — a genuine
// widening of Phase 2's own calendar-only scope, not a duplicate of it.

impl Exchange<ReflectedLocalDateTime, ProvenLocalDateTimeCarrier<JiffDateTime>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedLocalDateTime,
    ) -> Result<ProvenLocalDateTimeCarrier<JiffDateTime>, TemporalError> {
        let datetime = local_date_time_descriptor_to_jiff_civil_datetime(input.descriptor())?;
        let token = <ReflectedLocalDateTime as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenLocalDateTimeCarrier::<JiffDateTime>::new(
            JiffDateTime(datetime),
            token,
        ))
    }
}

impl Exchange<ProvenLocalDateTimeCarrier<JiffDateTime>, ReflectedLocalDateTime, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenLocalDateTimeCarrier<JiffDateTime>,
    ) -> Result<ReflectedLocalDateTime, TemporalError> {
        let descriptor = jiff_civil_datetime_to_local_date_time_descriptor(input.carrier().0)?;
        let token =
            <ProvenLocalDateTimeCarrier<JiffDateTime> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedLocalDateTime::new(descriptor, token))
    }
}
