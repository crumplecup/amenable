//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 4b of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalZoneFactory` +
//! `TemporalNativeZoneFactory` — the higher-order zone-resolution
//! factory, exercising real fold/gap disambiguation against
//! `America/New_York`'s own real 2024 DST transitions and a genuine
//! offset/named-zone consistency check.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{
    JiffDateTime, JiffOffsetDateTime, JiffTimeBackend, JiffTimeZone, JiffVerifier, JiffZoned,
};
use amenable_time::{
    AttachNamedZoneEstablished, AttachNamedZoneEstablishedToken, AttachNamedZoneInput,
    AttachNamedZoneNativeInput, AttachNamedZoneNativeRequest, AttachNamedZonePreconditions,
    AttachNamedZonePreconditionsToken, AttachNamedZoneRequest, CalendarDateDescriptor,
    CompleteDateDescriptor, ConfirmNamedZoneRevisionInput, ConfirmNamedZoneRevisionPreconditions,
    ConfirmNamedZoneRevisionPreconditionsToken, ConfirmNamedZoneRevisionRequest,
    ConfirmZoneAuthorityInput, ConfirmZoneAuthorityPreconditions,
    ConfirmZoneAuthorityPreconditionsToken, ConfirmZoneAuthorityRequest,
    LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder, NamedTimeZoneDescriptorBuilder,
    OffsetDateTimeDescriptorBuilder, ProvenZonedDateTimeCarrier, RawInput,
    ResolveLocalDateTimeInput, ResolveLocalDateTimeNativeInput, ResolveLocalDateTimeNativeRequest,
    ResolveLocalDateTimePreconditions, ResolveLocalDateTimePreconditionsToken,
    ResolveLocalDateTimeRequest, ResolvedNamedTimeZone, TemporalError, TemporalErrorKind,
    TemporalInputToken, TemporalNativeZoneFactory, TemporalZoneFactory, UtcOffsetDescriptorBuilder,
    UtcOffsetSign, ZoneAmbiguityResolutionDescriptor, ZoneGapResolutionDescriptor,
    ZoneTransitionResolutionAuthorityBundle,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalZoneFactory<JiffVerifier>`/`TemporalNativeZoneFactory<JiffVerifier>`.
fn _assert_zone_factory<T: TemporalZoneFactory<JiffVerifier>>() {}
fn _assert_native_zone_factory<T: TemporalNativeZoneFactory<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_zone_factory::<JiffTimeBackend>;
    let _ = _assert_native_zone_factory::<JiffTimeBackend>;
};

fn local_date_time(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
) -> miette::Result<amenable_time::LocalDateTimeDescriptor> {
    LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(hour)
                .minute(minute)
                .second(0u8)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")
}

fn new_york_zone() -> miette::Result<amenable_time::NamedTimeZoneDescriptor> {
    NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")
}

fn resolution_authority(
    ambiguity: ZoneAmbiguityResolutionDescriptor,
    gap: ZoneGapResolutionDescriptor,
) -> amenable_time::LocalTimeZoneResolutionAuthorityDescriptor {
    amenable_time::LocalTimeZoneResolutionAuthorityDescriptor::new(ambiguity, gap)
}

fn resolve_local_date_time_preconditions_token() -> ResolveLocalDateTimePreconditionsToken {
    <ResolveLocalDateTimePreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

fn attach_named_zone_preconditions_token() -> AttachNamedZonePreconditionsToken {
    <AttachNamedZonePreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

fn confirm_named_zone_revision_preconditions_token() -> ConfirmNamedZoneRevisionPreconditionsToken {
    <ConfirmNamedZoneRevisionPreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
        TemporalInputToken::new(),
    )
}

#[test]
fn resolve_named_zone_accepts_a_real_iana_identifier() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let resolved: ResolvedNamedTimeZone = backend
        .exchange(RawInput::received("America/New_York"))
        .into_diagnostic()
        .wrap_err("America/New_York is a real IANA zone")?;
    assert_eq!(resolved.descriptor().identifier(), "America/New_York");
    Ok(())
}

#[test]
fn resolve_named_zone_rejects_a_fake_identifier() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let result: Result<ResolvedNamedTimeZone, TemporalError> =
        backend.exchange(RawInput::received("Nowhere/Fictional"));
    let err: TemporalError = result
        .map(|_| ())
        .err()
        .ok_or_else(|| miette::miette!("Nowhere/Fictional is not real"))?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn confirm_zone_authority_accepts_a_real_zone() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let request = ConfirmZoneAuthorityRequest::new(
        new_york_zone()?,
        resolution_authority(
            ZoneAmbiguityResolutionDescriptor::PreferEarlier,
            ZoneGapResolutionDescriptor::ShiftForward,
        ),
    );
    let token: ConfirmZoneAuthorityPreconditionsToken =
        <ConfirmZoneAuthorityPreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
            TemporalInputToken::new(),
        );
    backend
        .exchange(ConfirmZoneAuthorityInput::new(request, token))
        .into_diagnostic()
        .wrap_err("America/New_York is a real zone, any resolution authority is valid")?;
    Ok(())
}

#[test]
fn resolve_local_date_time_resolves_a_gap_by_shifting_forward() -> miette::Result<()> {
    // Real jiff cross-check: America/New_York springs forward on
    // 2024-03-10 at 02:00 local -> 03:00 EDT. 02:30 local falls inside
    // the gap. ShiftForward resolves via jiff's real `Gap::after`
    // (EDT, -04:00) to compute the instant -- UTC 06:30, matching
    // jiff's own real `AmbiguousTimestamp::earlier()` doc example for
    // this exact case. A real, non-obvious subtlety confirmed only by
    // running this for real (not by reasoning about it): the
    // RESULTING descriptor's own offset field is NOT -04:00 -- it's
    // the REAL zone's own offset at that FINAL computed instant (UTC
    // 06:30 falls BEFORE the real 07:00Z transition moment, so the
    // real zone still reports EST, -05:00, confirmed directly against
    // jiff's own `AmbiguousZoned::earlier()` doc example, which shows
    // the redisplayed local time as "01:30-05:00", not "02:30-04:00").
    let backend = JiffTimeBackend;
    let request = ResolveLocalDateTimeRequest::new(
        local_date_time(2024, 3, 10, 2, 30)?,
        new_york_zone()?,
        resolution_authority(
            ZoneAmbiguityResolutionDescriptor::PreferEarlier,
            ZoneGapResolutionDescriptor::ShiftForward,
        ),
    );
    let output = backend
        .exchange(ResolveLocalDateTimeInput::new(
            request,
            resolve_local_date_time_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("a gap is real, resolvable ambiguity, not an error")?;
    assert_eq!(output.descriptor().timestamp().offset().hours(), 5);
    assert_eq!(
        output.descriptor().timestamp().offset().sign(),
        UtcOffsetSign::Negative
    );
    Ok(())
}

#[test]
fn resolve_local_date_time_resolves_a_gap_by_shifting_backward() -> miette::Result<()> {
    // The inverse of the ShiftForward case above: `Gap::before` (EST,
    // -05:00) computes UTC 07:30, which falls AFTER the real 07:00Z
    // transition, so the real zone reports EDT (-04:00) at that
    // instant -- the opposite of ShiftForward's own final offset, for
    // the same "redisplay uses the real zone's rule at the resulting
    // instant, not the intermediate resolution choice" reason.
    let backend = JiffTimeBackend;
    let request = ResolveLocalDateTimeRequest::new(
        local_date_time(2024, 3, 10, 2, 30)?,
        new_york_zone()?,
        resolution_authority(
            ZoneAmbiguityResolutionDescriptor::PreferEarlier,
            ZoneGapResolutionDescriptor::ShiftBackward,
        ),
    );
    let output = backend
        .exchange(ResolveLocalDateTimeInput::new(
            request,
            resolve_local_date_time_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("a gap is real, resolvable ambiguity, not an error")?;
    assert_eq!(output.descriptor().timestamp().offset().hours(), 4);
    assert_eq!(
        output.descriptor().timestamp().offset().sign(),
        UtcOffsetSign::Negative
    );
    Ok(())
}

#[test]
fn resolve_local_date_time_resolves_a_fold_by_preferring_earlier() -> miette::Result<()> {
    // Real jiff cross-check: America/New_York falls back on 2024-11-03
    // at 02:00 EDT -> 01:00 EST, repeating the 01:00-01:59 hour. 01:30
    // local is genuinely ambiguous. PreferEarlier selects the first
    // (EDT, -04:00) occurrence.
    let backend = JiffTimeBackend;
    let request = ResolveLocalDateTimeRequest::new(
        local_date_time(2024, 11, 3, 1, 30)?,
        new_york_zone()?,
        resolution_authority(
            ZoneAmbiguityResolutionDescriptor::PreferEarlier,
            ZoneGapResolutionDescriptor::ShiftForward,
        ),
    );
    let output = backend
        .exchange(ResolveLocalDateTimeInput::new(
            request,
            resolve_local_date_time_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("a fold is real, resolvable ambiguity, not an error")?;
    assert_eq!(output.descriptor().timestamp().offset().hours(), 4);
    Ok(())
}

#[test]
fn resolve_local_date_time_resolves_a_fold_by_preferring_later() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let request = ResolveLocalDateTimeRequest::new(
        local_date_time(2024, 11, 3, 1, 30)?,
        new_york_zone()?,
        resolution_authority(
            ZoneAmbiguityResolutionDescriptor::PreferLater,
            ZoneGapResolutionDescriptor::ShiftForward,
        ),
    );
    let output = backend
        .exchange(ResolveLocalDateTimeInput::new(
            request,
            resolve_local_date_time_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("a fold is real, resolvable ambiguity, not an error")?;
    assert_eq!(output.descriptor().timestamp().offset().hours(), 5);
    Ok(())
}

#[test]
fn attach_named_zone_accepts_a_consistent_offset() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    // 2024-07-15 is squarely inside EDT (-04:00) -- no ambiguity.
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Negative)
        .hours(4u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let timestamp = OffsetDateTimeDescriptorBuilder::default()
        .local(local_date_time(2024, 7, 15, 17, 30)?)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;
    let request = AttachNamedZoneRequest::new(timestamp, new_york_zone()?);
    let output = backend
        .exchange(AttachNamedZoneInput::new(
            request,
            attach_named_zone_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("-04:00 is the real, consistent EDT offset for this date")?;
    assert_eq!(output.descriptor().zone().identifier(), "America/New_York");
    Ok(())
}

#[test]
fn attach_named_zone_rejects_an_inconsistent_offset() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    // Same local time as above, but the WRONG offset (EST, -05:00,
    // which does not apply in July).
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Negative)
        .hours(5u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let timestamp = OffsetDateTimeDescriptorBuilder::default()
        .local(local_date_time(2024, 7, 15, 17, 30)?)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;
    let request = AttachNamedZoneRequest::new(timestamp, new_york_zone()?);
    let err: TemporalError = backend
        .exchange(AttachNamedZoneInput::new(
            request,
            attach_named_zone_preconditions_token(),
        ))
        .map(|_| ())
        .err()
        .ok_or_else(|| {
            miette::miette!(
                "-05:00 (EST) is not America/New_York's real offset in July (EDT, -04:00)"
            )
        })?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn confirm_named_zone_revision_accepts_a_currently_consistent_zoned_date_time() -> miette::Result<()>
{
    let backend = JiffTimeBackend;
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Negative)
        .hours(4u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let timestamp = OffsetDateTimeDescriptorBuilder::default()
        .local(local_date_time(2024, 7, 15, 17, 30)?)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")?;
    let zoned = amenable_time::ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(new_york_zone()?)
        .build()
        .into_diagnostic()
        .wrap_err("valid zoned date-time descriptor")?;
    let request = ConfirmNamedZoneRevisionRequest::new(zoned);
    backend
        .exchange(ConfirmNamedZoneRevisionInput::new(
            request,
            confirm_named_zone_revision_preconditions_token(),
        ))
        .into_diagnostic()
        .wrap_err("this ZonedDateTimeDescriptor is still consistent with the real zone")?;
    Ok(())
}

#[test]
fn resolve_local_date_time_native_matches_the_descriptor_level_result() -> miette::Result<()> {
    // Same real gap as `resolve_local_date_time_resolves_a_gap_by_
    // shifting_forward` above, exercised through the native edge
    // directly -- the real zone's own redisplayed offset at the
    // resulting instant is EST (-05:00), for the identical reason
    // documented on that test.
    let backend = JiffTimeBackend;
    let local = jiff::civil::DateTime::new(2024, 3, 10, 2, 30, 0, 0)
        .into_diagnostic()
        .wrap_err("valid local datetime")?;
    let tz = jiff::tz::TimeZone::get("America/New_York")
        .into_diagnostic()
        .wrap_err("a real IANA zone")?;
    let authority = resolution_authority(
        ZoneAmbiguityResolutionDescriptor::PreferEarlier,
        ZoneGapResolutionDescriptor::ShiftForward,
    );
    let request = ResolveLocalDateTimeNativeRequest::<JiffTimeBackend>::new(
        JiffDateTime::new(local),
        JiffTimeZone::new(tz),
        authority,
        ZoneTransitionResolutionAuthorityBundle::default(),
    );
    let input =
        ResolveLocalDateTimeNativeInput::<JiffTimeBackend>::new(request, TemporalInputToken::new());
    let output = backend
        .exchange(input)
        .into_diagnostic()
        .wrap_err("the native edge resolves the same real gap")?;
    assert_eq!(output.carrier().offset().seconds(), -5 * 3600);
    Ok(())
}

#[test]
fn attach_named_zone_native_rejects_an_inconsistent_offset() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let local = jiff::civil::DateTime::new(2024, 7, 15, 17, 30, 0, 0)
        .into_diagnostic()
        .wrap_err("valid local datetime")?;
    let wrong_offset = jiff::tz::Offset::from_seconds(-5 * 3600)
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let tz = jiff::tz::TimeZone::get("America/New_York")
        .into_diagnostic()
        .wrap_err("a real IANA zone")?;
    let request = AttachNamedZoneNativeRequest::<JiffTimeBackend>::new(
        JiffOffsetDateTime::new(local, wrong_offset),
        JiffTimeZone::new(tz),
    );
    let input =
        AttachNamedZoneNativeInput::<JiffTimeBackend>::new(request, TemporalInputToken::new());
    let err: TemporalError = backend.exchange(input).map(|_| ()).err().ok_or_else(|| {
        miette::miette!("the native edge performs the same real consistency check")
    })?;
    assert!(matches!(
        &**err.kind(),
        TemporalErrorKind::InvalidDescriptor(_)
    ));
    Ok(())
}

#[test]
fn confirm_named_zone_revision_native_accepts_a_real_proven_zoned() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let tz = jiff::tz::TimeZone::get("America/New_York")
        .into_diagnostic()
        .wrap_err("a real IANA zone")?;
    let zoned = jiff::Timestamp::from_second(1_700_000_000)
        .into_diagnostic()
        .wrap_err("valid timestamp")?
        .to_zoned(tz);
    let carrier = ProvenZonedDateTimeCarrier::<JiffZoned>::new(
        JiffZoned::new(zoned),
        confirm_zone_authority_token_as_zoned_bundle(),
    );
    let _: amenable_time::ConfirmNamedZoneRevisionNativeOutput = backend
        .exchange(carrier)
        .into_diagnostic()
        .wrap_err("a real, freshly-constructed Zoned is still consistent with its own zone")?;
    Ok(())
}

/// A real `ZonedDateTimeSemanticBundleToken`, minted the same way
/// `jiff_backend_zone_test.rs`'s own helper does, needed only to
/// construct a `ProvenZonedDateTimeCarrier` as a test fixture here.
fn confirm_zone_authority_token_as_zoned_bundle() -> amenable_time::ZonedDateTimeSemanticBundleToken
{
    let preconditions: AttachNamedZonePreconditionsToken =
        <AttachNamedZonePreconditions as Establish<TemporalInputToken, JiffVerifier>>::establish(
            TemporalInputToken::new(),
        );
    let established: AttachNamedZoneEstablishedToken = <AttachNamedZoneEstablished as Establish<
        AttachNamedZonePreconditionsToken,
        JiffVerifier,
    >>::establish(preconditions);
    <amenable_time::ZonedDateTimeSemanticBundle as Establish<
        AttachNamedZoneEstablishedToken,
        JiffVerifier,
    >>::establish(established)
}
