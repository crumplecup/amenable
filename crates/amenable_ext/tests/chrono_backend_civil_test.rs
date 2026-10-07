//! `ChronoTimeBackend`'s chrono-backed `Exchange` bodies: the civil bridge,
//! `TemporalCivilNativeBridge` over `chrono::NaiveDate`/`NaiveTime`/
//! `NaiveDateTime`. Mirrors `jiff_backend_civil_test.rs`.
//!
//! Exercises all three complete-date forms (calendar, ordinal, week) on the
//! realize direction, a fractional-second round trip, and the two rejections
//! the bridge makes explicitly: a leap second and an impossible date.

#![cfg(feature = "chrono")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{ChronoDateTime, ChronoTimeBackend, ChronoVerifier};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, LocalDateTimeDescriptorBuilder,
    LocalDateTimeProof, LocalDateTimeProofToken, LocalDateTimeSemanticBundle,
    LocalDateTimeSemanticBundleToken, LocalTimeDescriptorBuilder, OrdinalDateDescriptor,
    ProvenLocalDateTimeCarrier, ReflectedLocalDateTime, TemporalCivilNativeBridge,
    TemporalInputToken, WeekDateDescriptor,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `ChronoTimeBackend` stops resolving as a real
// `TemporalCivilNativeBridge<ChronoVerifier>`.
fn _assert_civil_bridge<T: TemporalCivilNativeBridge<ChronoVerifier>>() {}
const _: () = {
    let _ = _assert_civil_bridge::<ChronoTimeBackend>;
};

fn local_date_time_bundle_token() -> LocalDateTimeSemanticBundleToken {
    let proof: LocalDateTimeProofToken = <LocalDateTimeProof as Establish<
        TemporalInputToken,
        ChronoVerifier,
    >>::establish(TemporalInputToken::new());
    <LocalDateTimeSemanticBundle as Establish<LocalDateTimeProofToken, ChronoVerifier>>::establish(
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
    let backend = ChronoTimeBackend;
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(2024, 3, 10),
        ))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let carrier: ProvenLocalDateTimeCarrier<ChronoDateTime> = backend
        .exchange(ReflectedLocalDateTime::new(
            descriptor,
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a calendar date realizes to a real chrono::NaiveDateTime")?;
    let datetime = **carrier.carrier();

    assert_eq!(
        Some(datetime.date()),
        chrono::NaiveDate::from_ymd_opt(2024, 3, 10)
    );
    assert_eq!(
        Some(datetime.time()),
        chrono::NaiveTime::from_hms_opt(12, 0, 0)
    );
    Ok(())
}

#[test]
fn realize_local_date_time_resolves_an_ordinal_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    // Day 70 of 2024 (a leap year) is 2024-03-10.
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Ordinal(OrdinalDateDescriptor::new(
            2024, 70,
        )))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let carrier: ProvenLocalDateTimeCarrier<ChronoDateTime> = backend
        .exchange(ReflectedLocalDateTime::new(
            descriptor,
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("an ordinal date realizes to a real chrono::NaiveDateTime")?;
    let datetime = **carrier.carrier();

    assert_eq!(
        Some(datetime.date()),
        chrono::NaiveDate::from_ymd_opt(2024, 3, 10)
    );
    Ok(())
}

#[test]
fn realize_local_date_time_resolves_a_week_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    // ISO week date 2024-W10-7 (Sunday) is 2024-03-10.
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Week(WeekDateDescriptor::new(
            2024, 10, 7,
        )))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let carrier: ProvenLocalDateTimeCarrier<ChronoDateTime> = backend
        .exchange(ReflectedLocalDateTime::new(
            descriptor,
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a week date realizes to a real chrono::NaiveDateTime")?;
    let datetime = **carrier.carrier();

    assert_eq!(
        Some(datetime.date()),
        chrono::NaiveDate::from_ymd_opt(2024, 3, 10)
    );
    Ok(())
}

#[test]
fn local_date_time_round_trips_with_fractional_seconds() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    let original = chrono::NaiveDate::from_ymd_opt(2023, 11, 5)
        .and_then(|d| d.and_hms_nano_opt(1, 30, 0, 250_000_000))
        .ok_or_else(|| miette::miette!("original date-time is valid"))?;

    // Reflect: chrono value -> descriptor (always calendar form).
    let reflected = backend
        .exchange(ProvenLocalDateTimeCarrier::<ChronoDateTime>::new(
            ChronoDateTime::new(original),
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a real date-time reflects to a descriptor")?;

    // Realize: descriptor -> chrono value. It must come back unchanged.
    let round_tripped = **backend
        .exchange(ReflectedLocalDateTime::new(
            reflected.descriptor().clone(),
            local_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("the reflected descriptor realizes back")?
        .carrier();

    assert_eq!(round_tripped, original);
    Ok(())
}

#[test]
fn reflect_rejects_a_leap_second_explicitly() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    // chrono encodes a leap second as nanoseconds past 999_999_999 on second 59.
    let leap = chrono::NaiveDate::from_ymd_opt(2016, 12, 31)
        .and_then(|d| d.and_hms_nano_opt(23, 59, 59, 1_500_000_000))
        .ok_or_else(|| miette::miette!("a leap-second value is constructible"))?;

    let outcome = backend.exchange(ProvenLocalDateTimeCarrier::<ChronoDateTime>::new(
        ChronoDateTime::new(leap),
        local_date_time_bundle_token(),
    ));
    assert!(
        outcome.is_err(),
        "a leap second must be rejected, not silently adjusted"
    );
    Ok(())
}

#[test]
fn realize_rejects_an_impossible_calendar_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    // 2023 is not a leap year, so February 29th does not exist.
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(2023, 2, 29),
        ))
        .time(local_time()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;

    let outcome = backend.exchange(ReflectedLocalDateTime::new(
        descriptor,
        local_date_time_bundle_token(),
    ));
    assert!(
        outcome.is_err(),
        "February 29th in a common year must be rejected"
    );
    Ok(())
}
