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
//! **Phase 1 (this file, so far): `TemporalDurationProps` +
//! `TemporalDurationNativeBridge` over `jiff::Span`.** Every other
//! `Temporal*Props`/`NativeBridge`/`Factory` family named in the plan
//! doc's checklist lands in later commits, each widening this same
//! `JiffTimeBackend` struct with its own real `Exchange` impls.

use amenable_core::{
    ClassifiedWitness, Exchange, Metadata, OwnedEntry, Provenance, Sidecar, Standard, Verifier,
    Witness, WitnessSupportSummary,
};
use amenable_time::{
    DurationDescriptor, DurationDescriptorBuilder, DurationFractionDescriptor,
    ProvenDurationCarrier, ReflectedDuration, TemporalComponent, TemporalDurationProps,
    TemporalError, TemporalErrorKind, TemporalProvenance,
};

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
