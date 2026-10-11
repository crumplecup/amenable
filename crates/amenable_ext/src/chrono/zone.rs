use crate::chrono::identity::{ChronoTimeBackend, ChronoVerifier};
use crate::chrono::instant::{
    chrono_to_offset_date_time_descriptor, offset_date_time_descriptor_to_chrono,
};
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    InvalidDescriptorSource, NamedTimeZoneDescriptor, NamedTimeZoneDescriptorBuilder,
    ProvenNamedTimeZoneCarrier, ProvenZonedDateTimeCarrier, ReflectedNamedTimeZone,
    ReflectedZonedDateTime, TemporalError, TemporalErrorKind, TemporalZoneProps,
    ZonedDateTimeDescriptor, ZonedDateTimeDescriptorBuilder,
};
use chrono::{Offset, TimeZone};

// ── Carriers ─────────────────────────────────────────────────────────

/// A [`chrono_tz::Tz`] as a named-time-zone carrier.
///
/// `chrono_tz::Tz` derives `Clone, Copy, PartialEq, Eq, Hash` and has a
/// real manual `Default` (confirmed in source: `Tz::UTC`), so a bare
/// `#[evidence(basis = "Self")]` works, no `basis_ctor` needed — unlike
/// jiff's `JiffTimeZone`, which has no `Default` at all.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(basis = "Self")]
pub struct ChronoTz(
    /// The wrapped named zone.
    chrono_tz::Tz,
);

/// A [`chrono::DateTime<chrono_tz::Tz>`] as a zoned-date-time carrier.
///
/// Unlike `Utc`/`FixedOffset`/`Local`, `chrono::DateTime<Tz>` has no
/// generic `Default` for an arbitrary `Tz: TimeZone` (confirmed in
/// source: only `Utc`, `Local`, and `FixedOffset` get their own manual
/// impls) — this wrapper needs a `basis_ctor`, built from the same real,
/// infallible `TimeZone::from_utc_datetime` chrono's own `Default` impls
/// use internally.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_more::Deref,
    derive_new::new,
)]
#[evidence(
    basis = "Self",
    basis_ctor = "Self(chrono_tz::Tz::UTC.from_utc_datetime(&chrono::NaiveDateTime::default()))"
)]
pub struct ChronoZoned(
    /// The wrapped zoned date-time.
    chrono::DateTime<chrono_tz::Tz>,
);

// ── Props ────────────────────────────────────────────────────────────

impl TemporalZoneProps for ChronoTimeBackend {
    type NamedTimeZone = ChronoTz;
    type ZonedDateTime = ChronoZoned;
}

// ── Descriptor conversions ───────────────────────────────────────────

/// Build a [`TemporalError`] for a descriptor that chrono-tz cannot
/// represent.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn invalid(detail: String) -> TemporalError {
    TemporalError::new(TemporalErrorKind::InvalidDescriptor(
        InvalidDescriptorSource::new(detail),
    ))
}

/// Resolve a named-time-zone descriptor to a real [`chrono_tz::Tz`].
///
/// Real IANA zone lookup via `Tz::from_str`. chrono-tz's own
/// `tzdb_revision` has no runtime-queryable concept at all — the zone
/// table is generated at compile time from whichever `tz` database
/// version `chrono-tz-build` was run against — so, like jiff's own
/// bridge, the descriptor's `tzdb_revision` field is ignored on this
/// direction rather than pretending to honor a different one.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn named_time_zone_descriptor_to_chrono_tz(
    descriptor: &NamedTimeZoneDescriptor,
) -> Result<chrono_tz::Tz, TemporalError> {
    descriptor
        .identifier()
        .parse()
        .map_err(|err: chrono_tz::ParseError| {
            invalid(format!(
                "could not resolve IANA time zone {:?}: {err}",
                descriptor.identifier()
            ))
        })
}

/// Decompose a real [`chrono_tz::Tz`] back into a named-time-zone
/// descriptor.
///
/// Unlike jiff's `TimeZone` (which can also represent "unknown" or a
/// bare fixed offset with no IANA identifier), every `chrono_tz::Tz`
/// variant — including `Tz::UTC` — IS a real, named IANA zone
/// (confirmed in source: `Tz::name()` is total, no `Option`), so this
/// direction never needs an `Unsupported` case.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(tz)))]
pub(super) fn chrono_tz_to_named_time_zone_descriptor(
    tz: chrono_tz::Tz,
) -> Result<NamedTimeZoneDescriptor, TemporalError> {
    NamedTimeZoneDescriptorBuilder::default()
        .identifier(tz.name().to_owned())
        .build()
        .map_err(|err| {
            invalid(format!(
                "could not build a named time zone descriptor: {err}"
            ))
        })
}

/// Whether `offset` is one of the real offsets chrono-tz's own rules
/// produce for local wall-clock time `local` in zone `tz`.
///
/// Mirrors the jiff backend's `offset_is_consistent_with_named_zone`
/// exactly, over chrono's own three-variant `MappedLocalTime` instead of
/// jiff's `AmbiguousOffset`: `Single` must match exactly, `Ambiguous`
/// (chrono's fold case) must match either candidate, and `None` (a real
/// gap — the local time never occurred under this zone's rules) is never
/// consistent with any offset.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(local, tz)))]
pub(super) fn offset_is_consistent_with_named_zone(
    local: chrono::NaiveDateTime,
    tz: chrono_tz::Tz,
    offset: chrono::FixedOffset,
) -> bool {
    match tz.offset_from_local_datetime(&local) {
        chrono::MappedLocalTime::Single(real) => offset == real.fix(),
        chrono::MappedLocalTime::Ambiguous(earliest, latest) => {
            offset == earliest.fix() || offset == latest.fix()
        }
        chrono::MappedLocalTime::None => false,
    }
}

/// Resolve a zoned date-time descriptor to a real
/// `chrono::DateTime<chrono_tz::Tz>`.
///
/// The descriptor's own offset pins the exact instant (via
/// `offset_date_time_descriptor_to_chrono`'s own `FixedOffset` bridge,
/// which never has ambiguity); that instant is then re-attached to the
/// real named zone, after checking the offset really is one chrono-tz's
/// own rules produce for that local time — the same consistency check
/// the jiff backend's bridge applies, not skipped here from the start.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn zoned_date_time_descriptor_to_chrono(
    descriptor: &ZonedDateTimeDescriptor,
) -> Result<chrono::DateTime<chrono_tz::Tz>, TemporalError> {
    let pinned = offset_date_time_descriptor_to_chrono(descriptor.timestamp())?;
    let local = pinned.naive_local();
    let offset = *pinned.offset();
    let tz = named_time_zone_descriptor_to_chrono_tz(descriptor.zone())?;
    if !offset_is_consistent_with_named_zone(local, tz, offset) {
        return Err(invalid(format!(
            "offset {offset:?} is not one of the real offsets {}'s own rules produce for local \
             time {local}",
            tz.name(),
        )));
    }
    Ok(pinned.with_timezone(&tz))
}

/// Decompose a real `chrono::DateTime<chrono_tz::Tz>` back into a zoned
/// date-time descriptor.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(zoned)))]
pub(super) fn chrono_to_zoned_date_time_descriptor(
    zoned: chrono::DateTime<chrono_tz::Tz>,
) -> Result<ZonedDateTimeDescriptor, TemporalError> {
    let timestamp =
        chrono_to_offset_date_time_descriptor(zoned.with_timezone(&zoned.offset().fix()))?;
    let zone = chrono_tz_to_named_time_zone_descriptor(zoned.timezone())?;
    ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(zone)
        .build()
        .map_err(|err| {
            invalid(format!(
                "could not build a zoned date-time descriptor: {err}"
            ))
        })
}

// ── Zone native bridge ───────────────────────────────────────────────
//
// `TemporalZoneNativeBridge<ChronoVerifier>` is the `realize_named_time_
// zone` / `reflect_named_time_zone` / `realize_zoned_date_time` /
// `reflect_zoned_date_time` four-edge bundle. All real: named-zone
// resolution goes through chrono-tz's actual compiled-in IANA table, and
// the zoned-date-time pair composes it with the instant bridge's own
// offset-date-time conversion.

impl Exchange<ReflectedNamedTimeZone, ProvenNamedTimeZoneCarrier<ChronoTz>, ChronoVerifier>
    for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedNamedTimeZone,
    ) -> Result<ProvenNamedTimeZoneCarrier<ChronoTz>, TemporalError> {
        let tz = named_time_zone_descriptor_to_chrono_tz(input.descriptor())?;
        let token = <ReflectedNamedTimeZone as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ProvenNamedTimeZoneCarrier::<ChronoTz>::new(
            ChronoTz::new(tz),
            token,
        ))
    }
}

impl Exchange<ProvenNamedTimeZoneCarrier<ChronoTz>, ReflectedNamedTimeZone, ChronoVerifier>
    for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenNamedTimeZoneCarrier<ChronoTz>,
    ) -> Result<ReflectedNamedTimeZone, TemporalError> {
        let descriptor = chrono_tz_to_named_time_zone_descriptor(**input.carrier())?;
        let token =
            <ProvenNamedTimeZoneCarrier<ChronoTz> as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ReflectedNamedTimeZone::new(descriptor, token))
    }
}

impl Exchange<ReflectedZonedDateTime, ProvenZonedDateTimeCarrier<ChronoZoned>, ChronoVerifier>
    for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedZonedDateTime,
    ) -> Result<ProvenZonedDateTimeCarrier<ChronoZoned>, TemporalError> {
        let zoned = zoned_date_time_descriptor_to_chrono(input.descriptor())?;
        let token = <ReflectedZonedDateTime as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ProvenZonedDateTimeCarrier::<ChronoZoned>::new(
            ChronoZoned::new(zoned),
            token,
        ))
    }
}

impl Exchange<ProvenZonedDateTimeCarrier<ChronoZoned>, ReflectedZonedDateTime, ChronoVerifier>
    for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<ChronoZoned>,
    ) -> Result<ReflectedZonedDateTime, TemporalError> {
        let descriptor = chrono_to_zoned_date_time_descriptor(**input.carrier())?;
        let token =
            <ProvenZonedDateTimeCarrier<ChronoZoned> as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ReflectedZonedDateTime::new(descriptor, token))
    }
}
