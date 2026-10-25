use crate::jiff::backend::duration::jiff_span_to_duration_descriptor;
use crate::jiff::backend::instant::offset_date_time_descriptor_to_jiff_parts;
use crate::{JiffTimeBackend, JiffVerifier};
use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    DurationFormValid, IntervalEndpointsReversedSource, JiffInstantPinFailedSource,
    JiffParseRejectedSource, OrderOffsetEndpointsEstablished, OrderOffsetEndpointsInput,
    OrderOffsetEndpointsOutput, OrderOffsetEndpointsPreconditionsToken, ParsedDuration,
    ParsedRecurringInterval, ParsedTimeInterval, RawInput, TemporalError, TemporalErrorKind,
    TemporalInputToken, UnsupportedCase, UnsupportedSource,
};

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
            TemporalError::new(TemporalErrorKind::JiffParseRejected(
                JiffParseRejectedSource::new("ISO 8601 duration", err),
            ))
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
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(case)))]
fn unsupported_interval_endpoint_parse(case: UnsupportedCase) -> TemporalError {
    TemporalError::new(TemporalErrorKind::Unsupported(UnsupportedSource::closed(
        case,
    )))
}

impl Exchange<RawInput, ParsedRecurringInterval, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedRecurringInterval, TemporalError> {
        Err(unsupported_interval_endpoint_parse(UnsupportedCase::JiffIntervalEndpointParseRecurringInterval))
    }
}

impl Exchange<RawInput, ParsedTimeInterval, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedTimeInterval, TemporalError> {
        Err(unsupported_interval_endpoint_parse(UnsupportedCase::JiffIntervalEndpointParseTimeInterval))
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
                TemporalError::new(TemporalErrorKind::JiffInstantPinFailed(
                    JiffInstantPinFailedSource::new(err),
                ))
            })
        };
        let start = pin(start_offset, start_local)?;
        let end = pin(end_offset, end_local)?;

        if start > end {
            return Err(TemporalError::new(TemporalErrorKind::IntervalEndpointsReversed(
                IntervalEndpointsReversedSource::new(start, end),
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
