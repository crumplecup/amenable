//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 8 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalTimeIntervalProps`
//! and `TemporalRecurringIntervalProps` plus their `NativeBridge`s, and
//! `TemporalNativeIntervalFactory`'s own `order_offset_endpoints_native`
//! edge.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{
    JiffOffsetDateTime, JiffRecurringInterval, JiffTimeBackend, JiffTimeInterval, JiffVerifier,
};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, DecadeDescriptorBuilder, DurationDescriptor,
    DurationDescriptorBuilder, LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder,
    OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder, OrderOffsetEndpointsNativeInput,
    OrderOffsetEndpointsNativeRequest, ProvenRecurringIntervalCarrier, ProvenTimeIntervalCarrier,
    QualifiedOrBareTemporalValueDescriptor, RecurringIntervalDescriptorBuilder,
    RecurringIntervalFormValid, RecurringIntervalSemanticBundle,
    RecurringIntervalSemanticBundleToken, ReflectedRecurringInterval, ReflectedTimeInterval,
    TemporalError, TemporalErrorKind, TemporalInputToken, TemporalNativeIntervalFactory,
    TemporalRecurringIntervalNativeBridge, TemporalTimeIntervalNativeBridge,
    TemporalValueDescriptor, TimeIntervalDescriptor, TimeIntervalEndpoint, TimeIntervalProof,
    TimeIntervalRepresentation, TimeIntervalSemanticBundle, TimeIntervalSemanticBundleToken,
    UtcOffsetDescriptorBuilder, UtcOffsetSign,
};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalTimeIntervalNativeBridge<JiffVerifier>`/
// `TemporalRecurringIntervalNativeBridge<JiffVerifier>`/
// `TemporalNativeIntervalFactory<JiffVerifier>`.
fn _assert_time_interval_bridge<T: TemporalTimeIntervalNativeBridge<JiffVerifier>>() {}
fn _assert_recurring_interval_bridge<T: TemporalRecurringIntervalNativeBridge<JiffVerifier>>() {}
fn _assert_native_interval_factory<T: TemporalNativeIntervalFactory<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_time_interval_bridge::<JiffTimeBackend>;
    let _ = _assert_recurring_interval_bridge::<JiffTimeBackend>;
    let _ = _assert_native_interval_factory::<JiffTimeBackend>;
};

fn time_interval_bundle_token() -> TimeIntervalSemanticBundleToken {
    let proof = <TimeIntervalProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    );
    <TimeIntervalSemanticBundle as Establish<
        amenable_time::TimeIntervalProofToken,
        JiffVerifier,
    >>::establish(proof)
}

fn recurring_interval_bundle_token() -> RecurringIntervalSemanticBundleToken {
    let valid =
        <RecurringIntervalFormValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            TemporalInputToken::new(),
        );
    <RecurringIntervalSemanticBundle as Establish<
        amenable_time::RecurringIntervalFormValidToken,
        JiffVerifier,
    >>::establish(valid)
}

fn offset_date_time_descriptor(
    (year, month, day): (i32, u8, u8),
    (hour, minute, second): (u8, u8, u8),
    (sign, offset_hours): (UtcOffsetSign, u8),
) -> OffsetDateTimeDescriptor {
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
                .expect("valid local time"),
        )
        .build()
        .expect("valid local date-time");
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(offset_hours)
        .build()
        .expect("valid offset");
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .expect("valid offset date-time descriptor")
}

fn offset_value_endpoint(descriptor: OffsetDateTimeDescriptor) -> TimeIntervalEndpoint {
    TimeIntervalEndpoint::Value(QualifiedOrBareTemporalValueDescriptor::Bare(
        TemporalValueDescriptor::OffsetDateTime(descriptor),
    ))
}

fn one_day_duration() -> DurationDescriptor {
    DurationDescriptorBuilder::default()
        .days(1u32)
        .build()
        .expect("valid duration descriptor")
}

#[test]
fn realizes_and_reflects_a_start_end_interval_of_offset_date_times() {
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let representation = TimeIntervalRepresentation::StartEnd {
        start: offset_value_endpoint(start),
        end: offset_value_endpoint(end),
    };
    let descriptor = TimeIntervalDescriptor::new(representation);

    let carrier: ProvenTimeIntervalCarrier<JiffTimeInterval> = backend
        .exchange(ReflectedTimeInterval::new(
            descriptor.clone(),
            time_interval_bundle_token(),
        ))
        .expect("a real offset-date-time interval realizes");
    let reflected: ReflectedTimeInterval = backend
        .exchange(carrier)
        .expect("the native carrier reflects back");

    assert_eq!(reflected.descriptor(), &descriptor);
}

#[test]
fn realizes_and_reflects_open_and_unknown_boundaries() {
    let backend = JiffTimeBackend;
    let descriptor = TimeIntervalDescriptor::new(TimeIntervalRepresentation::StartEnd {
        start: TimeIntervalEndpoint::Open,
        end: TimeIntervalEndpoint::Unknown,
    });

    let carrier: ProvenTimeIntervalCarrier<JiffTimeInterval> = backend
        .exchange(ReflectedTimeInterval::new(
            descriptor.clone(),
            time_interval_bundle_token(),
        ))
        .expect("Open/Unknown boundaries carry no value to convert, real success");
    let reflected: ReflectedTimeInterval = backend
        .exchange(carrier)
        .expect("Open/Unknown round-trip without loss");

    assert_eq!(reflected.descriptor(), &descriptor);
}

#[test]
fn realizes_and_reflects_a_start_duration_interval() {
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let descriptor = TimeIntervalDescriptor::new(TimeIntervalRepresentation::StartDuration {
        start: offset_value_endpoint(start),
        duration: one_day_duration(),
    });

    let carrier: ProvenTimeIntervalCarrier<JiffTimeInterval> = backend
        .exchange(ReflectedTimeInterval::new(
            descriptor.clone(),
            time_interval_bundle_token(),
        ))
        .expect("a real start+duration interval realizes");
    let reflected: ReflectedTimeInterval = backend
        .exchange(carrier)
        .expect("the native carrier reflects back");

    assert_eq!(reflected.descriptor(), &descriptor);
}

#[test]
fn rejects_an_out_of_scope_endpoint_form() {
    let backend = JiffTimeBackend;
    let decade = DecadeDescriptorBuilder::default()
        .ordinal(202u16)
        .build()
        .expect("valid decade descriptor");
    let descriptor = TimeIntervalDescriptor::new(TimeIntervalRepresentation::StartEnd {
        start: TimeIntervalEndpoint::Value(QualifiedOrBareTemporalValueDescriptor::Bare(
            TemporalValueDescriptor::Decade(decade),
        )),
        end: TimeIntervalEndpoint::Open,
    });

    let result: Result<ProvenTimeIntervalCarrier<JiffTimeInterval>, TemporalError> = backend
        .exchange(ReflectedTimeInterval::new(
            descriptor,
            time_interval_bundle_token(),
        ));
    let err = result.expect_err("a decade endpoint is the CalConnect/ISO 8601-2 extension family");
    assert!(matches!(err.kind(), TemporalErrorKind::Unsupported(_)));
}

#[test]
fn realizes_and_reflects_a_bounded_recurring_interval() {
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let interval = TimeIntervalDescriptor::new(TimeIntervalRepresentation::StartEnd {
        start: offset_value_endpoint(start),
        end: offset_value_endpoint(end),
    });
    let descriptor = RecurringIntervalDescriptorBuilder::default()
        .repetitions(5u32)
        .interval(interval)
        .build()
        .expect("valid recurring interval descriptor");

    let carrier: ProvenRecurringIntervalCarrier<JiffRecurringInterval> = backend
        .exchange(ReflectedRecurringInterval::new(
            descriptor.clone(),
            recurring_interval_bundle_token(),
        ))
        .expect("a bounded recurring interval realizes");
    let reflected: ReflectedRecurringInterval = backend
        .exchange(carrier)
        .expect("the native carrier reflects back");

    assert_eq!(reflected.descriptor(), &descriptor);
}

#[test]
fn realizes_and_reflects_an_unbounded_recurring_interval() {
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0));
    let interval = TimeIntervalDescriptor::new(TimeIntervalRepresentation::StartEnd {
        start: offset_value_endpoint(start),
        end: offset_value_endpoint(end),
    });
    let descriptor = RecurringIntervalDescriptorBuilder::default()
        .interval(interval)
        .build()
        .expect("valid recurring interval descriptor");
    assert_eq!(descriptor.repetitions(), None);

    let carrier: ProvenRecurringIntervalCarrier<JiffRecurringInterval> = backend
        .exchange(ReflectedRecurringInterval::new(
            descriptor.clone(),
            recurring_interval_bundle_token(),
        ))
        .expect("an unbounded recurring interval realizes");
    let reflected: ReflectedRecurringInterval = backend
        .exchange(carrier)
        .expect("the native carrier reflects back");

    assert_eq!(reflected.descriptor(), &descriptor);
}

#[test]
fn order_offset_endpoints_native_accepts_a_chronologically_ordered_pair() {
    let backend = JiffTimeBackend;
    let start = jiff::civil::DateTime::new(2024, 1, 1, 0, 0, 0, 0).expect("valid datetime");
    let end = jiff::civil::DateTime::new(2024, 1, 2, 0, 0, 0, 0).expect("valid datetime");
    let offset = jiff::tz::Offset::UTC;
    let request = OrderOffsetEndpointsNativeRequest::new(
        JiffOffsetDateTime {
            local: start,
            offset,
        },
        JiffOffsetDateTime { local: end, offset },
    );

    backend
        .exchange(OrderOffsetEndpointsNativeInput::new(
            request,
            TemporalInputToken::new(),
        ))
        .expect("start genuinely precedes end");
}

#[test]
fn order_offset_endpoints_native_rejects_a_reversed_pair() {
    let backend = JiffTimeBackend;
    let start = jiff::civil::DateTime::new(2024, 1, 2, 0, 0, 0, 0).expect("valid datetime");
    let end = jiff::civil::DateTime::new(2024, 1, 1, 0, 0, 0, 0).expect("valid datetime");
    let offset = jiff::tz::Offset::UTC;
    let request = OrderOffsetEndpointsNativeRequest::new(
        JiffOffsetDateTime {
            local: start,
            offset,
        },
        JiffOffsetDateTime { local: end, offset },
    );

    let result: Result<amenable_time::OrderOffsetEndpointsNativeOutput, TemporalError> = backend
        .exchange(OrderOffsetEndpointsNativeInput::new(
            request,
            TemporalInputToken::new(),
        ));
    let err = result.expect_err("start is genuinely after end");
    assert!(matches!(
        err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
}
