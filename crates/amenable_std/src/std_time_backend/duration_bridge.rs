use std::time::Duration;

use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    DurationDescriptor, DurationDescriptorBuilder, InvalidDescriptorSource, ProvenDurationCarrier,
    ReflectedDuration, TemporalError, TemporalErrorKind, UnsupportedSource,
};

use crate::{CanaryVerifier, StdDuration, StdTimeBackend};

// ── Real conversions to `std::time` ─────────────────────────────────

/// Resolve an ISO 8601 duration descriptor to a `std::time::Duration`.
///
/// Rejects the calendar-variable components (years, months) and fractional
/// suffixes: a `std::time::Duration` is a fixed span of whole nanoseconds,
/// so those simply do not convert.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
fn duration_descriptor_to_std(descriptor: &DurationDescriptor) -> Result<Duration, TemporalError> {
    if descriptor.years() != 0 || descriptor.months() != 0 {
        return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(
                "a duration with year or month components has no fixed std::time::Duration"
                    .to_owned(),
            ),
        )));
    }
    if descriptor.fractional_component().is_some() {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(
                "the std::time canary carries whole-second durations only".to_owned(),
            ),
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
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "duration's {days}-day span exceeds the descriptor's u32 day component: {err}"
            )),
        ))
    })?;

    DurationDescriptorBuilder::default()
        .days(days)
        .hours(u32::try_from(hours).unwrap_or_default())
        .minutes(u32::try_from(minutes).unwrap_or_default())
        .seconds(u32::try_from(seconds).unwrap_or_default())
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build a duration descriptor: {err}"
                )),
            ))
        })
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
            StdDuration::new(span),
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
        let descriptor = std_to_duration_descriptor(**input.carrier())?;
        let token =
            <ProvenDurationCarrier<StdDuration> as Sidecar<CanaryVerifier>>::sidecar(&input);
        Ok(ReflectedDuration::new(descriptor, token))
    }
}
