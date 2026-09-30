use super::instant::JiffOffsetDateTime;
use super::types::{
    JiffRecurringInterval, JiffTimeBackend, JiffTimeInterval, JiffTimeIntervalEndpoint,
    JiffTimeIntervalRepresentation, JiffVerifier,
};
use crate::jiff::backend::duration::duration_descriptor_to_jiff_span;
use crate::jiff::backend::duration::jiff_span_to_duration_descriptor;
use crate::jiff::backend::instant::complete_date_descriptor_to_jiff_date;
use crate::jiff::backend::instant::jiff_civil_datetime_to_local_date_time_descriptor;
use crate::jiff::backend::instant::jiff_parts_to_offset_date_time_descriptor;
use crate::jiff::backend::instant::local_date_time_descriptor_to_jiff_civil_datetime;
use crate::jiff::backend::instant::offset_date_time_descriptor_to_jiff_parts;
use crate::jiff::backend::zone_conversions::jiff_zoned_to_zoned_date_time_descriptor;
use crate::jiff::backend::zone_conversions::zoned_date_time_descriptor_to_jiff_zoned;
use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, IntervalEndpointOrderingBundle,
    InvalidDescriptorSource, OrderOffsetEndpointsEstablished, OrderOffsetEndpointsEstablishedToken,
    OrderOffsetEndpointsNativeInput, OrderOffsetEndpointsNativeOutput,
    ProvenRecurringIntervalCarrier, ProvenTimeIntervalCarrier,
    QualifiedOrBareTemporalValueDescriptor, RecurringIntervalDescriptorBuilder,
    ReflectedRecurringInterval, ReflectedTimeInterval, TemporalError, TemporalErrorKind,
    TemporalInputToken, TemporalValueDescriptor, TimeIntervalDescriptor, TimeIntervalEndpoint,
    TimeIntervalRepresentation, UnsupportedSource,
};

/// Decompose a real `jiff::civil::Date` into a calendar date
/// descriptor. No existing helper covers this direction alone: every
/// prior phase only ever needed it as part of a larger local-date-time
/// decomposition.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(date)))]
pub(super) fn jiff_date_to_calendar_date_descriptor(
    date: jiff::civil::Date,
) -> Result<CalendarDateDescriptor, TemporalError> {
    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "date's {field} does not fit the descriptor's component range: {err}"
            )),
        ))
    };
    let year = i32::from(date.year());
    let month = u8::try_from(date.month()).map_err(|e| out_of_range("month", e))?;
    let day = u8::try_from(date.day()).map_err(|e| out_of_range("day", e))?;
    Ok(CalendarDateDescriptor::new(year, month, day))
}

/// Resolve a bare temporal value to a real jiff time-interval endpoint.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(value)))]
fn temporal_value_descriptor_to_jiff_endpoint(
    value: &TemporalValueDescriptor,
) -> Result<JiffTimeIntervalEndpoint, TemporalError> {
    match value {
        TemporalValueDescriptor::CalendarDate(date) => Ok(JiffTimeIntervalEndpoint::CalendarDate(
            complete_date_descriptor_to_jiff_date(CompleteDateDescriptor::Calendar(*date))?,
        )),
        TemporalValueDescriptor::OrdinalDate(date) => Ok(JiffTimeIntervalEndpoint::CalendarDate(
            complete_date_descriptor_to_jiff_date(CompleteDateDescriptor::Ordinal(*date))?,
        )),
        TemporalValueDescriptor::WeekDate(date) => Ok(JiffTimeIntervalEndpoint::CalendarDate(
            complete_date_descriptor_to_jiff_date(CompleteDateDescriptor::Week(*date))?,
        )),
        TemporalValueDescriptor::LocalDateTime(datetime) => {
            Ok(JiffTimeIntervalEndpoint::LocalDateTime(
                local_date_time_descriptor_to_jiff_civil_datetime(datetime)?,
            ))
        }
        TemporalValueDescriptor::OffsetDateTime(datetime) => {
            let (local, offset) = offset_date_time_descriptor_to_jiff_parts(datetime)?;
            Ok(JiffTimeIntervalEndpoint::OffsetDateTime(
                JiffOffsetDateTime::new(local, offset),
            ))
        }
        TemporalValueDescriptor::ZonedDateTime(datetime) => {
            Ok(JiffTimeIntervalEndpoint::ZonedDateTime(
                zoned_date_time_descriptor_to_jiff_zoned(datetime)?,
            ))
        }
        other => Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(format!(
                "jiff backend has no representation for the {other:?} interval-endpoint \
             form (ISO 8601-2 / CalConnect extension family, out of scope)"
            )),
        ))),
    }
}

/// Decompose a real jiff time-interval endpoint back into a bare
/// temporal value.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(endpoint)))]
fn jiff_endpoint_to_temporal_value_descriptor(
    endpoint: &JiffTimeIntervalEndpoint,
) -> Result<TemporalValueDescriptor, TemporalError> {
    match endpoint {
        JiffTimeIntervalEndpoint::Open | JiffTimeIntervalEndpoint::Unknown => {
            Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(
                    "an Open/Unknown endpoint carries no temporal value to decompose".to_owned(),
                ),
            )))
        }
        JiffTimeIntervalEndpoint::CalendarDate(date) => Ok(TemporalValueDescriptor::CalendarDate(
            jiff_date_to_calendar_date_descriptor(*date)?,
        )),
        JiffTimeIntervalEndpoint::LocalDateTime(datetime) => {
            Ok(TemporalValueDescriptor::LocalDateTime(
                jiff_civil_datetime_to_local_date_time_descriptor(*datetime)?,
            ))
        }
        JiffTimeIntervalEndpoint::OffsetDateTime(datetime) => {
            Ok(TemporalValueDescriptor::OffsetDateTime(
                jiff_parts_to_offset_date_time_descriptor(*datetime.local(), *datetime.offset())?,
            ))
        }
        JiffTimeIntervalEndpoint::ZonedDateTime(zoned) => {
            Ok(TemporalValueDescriptor::ZonedDateTime(
                jiff_zoned_to_zoned_date_time_descriptor(zoned)?,
            ))
        }
    }
}

/// Resolve a neutral time-interval endpoint descriptor to a real jiff
/// endpoint.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(endpoint)))]
fn time_interval_endpoint_descriptor_to_jiff(
    endpoint: &TimeIntervalEndpoint,
) -> Result<JiffTimeIntervalEndpoint, TemporalError> {
    match endpoint {
        TimeIntervalEndpoint::Open => Ok(JiffTimeIntervalEndpoint::Open),
        TimeIntervalEndpoint::Unknown => Ok(JiffTimeIntervalEndpoint::Unknown),
        TimeIntervalEndpoint::Value(QualifiedOrBareTemporalValueDescriptor::Bare(value)) => {
            temporal_value_descriptor_to_jiff_endpoint(value)
        }
        TimeIntervalEndpoint::Value(QualifiedOrBareTemporalValueDescriptor::Qualified(_)) => Err(
            TemporalError::new(TemporalErrorKind::Unsupported(UnsupportedSource::new(
                "jiff backend has no representation for an explicitly qualified \
                 (ISO 8601-2) interval-endpoint value"
                    .to_owned(),
            ))),
        ),
    }
}

/// Decompose a real jiff endpoint back into a neutral descriptor.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(endpoint)))]
fn jiff_endpoint_to_time_interval_endpoint_descriptor(
    endpoint: &JiffTimeIntervalEndpoint,
) -> Result<TimeIntervalEndpoint, TemporalError> {
    match endpoint {
        JiffTimeIntervalEndpoint::Open => Ok(TimeIntervalEndpoint::Open),
        JiffTimeIntervalEndpoint::Unknown => Ok(TimeIntervalEndpoint::Unknown),
        other => Ok(TimeIntervalEndpoint::Value(
            QualifiedOrBareTemporalValueDescriptor::Bare(
                jiff_endpoint_to_temporal_value_descriptor(other)?,
            ),
        )),
    }
}

/// Resolve a neutral time-interval representation to its real jiff
/// counterpart.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(representation)))]
fn time_interval_representation_to_jiff(
    representation: &TimeIntervalRepresentation,
) -> Result<JiffTimeIntervalRepresentation, TemporalError> {
    match representation {
        TimeIntervalRepresentation::StartEnd { start, end } => {
            Ok(JiffTimeIntervalRepresentation::StartEnd {
                start: time_interval_endpoint_descriptor_to_jiff(start)?,
                end: time_interval_endpoint_descriptor_to_jiff(end)?,
            })
        }
        TimeIntervalRepresentation::StartDuration { start, duration } => {
            Ok(JiffTimeIntervalRepresentation::StartDuration {
                start: time_interval_endpoint_descriptor_to_jiff(start)?,
                duration: duration_descriptor_to_jiff_span(duration)?,
            })
        }
        TimeIntervalRepresentation::DurationEnd { duration, end } => {
            Ok(JiffTimeIntervalRepresentation::DurationEnd {
                duration: duration_descriptor_to_jiff_span(duration)?,
                end: time_interval_endpoint_descriptor_to_jiff(end)?,
            })
        }
    }
}

/// Decompose a real jiff time-interval representation back into a
/// neutral descriptor.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(representation)))]
fn jiff_representation_to_time_interval_representation(
    representation: &JiffTimeIntervalRepresentation,
) -> Result<TimeIntervalRepresentation, TemporalError> {
    match representation {
        JiffTimeIntervalRepresentation::StartEnd { start, end } => {
            Ok(TimeIntervalRepresentation::StartEnd {
                start: jiff_endpoint_to_time_interval_endpoint_descriptor(start)?,
                end: jiff_endpoint_to_time_interval_endpoint_descriptor(end)?,
            })
        }
        JiffTimeIntervalRepresentation::StartDuration { start, duration } => {
            Ok(TimeIntervalRepresentation::StartDuration {
                start: jiff_endpoint_to_time_interval_endpoint_descriptor(start)?,
                duration: jiff_span_to_duration_descriptor(*duration)?,
            })
        }
        JiffTimeIntervalRepresentation::DurationEnd { duration, end } => {
            Ok(TimeIntervalRepresentation::DurationEnd {
                duration: jiff_span_to_duration_descriptor(*duration)?,
                end: jiff_endpoint_to_time_interval_endpoint_descriptor(end)?,
            })
        }
    }
}

impl Exchange<ReflectedTimeInterval, ProvenTimeIntervalCarrier<JiffTimeInterval>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedTimeInterval,
    ) -> Result<ProvenTimeIntervalCarrier<JiffTimeInterval>, TemporalError> {
        let representation =
            time_interval_representation_to_jiff(input.descriptor().representation())?;
        let token = <ReflectedTimeInterval as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenTimeIntervalCarrier::<JiffTimeInterval>::new(
            JiffTimeInterval::new(representation),
            token,
        ))
    }
}

impl Exchange<ProvenTimeIntervalCarrier<JiffTimeInterval>, ReflectedTimeInterval, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenTimeIntervalCarrier<JiffTimeInterval>,
    ) -> Result<ReflectedTimeInterval, TemporalError> {
        let representation = jiff_representation_to_time_interval_representation(input.carrier())?;
        let token =
            <ProvenTimeIntervalCarrier<JiffTimeInterval> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedTimeInterval::new(
            TimeIntervalDescriptor::new(representation),
            token,
        ))
    }
}

impl
    Exchange<
        ReflectedRecurringInterval,
        ProvenRecurringIntervalCarrier<JiffRecurringInterval>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedRecurringInterval,
    ) -> Result<ProvenRecurringIntervalCarrier<JiffRecurringInterval>, TemporalError> {
        let descriptor = input.descriptor();
        let representation =
            time_interval_representation_to_jiff(descriptor.interval().representation())?;
        let token = <ReflectedRecurringInterval as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(
            ProvenRecurringIntervalCarrier::<JiffRecurringInterval>::new(
                JiffRecurringInterval::new(
                    descriptor.repetitions(),
                    JiffTimeInterval::new(representation),
                ),
                token,
            ),
        )
    }
}

impl
    Exchange<
        ProvenRecurringIntervalCarrier<JiffRecurringInterval>,
        ReflectedRecurringInterval,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenRecurringIntervalCarrier<JiffRecurringInterval>,
    ) -> Result<ReflectedRecurringInterval, TemporalError> {
        let carrier = input.carrier();
        let representation =
            jiff_representation_to_time_interval_representation(carrier.interval())?;
        let mut builder = RecurringIntervalDescriptorBuilder::default();
        if let Some(repetitions) = *carrier.repetitions() {
            builder = builder.repetitions(repetitions);
        }
        let descriptor = builder
            .interval(TimeIntervalDescriptor::new(representation))
            .build()
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                    InvalidDescriptorSource::new(format!(
                        "could not build a recurring interval descriptor: {err}"
                    )),
                ))
            })?;
        let token = <ProvenRecurringIntervalCarrier<JiffRecurringInterval> as Sidecar<
            JiffVerifier,
        >>::sidecar(&input);
        Ok(ReflectedRecurringInterval::new(descriptor, token))
    }
}

impl
    Exchange<
        OrderOffsetEndpointsNativeInput<JiffTimeBackend>,
        OrderOffsetEndpointsNativeOutput,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: OrderOffsetEndpointsNativeInput<JiffTimeBackend>,
    ) -> Result<OrderOffsetEndpointsNativeOutput, TemporalError> {
        let request = input.request();
        let pin = |odt: &JiffOffsetDateTime| {
            odt.offset().to_timestamp(*odt.local()).map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                    InvalidDescriptorSource::new(format!(
                        "could not pin the offset date-time to a fixed instant: {err}"
                    )),
                ))
            })
        };
        let start = pin(request.start())?;
        let end = pin(request.end())?;

        if start > end {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "interval start ({start}) is after its end ({end})"
                )),
            )));
        }

        // The full chain from scratch: this native edge's own input
        // carries a bare `TemporalInputToken`, a different proposition
        // than either descriptor-level hop `order_offset_endpoints`
        // already walks, so both hops are walked again here, ending at
        // the native factory's own `IntervalEndpointOrderingBundle`.
        let input_token = <OrderOffsetEndpointsNativeInput<JiffTimeBackend> as Sidecar<
            JiffVerifier,
        >>::sidecar(&input);
        let preconditions_token = <amenable_time::OrderOffsetEndpointsPreconditions as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(input_token);
        let established_token = <OrderOffsetEndpointsEstablished as Establish<
            amenable_time::OrderOffsetEndpointsPreconditionsToken,
            JiffVerifier,
        >>::establish(preconditions_token);
        let bundle_token = <IntervalEndpointOrderingBundle as Establish<
            OrderOffsetEndpointsEstablishedToken,
            JiffVerifier,
        >>::establish(established_token);
        Ok(OrderOffsetEndpointsNativeOutput::new(
            IntervalEndpointOrderingBundle::default(),
            bundle_token,
        ))
    }
}
