//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 3 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalCivilProps` +
//! `TemporalCivilNativeBridge` over `jiff::civil::{Date,Time,DateTime,
//! ISOWeekDate}`. Exercises all three complete-date forms (calendar,
//! ordinal, week) through the widened `LocalDateTime` realize/reflect
//! pair — Phase 2's own calendar-only restriction no longer applies.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffDateTime, JiffTimeBackend, JiffVerifier};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, LocalDateTimeDescriptorBuilder,
    LocalDateTimeProof, LocalDateTimeProofToken, LocalDateTimeSemanticBundle,
    LocalDateTimeSemanticBundleToken, LocalTimeDescriptorBuilder, OrdinalDateDescriptor,
    ProvenLocalDateTimeCarrier, ReflectedLocalDateTime, TemporalCivilNativeBridge,
    TemporalInputToken, WeekDateDescriptor,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalCivilNativeBridge<JiffVerifier>`.
fn _assert_civil_bridge<T: TemporalCivilNativeBridge<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_civil_bridge::<JiffTimeBackend>;
};

fn local_date_time_bundle_token() -> LocalDateTimeSemanticBundleToken {
    let proof: LocalDateTimeProofToken = <LocalDateTimeProof as Establish<
        TemporalInputToken,
        JiffVerifier,
    >>::establish(TemporalInputToken::new());
    <LocalDateTimeSemanticBundle as Establish<LocalDateTimeProofToken, JiffVerifier>>::establish(
        proof,
    )
}

fn local_time() -> miette::Result<amenable_time::LocalTimeDescriptor> {
    LocalTimeDescriptorBuilder::default()
        .hour(12u8)
        .minute(0u8)
        .second(0u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid local time")
}

#[test]
fn realize_local_date_time_resolves_a_calendar_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(2024, 3, 10),
        ))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let carrier: ProvenLocalDateTimeCarrier<JiffDateTime> = backend
        .exchange(ReflectedLocalDateTime::new(
            descriptor,
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a calendar date realizes to a real jiff::civil::DateTime")?;
    let datetime = **carrier.carrier();

    assert_eq!(datetime.year(), 2024);
    assert_eq!(datetime.month(), 3);
    assert_eq!(datetime.day(), 10);
    Ok(())
}

#[test]
fn realize_local_date_time_resolves_an_ordinal_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    // Day 70 of 2024 (a leap year) is 2024-03-10.
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Ordinal(OrdinalDateDescriptor::new(
            2024, 70,
        )))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let carrier: ProvenLocalDateTimeCarrier<JiffDateTime> = backend
        .exchange(ReflectedLocalDateTime::new(
            descriptor,
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("an ordinal date realizes to a real jiff::civil::DateTime")?;
    let datetime = **carrier.carrier();

    assert_eq!(datetime.year(), 2024);
    assert_eq!(datetime.month(), 3);
    assert_eq!(datetime.day(), 10);
    Ok(())
}

#[test]
fn realize_local_date_time_resolves_a_week_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    // ISO week date 2024-W10-7 (Sunday) is 2024-03-10.
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Week(WeekDateDescriptor::new(
            2024, 10, 7,
        )))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let carrier: ProvenLocalDateTimeCarrier<JiffDateTime> = backend
        .exchange(ReflectedLocalDateTime::new(
            descriptor,
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a week date realizes to a real jiff::civil::DateTime")?;
    let datetime = **carrier.carrier();

    assert_eq!(datetime.year(), 2024);
    assert_eq!(datetime.month(), 3);
    assert_eq!(datetime.day(), 10);
    Ok(())
}

#[test]
fn local_date_time_round_trips_through_a_real_jiff_datetime_and_canonicalizes_to_calendar()
-> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = JiffTimeBackend;
    let original = jiff::civil::DateTime::new(2023, 11, 5, 1, 30, 0, 250_000_000)
        .into_diagnostic()
        .wrap_err("valid datetime")?;
    let carrier = ProvenLocalDateTimeCarrier::<JiffDateTime>::new(
        JiffDateTime::new(original),
        local_date_time_bundle_token(),
    );

    let reflected: ReflectedLocalDateTime = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real jiff::civil::DateTime reflects to a descriptor")?;
    assert!(matches!(
        reflected.descriptor().date(),
        CompleteDateDescriptor::Calendar(_)
    ));

    let round_tripped: ProvenLocalDateTimeCarrier<JiffDateTime> = backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to an equivalent date-time")?;

    assert_eq!(**round_tripped.carrier(), original);
    Ok(())
}
