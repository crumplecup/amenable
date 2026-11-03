use crate::chrono::civil::local_date_time_descriptor_to_naive;
use crate::chrono::civil_reflect::naive_date_time_to_local_date_time_descriptor;
use crate::chrono::identity::{ChronoTimeBackend, ChronoVerifier};
use crate::chrono::utc_offset::utc_offset_descriptor_to_fixed_offset;
use crate::chrono::utc_offset_reflect::fixed_offset_to_utc_offset_descriptor;
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    NoUniqueLocalMappingSource, OffsetDateTimeBuilderFailedSource, OffsetDateTimeDescriptor,
    OffsetDateTimeDescriptorBuilder, ProvenOffsetDateTimeCarrier, ReflectedOffsetDateTime,
    TemporalError, TemporalErrorKind, TemporalInstantProps,
};

// ── Carriers ─────────────────────────────────────────────────────────

/// A [`chrono::FixedOffset`] as a UTC-offset carrier.
///
/// `chrono::FixedOffset` derives only `PartialEq, Eq, Hash, Copy, Clone`
/// (confirmed by reading chrono's real source) — no `Default`. Same
/// `basis_ctor` need as the jiff backend's `JiffOffset`: this wrapper
/// supplies `FixedOffset::east_opt(0).unwrap()` (a real, concrete zero
/// offset) as its root basis instead.
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
    basis_ctor = "Self(chrono::FixedOffset::east_opt(0).expect(\"0 is in FixedOffset's representable range\"))"
)]
pub struct ChronoOffset(
    /// The wrapped offset.
    chrono::FixedOffset,
);

/// A [`chrono::DateTime<chrono::FixedOffset>`] as an offset-aware
/// date-time carrier.
///
/// Unlike jiff (which has no first-class "offset date-time" type and
/// needs a small composite carrier), chrono's `DateTime<FixedOffset>` is
/// already exactly this: a local date-time paired with the fixed offset
/// it was recorded at. `DateTime<FixedOffset>` has a real `Default`
/// (confirmed in source: `FixedOffset::west_opt(0).unwrap().from_utc_
/// datetime(&NaiveDateTime::default())`), so no `basis_ctor` is needed.
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
pub struct ChronoOffsetDateTime(
    /// The wrapped offset date-time.
    chrono::DateTime<chrono::FixedOffset>,
);

/// A [`chrono::DateTime<chrono::Utc>`] as a fixed-instant carrier.
///
/// `DateTime<Utc>` has a real `Default` (confirmed in source), so no
/// `basis_ctor` is needed, the same as `ChronoOffsetDateTime`.
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
pub struct ChronoInstant(
    /// The wrapped instant.
    chrono::DateTime<chrono::Utc>,
);

// ── Props ────────────────────────────────────────────────────────────

impl TemporalInstantProps for ChronoTimeBackend {
    type UtcOffset = ChronoOffset;
    type OffsetDateTime = ChronoOffsetDateTime;
    type Instant = ChronoInstant;
}

// ── Composition ──────────────────────────────────────────────────────

/// Resolve a full offset date-time descriptor to a real
/// `chrono::DateTime<chrono::FixedOffset>`.
///
/// Reuses the civil bridge's own local-date-time conversion for the local
/// half. `NaiveDateTime::and_local_timezone` is the real, non-deprecated
/// way to attach an offset (`DateTime::from_local` was deprecated in
/// 0.4.27) — confirmed via chrono's real source that `FixedOffset`'s own
/// `TimeZone::offset_from_local_datetime` always returns `MappedLocalTime::
/// Single`, since a fixed offset has no ambiguity or gap to report, so the
/// `None` arm below is unreachable in practice but still a real `Err`
/// rather than an unwrap.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub fn offset_date_time_descriptor_to_chrono(
    descriptor: &OffsetDateTimeDescriptor,
) -> Result<chrono::DateTime<chrono::FixedOffset>, TemporalError> {
    let local = local_date_time_descriptor_to_naive(descriptor.local())?;
    let offset = utc_offset_descriptor_to_fixed_offset(descriptor.offset())?;
    // `NoUniqueLocalMappingSource` carries no payload beyond
    // provenance -- see that type's own doc comment: this branch is
    // genuinely unreachable for `FixedOffset` (confirmed above: a
    // fixed offset has no ambiguity or gap to report), so there's no
    // real value to add, and formatting either `local`/`offset`
    // symbolically was separately confirmed Kani-expensive for the
    // same `Formatter::padding` reason.
    local
        .and_local_timezone(offset)
        .single()
        .ok_or_else(|| TemporalError::new(TemporalErrorKind::NoUniqueLocalMapping(
            NoUniqueLocalMappingSource::new(),
        )))
}

/// The inverse of [`offset_date_time_descriptor_to_chrono`].
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(datetime)))]
pub fn chrono_to_offset_date_time_descriptor(
    datetime: chrono::DateTime<chrono::FixedOffset>,
) -> Result<OffsetDateTimeDescriptor, TemporalError> {
    let local = naive_date_time_to_local_date_time_descriptor(datetime.naive_local())?;
    let offset = fixed_offset_to_utc_offset_descriptor(*datetime.offset())?;
    // Both required fields (`local`/`offset`) are always set above, so
    // this can't genuinely fail -- a static message, not
    // `format!("...{err}")`, for the same reason given in
    // `fixed_offset_to_utc_offset_descriptor`.
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::OffsetDateTimeBuilderFailed(
                OffsetDateTimeBuilderFailedSource::new(err),
            ))
        })
}

// ── Instant native bridge ────────────────────────────────────────────
//
// `TemporalInstantNativeBridge<ChronoVerifier>` is the `realize_offset_
// date_time` / `reflect_offset_date_time` inverse pair. `UtcOffset` and
// `Instant` are plain associated types with no Exchange obligation of
// their own in this trait (see its definition in amenable_time).

impl
    Exchange<
        ReflectedOffsetDateTime,
        ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>,
        ChronoVerifier,
    > for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedOffsetDateTime,
    ) -> Result<ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>, TemporalError> {
        let datetime = offset_date_time_descriptor_to_chrono(input.descriptor())?;
        let token = <ReflectedOffsetDateTime as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ProvenOffsetDateTimeCarrier::<ChronoOffsetDateTime>::new(
            ChronoOffsetDateTime::new(datetime),
            token,
        ))
    }
}

impl
    Exchange<
        ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>,
        ReflectedOffsetDateTime,
        ChronoVerifier,
    > for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime>,
    ) -> Result<ReflectedOffsetDateTime, TemporalError> {
        let descriptor = chrono_to_offset_date_time_descriptor(**input.carrier())?;
        let token = <ProvenOffsetDateTimeCarrier<ChronoOffsetDateTime> as Sidecar<
            ChronoVerifier,
        >>::sidecar(&input);
        Ok(ReflectedOffsetDateTime::new(descriptor, token))
    }
}
