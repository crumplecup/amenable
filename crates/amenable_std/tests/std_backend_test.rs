//! The `std::time` canary backend implements the slice of the temporal
//! trait surface that `std::time` can honestly back — and its `Exchange`
//! bodies *execute* the same contracts the formal backends prove. This
//! test is a runtime oracle: it feeds the canary inputs that satisfy and
//! violate `IntervalEndpointsOrdered` and checks it accepts / rejects
//! accordingly, and round-trips durations through a real
//! `std::time::Duration`.

use miette::{IntoDiagnostic, WrapErr};
use std::time::Duration;

use amenable_core::{Establish, Exchange};
use amenable_std::{CanaryVerifier, StdDuration, StdTimeBackend};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, DurationDescriptorBuilder, DurationFormValid,
    DurationFormValidToken, DurationSemanticBundle, DurationSemanticBundleToken,
    LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder, OffsetDateTimeDescriptor,
    OffsetDateTimeDescriptorBuilder, OrderOffsetEndpointsInput, OrderOffsetEndpointsPreconditions,
    OrderOffsetEndpointsPreconditionsToken, OrderOffsetEndpointsRequest, ParsedDuration,
    ProvenDurationCarrier, RawInput, ReflectedDuration, TemporalDurationNativeBridge,
    TemporalError, TemporalErrorKind, TemporalInputToken, TemporalIntervalFactory,
    TemporalReporter, UtcOffsetDescriptorBuilder, UtcOffsetSign,
};

// The canary proper: these fail to compile if a bundle trait stops
// resolving with the `std::time` concrete types.
fn _assert_interval_factory<T: TemporalIntervalFactory<CanaryVerifier>>() {}
fn _assert_duration_bridge<T: TemporalDurationNativeBridge<CanaryVerifier>>() {}
const _: () = {
    let _ = _assert_interval_factory::<StdTimeBackend>;
    let _ = _assert_duration_bridge::<StdTimeBackend>;
};

/// A complete-calendar-date offset timestamp at whole-hour precision.
fn timestamp(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    offset_hours: i8,
) -> miette::Result<OffsetDateTimeDescriptor> {
    let time = LocalTimeDescriptorBuilder::default()
        .hour(hour)
        .minute(0u8)
        .second(0u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid local time")?;
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(time)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let sign = if offset_hours < 0 {
        UtcOffsetSign::Negative
    } else {
        UtcOffsetSign::Positive
    };
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(offset_hours.unsigned_abs())
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time")
}

fn order_input(
    start: OffsetDateTimeDescriptor,
    end: OffsetDateTimeDescriptor,
) -> OrderOffsetEndpointsInput {
    let token: OrderOffsetEndpointsPreconditionsToken =
        <OrderOffsetEndpointsPreconditions as Establish<TemporalInputToken, CanaryVerifier>>::establish(
            TemporalInputToken::new(),
        );
    OrderOffsetEndpointsInput::new(OrderOffsetEndpointsRequest::new(start, end), token)
}

fn duration_bundle_token() -> DurationSemanticBundleToken {
    let form: DurationFormValidToken = <DurationFormValid as Establish<
        TemporalInputToken,
        CanaryVerifier,
    >>::establish(TemporalInputToken::new());
    <DurationSemanticBundle as Establish<DurationFormValidToken, CanaryVerifier>>::establish(form)
}

#[test]
fn reporter_declares_the_std_time_capability_slice() {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    assert!(backend.supported_serialization_profiles().is_empty());
    assert_eq!(backend.max_fractional_second_digits(), Some(9));
    assert!(!backend.supports_leap_seconds());
    assert!(!backend.supports_named_zone_round_trip());
    assert!(!backend.supports_end_of_day_twenty_four());
    assert!(!backend.supports_unknown_local_offset());
    assert_eq!(backend.current_tzdb_revision(), None);

    // The `capabilities()` surface renders every query in one block.
    let rendered = backend.capabilities().to_string();
    assert!(rendered.contains("max fractional-sec digits  : 9"));
    assert!(rendered.contains("leap seconds               : no"));
    assert!(rendered.contains("serialization profiles     : (none)"));
}

#[test]
fn text_parse_edges_report_unsupported() {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    let out: Result<ParsedDuration, TemporalError> =
        backend.exchange(RawInput::received("P1Y2M3DT4H5M6S"));
    assert!(matches!(
        &**out.unwrap_err().kind(),
        TemporalErrorKind::Unsupported(_)
    ));
}

#[test]
fn ordered_endpoints_establish_the_ordering_proposition() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    let input = order_input(timestamp(2020, 1, 1, 0, 0)?, timestamp(2021, 6, 15, 12, 0)?);
    let output = backend
        .exchange(input)
        .into_diagnostic()
        .wrap_err("2020-01-01 precedes 2021-06-15")?;
    let _ = output.established();
    Ok(())
}

#[test]
fn reversed_endpoints_are_rejected() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    let input = order_input(timestamp(2021, 6, 15, 12, 0)?, timestamp(2020, 1, 1, 0, 0)?);
    let err =
        backend.exchange(input).map(|_| ()).err().ok_or_else(|| {
            miette::miette!("a later start than end has no ordering to establish")
        })?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn offsets_are_normalised_before_comparison() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    // 2020-01-01T00:00+05:00 is the same instant as 2019-12-31T19:00Z,
    // which precedes 2020-01-01T00:00Z.
    let input = order_input(timestamp(2020, 1, 1, 0, 5)?, timestamp(2020, 1, 1, 0, 0)?);
    backend
        .exchange(input)
        .into_diagnostic()
        .wrap_err("the +05:00 start is earlier once normalised to UTC")?;
    Ok(())
}

#[test]
fn realize_duration_converts_whole_second_spans() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    let descriptor = DurationDescriptorBuilder::default()
        .hours(1u32)
        .minutes(30u32)
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;
    let carrier: ProvenDurationCarrier<StdDuration> = backend
        .exchange(ReflectedDuration::new(descriptor, duration_bundle_token()))
        .into_diagnostic()
        .wrap_err("PT1H30M is a fixed span")?;
    assert_eq!(**carrier.carrier(), Duration::from_secs(5400));
    Ok(())
}

#[test]
fn realize_duration_rejects_calendar_variable_components() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    let descriptor = DurationDescriptorBuilder::default()
        .years(1u32)
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;
    let err = backend
        .exchange(ReflectedDuration::new(descriptor, duration_bundle_token()))
        .map(|_| ())
        .err()
        .ok_or_else(|| miette::miette!("a year is not a fixed std::time::Duration"))?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn duration_round_trips_through_std_time() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = StdTimeBackend;
    let span = Duration::from_secs(86_400 + 3600 + 60 + 1);
    let carrier =
        ProvenDurationCarrier::<StdDuration>::new(StdDuration::new(span), duration_bundle_token());

    let reflected: ReflectedDuration = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a whole-second span reflects to a descriptor")?;
    let round_tripped: ProvenDurationCarrier<StdDuration> = backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to a span")?;

    assert_eq!(**round_tripped.carrier(), span);
    Ok(())
}
