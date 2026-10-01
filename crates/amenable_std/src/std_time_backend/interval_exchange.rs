use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    CompleteDateDescriptor, InvalidDescriptorSource, OffsetDateTimeDescriptor,
    OrderOffsetEndpointsEstablished, OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput,
    OrderOffsetEndpointsPreconditionsToken, ParsedDuration, ParsedRecurringInterval,
    ParsedTimeInterval, RawInput, TemporalError, TemporalErrorKind, UnsupportedSource,
    UtcOffsetSign,
};

use crate::{CanaryVerifier, StdTimeBackend};

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
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
fn offset_datetime_to_epoch_seconds(
    descriptor: &OffsetDateTimeDescriptor,
) -> Result<i64, TemporalError> {
    let local = descriptor.local();
    let CompleteDateDescriptor::Calendar(date) = local.date() else {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(
                "std::time canary resolves complete calendar dates only, not ordinal or week dates"
                    .to_owned(),
            ),
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
    TemporalError::new(TemporalErrorKind::Unsupported(UnsupportedSource::new(
        format!(
            "std::time cannot parse {edge}: no ISO 8601 / RFC 3339 parser in the standard library"
        ),
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
                InvalidDescriptorSource::new(format!(
                    "interval start ({start}s) is after its end ({end}s)"
                )),
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
