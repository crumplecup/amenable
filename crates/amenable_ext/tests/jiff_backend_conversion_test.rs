//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 5 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalConversionFactory`
//! and `TemporalNativeConversionFactory`, covering real UTC
//! normalization, real named-zone stripping (reusing Phase 4b's own
//! consistency-checked zoned conversion), and real, honest lossless-
//! vs-lossy sub-second precision adjustment.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffOffsetDateTime, JiffTimeBackend, JiffVerifier, JiffZoned};
use amenable_time::{
    AdjustPrecisionLosslesslyInput, AdjustPrecisionLosslesslyNativeInput,
    AdjustPrecisionLosslesslyNativeRequest, AdjustPrecisionLosslesslyPreconditions,
    AdjustPrecisionLosslesslyPreconditionsToken, AdjustPrecisionLosslesslyRequest,
    AttachNamedZoneEstablished, AttachNamedZoneEstablishedToken, AttachNamedZonePreconditions,
    AttachNamedZonePreconditionsToken, CalendarDateDescriptor, CompleteDateDescriptor,
    FractionalSecondDescriptor, LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder,
    LossyConversionAuthorityBundle, NamedTimeZoneDescriptorBuilder, NormalizeToUtcInput,
    NormalizeToUtcPreconditions, NormalizeToUtcPreconditionsToken, NormalizeToUtcRequest,
    OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder, OffsetDateTimeProof,
    OffsetDateTimeProofToken, OffsetDateTimeSemanticBundle, OffsetDateTimeSemanticBundleToken,
    PrecisionDescriptorBuilder, ProvenOffsetDateTimeCarrier, ProvenZonedDateTimeCarrier,
    RoundingModeDescriptor, StripNamedZoneInput, StripNamedZonePreconditions,
    StripNamedZonePreconditionsToken, StripNamedZoneRequest, TemporalComponent,
    TemporalConversionFactory, TemporalError, TemporalErrorKind, TemporalInputToken,
    TemporalNativeConversionFactory, TruncateSubsecondsInput, TruncateSubsecondsNativeInput,
    TruncateSubsecondsNativeRequest, TruncateSubsecondsPreconditions,
    TruncateSubsecondsPreconditionsToken, TruncateSubsecondsRequest, UtcOffsetDescriptorBuilder,
    UtcOffsetSign, ZonedDateTimeDescriptor, ZonedDateTimeDescriptorBuilder,
    ZonedDateTimeSemanticBundle, ZonedDateTimeSemanticBundleToken,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalConversionFactory<JiffVerifier>` / `TemporalNativeConversionFactory<JiffVerifier>`.
fn _assert_conversion_factory<T: TemporalConversionFactory<JiffVerifier>>() {}
fn _assert_native_conversion_factory<T: TemporalNativeConversionFactory<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_conversion_factory::<JiffTimeBackend>;
    let _ = _assert_native_conversion_factory::<JiffTimeBackend>;
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

fn zoned_date_time_bundle_token() -> ZonedDateTimeSemanticBundleToken {
    let preconditions = <AttachNamedZonePreconditions as Establish<
        TemporalInputToken,
        JiffVerifier,
    >>::establish(TemporalInputToken::new());
    let established = <AttachNamedZoneEstablished as Establish<
        AttachNamedZonePreconditionsToken,
        JiffVerifier,
    >>::establish(preconditions);
    <ZonedDateTimeSemanticBundle as Establish<AttachNamedZoneEstablishedToken, JiffVerifier>>::establish(
        established,
    )
}

fn normalize_to_utc_preconditions_token() -> NormalizeToUtcPreconditionsToken {
    <NormalizeToUtcPreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

fn strip_named_zone_preconditions_token() -> StripNamedZonePreconditionsToken {
    <StripNamedZonePreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

fn adjust_precision_losslessly_preconditions_token() -> AdjustPrecisionLosslesslyPreconditionsToken
{
    <AdjustPrecisionLosslesslyPreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

fn truncate_subseconds_preconditions_token() -> TruncateSubsecondsPreconditionsToken {
    <TruncateSubsecondsPreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

/// Build an offset date-time descriptor with an optional fractional-
/// second digit string on its local time.
fn offset_date_time_descriptor(
    (year, month, day): (i32, u8, u8),
    (hour, minute, second): (u8, u8, u8),
    fractional_digits: Option<&str>,
    (sign, offset_hours): (UtcOffsetSign, u8),
) -> miette::Result<OffsetDateTimeDescriptor> {
    let mut time_builder = LocalTimeDescriptorBuilder::default()
        .hour(hour)
        .minute(minute)
        .second(second);
    if let Some(digits) = fractional_digits {
        time_builder = time_builder.fractional_second(FractionalSecondDescriptor::new(digits));
    }
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(
            time_builder
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

fn fractional_digits_of(descriptor: &OffsetDateTimeDescriptor) -> Option<String> {
    descriptor
        .local()
        .time()
        .fractional_second()
        .as_ref()
        .map(|fraction| fraction.digits().to_owned())
}

#[test]
fn normalize_to_utc_shifts_a_negative_offset_forward() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = offset_date_time_descriptor(
        (2024, 3, 10),
        (13, 30, 0),
        None,
        (UtcOffsetSign::Negative, 4),
    )?;

    let output = backend
        .exchange(NormalizeToUtcInput::new(
            NormalizeToUtcRequest::new(descriptor),
            normalize_to_utc_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("a real offset date-time normalizes to UTC")?;
    let normalized = output.descriptor();

    assert_eq!(normalized.offset().sign(), UtcOffsetSign::Positive);
    assert_eq!(normalized.offset().hours(), 0);
    assert_eq!(normalized.local().time().hour(), 17);
    assert_eq!(normalized.local().time().minute(), 30);
    Ok(())
}

#[test]
fn normalize_to_utc_native_matches_the_descriptor_level_result() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = jiff::civil::DateTime::new(2024, 3, 10, 13, 30, 0, 0)
        .into_diagnostic()
        .wrap_err("valid datetime")?;
    let offset = jiff::tz::Offset::from_seconds(-4 * 3600)
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let carrier = ProvenOffsetDateTimeCarrier::<JiffOffsetDateTime>::new(
        JiffOffsetDateTime::new(local, offset),
        offset_date_time_bundle_token(),
    );

    let native: amenable_time::NormalizeToUtcNativeOutput<JiffTimeBackend> = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real jiff offset date-time normalizes to UTC natively")?;
    let native = native.carrier();

    assert_eq!(*native.offset(), jiff::tz::Offset::UTC);
    assert_eq!(native.local().hour(), 17);
    assert_eq!(native.local().minute(), 30);
    Ok(())
}

#[test]
fn strip_named_zone_preserves_the_local_representation_and_offset() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let timestamp = offset_date_time_descriptor(
        (2024, 3, 10),
        (13, 30, 0),
        None,
        (UtcOffsetSign::Negative, 4),
    )?;
    let zone = NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;
    let descriptor: ZonedDateTimeDescriptor = ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(zone)
        .build()
        .into_diagnostic()
        .wrap_err("valid zoned date-time descriptor")?;

    let output = backend
        .exchange(StripNamedZoneInput::new(
            StripNamedZoneRequest::new(descriptor),
            strip_named_zone_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("a real zoned date-time strips to an offset date-time")?;
    let stripped = output.descriptor();

    assert_eq!(stripped.offset().sign(), UtcOffsetSign::Negative);
    assert_eq!(stripped.offset().hours(), 4);
    assert_eq!(stripped.local().time().hour(), 13);
    assert_eq!(stripped.local().time().minute(), 30);
    Ok(())
}

#[test]
fn strip_named_zone_native_matches_the_descriptor_level_result() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let tz = jiff::tz::TimeZone::get("America/New_York")
        .into_diagnostic()
        .wrap_err("a real IANA zone")?;
    let local = jiff::civil::DateTime::new(2024, 3, 10, 13, 30, 0, 0)
        .into_diagnostic()
        .wrap_err("valid datetime")?;
    let zoned = tz
        .to_zoned(local)
        .into_diagnostic()
        .wrap_err("an unambiguous local time")?;
    let carrier = ProvenZonedDateTimeCarrier::<JiffZoned>::new(
        JiffZoned::new(zoned),
        zoned_date_time_bundle_token(),
    );

    let stripped: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime> = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real jiff::Zoned strips to an offset date-time natively")?;
    let stripped = stripped.carrier();

    assert_eq!(stripped.local().hour(), 13);
    assert_eq!(stripped.local().minute(), 30);
    assert_eq!(stripped.offset().seconds(), -4 * 3600);
    Ok(())
}

#[test]
fn adjust_precision_losslessly_accepts_an_exact_target() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = offset_date_time_descriptor(
        (2024, 1, 1),
        (0, 0, 0),
        Some("123"),
        (UtcOffsetSign::Positive, 0),
    )?;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(3u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let output = backend
        .exchange(AdjustPrecisionLosslesslyInput::new(
            AdjustPrecisionLosslesslyRequest::new(descriptor, target),
            adjust_precision_losslessly_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("truncating to exactly the represented precision is lossless")?;
    assert_eq!(
        fractional_digits_of(output.descriptor()),
        Some("123".to_owned())
    );
    Ok(())
}

#[test]
fn adjust_precision_losslessly_rejects_a_narrower_target() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = offset_date_time_descriptor(
        (2024, 1, 1),
        (0, 0, 0),
        Some("123456"),
        (UtcOffsetSign::Positive, 0),
    )?;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(3u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let err: TemporalError = backend
        .exchange(AdjustPrecisionLosslesslyInput::new(
            AdjustPrecisionLosslesslyRequest::new(descriptor, target),
            adjust_precision_losslessly_preconditions_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!("truncating away real sub-second precision is not lossless")
        })?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn adjust_precision_losslessly_rejects_a_non_second_component() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor =
        offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), None, (UtcOffsetSign::Positive, 0))?;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Minute)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let err: TemporalError = backend
        .exchange(AdjustPrecisionLosslesslyInput::new(
            AdjustPrecisionLosslesslyRequest::new(descriptor, target),
            adjust_precision_losslessly_preconditions_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!("jiff's precision edges anchor at the second, not the minute")
        })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn adjust_precision_losslessly_native_matches_the_descriptor_level_result() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = jiff::civil::DateTime::new(2024, 1, 1, 0, 0, 0, 123_000_000)
        .into_diagnostic()
        .wrap_err("valid datetime")?;
    let offset = jiff::tz::Offset::UTC;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(3u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let native = backend
        .exchange(AdjustPrecisionLosslesslyNativeInput::new(
            AdjustPrecisionLosslesslyNativeRequest::new(
                JiffOffsetDateTime::new(local, offset),
                target,
            ),
            TemporalInputToken::new(),
        ))
        .into_diagnostic()
        .wrap_err("truncating to exactly the represented precision is lossless natively")?;
    let native = native.carrier();

    assert_eq!(native.local().subsec_nanosecond(), 123_000_000);
    assert_eq!(*native.offset(), offset);
    Ok(())
}

#[test]
fn truncate_subseconds_truncates_finer_digits() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = offset_date_time_descriptor(
        (2024, 1, 1),
        (0, 0, 0),
        Some("123456789"),
        (UtcOffsetSign::Positive, 0),
    )?;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(3u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let output = backend
        .exchange(TruncateSubsecondsInput::new(
            TruncateSubsecondsRequest::new(descriptor, target),
            truncate_subseconds_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("truncation always succeeds under the default Truncate rounding mode")?;
    assert_eq!(
        fractional_digits_of(output.descriptor()),
        Some("123".to_owned())
    );
    Ok(())
}

#[test]
fn truncate_subseconds_rejects_a_non_truncate_rounding_mode() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = offset_date_time_descriptor(
        (2024, 1, 1),
        (0, 0, 0),
        Some("123456789"),
        (UtcOffsetSign::Positive, 0),
    )?;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(3u8)
        .rounding_mode(RoundingModeDescriptor::HalfUp)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let err: TemporalError = backend
        .exchange(TruncateSubsecondsInput::new(
            TruncateSubsecondsRequest::new(descriptor, target),
            truncate_subseconds_preconditions_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| miette::miette!("this backend only implements Truncate rounding"))?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn truncate_subseconds_native_matches_the_descriptor_level_result() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = jiff::civil::DateTime::new(2024, 1, 1, 0, 0, 0, 123_456_789)
        .into_diagnostic()
        .wrap_err("valid datetime")?;
    let offset = jiff::tz::Offset::UTC;
    let target = PrecisionDescriptorBuilder::default()
        .smallest_component(TemporalComponent::Second)
        .fractional_digits(3u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid precision descriptor")?;

    let native = backend
        .exchange(TruncateSubsecondsNativeInput::new(
            TruncateSubsecondsNativeRequest::new(
                JiffOffsetDateTime::new(local, offset),
                target,
                LossyConversionAuthorityBundle::default(),
            ),
            TemporalInputToken::new(),
        ))
        .into_diagnostic()
        .wrap_err("truncation always succeeds under the default Truncate rounding mode natively")?;
    let native = native.carrier();

    assert_eq!(native.local().subsec_nanosecond(), 123_000_000);
    assert_eq!(*native.offset(), offset);
    Ok(())
}
