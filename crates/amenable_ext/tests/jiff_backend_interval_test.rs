//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 7 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalIntervalFactory`
//! — real duration parsing and real `order_offset_endpoints`
//! arithmetic; the two full interval-text-parse edges are an honest
//! `Unsupported` for now (they need `TemporalParser`, not yet built).

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffTimeBackend, JiffVerifier};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, LocalDateTimeDescriptorBuilder,
    LocalTimeDescriptorBuilder, OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder,
    OrderOffsetEndpointsInput, OrderOffsetEndpointsPreconditions,
    OrderOffsetEndpointsPreconditionsToken, OrderOffsetEndpointsRequest, ParsedDuration,
    ParsedRecurringInterval, ParsedTimeInterval, RawInput, TemporalError, TemporalErrorKind,
    TemporalInputToken, TemporalIntervalFactory, UtcOffsetDescriptorBuilder, UtcOffsetSign,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalIntervalFactory<JiffVerifier>`.
fn _assert_interval_factory<T: TemporalIntervalFactory<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_interval_factory::<JiffTimeBackend>;
};

fn order_offset_endpoints_preconditions_token() -> OrderOffsetEndpointsPreconditionsToken {
    <OrderOffsetEndpointsPreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

fn offset_date_time_descriptor(
    (year, month, day): (i32, u8, u8),
    (hour, minute, second): (u8, u8, u8),
    (sign, offset_hours): (UtcOffsetSign, u8),
) -> miette::Result<OffsetDateTimeDescriptor> {
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(hour)
                .minute(minute)
                .second(second)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(offset_hours)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")
}

#[test]
fn parses_a_real_iso8601_duration() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let parsed: ParsedDuration = backend
        .exchange(RawInput::received("P1Y2M3DT4H5M6.789S"))
        .into_diagnostic()
        .wrap_err("a real ISO 8601 duration parses via jiff::Span::FromStr")?;
    let descriptor = parsed.descriptor();

    assert_eq!(descriptor.years(), 1);
    assert_eq!(descriptor.months(), 2);
    assert_eq!(descriptor.days(), 3);
    assert_eq!(descriptor.hours(), 4);
    assert_eq!(descriptor.minutes(), 5);
    assert_eq!(descriptor.seconds(), 6);
    Ok(())
}

#[test]
fn rejects_malformed_duration_text() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let result: Result<ParsedDuration, TemporalError> =
        backend.exchange(RawInput::received("not a duration"));
    let err = result
        .err()
        .ok_or_else(|| miette::miette!("malformed text is not a real ISO 8601 duration"))?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::ParseRejected(_)));
    Ok(())
}

#[test]
fn recurring_interval_text_parse_is_honestly_unsupported() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let result: Result<ParsedRecurringInterval, TemporalError> =
        backend.exchange(RawInput::received("R5/2024-01-01/P1D"));
    let err = result
        .err()
        .ok_or_else(|| miette::miette!("no TemporalParser exists on this backend yet"))?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn time_interval_text_parse_is_honestly_unsupported() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let result: Result<ParsedTimeInterval, TemporalError> =
        backend.exchange(RawInput::received("2024-01-01/2024-01-02"));
    let err = result
        .err()
        .ok_or_else(|| miette::miette!("no TemporalParser exists on this backend yet"))?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn order_offset_endpoints_accepts_a_chronologically_ordered_pair() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;

    let output = backend
        .exchange(OrderOffsetEndpointsInput::new(
            OrderOffsetEndpointsRequest::new(start, end),
            order_offset_endpoints_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("start genuinely precedes end")?;
    let _ = output.established();
    Ok(())
}

#[test]
fn order_offset_endpoints_rejects_a_reversed_pair() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    // Same real instant expressed in two different offsets, reversed
    // start/end: 13:00-04:00 == 17:00+00:00, and the request below
    // passes the LATER instant as start and the EARLIER as end.
    let start = offset_date_time_descriptor((2024, 1, 2), (0, 0, 1), (UtcOffsetSign::Positive, 0))?;
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;

    let err: TemporalError = backend
        .exchange(OrderOffsetEndpointsInput::new(
            OrderOffsetEndpointsRequest::new(start, end),
            order_offset_endpoints_preconditions_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| miette::miette!("start is genuinely after end"))?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn order_offset_endpoints_compares_real_instants_not_local_clock_faces() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    // 23:00-05:00 on 2024-01-01 is the same real instant as 04:00+00:00
    // on 2024-01-02 -- equal instants order as start <= end, so this
    // must succeed even though the local clock digits look "later" on
    // the earlier-dated endpoint.
    let start =
        offset_date_time_descriptor((2024, 1, 1), (23, 0, 0), (UtcOffsetSign::Negative, 5))?;
    let end = offset_date_time_descriptor((2024, 1, 2), (4, 0, 0), (UtcOffsetSign::Positive, 0))?;

    backend
        .exchange(OrderOffsetEndpointsInput::new(
            OrderOffsetEndpointsRequest::new(start, end),
            order_offset_endpoints_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err(
            "the two descriptors represent the same real instant, which orders as start <= end",
        )?;
    Ok(())
}
