use crate::jiff::backend::instant::jiff_parts_to_offset_date_time_descriptor;
use crate::jiff::backend::instant::offset_date_time_descriptor_to_jiff_parts;
use crate::jiff::backend::zone_factory::offset_is_consistent_with_named_zone;
use crate::{JiffTimeBackend, JiffTimeZone, JiffVerifier, JiffZoned};
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    InvalidDescriptorSource, NamedTimeZoneDescriptor, NamedTimeZoneDescriptorBuilder,
    ProvenNamedTimeZoneCarrier, ProvenZonedDateTimeCarrier, ReflectedNamedTimeZone,
    ReflectedZonedDateTime, TemporalError, TemporalErrorKind, UnsupportedSource,
    ZonedDateTimeDescriptor, ZonedDateTimeDescriptorBuilder,
};

// ── Real conversions to/from jiff's zone types ──────────────────────

/// Resolve a named-time-zone descriptor to a real `jiff::tz::TimeZone`.
///
/// Real IANA zone lookup via `TimeZone::get`. The descriptor's own
/// `tzdb_revision` field is ignored on this direction: jiff's real
/// `TimeZone::get` has no parameter for selecting a specific tzdb
/// revision at all — it always resolves against whichever tzdb the
/// running process is linked against — so there is no honest way to
/// honor a *different* revision than that one, and pretending
/// otherwise would be dishonest. This mirrors `TemporalReporter::
/// current_tzdb_revision` staying `None`: jiff exposes no public API to
/// query the linked tzdb's own revision string either (confirmed by
/// checking its real `tz::db` module for a `version`/`revision`
/// function — none exists).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn named_time_zone_descriptor_to_jiff_time_zone(
    descriptor: &NamedTimeZoneDescriptor,
) -> Result<jiff::tz::TimeZone, TemporalError> {
    jiff::tz::TimeZone::get(descriptor.identifier()).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "could not resolve IANA time zone {:?}: {err}",
                descriptor.identifier()
            )),
        ))
    })
}

/// Decompose a real `jiff::tz::TimeZone` back into a named-time-zone
/// descriptor.
///
/// Real, honest scoping: a `TimeZone` with no IANA identifier at all —
/// `TimeZone::unknown()` (the special, explicitly-non-IANA `Etc/Unknown`
/// marker) or any `TimeZone::fixed(offset)`-constructed value — has no
/// named-zone descriptor to decompose into; that is a real `Unsupported`
/// error, not a fabricated identifier. `TimeZone::UTC` is genuinely
/// EXEMPT from this — real source confirms `iana_name()`'s own `UTC =>
/// Some("UTC")` match arm treats it as a real, valid identifier, a
/// finding this file's own first test attempt got wrong by assuming
/// resemblance to `Offset`'s unrelated "no identifier" shape rather
/// than checking `TimeZone::iana_name()`'s real match arms directly.
/// `tzdb_revision` is always `None` here for the same reason it's
/// ignored on the realize direction above.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tz)))]
fn jiff_time_zone_to_named_time_zone_descriptor(
    tz: &jiff::tz::TimeZone,
) -> Result<NamedTimeZoneDescriptor, TemporalError> {
    let identifier = tz.iana_name().ok_or_else(|| {
        TemporalError::new(TemporalErrorKind::Unsupported(UnsupportedSource::new(
            "this jiff::tz::TimeZone has no IANA identifier to decompose into a named-zone \
             descriptor (it is unknown, or a fixed offset)"
                .to_owned(),
        )))
    })?;
    NamedTimeZoneDescriptorBuilder::default()
        .identifier(identifier)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build a named time zone descriptor: {err}"
                )),
            ))
        })
}

/// Resolve a zoned date-time descriptor to a real `jiff::Zoned`.
///
/// The descriptor's own offset pins the exact instant (via a fixed-
/// offset zone, which never has ambiguity — the whole point of carrying
/// an explicit offset alongside the zone identity); that instant is then
/// re-attached to the real named zone the descriptor also names, via
/// `Timestamp::to_zoned` (infallible once the instant itself is known).
///
/// Retrofitted (Phase 4b/5) to genuinely check `offset_is_consistent_
/// with_named_zone` before pinning — this function originally accepted
/// ANY offset unconditionally, a real gap only found while building
/// `attach_named_zone`'s own consistency proof in Phase 4b, which is
/// supposed to be the FIRST place that fact gets established. Every
/// caller of this function (Phase 4's own `TemporalZoneNativeBridge`
/// realize body included) now gets the check for real, not just
/// `attach_named_zone`'s own callers.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn zoned_date_time_descriptor_to_jiff_zoned(
    descriptor: &ZonedDateTimeDescriptor,
) -> Result<jiff::Zoned, TemporalError> {
    let (local, offset) = offset_date_time_descriptor_to_jiff_parts(descriptor.timestamp())?;
    let named_tz = named_time_zone_descriptor_to_jiff_time_zone(descriptor.zone())?;
    if !offset_is_consistent_with_named_zone(local, &named_tz, offset) {
        return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "offset {offset:?} is not one of the real offsets {}'s own rules produce for local \
             time {local}",
                named_tz.iana_name().unwrap_or("<unnamed>"),
            )),
        )));
    }
    let timestamp = jiff::tz::TimeZone::fixed(offset)
        .to_zoned(local)
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not pin the offset date-time to a fixed instant: {err}"
                )),
            ))
        })?
        .timestamp();
    Ok(timestamp.to_zoned(named_tz))
}

/// Decompose a real `jiff::Zoned` back into a zoned date-time
/// descriptor.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(zoned)))]
pub(super) fn jiff_zoned_to_zoned_date_time_descriptor(
    zoned: &jiff::Zoned,
) -> Result<ZonedDateTimeDescriptor, TemporalError> {
    let timestamp = jiff_parts_to_offset_date_time_descriptor(zoned.datetime(), zoned.offset())?;
    let zone = jiff_time_zone_to_named_time_zone_descriptor(zoned.time_zone())?;
    ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(zone)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build a zoned date-time descriptor: {err}"
                )),
            ))
        })
}

// ── Zone native bridge ───────────────────────────────────────────────
//
// `TemporalZoneNativeBridge<JiffVerifier>` is the `realize_named_time_
// zone` / `reflect_named_time_zone` / `realize_zoned_date_time` /
// `reflect_zoned_date_time` four-edge bundle. All real: named-zone
// resolution goes through jiff's actual IANA tzdb lookup, and the
// zoned-date-time pair composes it with Phase 2's own offset-date-time
// conversion.

impl Exchange<ReflectedNamedTimeZone, ProvenNamedTimeZoneCarrier<JiffTimeZone>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedNamedTimeZone,
    ) -> Result<ProvenNamedTimeZoneCarrier<JiffTimeZone>, TemporalError> {
        let tz = named_time_zone_descriptor_to_jiff_time_zone(input.descriptor())?;
        let token = <ReflectedNamedTimeZone as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenNamedTimeZoneCarrier::<JiffTimeZone>::new(
            JiffTimeZone::new(tz),
            token,
        ))
    }
}

impl Exchange<ProvenNamedTimeZoneCarrier<JiffTimeZone>, ReflectedNamedTimeZone, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenNamedTimeZoneCarrier<JiffTimeZone>,
    ) -> Result<ReflectedNamedTimeZone, TemporalError> {
        let descriptor = jiff_time_zone_to_named_time_zone_descriptor(input.carrier())?;
        let token =
            <ProvenNamedTimeZoneCarrier<JiffTimeZone> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedNamedTimeZone::new(descriptor, token))
    }
}

impl Exchange<ReflectedZonedDateTime, ProvenZonedDateTimeCarrier<JiffZoned>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedZonedDateTime,
    ) -> Result<ProvenZonedDateTimeCarrier<JiffZoned>, TemporalError> {
        let zoned = zoned_date_time_descriptor_to_jiff_zoned(input.descriptor())?;
        let token = <ReflectedZonedDateTime as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenZonedDateTimeCarrier::<JiffZoned>::new(
            JiffZoned::new(zoned),
            token,
        ))
    }
}

impl Exchange<ProvenZonedDateTimeCarrier<JiffZoned>, ReflectedZonedDateTime, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<JiffZoned>,
    ) -> Result<ReflectedZonedDateTime, TemporalError> {
        let descriptor = jiff_zoned_to_zoned_date_time_descriptor(input.carrier())?;
        let token =
            <ProvenZonedDateTimeCarrier<JiffZoned> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedZonedDateTime::new(descriptor, token))
    }
}
