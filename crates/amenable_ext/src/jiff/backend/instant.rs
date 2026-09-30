use super::types::{JiffDateTime, JiffTimeBackend, JiffVerifier};
use crate::jiff::backend::duration::fractional_seconds_digits_to_nanos;
use crate::jiff::backend::duration::nanos_to_fractional_seconds_digits;
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, FractionalSecondDescriptor,
    InvalidDescriptorSource, LocalDateTimeDescriptor, LocalDateTimeDescriptorBuilder,
    LocalTimeDescriptorBuilder, OffsetDateTimeDescriptor, OffsetDateTimeDescriptorBuilder,
    ProvenLocalDateTimeCarrier, ProvenOffsetDateTimeCarrier, ReflectedLocalDateTime,
    ReflectedOffsetDateTime, TemporalError, TemporalErrorKind, TemporalInstantProps,
    UnsupportedSource, UtcOffsetDescriptor, UtcOffsetDescriptorBuilder, UtcOffsetRelationship,
    UtcOffsetSign,
};

// ── Phase 2: Instant ─────────────────────────────────────────────────

/// A [`jiff::Timestamp`] as a fixed-instant carrier.
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
pub struct JiffTimestamp(
    /// The wrapped instant.
    jiff::Timestamp,
);

/// A [`jiff::tz::Offset`] as a UTC-offset carrier.
///
/// `jiff::tz::Offset` derives only `Clone, Copy, Eq, Hash, PartialEq,
/// PartialOrd, Ord` — confirmed by reading jiff's real source, there is
/// deliberately no `Default` (unlike `jiff::civil::DateTime`, which has
/// a manual one via `DateTime::ZERO`). `#[derive(amenable_derive::
/// Evidence)]`'s own `basis = "Self"` mode needs `Self: Default` UNLESS
/// a `basis_ctor` is given explicitly — confirmed by reading the real
/// macro expansion, which only adds the `Default` where-bound when
/// `basis_ctor` is absent — so this wrapper supplies `Offset::UTC` (a
/// real, concrete jiff constant) as its root basis instead of relying
/// on a `Default` the wrapped type doesn't have.
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
#[evidence(basis = "Self", basis_ctor = "Self(jiff::tz::Offset::UTC)")]
pub struct JiffOffset(
    /// The wrapped offset.
    jiff::tz::Offset,
);

/// A real jiff-backed offset-aware date-time carrier: a local civil
/// date-time paired with the UTC offset it was recorded at.
///
/// `jiff` has no first-class "offset date-time" type of its own the way
/// the `time` crate does — `jiff::Zoned` always carries a full
/// `jiff::tz::TimeZone`, not a bare offset. This small composite is the
/// natural fit instead, built from two carriers jiff already supports
/// directly. Same `basis_ctor` need as `JiffOffset`'s own doc comment
/// documents (its `offset` field has no `Default`), supplied here as
/// `DateTime::ZERO` + `Offset::UTC`, both real jiff constants.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_getters::Getters,
    derive_new::new,
)]
#[evidence(
    basis = "Self",
    basis_ctor = "Self { local: jiff::civil::DateTime::ZERO, offset: jiff::tz::Offset::UTC }"
)]
pub struct JiffOffsetDateTime {
    /// The local (wall-clock) civil date-time.
    local: jiff::civil::DateTime,
    /// The UTC offset it was recorded at.
    offset: jiff::tz::Offset,
}

impl TemporalInstantProps for JiffTimeBackend {
    type UtcOffset = JiffOffset;
    type OffsetDateTime = JiffOffsetDateTime;
    type Instant = JiffTimestamp;
}

// ── Real conversions to/from jiff's civil/offset types ──────────────

/// Resolve a UTC-offset descriptor to a real `jiff::tz::Offset`.
///
/// Real, honest scoping: the RFC 9557-updated "unknown local offset"
/// relationship (`-00:00`) has no fixed-offset representation at all —
/// `jiff::tz::Offset` is always a concrete, known value — so that case
/// is a real `Unsupported` error, never silently coerced to `Offset::UTC`
/// or any other default.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn utc_offset_descriptor_to_jiff_offset(
    descriptor: UtcOffsetDescriptor,
) -> Result<jiff::tz::Offset, TemporalError> {
    if descriptor.relationship() == UtcOffsetRelationship::UnknownLocalOffset {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(
                "jiff::tz::Offset has no representation for the RFC 9557 unknown-local-offset case"
                    .to_owned(),
            ),
        )));
    }

    let magnitude =
        i32::from(descriptor.hours()) * 3600 + i32::from(descriptor.minutes().unwrap_or(0)) * 60;
    let seconds = match descriptor.sign() {
        UtcOffsetSign::Positive => magnitude,
        UtcOffsetSign::Negative => -magnitude,
    };

    jiff::tz::Offset::from_seconds(seconds).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "offset out of jiff::tz::Offset's representable range: {err}"
            )),
        ))
    })
}

/// Decompose a real `jiff::tz::Offset` back into a UTC-offset
/// descriptor.
///
/// Canonicalizes to the integral-hour form (`minutes: None`) whenever
/// the offset is an exact whole number of hours — `jiff::tz::Offset`
/// itself carries no trace of whether the original descriptor stated
/// minutes explicitly as `0` or omitted them, so this is the one
/// faithful choice, matching `UtcOffsetDescriptor`'s own documented
/// convention ("`None` denotes the integral-hour form").
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset)))]
pub(super) fn jiff_offset_to_utc_offset_descriptor(
    offset: jiff::tz::Offset,
) -> Result<UtcOffsetDescriptor, TemporalError> {
    let seconds = offset.seconds();
    let sign = if seconds < 0 {
        UtcOffsetSign::Negative
    } else {
        UtcOffsetSign::Positive
    };
    let magnitude = seconds.unsigned_abs();
    let hours = u8::try_from(magnitude / 3600).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "offset's hour component does not fit the descriptor's u8 component: {err}"
            )),
        ))
    })?;
    let remainder_minutes = (magnitude % 3600) / 60;

    let mut builder = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(hours);
    if remainder_minutes != 0 {
        let minutes = u8::try_from(remainder_minutes).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "offset's minute component does not fit the descriptor's u8 component: {err}"
                )),
            ))
        })?;
        builder = builder.minutes(minutes);
    }

    builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!("could not build a UTC offset descriptor: {err}")),
        ))
    })
}

/// Resolve a complete-date descriptor (calendar, ordinal, or week form)
/// to a real `jiff::civil::Date`.
///
/// All three forms are real, using jiff's own real cross-representation
/// support: `Date::new` directly for the calendar form;
/// `Date::new(year, 1, 1).with().day_of_year(n).build()` for the
/// ordinal form (confirmed via jiff's real source — this is the
/// documented, correct way to construct a date from its ordinal day,
/// not a hand-rolled day-count calculation); `Weekday::
/// from_monday_one_offset` (the same ISO weekday-number convention
/// already checked as a real witness for `civil::Weekday`) +
/// `ISOWeekDate::new(..).date()` for the week form.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(date)))]
pub(super) fn complete_date_descriptor_to_jiff_date(
    date: CompleteDateDescriptor,
) -> Result<jiff::civil::Date, TemporalError> {
    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "{field} does not fit jiff::civil::Date's component range: {err}"
            )),
        ))
    };
    let range_err = |err: jiff::Error| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "date out of jiff::civil::Date's representable range: {err}"
            )),
        ))
    };

    match date {
        CompleteDateDescriptor::Calendar(cal) => {
            let year = i16::try_from(cal.year()).map_err(|e| out_of_range("year", e))?;
            let month = i8::try_from(cal.month()).map_err(|e| out_of_range("month", e))?;
            let day = i8::try_from(cal.day()).map_err(|e| out_of_range("day", e))?;
            jiff::civil::Date::new(year, month, day).map_err(range_err)
        }
        CompleteDateDescriptor::Ordinal(ord) => {
            let year = i16::try_from(ord.year()).map_err(|e| out_of_range("year", e))?;
            let day_of_year =
                i16::try_from(ord.day_of_year()).map_err(|e| out_of_range("day_of_year", e))?;
            jiff::civil::Date::new(year, 1, 1)
                .map_err(range_err)?
                .with()
                .day_of_year(day_of_year)
                .build()
                .map_err(range_err)
        }
        CompleteDateDescriptor::Week(week) => {
            let week_year =
                i16::try_from(week.week_year()).map_err(|e| out_of_range("week_year", e))?;
            let week_number = i8::try_from(week.week()).map_err(|e| out_of_range("week", e))?;
            let weekday_number =
                i8::try_from(week.weekday()).map_err(|e| out_of_range("weekday", e))?;
            let weekday =
                jiff::civil::Weekday::from_monday_one_offset(weekday_number).map_err(range_err)?;
            jiff::civil::ISOWeekDate::new(week_year, week_number, weekday)
                .map(jiff::civil::ISOWeekDate::date)
                .map_err(range_err)
        }
    }
}

/// Resolve a local date-time descriptor to a real `jiff::civil::DateTime`.
///
/// Any of the three complete-date forms (calendar, ordinal, week) —
/// see [`complete_date_descriptor_to_jiff_date`].
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn local_date_time_descriptor_to_jiff_civil_datetime(
    descriptor: &LocalDateTimeDescriptor,
) -> Result<jiff::civil::DateTime, TemporalError> {
    let date = complete_date_descriptor_to_jiff_date(descriptor.date())?;
    let time = descriptor.time();

    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "{field} does not fit jiff::civil::Time's component range: {err}"
            )),
        ))
    };
    let hour = i8::try_from(time.hour()).map_err(|e| out_of_range("hour", e))?;
    let minute = i8::try_from(time.minute()).map_err(|e| out_of_range("minute", e))?;
    let second = i8::try_from(time.second()).map_err(|e| out_of_range("second", e))?;
    let subsec_nanosecond = match time.fractional_second() {
        None => 0,
        Some(fraction) => i32::try_from(fractional_seconds_digits_to_nanos(fraction.digits())?)
            .map_err(|e| out_of_range("fractional second", e))?,
    };
    let time = jiff::civil::Time::new(hour, minute, second, subsec_nanosecond).map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "local time out of jiff::civil::Time's representable range: {err}"
            )),
        ))
    })?;

    Ok(jiff::civil::DateTime::from_parts(date, time))
}

/// Decompose a real `jiff::civil::DateTime` back into a local date-time
/// descriptor — always as a complete CALENDAR date, regardless of which
/// of the three forms [`local_date_time_descriptor_to_jiff_civil_datetime`]
/// originally produced it: `jiff::civil::Date` carries no trace of
/// having been constructed via its ordinal or week-date cross-
/// representation, so calendar form is the one canonical, always-
/// available choice (the same precision-is-not-preserved reasoning
/// `jiff_offset_to_utc_offset_descriptor`'s own doc comment documents
/// for offset minutes).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(datetime)))]
pub(super) fn jiff_civil_datetime_to_local_date_time_descriptor(
    datetime: jiff::civil::DateTime,
) -> Result<LocalDateTimeDescriptor, TemporalError> {
    let out_of_range = |field: &str, err: std::num::TryFromIntError| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!(
                "datetime's {field} does not fit the descriptor's component range: {err}"
            )),
        ))
    };
    let year = i32::from(datetime.year());
    let month = u8::try_from(datetime.month()).map_err(|e| out_of_range("month", e))?;
    let day = u8::try_from(datetime.day()).map_err(|e| out_of_range("day", e))?;
    let hour = u8::try_from(datetime.hour()).map_err(|e| out_of_range("hour", e))?;
    let minute = u8::try_from(datetime.minute()).map_err(|e| out_of_range("minute", e))?;
    let second = u8::try_from(datetime.second()).map_err(|e| out_of_range("second", e))?;
    let nanos = datetime.subsec_nanosecond();

    let mut time_builder = LocalTimeDescriptorBuilder::default()
        .hour(hour)
        .minute(minute)
        .second(second);
    if nanos != 0 {
        time_builder = time_builder.fractional_second(FractionalSecondDescriptor::new(
            nanos_to_fractional_seconds_digits(i64::from(nanos)),
        ));
    }
    let time = time_builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::InvalidDescriptor(
            InvalidDescriptorSource::new(format!("could not build a local time descriptor: {err}")),
        ))
    })?;

    LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(time)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build a local date-time descriptor: {err}"
                )),
            ))
        })
}

/// Resolve a full offset date-time descriptor to its real jiff parts —
/// factored out of the `TemporalInstantNativeBridge` realize body so
/// Phase 4's own `ZonedDateTime` realize (which needs the identical
/// local + offset pair, pinned to a concrete instant before re-attaching
/// a real named zone) can reuse it rather than duplicating it.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn offset_date_time_descriptor_to_jiff_parts(
    descriptor: &OffsetDateTimeDescriptor,
) -> Result<(jiff::civil::DateTime, jiff::tz::Offset), TemporalError> {
    let local = local_date_time_descriptor_to_jiff_civil_datetime(descriptor.local())?;
    let offset = utc_offset_descriptor_to_jiff_offset(descriptor.offset())?;
    Ok((local, offset))
}

/// The inverse of [`offset_date_time_descriptor_to_jiff_parts`] —
/// factored out for the same reuse reason.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(local, offset)))]
pub(super) fn jiff_parts_to_offset_date_time_descriptor(
    local: jiff::civil::DateTime,
    offset: jiff::tz::Offset,
) -> Result<OffsetDateTimeDescriptor, TemporalError> {
    let local = jiff_civil_datetime_to_local_date_time_descriptor(local)?;
    let offset = jiff_offset_to_utc_offset_descriptor(offset)?;
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build an offset date-time descriptor: {err}"
                )),
            ))
        })
}

// ── Instant native bridge ────────────────────────────────────────────
//
// `TemporalInstantNativeBridge<JiffVerifier>` is the `realize_offset_
// date_time` / `reflect_offset_date_time` inverse pair. Both are real:
// they carry an offset-aware timestamp descriptor through an actual
// `jiff::civil::DateTime` + `jiff::tz::Offset` pair and back.

impl
    Exchange<ReflectedOffsetDateTime, ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedOffsetDateTime,
    ) -> Result<ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, TemporalError> {
        let (local, offset) = offset_date_time_descriptor_to_jiff_parts(input.descriptor())?;
        let token = <ReflectedOffsetDateTime as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenOffsetDateTimeCarrier::<JiffOffsetDateTime>::new(
            JiffOffsetDateTime { local, offset },
            token,
        ))
    }
}

impl
    Exchange<ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, ReflectedOffsetDateTime, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
    ) -> Result<ReflectedOffsetDateTime, TemporalError> {
        let carrier = input.carrier();
        let descriptor = jiff_parts_to_offset_date_time_descriptor(carrier.local, carrier.offset)?;
        let token =
            <ProvenOffsetDateTimeCarrier<JiffOffsetDateTime> as Sidecar<JiffVerifier>>::sidecar(
                &input,
            );
        Ok(ReflectedOffsetDateTime::new(descriptor, token))
    }
}

// ── Civil native bridge ──────────────────────────────────────────────
//
// `TemporalCivilNativeBridge<JiffVerifier>` is the `realize_local_date_
// time` / `reflect_local_date_time` inverse pair. Both are real, and
// both now cover all three complete-date forms (calendar, ordinal,
// week) via `complete_date_descriptor_to_jiff_date` above — a genuine
// widening of Phase 2's own calendar-only scope, not a duplicate of it.

impl Exchange<ReflectedLocalDateTime, ProvenLocalDateTimeCarrier<JiffDateTime>, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedLocalDateTime,
    ) -> Result<ProvenLocalDateTimeCarrier<JiffDateTime>, TemporalError> {
        let datetime = local_date_time_descriptor_to_jiff_civil_datetime(input.descriptor())?;
        let token = <ReflectedLocalDateTime as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ProvenLocalDateTimeCarrier::<JiffDateTime>::new(
            JiffDateTime::new(datetime),
            token,
        ))
    }
}

impl Exchange<ProvenLocalDateTimeCarrier<JiffDateTime>, ReflectedLocalDateTime, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenLocalDateTimeCarrier<JiffDateTime>,
    ) -> Result<ReflectedLocalDateTime, TemporalError> {
        let descriptor = jiff_civil_datetime_to_local_date_time_descriptor(**input.carrier())?;
        let token =
            <ProvenLocalDateTimeCarrier<JiffDateTime> as Sidecar<JiffVerifier>>::sidecar(&input);
        Ok(ReflectedLocalDateTime::new(descriptor, token))
    }
}
