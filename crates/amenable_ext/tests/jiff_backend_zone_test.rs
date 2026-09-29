//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 4 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalZoneProps` +
//! `TemporalZoneNativeBridge` over `jiff::tz::TimeZone`/`jiff::Zoned` —
//! real IANA tzdb lookups and zoned-instant construction.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffTimeBackend, JiffTimeZone, JiffVerifier, JiffZoned};
use amenable_time::{
    AttachNamedZoneEstablished, AttachNamedZoneEstablishedToken, AttachNamedZonePreconditions,
    AttachNamedZonePreconditionsToken, CalendarDateDescriptor, CompleteDateDescriptor,
    LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder, NamedTimeZoneDescriptorBuilder,
    NamedTimeZoneIdentityValid, NamedTimeZoneIdentityValidToken, NamedTimeZoneSemanticBundle,
    NamedTimeZoneSemanticBundleToken, OffsetDateTimeDescriptorBuilder, ProvenNamedTimeZoneCarrier,
    ProvenZonedDateTimeCarrier, ReflectedNamedTimeZone, ReflectedZonedDateTime, TemporalError,
    TemporalErrorKind, TemporalInputToken, TemporalZoneNativeBridge, UtcOffsetDescriptorBuilder,
    UtcOffsetSign, ZonedDateTimeDescriptorBuilder, ZonedDateTimeSemanticBundle,
    ZonedDateTimeSemanticBundleToken,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalZoneNativeBridge<JiffVerifier>`.
fn _assert_zone_bridge<T: TemporalZoneNativeBridge<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_zone_bridge::<JiffTimeBackend>;
};

fn named_time_zone_bundle_token() -> NamedTimeZoneSemanticBundleToken {
    let valid: NamedTimeZoneIdentityValidToken = <NamedTimeZoneIdentityValid as Establish<
        TemporalInputToken,
        JiffVerifier,
    >>::establish(TemporalInputToken::new());
    <NamedTimeZoneSemanticBundle as Establish<NamedTimeZoneIdentityValidToken, JiffVerifier>>::establish(valid)
}

fn zoned_date_time_bundle_token() -> ZonedDateTimeSemanticBundleToken {
    let preconditions: AttachNamedZonePreconditionsToken =
        <AttachNamedZonePreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
            TemporalInputToken::new(),
        );
    let established: AttachNamedZoneEstablishedToken = <AttachNamedZoneEstablished as Establish<
        AttachNamedZonePreconditionsToken,
        JiffVerifier,
    >>::establish(preconditions);
    <ZonedDateTimeSemanticBundle as Establish<AttachNamedZoneEstablishedToken, JiffVerifier>>::establish(
        established,
    )
}

#[test]
fn realize_named_time_zone_resolves_a_real_iana_zone() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;

    let carrier: ProvenNamedTimeZoneCarrier<JiffTimeZone> = backend
        .exchange(ReflectedNamedTimeZone::new(
            descriptor,
            named_time_zone_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("America/New_York is a real IANA zone")?;
    assert_eq!(carrier.carrier().iana_name(), Some("America/New_York"));
    Ok(())
}

#[test]
fn realize_named_time_zone_rejects_an_unknown_identifier() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = NamedTimeZoneDescriptorBuilder::default()
        .identifier("Nowhere/Fictional")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;

    let err: TemporalError = backend
        .exchange(ReflectedNamedTimeZone::new(
            descriptor,
            named_time_zone_bundle_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| miette::miette!("Nowhere/Fictional is not a real IANA zone"))?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn reflect_named_time_zone_accepts_utc_as_a_real_identifier() -> miette::Result<()> {
    // Real jiff source confirms TimeZone::UTC.iana_name() == Some("UTC")
    // -- UTC is genuinely a valid identifier, unlike Offset's own
    // unrelated "no identifier" shape a first attempt here assumed by
    // resemblance.
    let backend = JiffTimeBackend;
    let carrier = ProvenNamedTimeZoneCarrier::<JiffTimeZone>::new(
        JiffTimeZone::new(jiff::tz::TimeZone::UTC),
        named_time_zone_bundle_token(),
    );

    let reflected = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("TimeZone::UTC has a real IANA identifier, \"UTC\"")?;
    assert_eq!(reflected.descriptor().identifier(), "UTC");
    Ok(())
}

#[test]
fn reflect_named_time_zone_rejects_a_zone_with_no_iana_identifier() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let offset = jiff::tz::Offset::from_seconds(3600)
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let carrier = ProvenNamedTimeZoneCarrier::<JiffTimeZone>::new(
        JiffTimeZone::new(jiff::tz::TimeZone::fixed(offset)),
        named_time_zone_bundle_token(),
    );

    let err: TemporalError = backend.exchange(carrier).map(|_| ()).err().ok_or_else(|| {
        miette::miette!(
            "a fixed-offset TimeZone has no IANA identifier to decompose into a descriptor",
        )
    })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn realize_zoned_date_time_resolves_a_real_named_zone() -> miette::Result<()> {
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
        .hours(4u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let timestamp = OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;
    let zone = NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;
    let descriptor = ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(zone)
        .build()
        .into_diagnostic()
        .wrap_err("valid zoned date-time descriptor")?;

    let carrier: ProvenZonedDateTimeCarrier<JiffZoned> = backend
        .exchange(ReflectedZonedDateTime::new(
            descriptor,
            zoned_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("a -04:00 offset in America/New_York on 2024-03-10 resolves to a real Zoned")?;
    let zoned = carrier.carrier();

    assert_eq!(zoned.time_zone().iana_name(), Some("America/New_York"));
    assert_eq!(zoned.datetime().hour(), 13);
    assert_eq!(zoned.datetime().minute(), 30);
    assert_eq!(zoned.offset().seconds(), -4 * 3600);
    Ok(())
}

#[test]
fn zoned_date_time_round_trips_through_a_real_jiff_zoned() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let tz = jiff::tz::TimeZone::get("Europe/London")
        .into_diagnostic()
        .wrap_err("a real IANA zone")?;
    let original = jiff::Timestamp::from_second(1_700_000_000)
        .into_diagnostic()
        .wrap_err("valid timestamp")?
        .to_zoned(tz);
    let carrier = ProvenZonedDateTimeCarrier::<JiffZoned>::new(
        JiffZoned::new(original.clone()),
        zoned_date_time_bundle_token(),
    );

    let reflected: ReflectedZonedDateTime = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real jiff::Zoned reflects to a descriptor")?;
    let round_tripped: ProvenZonedDateTimeCarrier<JiffZoned> = backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to an equivalent Zoned")?;
    let round_tripped = round_tripped.carrier();

    assert_eq!(round_tripped.timestamp(), original.timestamp());
    assert_eq!(
        round_tripped.time_zone().iana_name(),
        original.time_zone().iana_name()
    );
    Ok(())
}
