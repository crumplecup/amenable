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
//! Offset`/a small `JiffOffsetDateTime` composite.** **Phase 3:
//! `TemporalCivilProps` + `TemporalCivilNativeBridge` over
//! `jiff::civil::{Date,Time,DateTime,ISOWeekDate}` — widens Phase 2's
//! own calendar-date-only `LocalDateTime` realize/reflect to real
//! ordinal- and week-date support too.** **Phase 4: `TemporalZoneProps`
//! plus `TemporalZoneNativeBridge` over `jiff::tz::TimeZone`/`jiff::Zoned`,
//! real IANA tzdb lookups and zoned-instant construction, the biggest
//! genuine capability jump over the `std::time` canary, which can't
//! touch named zones at all.** **Phase 4b: `TemporalZoneFactory` plus
//! `TemporalNativeZoneFactory`, the higher-order zone-resolution
//! factory, including real ambiguity (fold) and gap disambiguation and
//! a genuine offset/named-zone consistency check Phase 4's own native
//! bridge never performed.** **Phase 5: `TemporalConversionFactory`
//! plus `TemporalNativeConversionFactory` — real UTC normalization,
//! real named-zone stripping (reusing Phase 4b's own now-consistency-
//! checked zoned conversion), and real, honest lossless-vs-lossy
//! sub-second precision adjustment.** **Phase 6: `TemporalReporter`'s
//! real capability declaration, every flag checked against jiff's own
//! real source or docs rather than assumed from the canary's own
//! values.** **Phase 7 (this file, so far, on top of the phases above):
//! `TemporalIntervalFactory` — real duration parsing and real
//! `order_offset_endpoints` arithmetic; the two full interval-text-parse
//! edges are an honest `Unsupported` for now, since they need
//! `TemporalParser` (Phase 9), not yet built.** Every other
//! `Temporal*Props`/`NativeBridge`/`Factory` family named in the plan
//! doc's checklist lands in later commits, each widening this same
//! `JiffTimeBackend` struct with its own real `Exchange` impls.

use amenable_core::{
    ClassifiedWitness, Establish, Exchange, Metadata, OwnedEntry, Provenance, Sidecar, Standard,
    Verifier, Witness, WitnessSupportSummary,
};
use amenable_time::{
    AdjustPrecisionLosslesslyEstablished, AdjustPrecisionLosslesslyInput,
    AdjustPrecisionLosslesslyNativeEstablished, AdjustPrecisionLosslesslyNativeInput,
    AdjustPrecisionLosslesslyNativeOutput, AdjustPrecisionLosslesslyOutput,
    AdjustPrecisionLosslesslyPreconditionsToken, AttachNamedZoneEstablished,
    AttachNamedZoneEstablishedToken, AttachNamedZoneInput, AttachNamedZoneNativeInput,
    AttachNamedZoneOutput, AttachNamedZonePreconditions, AttachNamedZonePreconditionsToken,
    CalendarDateDescriptor, CompleteDateDescriptor, ConfirmNamedZoneRevisionEstablished,
    ConfirmNamedZoneRevisionEstablishedToken, ConfirmNamedZoneRevisionInput,
    ConfirmNamedZoneRevisionNativeOutput, ConfirmNamedZoneRevisionOutput,
    ConfirmNamedZoneRevisionPreconditions, ConfirmNamedZoneRevisionPreconditionsToken,
    ConfirmZoneAuthorityEstablished, ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput,
    ConfirmZoneAuthorityPreconditionsToken, DurationDescriptor, DurationDescriptorBuilder,
    DurationFormValid, DurationFractionDescriptor, FractionalSecondDescriptor,
    LocalDateTimeDescriptor, LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder,
    LocalTimeZoneResolutionAuthorityDescriptor, NamedTimeZoneDescriptor,
    NamedTimeZoneDescriptorBuilder, NamedTimeZoneIdentityValid, NamedTimeZoneRevisionBundle,
    NormalizeToUtcEstablished, NormalizeToUtcInput, NormalizeToUtcNativeEstablished,
    NormalizeToUtcNativeOutput, NormalizeToUtcOutput, NormalizeToUtcPreconditionsToken,
    OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder, OffsetDateTimeProof,
    OffsetDateTimeProofToken, OffsetDateTimeSemanticBundle, OrderOffsetEndpointsEstablished,
    OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput, OrderOffsetEndpointsPreconditionsToken,
    ParsedDuration, ParsedRecurringInterval, ParsedTimeInterval, PrecisionDescriptor,
    ProvenDurationCarrier, ProvenLocalDateTimeCarrier, ProvenNamedTimeZoneCarrier,
    ProvenOffsetDateTimeCarrier, ProvenZonedDateTimeCarrier, RawInput, ReflectedDuration,
    ReflectedLocalDateTime, ReflectedNamedTimeZone, ReflectedOffsetDateTime,
    ReflectedZonedDateTime, ResolveLocalDateTimeEstablished, ResolveLocalDateTimeInput,
    ResolveLocalDateTimeNativeEstablished, ResolveLocalDateTimeNativeInput,
    ResolveLocalDateTimeNativeOutput, ResolveLocalDateTimeOutput,
    ResolveLocalDateTimePreconditionsToken, ResolvedNamedTimeZone, RoundingModeDescriptor,
    SerializationProfile, StripNamedZoneEstablished, StripNamedZoneInput, StripNamedZoneOutput,
    StripNamedZonePreconditionsToken, TemporalCivilProps, TemporalComponent, TemporalDurationProps,
    TemporalError, TemporalErrorKind, TemporalInputToken, TemporalInstantProps, TemporalProvenance,
    TemporalReporter, TemporalZoneProps, TruncateSubsecondsEstablished, TruncateSubsecondsInput,
    TruncateSubsecondsNativeEstablished, TruncateSubsecondsNativeInput,
    TruncateSubsecondsNativeOutput, TruncateSubsecondsOutput, TruncateSubsecondsPreconditionsToken,
    UtcOffsetDescriptor, UtcOffsetDescriptorBuilder, UtcOffsetRelationship, UtcOffsetSign,
    ZoneAmbiguityResolutionDescriptor, ZoneGapResolutionDescriptor, ZonedDateTimeDescriptor,
    ZonedDateTimeDescriptorBuilder, ZonedDateTimeSemanticBundle,
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

// ── Phase 4: Zone ────────────────────────────────────────────────────

/// A [`jiff::tz::TimeZone`] as a named-time-zone carrier.
///
/// `jiff::tz::TimeZone` derives only `Clone, Eq, PartialEq` (confirmed
/// by reading jiff's real source — no `Copy`, no `Hash`, no `Default`,
/// consistent with it being a hand-rolled pointer-tagged union that may
/// own real TZif data) — this wrapper's own derive list matches, and,
/// same as `JiffOffset`'s own doc comment, needs an explicit
/// `basis_ctor` (`TimeZone::UTC`, a real jiff constant) since there is
/// no `Default` to fall back to.
#[derive(Debug, Clone, PartialEq, Eq, amenable_derive::Evidence)]
#[evidence(basis = "Self", basis_ctor = "Self(jiff::tz::TimeZone::UTC)")]
pub struct JiffTimeZone(
    /// The wrapped time zone.
    pub jiff::tz::TimeZone,
);

/// A [`jiff::Zoned`] as a zoned-date-time carrier.
///
/// `jiff::Zoned` derives only `Clone` (confirmed by reading jiff's real
/// source), but DOES have a manual `Default` impl — unlike `JiffTimeZone`
/// above, a bare `#[evidence(basis = "Self")]` works here without a
/// `basis_ctor` override.
#[derive(Debug, Clone, Default, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct JiffZoned(
    /// The wrapped zoned date-time.
    pub jiff::Zoned,
);

impl TemporalZoneProps for JiffTimeBackend {
    type NamedTimeZone = JiffTimeZone;
    type ZonedDateTime = JiffZoned;
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

/// Resolve a full offset date-time descriptor to its real jiff parts —
/// factored out of the `TemporalInstantNativeBridge` realize body so
/// Phase 4's own `ZonedDateTime` realize (which needs the identical
/// local + offset pair, pinned to a concrete instant before re-attaching
/// a real named zone) can reuse it rather than duplicating it.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn offset_date_time_descriptor_to_jiff_parts(
    descriptor: &OffsetDateTimeDescriptor,
) -> Result<(jiff::civil::DateTime, jiff::tz::Offset), TemporalError> {
    let local = local_date_time_descriptor_to_jiff_civil_datetime(descriptor.local())?;
    let offset = utc_offset_descriptor_to_jiff_offset(descriptor.offset())?;
    Ok((local, offset))
}

/// The inverse of [`offset_date_time_descriptor_to_jiff_parts`] —
/// factored out for the same reuse reason.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn jiff_parts_to_offset_date_time_descriptor(
    local: jiff::civil::DateTime,
    offset: jiff::tz::Offset,
) -> Result<OffsetDateTimeDescriptor, TemporalError> {
    let local = jiff_civil_datetime_to_local_date_time_descriptor(local)?;
    let offset = jiff_offset_to_utc_offset_descriptor(offset)?;
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not build an offset date-time descriptor: {err}"
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
        let (local, offset) = offset_date_time_descriptor_to_jiff_parts(input.descriptor())?;
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
        let descriptor = jiff_parts_to_offset_date_time_descriptor(carrier.local, carrier.offset)?;
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

// ── Real conversions to/from jiff's zone types ──────────────────────

/// Resolve a named-time-zone descriptor to a real `jiff::tz::TimeZone`.
///
/// Real IANA zone lookup via `TimeZone::get`. The descriptor's own
/// `tzdb_revision` field is ignored on this direction: jiff's real
/// `TimeZone::get` has no parameter for selecting a specific tzdb
/// revision at all — it always resolves against whichever tzdb the
/// running process is linked against — so there is no honest way to
/// honor a *different* revision than that one, and pretending
/// otherwise would be dishonest. This mirrors `TemporalReporter::
/// current_tzdb_revision` staying `None`: jiff exposes no public API to
/// query the linked tzdb's own revision string either (confirmed by
/// checking its real `tz::db` module for a `version`/`revision`
/// function — none exists).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn named_time_zone_descriptor_to_jiff_time_zone(
    descriptor: &NamedTimeZoneDescriptor,
) -> Result<jiff::tz::TimeZone, TemporalError> {
    jiff::tz::TimeZone::get(descriptor.identifier()).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "could not resolve IANA time zone {:?}: {err}",
            descriptor.identifier()
        )))
    })
}

/// Decompose a real `jiff::tz::TimeZone` back into a named-time-zone
/// descriptor.
///
/// Real, honest scoping: a `TimeZone` with no IANA identifier at all —
/// `TimeZone::unknown()` (the special, explicitly-non-IANA `Etc/Unknown`
/// marker) or any `TimeZone::fixed(offset)`-constructed value — has no
/// named-zone descriptor to decompose into; that is a real `Unsupported`
/// error, not a fabricated identifier. `TimeZone::UTC` is genuinely
/// EXEMPT from this — real source confirms `iana_name()`'s own `UTC =>
/// Some("UTC")` match arm treats it as a real, valid identifier, a
/// finding this file's own first test attempt got wrong by assuming
/// resemblance to `Offset`'s unrelated "no identifier" shape rather
/// than checking `TimeZone::iana_name()`'s real match arms directly.
/// `tzdb_revision` is always `None` here for the same reason it's
/// ignored on the realize direction above.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn jiff_time_zone_to_named_time_zone_descriptor(
    tz: &jiff::tz::TimeZone,
) -> Result<NamedTimeZoneDescriptor, TemporalError> {
    let identifier = tz.iana_name().ok_or_else(|| {
        TemporalError::new(TemporalErrorKind::Unsupported(
            "this jiff::tz::TimeZone has no IANA identifier to decompose into a named-zone \
             descriptor (it is unknown, or a fixed offset)"
                .to_owned(),
        ))
    })?;
    NamedTimeZoneDescriptorBuilder::default()
        .identifier(identifier)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not build a named time zone descriptor: {err}"
            )))
        })
}

/// Resolve a zoned date-time descriptor to a real `jiff::Zoned`.
///
/// The descriptor's own offset pins the exact instant (via a fixed-
/// offset zone, which never has ambiguity — the whole point of carrying
/// an explicit offset alongside the zone identity); that instant is then
/// re-attached to the real named zone the descriptor also names, via
/// `Timestamp::to_zoned` (infallible once the instant itself is known).
///
/// Retrofitted (Phase 4b/5) to genuinely check `offset_is_consistent_
/// with_named_zone` before pinning — this function originally accepted
/// ANY offset unconditionally, a real gap only found while building
/// `attach_named_zone`'s own consistency proof in Phase 4b, which is
/// supposed to be the FIRST place that fact gets established. Every
/// caller of this function (Phase 4's own `TemporalZoneNativeBridge`
/// realize body included) now gets the check for real, not just
/// `attach_named_zone`'s own callers.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn zoned_date_time_descriptor_to_jiff_zoned(
    descriptor: &ZonedDateTimeDescriptor,
) -> Result<jiff::Zoned, TemporalError> {
    let (local, offset) = offset_date_time_descriptor_to_jiff_parts(descriptor.timestamp())?;
    let named_tz = named_time_zone_descriptor_to_jiff_time_zone(descriptor.zone())?;
    if !offset_is_consistent_with_named_zone(local, &named_tz, offset) {
        return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            format!(
                "offset {offset:?} is not one of the real offsets {}'s own rules produce for local \
             time {local}",
                named_tz.iana_name().unwrap_or("<unnamed>"),
            ),
        )));
    }
    let timestamp = jiff::tz::TimeZone::fixed(offset)
        .to_zoned(local)
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not pin the offset date-time to a fixed instant: {err}"
            )))
        })?
        .timestamp();
    Ok(timestamp.to_zoned(named_tz))
}

/// Decompose a real `jiff::Zoned` back into a zoned date-time
/// descriptor.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn jiff_zoned_to_zoned_date_time_descriptor(
    zoned: &jiff::Zoned,
) -> Result<ZonedDateTimeDescriptor, TemporalError> {
    let timestamp = jiff_parts_to_offset_date_time_descriptor(zoned.datetime(), zoned.offset())?;
    let zone = jiff_time_zone_to_named_time_zone_descriptor(zoned.time_zone())?;
    ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(zone)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not build a zoned date-time descriptor: {err}"
            )))
        })
}

// ── Zone native bridge ───────────────────────────────────────────────
//
// `TemporalZoneNativeBridge<JiffVerifier>` is the `realize_named_time_
// zone` / `reflect_named_time_zone` / `realize_zoned_date_time` /
// `reflect_zoned_date_time` four-edge bundle. All real: named-zone
// resolution goes through jiff's actual IANA tzdb lookup, and the
// zoned-date-time pair composes it with Phase 2's own offset-date-time
// conversion.

impl Exchange<ReflectedNamedTimeZone, ProvenNamedTimeZoneCarrier<JiffTimeZone>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedNamedTimeZone,
    ) -> Result<ProvenNamedTimeZoneCarrier<JiffTimeZone>, TemporalError> {
        let tz = named_time_zone_descriptor_to_jiff_time_zone(input.descriptor())?;
        let token = <ReflectedNamedTimeZone as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenNamedTimeZoneCarrier::<JiffTimeZone>::new(
            JiffTimeZone(tz),
            token,
        ))
    }
}

impl Exchange<ProvenNamedTimeZoneCarrier<JiffTimeZone>, ReflectedNamedTimeZone, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenNamedTimeZoneCarrier<JiffTimeZone>,
    ) -> Result<ReflectedNamedTimeZone, TemporalError> {
        let descriptor = jiff_time_zone_to_named_time_zone_descriptor(&input.carrier().0)?;
        let token =
            <ProvenNamedTimeZoneCarrier<JiffTimeZone> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedNamedTimeZone::new(descriptor, token))
    }
}

impl Exchange<ReflectedZonedDateTime, ProvenZonedDateTimeCarrier<JiffZoned>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedZonedDateTime,
    ) -> Result<ProvenZonedDateTimeCarrier<JiffZoned>, TemporalError> {
        let zoned = zoned_date_time_descriptor_to_jiff_zoned(input.descriptor())?;
        let token = <ReflectedZonedDateTime as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenZonedDateTimeCarrier::<JiffZoned>::new(
            JiffZoned(zoned),
            token,
        ))
    }
}

impl Exchange<ProvenZonedDateTimeCarrier<JiffZoned>, ReflectedZonedDateTime, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<JiffZoned>,
    ) -> Result<ReflectedZonedDateTime, TemporalError> {
        let descriptor = jiff_zoned_to_zoned_date_time_descriptor(&input.carrier().0)?;
        let token =
            <ProvenZonedDateTimeCarrier<JiffZoned> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedZonedDateTime::new(descriptor, token))
    }
}

// ── Real zone-transition-resolution logic (Phase 4b core) ──────────
//
// amenable_time's own `LocalTimeZoneResolutionAuthorityDescriptor`
// splits disambiguation into TWO independent axes — `ambiguity`
// (earlier/later, for a *fold*) and `gap` (forward/backward, for a
// *gap*) — where jiff's own convenience methods (`.compatible()`/
// `.earlier()`/`.later()`) apply the SAME direction to both cases at
// once. Matching directly on `jiff::tz::AmbiguousOffset`'s own real
// variants (confirmed via source read) is required to honor the two
// axes independently; a first design draft assumed the two cases
// shared the same "earlier means before-field" convention and got it
// wrong — jiff's own real `earlier()`/`later()` doc comments (and their
// `match` bodies) confirm the GAP case is inverted relative to naive
// expectation: "earlier" (the earlier resulting INSTANT) uses the
// `after` offset for a gap (shifting the skipped local time forward,
// which produces an earlier real instant than the naive `before`
// offset would), and "later" uses `before`. For a fold, the mapping is
// the direct one: "earlier" uses `before`, "later" uses `after`.
// amenable_time's own `ShiftForward`/`ShiftBackward` naming names the
// LOCAL-TIME-axis direction directly (forward past the gap = `after`,
// backward before it = `before`), so no inversion is needed there.

/// Resolve a real `jiff::tz::AmbiguousOffset` against amenable_time's
/// own two-axis resolution authority.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn resolve_ambiguous_offset(
    ambiguous: jiff::tz::AmbiguousOffset,
    authority: &LocalTimeZoneResolutionAuthorityDescriptor,
) -> jiff::tz::Offset {
    match ambiguous {
        jiff::tz::AmbiguousOffset::Unambiguous { offset } => offset,
        jiff::tz::AmbiguousOffset::Gap { before, after } => match authority.gap() {
            ZoneGapResolutionDescriptor::ShiftForward => after,
            ZoneGapResolutionDescriptor::ShiftBackward => before,
        },
        jiff::tz::AmbiguousOffset::Fold { before, after } => match authority.ambiguity() {
            ZoneAmbiguityResolutionDescriptor::PreferEarlier => before,
            ZoneAmbiguityResolutionDescriptor::PreferLater => after,
        },
    }
}

/// Check whether a given offset is one jiff's real zone rules could
/// actually produce for the given local wall-clock time — i.e. it's
/// either the unambiguous offset, or one of the real gap/fold
/// candidates.
///
/// A real, previously-missing check: Phase 4's own `zoned_date_time_
/// descriptor_to_jiff_zoned` (the `TemporalZoneNativeBridge` realize
/// body) never validated this at all — it pinned the instant via
/// `TimeZone::fixed(offset)`, which accepts ANY offset value
/// unconditionally, silently producing a `Zoned` whose local wall-clock
/// reading (recomputed from the real named zone's own rules at the
/// resulting instant) can differ from what was actually requested when
/// the given offset was wrong. `attach_named_zone`/`attach_named_zone_
/// native` below are the edges that are actually supposed to prove
/// `OffsetConsistentWithNamedZone`, so this check belongs here, for
/// real, rather than staying silently absent.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn offset_is_consistent_with_named_zone(
    local: jiff::civil::DateTime,
    tz: &jiff::tz::TimeZone,
    offset: jiff::tz::Offset,
) -> bool {
    match tz.to_ambiguous_zoned(local).offset() {
        jiff::tz::AmbiguousOffset::Unambiguous { offset: real } => offset == real,
        jiff::tz::AmbiguousOffset::Gap { before, after }
        | jiff::tz::AmbiguousOffset::Fold { before, after } => offset == before || offset == after,
    }
}

/// Resolve a local wall-clock date-time against a real named zone,
/// applying amenable_time's own resolution authority to any genuine
/// gap/fold ambiguity.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn resolve_local_date_time_to_jiff_zoned(
    local: jiff::civil::DateTime,
    tz: jiff::tz::TimeZone,
    authority: &LocalTimeZoneResolutionAuthorityDescriptor,
) -> Result<jiff::Zoned, TemporalError> {
    let ambiguous = tz.to_ambiguous_zoned(local).offset();
    let offset = resolve_ambiguous_offset(ambiguous, authority);
    let timestamp = offset.to_timestamp(local).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "resolved local date-time out of jiff's representable instant range: {err}"
        )))
    })?;
    Ok(timestamp.to_zoned(tz))
}

/// Attach a real named zone to an explicit offset date-time, after
/// genuinely checking the given offset is consistent with what the
/// zone's own real rules say for that local time.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn attach_named_zone_to_jiff_zoned(
    local: jiff::civil::DateTime,
    offset: jiff::tz::Offset,
    tz: jiff::tz::TimeZone,
) -> Result<jiff::Zoned, TemporalError> {
    if !offset_is_consistent_with_named_zone(local, &tz, offset) {
        return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            format!(
                "offset {offset:?} is not one of the real offsets {}'s own rules produce for local \
             time {local}",
                tz.iana_name().unwrap_or("<unnamed>"),
            ),
        )));
    }
    let timestamp = offset.to_timestamp(local).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
            "could not pin the offset date-time to a fixed instant: {err}"
        )))
    })?;
    Ok(timestamp.to_zoned(tz))
}

// ── Zone factory (descriptor-level) ─────────────────────────────────
//
// `TemporalZoneFactory<JiffVerifier>`'s 5 edges. Unlike
// `TemporalZoneNativeBridge`, these operate on neutral descriptors
// throughout — `resolve_named_zone`'s raw-text parse, plus 4
// descriptor-to-descriptor computations reusing the core logic above
// and Phase 2/4's own conversion helpers.

impl Exchange<RawInput, ResolvedNamedTimeZone, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ResolvedNamedTimeZone, TemporalError> {
        let identifier = input.as_str();
        jiff::tz::TimeZone::get(identifier).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not resolve IANA time zone {identifier:?}: {err}"
            )))
        })?;
        let descriptor = NamedTimeZoneDescriptorBuilder::default()
            .identifier(identifier)
            .build()
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                    "could not build a named time zone descriptor: {err}"
                )))
            })?;
        let input_token = <RawInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token =
            <NamedTimeZoneIdentityValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
                input_token,
            );
        Ok(ResolvedNamedTimeZone::new(descriptor, token))
    }
}

impl Exchange<ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ConfirmZoneAuthorityInput,
    ) -> Result<ConfirmZoneAuthorityOutput, TemporalError> {
        // jiff's own Disambiguation semantics apply uniformly to any
        // real named zone -- no zone-specific restriction on which
        // resolution authority is valid -- so the real check here is
        // exactly the zone identity itself.
        let _tz = named_time_zone_descriptor_to_jiff_time_zone(input.request().zone())?;
        let input_token = <ConfirmZoneAuthorityInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <ConfirmZoneAuthorityEstablished as Establish<
            ConfirmZoneAuthorityPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ConfirmZoneAuthorityOutput::new(
            ConfirmZoneAuthorityEstablished::default(),
            token,
        ))
    }
}

impl Exchange<ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ResolveLocalDateTimeInput,
    ) -> Result<ResolveLocalDateTimeOutput, TemporalError> {
        let request = input.request();
        let local = local_date_time_descriptor_to_jiff_civil_datetime(request.timestamp())?;
        let tz = named_time_zone_descriptor_to_jiff_time_zone(request.zone())?;
        let zoned =
            resolve_local_date_time_to_jiff_zoned(local, tz, request.resolution_authority())?;
        let descriptor = jiff_zoned_to_zoned_date_time_descriptor(&zoned)?;
        let input_token = <ResolveLocalDateTimeInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <ResolveLocalDateTimeEstablished as Establish<
            ResolveLocalDateTimePreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ResolveLocalDateTimeOutput::new(descriptor, token))
    }
}

impl Exchange<AttachNamedZoneInput, AttachNamedZoneOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AttachNamedZoneInput,
    ) -> Result<AttachNamedZoneOutput, TemporalError> {
        let request = input.request();
        let (local, offset) = offset_date_time_descriptor_to_jiff_parts(request.timestamp())?;
        let tz = named_time_zone_descriptor_to_jiff_time_zone(request.zone())?;
        let zoned = attach_named_zone_to_jiff_zoned(local, offset, tz)?;
        let descriptor = jiff_zoned_to_zoned_date_time_descriptor(&zoned)?;
        let input_token = <AttachNamedZoneInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <AttachNamedZoneEstablished as Establish<
            AttachNamedZonePreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(AttachNamedZoneOutput::new(descriptor, token))
    }
}

impl Exchange<ConfirmNamedZoneRevisionInput, ConfirmNamedZoneRevisionOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ConfirmNamedZoneRevisionInput,
    ) -> Result<ConfirmNamedZoneRevisionOutput, TemporalError> {
        // Real, honest scoping: jiff exposes no tzdb-revision concept at
        // all (Phase 4's own finding), so "tracks the tzdb revision" is
        // trivially satisfied for this backend -- there is only ever
        // one revision, whichever the running process is linked
        // against. What IS real and worth checking here: that the
        // descriptor's own offset is STILL consistent with the real
        // zone's rules -- the same check `attach_named_zone` performs,
        // re-run rather than assumed to still hold.
        let descriptor = input.request().timestamp();
        let (local, offset) = offset_date_time_descriptor_to_jiff_parts(descriptor.timestamp())?;
        let tz = named_time_zone_descriptor_to_jiff_time_zone(descriptor.zone())?;
        if !offset_is_consistent_with_named_zone(local, &tz, offset) {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                format!(
                    "offset {offset:?} is no longer consistent with {}'s real rules for local time \
                 {local}",
                    tz.iana_name().unwrap_or("<unnamed>"),
                ),
            )));
        }
        let input_token = <ConfirmNamedZoneRevisionInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <ConfirmNamedZoneRevisionEstablished as Establish<
            ConfirmNamedZoneRevisionPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ConfirmNamedZoneRevisionOutput::new(
            ConfirmNamedZoneRevisionEstablished::default(),
            token,
        ))
    }
}

// ── Zone factory (native-level) ─────────────────────────────────────
//
// `TemporalNativeZoneFactory<JiffVerifier>`'s 3 edges. These operate
// directly on already-native jiff values (no descriptor conversion at
// all), reusing the exact same core resolution/consistency logic above.

impl
    Exchange<
        ResolveLocalDateTimeNativeInput<JiffTimeBackend>,
        ResolveLocalDateTimeNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ResolveLocalDateTimeNativeInput<JiffTimeBackend>,
    ) -> Result<ResolveLocalDateTimeNativeOutput<JiffTimeBackend>, TemporalError> {
        let request = input.request();
        let zoned = resolve_local_date_time_to_jiff_zoned(
            request.timestamp().0,
            request.zone().0.clone(),
            request.resolution_authority(),
        )?;
        let input_token = <ResolveLocalDateTimeNativeInput<JiffTimeBackend> as Sidecar<
            JiffVerifier,
        >>::sidecar(&input);
        let token = <ResolveLocalDateTimeNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(ResolveLocalDateTimeNativeOutput::new(
            JiffZoned(zoned),
            token,
        ))
    }
}

impl
    Exchange<
        AttachNamedZoneNativeInput<JiffTimeBackend>,
        ProvenZonedDateTimeCarrier<JiffZoned>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AttachNamedZoneNativeInput<JiffTimeBackend>,
    ) -> Result<ProvenZonedDateTimeCarrier<JiffZoned>, TemporalError> {
        let request = input.request();
        let zoned = attach_named_zone_to_jiff_zoned(
            request.timestamp().local,
            request.timestamp().offset,
            request.zone().0.clone(),
        )?;
        // Unlike the other native-factory edge above, this edge's own
        // output shares `ZonedDateTimeSemanticBundleToken` with the
        // descriptor-level `attach_named_zone` and Phase 4's own
        // TemporalZoneNativeBridge -- there is exactly one real
        // Establish chain for that proposition (`AttachNamedZonePreconditions`
        // -> `AttachNamedZoneEstablished` -> `ZonedDateTimeSemanticBundle`),
        // so this edge walks the SAME chain internally, starting from
        // its own bare `TemporalInputToken` (the native factory's
        // deliberately minimal precondition -- the real work is this
        // exchange body's own runtime check above, not a richer
        // precondition chain).
        let input_token =
            <AttachNamedZoneNativeInput<JiffTimeBackend> as Sidecar<JiffVerifier>>::sidecar(&input);
        let preconditions_token = <AttachNamedZonePreconditions as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(input_token);
        let established_token = <AttachNamedZoneEstablished as Establish<
            AttachNamedZonePreconditionsToken,
            JiffVerifier,
        >>::establish(preconditions_token);
        let bundle_token = <ZonedDateTimeSemanticBundle as Establish<
            AttachNamedZoneEstablishedToken,
            JiffVerifier,
        >>::establish(established_token);
        Ok(ProvenZonedDateTimeCarrier::<JiffZoned>::new(
            JiffZoned(zoned),
            bundle_token,
        ))
    }
}

impl
    Exchange<
        ProvenZonedDateTimeCarrier<JiffZoned>,
        ConfirmNamedZoneRevisionNativeOutput,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<JiffZoned>,
    ) -> Result<ConfirmNamedZoneRevisionNativeOutput, TemporalError> {
        let zoned = &input.carrier().0;
        let tz = zoned.time_zone();
        let local = zoned.datetime();
        let offset = zoned.offset();
        if !offset_is_consistent_with_named_zone(local, tz, offset) {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                format!(
                    "offset {offset:?} is no longer consistent with {}'s real rules for local time \
                 {local}",
                    tz.iana_name().unwrap_or("<unnamed>"),
                ),
            )));
        }
        // This edge's own input is an already-proven native carrier,
        // not a fresh `TemporalInputToken`-backed request -- there is
        // no existing token on `input` that starts the
        // `ConfirmNamedZoneRevisionPreconditions` chain (its own
        // credential is `TemporalInputToken`, a different proposition
        // than `ZonedDateTimeSemanticBundleToken`), so a fresh root
        // credential is synthesized here, the same way every test
        // helper in this backend's own test suite starts an Establish
        // chain from `TemporalInputToken::new()`.
        let preconditions_token = <ConfirmNamedZoneRevisionPreconditions as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        let established_token = <ConfirmNamedZoneRevisionEstablished as Establish<
            ConfirmNamedZoneRevisionPreconditionsToken,
            JiffVerifier,
        >>::establish(preconditions_token);
        let bundle_token = <NamedTimeZoneRevisionBundle as Establish<
            ConfirmNamedZoneRevisionEstablishedToken,
            JiffVerifier,
        >>::establish(established_token);
        Ok(ConfirmNamedZoneRevisionNativeOutput::new(
            NamedTimeZoneRevisionBundle::default(),
            bundle_token,
        ))
    }
}

// ── Conversion factory (Phase 5) ────────────────────────────────────
//
// `TemporalConversionFactory<JiffVerifier>` plus its native counterpart:
// UTC normalization, named-zone stripping (reusing Phase 4b's own
// consistency-checked `zoned_date_time_descriptor_to_jiff_zoned`), and
// sub-second precision reduction. All four are real jiff arithmetic,
// not stand-ins.

/// Validate that `target` anchors precision at the second and return
/// its declared fractional-digit count (`0` when absent).
///
/// jiff's civil time is always second-plus-nanosecond in shape --
/// there is no jiff concept of a "smallest component" coarser than a
/// second for THIS pair of edges (that belongs to the civil/ordinal
/// round trip built in Phase 3), so any `smallest_component` other
/// than `Second` is a real, honest `Unsupported` case here, not a
/// silent no-op.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn require_second_precision(target: &PrecisionDescriptor) -> Result<u8, TemporalError> {
    if target.smallest_component() != TemporalComponent::Second {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(format!(
            "jiff backend's precision-adjustment edges anchor at the second; \
             smallest_component = {} is not supported",
            target.smallest_component(),
        ))));
    }
    let digits = target.fractional_digits().unwrap_or(0);
    if digits > 9 {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(format!(
            "jiff's civil time is nanosecond-precision (9 fractional digits); \
             {digits} fractional digits exceeds its representable range"
        ))));
    }
    Ok(digits)
}

/// Zero out every nanosecond digit beyond `digits` (0..=9).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn truncate_nanos_to_digits(nanos: i32, digits: u8) -> i32 {
    if digits >= 9 {
        return nanos;
    }
    let scale = 10i32.pow(9 - u32::from(digits));
    (nanos / scale) * scale
}

/// Adjust `local`'s sub-second precision to `target_precision`,
/// honoring `lossless` (reject any real information loss) vs. lossy
/// (require the declared rounding mode to be `Truncate`, since that is
/// the only rounding arithmetic this backend implements -- a non-
/// `Truncate` rounding mode is a real, honest `Unsupported` case, not
/// silently ignored or approximated).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn adjust_jiff_local_precision(
    local: jiff::civil::DateTime,
    target_precision: &PrecisionDescriptor,
    lossless: bool,
) -> Result<jiff::civil::DateTime, TemporalError> {
    let digits = require_second_precision(target_precision)?;
    let original_nanos = local.subsec_nanosecond();
    let truncated_nanos = truncate_nanos_to_digits(original_nanos, digits);
    if truncated_nanos != original_nanos {
        if lossless {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                format!(
                    "adjusting to {digits} fractional digits would discard non-zero \
                 sub-second precision ({original_nanos} nanoseconds); not lossless"
                ),
            )));
        }
        match target_precision.rounding_mode() {
            None | Some(RoundingModeDescriptor::Truncate) => {}
            Some(other) => {
                return Err(TemporalError::new(TemporalErrorKind::Unsupported(format!(
                    "jiff backend only implements truncation for sub-second \
                     precision reduction, not rounding mode {other}"
                ))));
            }
        }
    }
    local
        .with()
        .subsec_nanosecond(truncated_nanos)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not rebuild the precision-adjusted local time: {err}"
            )))
        })
}

/// Descriptor-level wrapper over [`adjust_jiff_local_precision`].
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn adjust_offset_date_time_precision(
    descriptor: &OffsetDateTimeDescriptor,
    target_precision: &PrecisionDescriptor,
    lossless: bool,
) -> Result<OffsetDateTimeDescriptor, TemporalError> {
    let (local, offset) = offset_date_time_descriptor_to_jiff_parts(descriptor)?;
    let adjusted_local = adjust_jiff_local_precision(local, target_precision, lossless)?;
    jiff_parts_to_offset_date_time_descriptor(adjusted_local, offset)
}

impl Exchange<NormalizeToUtcInput, NormalizeToUtcOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: NormalizeToUtcInput) -> Result<NormalizeToUtcOutput, TemporalError> {
        let (local, offset) =
            offset_date_time_descriptor_to_jiff_parts(input.request().timestamp())?;
        let timestamp = offset.to_timestamp(local).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not pin the offset date-time to a fixed instant: {err}"
            )))
        })?;
        let utc = timestamp.to_zoned(jiff::tz::TimeZone::UTC);
        let descriptor = jiff_parts_to_offset_date_time_descriptor(utc.datetime(), utc.offset())?;
        let input_token = <NormalizeToUtcInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <NormalizeToUtcEstablished as Establish<
            NormalizeToUtcPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(NormalizeToUtcOutput::new(descriptor, token))
    }
}

impl Exchange<StripNamedZoneInput, StripNamedZoneOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: StripNamedZoneInput) -> Result<StripNamedZoneOutput, TemporalError> {
        let zoned = zoned_date_time_descriptor_to_jiff_zoned(input.request().timestamp())?;
        let descriptor =
            jiff_parts_to_offset_date_time_descriptor(zoned.datetime(), zoned.offset())?;
        let input_token = <StripNamedZoneInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <StripNamedZoneEstablished as Establish<
            StripNamedZonePreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(StripNamedZoneOutput::new(descriptor, token))
    }
}

impl Exchange<AdjustPrecisionLosslesslyInput, AdjustPrecisionLosslesslyOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AdjustPrecisionLosslesslyInput,
    ) -> Result<AdjustPrecisionLosslesslyOutput, TemporalError> {
        let request = input.request();
        let descriptor = adjust_offset_date_time_precision(
            request.timestamp(),
            request.target_precision(),
            true,
        )?;
        let input_token =
            <AdjustPrecisionLosslesslyInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <AdjustPrecisionLosslesslyEstablished as Establish<
            AdjustPrecisionLosslesslyPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(AdjustPrecisionLosslesslyOutput::new(descriptor, token))
    }
}

impl Exchange<TruncateSubsecondsInput, TruncateSubsecondsOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: TruncateSubsecondsInput,
    ) -> Result<TruncateSubsecondsOutput, TemporalError> {
        let request = input.request();
        let descriptor = adjust_offset_date_time_precision(
            request.timestamp(),
            request.target_precision(),
            false,
        )?;
        let input_token = <TruncateSubsecondsInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <TruncateSubsecondsEstablished as Establish<
            TruncateSubsecondsPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(TruncateSubsecondsOutput::new(descriptor, token))
    }
}

// ── Native conversion factory (Phase 5) ─────────────────────────────
//
// The native mirror of the four edges above, operating directly on
// `JiffOffsetDateTime`/`JiffZoned` carriers instead of descriptors.
// `strip_named_zone_native` has no separate output wrapper -- its own
// trait bound (`TemporalNativeConversionFactory`) returns
// `ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>` directly, since
// the emitted carrier's own `OffsetDateTimeSemanticBundleToken` is
// already the right shape.

impl
    Exchange<
        ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
        NormalizeToUtcNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
    ) -> Result<NormalizeToUtcNativeOutput<JiffTimeBackend>, TemporalError> {
        let carrier = input.carrier();
        let timestamp = carrier.offset.to_timestamp(carrier.local).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                "could not pin the offset date-time to a fixed instant: {err}"
            )))
        })?;
        let utc = timestamp.to_zoned(jiff::tz::TimeZone::UTC);
        let native = JiffOffsetDateTime {
            local: utc.datetime(),
            offset: utc.offset(),
        };
        // This edge's own input carries an `OffsetDateTimeSemanticBundleToken`,
        // a different proposition than `NormalizeToUtcNativeEstablished`'s own
        // single-hop `TemporalInputToken` credential -- so a fresh root
        // credential is synthesized here, the same way Phase 4b's own
        // `ConfirmNamedZoneRevisionNativeOutput` edge already does.
        let token = <NormalizeToUtcNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        Ok(NormalizeToUtcNativeOutput::<JiffTimeBackend>::new(
            native, token,
        ))
    }
}

impl
    Exchange<
        ProvenZonedDateTimeCarrier<JiffZoned>,
        ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<JiffZoned>,
    ) -> Result<ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, TemporalError> {
        let zoned = &input.carrier().0;
        let native = JiffOffsetDateTime {
            local: zoned.datetime(),
            offset: zoned.offset(),
        };
        // Fresh two-hop chain (`TemporalInputToken` -> `OffsetDateTimeProof`
        // -> `OffsetDateTimeSemanticBundle`): the input's own
        // `ZonedDateTimeSemanticBundleToken` proves a different proposition
        // than the output's `OffsetDateTimeSemanticBundleToken`.
        let proof_token =
            <OffsetDateTimeProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
                TemporalInputToken::new(),
            );
        let bundle_token = <OffsetDateTimeSemanticBundle as Establish<
            OffsetDateTimeProofToken,
            JiffVerifier,
        >>::establish(proof_token);
        Ok(ProvenOffsetDateTimeCarrier::<JiffOffsetDateTime>::new(
            native,
            bundle_token,
        ))
    }
}

impl
    Exchange<
        AdjustPrecisionLosslesslyNativeInput<JiffTimeBackend>,
        AdjustPrecisionLosslesslyNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AdjustPrecisionLosslesslyNativeInput<JiffTimeBackend>,
    ) -> Result<AdjustPrecisionLosslesslyNativeOutput<JiffTimeBackend>, TemporalError> {
        let request = input.request();
        let adjusted_local = adjust_jiff_local_precision(
            request.timestamp().local,
            request.target_precision(),
            true,
        )?;
        let native = JiffOffsetDateTime {
            local: adjusted_local,
            offset: request.timestamp().offset,
        };
        let token = <AdjustPrecisionLosslesslyNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        Ok(AdjustPrecisionLosslesslyNativeOutput::<JiffTimeBackend>::new(native, token))
    }
}

impl
    Exchange<
        TruncateSubsecondsNativeInput<JiffTimeBackend>,
        TruncateSubsecondsNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: TruncateSubsecondsNativeInput<JiffTimeBackend>,
    ) -> Result<TruncateSubsecondsNativeOutput<JiffTimeBackend>, TemporalError> {
        let request = input.request();
        let adjusted_local = adjust_jiff_local_precision(
            request.timestamp().local,
            request.target_precision(),
            false,
        )?;
        let native = JiffOffsetDateTime {
            local: adjusted_local,
            offset: request.timestamp().offset,
        };
        let token = <TruncateSubsecondsNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        Ok(TruncateSubsecondsNativeOutput::<JiffTimeBackend>::new(
            native, token,
        ))
    }
}

// ── Reporter (Phase 6) ───────────────────────────────────────────────
//
// `TemporalReporter`'s real capability declaration. Every flag below
// is checked against jiff's own real source/docs, not assumed from the
// canary's own values:
//
// - `max_fractional_second_digits`: `Some(9)` -- jiff's civil time and
//   `Span` are both nanosecond-precision throughout (confirmed in
//   Phases 1-5's own real conversions).
// - `supports_leap_seconds`: `false` -- jiff's own docs state outright
//   "Jiff does not support leap seconds. Jiff behaves as if they don't
//   exist" (verbatim, `civil::DateTime`/`Timestamp`/`civil::Time`/
//   `Zoned`'s own doc comments).
// - `supports_unknown_local_offset`: `false` -- Phase 2's own
//   `realize_offset_date_time_rejects_the_unknown_local_offset_case`
//   test already confirmed `jiff::tz::Offset` has no representation
//   for RFC 9557's `-00:00` convention at all.
// - `supports_named_zone_round_trip`: `true` -- real IANA tzdb lookups
//   via `TimeZone::get`, exercised for real since Phase 4.
// - `supports_end_of_day_twenty_four`: `false` -- `jiff::civil::Time::MAX`
//   is `23:59:59.999999999`; jiff has no `24:00:00` representation.
// - `current_tzdb_revision`: `None` -- jiff exposes no public API to
//   query the linked tzdb's own revision string (checked its real
//   `tz::db` module directly in Phase 4); re-confirmed here, not
//   reassumed.
// - `supported_serialization_profiles`: empty, matching the canary --
//   an honest declaration, not a pessimistic one: no `TemporalParser`/
//   `TemporalFormatter` edge exists on this backend yet (Phases 9-10),
//   so no serialization profile is actually reachable through this
//   backend's own `Exchange` surface today, even though jiff's real
//   `fmt::temporal` module could back several of them once those
//   phases land.
impl TemporalReporter for JiffTimeBackend {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn supported_serialization_profiles(&self) -> Vec<SerializationProfile> {
        Vec::new()
    }

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
        true
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

// ── Interval factory (Phase 7) ──────────────────────────────────────
//
// `TemporalIntervalFactory<JiffVerifier>` is a blanket-impl'd supertrait
// bundle over four `Exchange`s (mirrors `TemporalReporter`'s own
// non-`Exchange` shape by contrast -- this family IS four real edges).
//
// `order_offset_endpoints` is real: it resolves both endpoints to real
// `jiff::Timestamp` values (through the same `offset_date_time_
// descriptor_to_jiff_parts` helper Phase 2 already established) and
// compares them via jiff's own real `Ord` impl -- genuine Gregorian
// calendar arithmetic replacing the std canary's own hand-rolled
// `days_from_civil`.
//
// `duration` text-parsing is real via `jiff::Span: FromStr`. The other
// two text-parse edges (`ParsedTimeInterval`/`ParsedRecurringInterval`)
// are a real, honest `Unsupported` for now, not a stand-in: their
// endpoint values are `TemporalValueDescriptor`, a broad enum spanning
// every temporal form in the whole accord (calendar/ordinal/week dates,
// local/offset/zoned date-times, reduced-precision and CalConnect-only
// forms) -- parsing arbitrary interval-endpoint TEXT into that enum is
// squarely `TemporalParser`'s own job (Phase 9), not yet built on this
// backend. This is a real dependency the plan's own phase-ordering
// underestimated (found by reading `TemporalValueDescriptor`'s actual
// breadth, not assumed), not a shortcut -- see the plan doc's Phase 7
// findings.

impl Exchange<RawInput, ParsedDuration, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedDuration, TemporalError> {
        let text = input.as_str();
        let span: jiff::Span = text.parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseRejected {
                profile: "ISO 8601 duration".to_owned(),
                detail: format!("{err}"),
            })
        })?;
        let descriptor = jiff_span_to_duration_descriptor(span)?;
        let input_token = <RawInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <DurationFormValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            input_token,
        );
        Ok(ParsedDuration::new(descriptor, token))
    }
}

/// Report that arbitrary interval-endpoint text parsing is not yet
/// implemented on this backend (see the module doc comment above).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn unsupported_interval_endpoint_parse(edge: &'static str) -> TemporalError {
    TemporalError::new(TemporalErrorKind::Unsupported(format!(
        "this backend cannot yet parse an arbitrary temporal value as an interval \
         endpoint for {edge}: that requires TemporalParser (Phase 9), not yet built here"
    )))
}

impl Exchange<RawInput, ParsedRecurringInterval, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedRecurringInterval, TemporalError> {
        Err(unsupported_interval_endpoint_parse("a recurring interval"))
    }
}

impl Exchange<RawInput, ParsedTimeInterval, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedTimeInterval, TemporalError> {
        Err(unsupported_interval_endpoint_parse("a time interval"))
    }
}

impl Exchange<OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: OrderOffsetEndpointsInput,
    ) -> Result<OrderOffsetEndpointsOutput, TemporalError> {
        let request = input.request();
        let (start_local, start_offset) =
            offset_date_time_descriptor_to_jiff_parts(request.start())?;
        let (end_local, end_offset) = offset_date_time_descriptor_to_jiff_parts(request.end())?;
        let pin = |offset: jiff::tz::Offset, local: jiff::civil::DateTime| {
            offset.to_timestamp(local).map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(format!(
                    "could not pin the offset date-time to a fixed instant: {err}"
                )))
            })
        };
        let start = pin(start_offset, start_local)?;
        let end = pin(end_offset, end_local)?;

        if start > end {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                format!("interval start ({start}) is after its end ({end})"),
            )));
        }

        let input_token = <OrderOffsetEndpointsInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <OrderOffsetEndpointsEstablished as Establish<
            OrderOffsetEndpointsPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(OrderOffsetEndpointsOutput::new(
            OrderOffsetEndpointsEstablished::default(),
            token,
        ))
    }
}
