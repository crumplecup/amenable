//! `ChronoTimeBackend`'s chrono-backed `Exchange` bodies: the instant
//! bridge, `TemporalInstantNativeBridge` over
//! `chrono::DateTime<chrono::FixedOffset>`. Mirrors
//! `jiff_backend_instant_test.rs`.

#![cfg(feature = "chrono")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{ChronoOffsetDateTime, ChronoTimeBackend, ChronoVerifier};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, LocalDateTimeDescriptorBuilder,
    LocalTimeDescriptorBuilder, OffsetDateTimeDescriptorBuilder, OffsetDateTimeProof,
    OffsetDateTimeProofToken, OffsetDateTimeSemanticBundle, OffsetDateTimeSemanticBundleToken,
    ProvenOffsetDateTimeCarrier, ReflectedOffsetDateTime, TemporalError, TemporalErrorKind,
    TemporalInputToken, TemporalInstantNativeBridge, UtcOffsetDescriptorBuilder, UtcOffsetSign,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `ChronoTimeBackend` stops resolving as a real
// `TemporalInstantNativeBridge<ChronoVerifier>`.
fn _assert_instant_bridge<T: TemporalInstantNativeBridge<ChronoVerifier>>() {}
const _: () = {
    let _ = _assert_instant_bridge::<ChronoTimeBackend>;
};

fn offset_date_time_bundle_token() -> OffsetDateTimeSemanticBundleToken {
    let proof: OffsetDateTimeProofToken = <OffsetDateTimeProof as Establish<
        TemporalInputToken,
        ChronoVerifier,
    >>::establish(TemporalInputToken::new());
    <OffsetDateTimeSemanticBundle as Establish<OffsetDateTimeProofToken, ChronoVerifier>>::establish(
        proof,
    )
}

#[test]
fn realize_offset_date_time_resolves_a_calendar_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
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

    let carrier: ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime> = backend
        .exchange(ReflectedOffsetDateTime::new(
            descriptor,
            offset_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a complete calendar date realizes to a real chrono::DateTime<FixedOffset>")?;
    let datetime = **carrier.carrier();

    assert_eq!(
        datetime.naive_local().date(),
        chrono::NaiveDate::from_ymd_opt(2024, 3, 10).unwrap()
    );
    assert_eq!(
        datetime.naive_local().time(),
        chrono::NaiveTime::from_hms_opt(13, 30, 0).unwrap()
    );
    assert_eq!(datetime.offset().local_minus_utc(), -5 * 3600);
    Ok(())
}

#[test]
fn realize_offset_date_time_resolves_an_ordinal_or_week_date() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
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
        .wrap_err("an ordinal date realizes through the shared civil conversion")?;
    let datetime = **carrier.carrier();
    assert_eq!(
        datetime.naive_local().date(),
        chrono::NaiveDate::from_ymd_opt(2024, 3, 10).unwrap()
    );
    Ok(())
}

#[test]
fn realize_offset_date_time_rejects_the_unknown_local_offset_case() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
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
            miette::miette!("chrono::FixedOffset has no unknown-local-offset representation")
        })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn offset_date_time_round_trips_through_real_chrono_types() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    let offset = chrono::FixedOffset::east_opt(-4 * 3600)
        .ok_or_else(|| miette::miette!("-4 hours is a valid fixed offset"))?;
    let naive = chrono::NaiveDate::from_ymd_opt(2023, 11, 5)
        .and_then(|d| d.and_hms_opt(1, 30, 0))
        .ok_or_else(|| miette::miette!("valid naive date-time"))?;
    let original = naive
        .and_local_timezone(offset)
        .single()
        .ok_or_else(|| miette::miette!("a fixed offset always maps uniquely"))?;
    let carrier = ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
        ChronoOffsetDateTime::new(original),
        offset_date_time_bundle_token(),
    );

    let reflected: ReflectedOffsetDateTime = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real chrono offset date-time reflects to a descriptor")?;
    let round_tripped = **backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to an equivalent offset date-time")?
        .carrier();

    assert_eq!(round_tripped, original);
    Ok(())
}
