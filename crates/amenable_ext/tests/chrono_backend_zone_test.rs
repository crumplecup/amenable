//! `ChronoTimeBackend`'s chrono-tz-backed `Exchange` bodies: the zone
//! bridge, `TemporalZoneNativeBridge` over
//! `chrono_tz::Tz`/`chrono::DateTime<chrono_tz::Tz>` -- real IANA zone
//! lookups and zoned-instant construction. Mirrors
//! `jiff_backend_zone_test.rs`.

#![cfg(feature = "chrono-tz")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{ChronoTimeBackend, ChronoTz, ChronoVerifier, ChronoZoned};
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

// Fails to compile if `ChronoTimeBackend` stops resolving as a real
// `TemporalZoneNativeBridge<ChronoVerifier>`.
fn _assert_zone_bridge<T: TemporalZoneNativeBridge<ChronoVerifier>>() {}
const _: () = {
    let _ = _assert_zone_bridge::<ChronoTimeBackend>;
};

fn named_time_zone_bundle_token() -> NamedTimeZoneSemanticBundleToken {
    let valid: NamedTimeZoneIdentityValidToken = <NamedTimeZoneIdentityValid as Establish<
        TemporalInputToken,
        ChronoVerifier,
    >>::establish(TemporalInputToken::new());
    <NamedTimeZoneSemanticBundle as Establish<NamedTimeZoneIdentityValidToken, ChronoVerifier>>::establish(valid)
}

fn zoned_date_time_bundle_token() -> ZonedDateTimeSemanticBundleToken {
    let preconditions: AttachNamedZonePreconditionsToken =
        <AttachNamedZonePreconditions as Establish<TemporalInputToken, ChronoVerifier>>::establish(
            TemporalInputToken::new(),
        );
    let established: AttachNamedZoneEstablishedToken = <AttachNamedZoneEstablished as Establish<
        AttachNamedZonePreconditionsToken,
        ChronoVerifier,
    >>::establish(preconditions);
    <ZonedDateTimeSemanticBundle as Establish<AttachNamedZoneEstablishedToken, ChronoVerifier>>::establish(
        established,
    )
}

#[test]
fn realize_named_time_zone_resolves_a_real_iana_zone() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
    let descriptor = NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;

    let carrier: ProvenNamedTimeZoneCarrier<ChronoTz> = backend
        .exchange(ReflectedNamedTimeZone::new(
            descriptor,
            named_time_zone_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err("America/New_York is a real IANA zone")?;
    assert_eq!(**carrier.carrier(), chrono_tz::America::New_York);
    Ok(())
}

#[test]
fn realize_named_time_zone_rejects_an_unknown_identifier() -> miette::Result<()> {
    amenable_core::init_tracing();
    let backend = ChronoTimeBackend;
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
        TemporalErrorKind::NamedZoneResolutionFailed(_)
    ));
    Ok(())
}

#[test]
fn reflect_named_time_zone_accepts_utc_as_a_real_identifier() -> miette::Result<()> {
    amenable_core::init_tracing();
    // Every chrono_tz::Tz variant, including UTC, is a real named IANA
    // zone (confirmed in source: Tz::name() is total, no Option) --
    // unlike jiff's TimeZone, chrono-tz has no "unknown"/"fixed offset
    // with no identifier" case to distinguish.
    let backend = ChronoTimeBackend;
    let carrier = ProvenNamedTimeZoneCarrier::<ChronoTz>::new(
        ChronoTz::new(chrono_tz::Tz::UTC),
        named_time_zone_bundle_token(),
    );

    let reflected = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("Tz::UTC has a real IANA identifier, \"UTC\"")?;
    assert_eq!(reflected.descriptor().identifier(), "UTC");
    Ok(())
}

#[test]
fn realize_zoned_date_time_resolves_a_real_named_zone() -> miette::Result<()> {
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

    let carrier: ProvenZonedDateTimeCarrier<ChronoZoned> = backend
        .exchange(ReflectedZonedDateTime::new(
            descriptor,
            zoned_date_time_bundle_token(),
        ))
        .into_diagnostic()
        .wrap_err(
            "a -04:00 offset in America/New_York on 2024-03-10 resolves to a real DateTime<Tz>",
        )?;
    let zoned = **carrier.carrier();

    assert_eq!(zoned.timezone(), chrono_tz::America::New_York);
    use chrono::{Offset, Timelike};
    assert_eq!(zoned.hour(), 13);
    assert_eq!(zoned.minute(), 30);
    assert_eq!(zoned.offset().fix().local_minus_utc(), -4 * 3600);
    Ok(())
}

#[test]
fn realize_zoned_date_time_rejects_an_offset_inconsistent_with_the_zone() -> miette::Result<()> {
    amenable_core::init_tracing();
    // On 2024-03-10, America/New_York springs forward at 02:00 local to
    // 03:00 EDT (-04:00) -- -05:00 (EST) is not a real offset that zone
    // produces for 13:30 local that day, so a descriptor claiming it must
    // be rejected, not silently accepted.
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

    let err: TemporalError = backend
        .exchange(ReflectedZonedDateTime::new(
            descriptor,
            zoned_date_time_bundle_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!("-05:00 is not a real America/New_York offset on this date")
        })?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::OffsetInconsistentWithNamedZone(_)
    ));
    Ok(())
}

#[test]
fn zoned_date_time_round_trips_through_a_real_chrono_tz_value() -> miette::Result<()> {
    amenable_core::init_tracing();
    let tz: chrono_tz::Tz = "Europe/London"
        .parse()
        .into_diagnostic()
        .wrap_err("a real IANA zone")?;
    let original = chrono::DateTime::from_timestamp(1_700_000_000, 0)
        .ok_or_else(|| miette::miette!("valid timestamp"))?
        .with_timezone(&tz);
    let backend = ChronoTimeBackend;
    let carrier = ProvenZonedDateTimeCarrier::<ChronoZoned>::new(
        ChronoZoned::new(original),
        zoned_date_time_bundle_token(),
    );

    let reflected: ReflectedZonedDateTime = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real chrono DateTime<Tz> reflects to a descriptor")?;
    let round_tripped = **backend
        .exchange(reflected)
        .into_diagnostic()
        .wrap_err("the descriptor realizes back to an equivalent DateTime<Tz>")?
        .carrier();

    assert_eq!(round_tripped, original);
    assert_eq!(round_tripped.timezone(), original.timezone());
    Ok(())
}
