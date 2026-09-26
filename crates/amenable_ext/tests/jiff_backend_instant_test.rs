//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 2 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalInstantProps` +
//! `TemporalInstantNativeBridge` over `jiff::civil::DateTime` +
//! `jiff::tz::Offset`.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffOffsetDateTime, JiffTimeBackend, JiffVerifier};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, LocalDateTimeDescriptorBuilder,
    LocalTimeDescriptorBuilder, OffsetDateTimeDescriptorBuilder, OffsetDateTimeProof,
    OffsetDateTimeProofToken, OffsetDateTimeSemanticBundle, OffsetDateTimeSemanticBundleToken,
    ProvenOffsetDateTimeCarrier, ReflectedOffsetDateTime, TemporalError, TemporalErrorKind,
    TemporalInputToken, TemporalInstantNativeBridge, UtcOffsetDescriptorBuilder, UtcOffsetSign,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalInstantNativeBridge<JiffVerifier>`.
fn _assert_instant_bridge<T: TemporalInstantNativeBridge<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_instant_bridge::<JiffTimeBackend>;
};

fn offset_date_time_bundle_token() -> OffsetDateTimeSemanticBundleToken {
    let proof: OffsetDateTimeProofToken = <OffsetDateTimeProof as Establish<
        TemporalInputToken,
        JiffVerifier,
    >>::establish(TemporalInputToken::new());
    <OffsetDateTimeSemanticBundle as Establish<OffsetDateTimeProofToken, JiffVerifier>>::establish(
        proof,
    )
}

#[test]
fn realize_offset_date_time_round_trips_a_calendar_date() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(2024, 3, 10),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(13u8)
                .minute(30u8)
                .second(0u8)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Negative)
        .hours(5u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let descriptor = OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;

    let carrier: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime> = backend
        .exchange(ReflectedOffsetDateTime::new(
            descriptor,
            offset_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a complete calendar date realizes to a real jiff::civil::DateTime + Offset")?;
    let jiff_offset_date_time = carrier.carrier();

    assert_eq!(jiff_offset_date_time.local.year(), 2024);
    assert_eq!(jiff_offset_date_time.local.month(), 3);
    assert_eq!(jiff_offset_date_time.local.day(), 10);
    assert_eq!(jiff_offset_date_time.local.hour(), 13);
    assert_eq!(jiff_offset_date_time.local.minute(), 30);
    assert_eq!(jiff_offset_date_time.offset.seconds(), -5 * 3600);
    Ok(())
}

#[test]
fn realize_offset_date_time_resolves_an_ordinal_or_week_date() -> miette::Result<()> {
    // Phase 2 originally rejected these (calendar dates only); Phase 3's
    // TemporalCivilProps work widened the shared
    // local_date_time_descriptor_to_jiff_civil_datetime helper this
    // bridge itself calls, so both now resolve for real -- confirmed
    // here, not just in jiff_backend_civil_test.rs's own coverage.
    let backend = JiffTimeBackend;
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Ordinal(
            amenable_time::OrdinalDateDescriptor::new(2024, 70),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(0u8)
                .minute(0u8)
                .second(0u8)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Positive)
        .hours(0u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let descriptor = OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;

    let carrier = backend
        .exchange(ReflectedOffsetDateTime::new(
            descriptor,
            offset_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("Phase 3 widened this to resolve ordinal dates too")?;
    let jiff_offset_date_time = carrier.carrier();
    assert_eq!(jiff_offset_date_time.local.year(), 2024);
    assert_eq!(jiff_offset_date_time.local.month(), 3);
    assert_eq!(jiff_offset_date_time.local.day(), 10);
    Ok(())
}

#[test]
fn realize_offset_date_time_rejects_the_unknown_local_offset_case() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(2024, 1, 1),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(0u8)
                .minute(0u8)
                .second(0u8)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Positive)
        .hours(0u8)
        .relationship(amenable_time::UtcOffsetRelationship::UnknownLocalOffset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let descriptor = OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;

    let err: TemporalError = backend
        .exchange(ReflectedOffsetDateTime::new(
            descriptor,
            offset_date_time_bundle_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!("jiff::tz::Offset has no unknown-local-offset representation")
        })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn offset_date_time_round_trips_through_real_jiff_types() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = jiff::civil::DateTime::new(2023, 11, 5, 1, 30, 0, 0)
        .into_diagnostic()
        .wrap_err("valid datetime")?;
    let offset = jiff::tz::Offset::from_seconds(-4 * 3600)
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let carrier = ProvenOffsetDateTimeCarrier::<JiffOffsetDateTime>::new(
        JiffOffsetDateTime { local, offset },
        offset_date_time_bundle_token(),
    );

    let reflected: ReflectedOffsetDateTime = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real jiff offset date-time reflects to a descriptor")?;
    let round_tripped: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime> = backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to an equivalent offset date-time")?;
    let round_tripped = round_tripped.carrier();

    assert_eq!(round_tripped.local, local);
    assert_eq!(round_tripped.offset, offset);
    Ok(())
}
