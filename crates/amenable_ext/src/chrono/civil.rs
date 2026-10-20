use crate::chrono::identity::{ChronoTimeBackend, ChronoVerifier};
use crate::temporal_fraction::{
    fractional_seconds_digits_to_nanos, nanos_to_fractional_seconds_digits,
};
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, FractionalSecondDescriptor,
    InvalidDescriptorSource, LeapSecondNotRepresentableSource, LocalDateTimeDescriptor,
    LocalDateTimeDescriptorBuilder, LocalTimeDescriptorBuilder, ProvenLocalDateTimeCarrier,
    ReflectedLocalDateTime, TemporalCivilProps, TemporalError, TemporalErrorKind,
};

// ── Carriers ─────────────────────────────────────────────────────────

/// A [`chrono::NaiveDate`] as a calendar, ordinal, or week date carrier.
///
/// One real chrono type backs all three complete-date slots once resolved to
/// a concrete date. This is the same "one carrier, several associated-type
/// slots" pattern the jiff backend uses for `jiff::civil::Date`.
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
pub struct ChronoDate(
    /// The wrapped date.
    chrono::NaiveDate,
);

/// A [`chrono::NaiveTime`] as a local time-of-day carrier.
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
pub struct ChronoTime(
    /// The wrapped time.
    chrono::NaiveTime,
);

/// A [`chrono::NaiveDateTime`] as a local date-time carrier.
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
pub struct ChronoDateTime(
    /// The wrapped date-time.
    chrono::NaiveDateTime,
);

/// A reduced-precision calendar date carrier.
///
/// chrono has no partial (year-only or year-month-only) date type, so this is
/// a thin wrapper with the same two forms as
/// `amenable_time::ReducedCalendarDateDescriptor`. It has no bridge Exchange
/// yet: the civil bridge does not cover it, and only the parser (Phase 12)
/// will need it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum ChronoReducedCalendarDate {
    /// Year-only calendar date form.
    Year {
        /// Signed calendar year.
        year: i32,
    },
    /// Year-month calendar date form.
    YearMonth {
        /// Signed calendar year.
        year: i32,
        /// Calendar month number.
        month: u8,
    },
}

impl Default for ChronoReducedCalendarDate {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Year { year: 0 }
    }
}

/// A reduced-precision local time-of-day carrier.
///
/// Same status as [`ChronoReducedCalendarDate`]: a real shape with no bridge
/// Exchange yet. chrono's `NaiveTime` also has no hour-only or hour-minute
/// form, and fractional hours and minutes are not representable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum ChronoReducedLocalTime {
    /// Hour-only local time form.
    Hour {
        /// Hour of day.
        hour: u8,
    },
    /// Hour-minute local time form.
    HourMinute {
        /// Hour of day.
        hour: u8,
        /// Minute of hour.
        minute: u8,
    },
}

impl Default for ChronoReducedLocalTime {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Hour { hour: 0 }
    }
}

// ── Props ────────────────────────────────────────────────────────────

impl TemporalCivilProps for ChronoTimeBackend {
    type CalendarDate = ChronoDate;
    type ReducedCalendarDate = ChronoReducedCalendarDate;
    type OrdinalDate = ChronoDate;
    type WeekDate = ChronoDate;
    type LocalTime = ChronoTime;
    type ReducedLocalTime = ChronoReducedLocalTime;
    type LocalDateTime = ChronoDateTime;
}

// ── Descriptor conversions ───────────────────────────────────────────

/// Build a [`TemporalError`] for a descriptor that chrono cannot represent.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn invalid(detail: String) -> TemporalError {
    TemporalError::new(TemporalErrorKind::InvalidDescriptor(
        InvalidDescriptorSource::new(detail),
    ))
}

/// Map an ISO weekday number (`1` = Monday, `7` = Sunday) to chrono's
/// weekday. chrono has no ISO-number constructor, so this is explicit.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
fn iso_weekday(number: u8) -> Result<chrono::Weekday, TemporalError> {
    match number {
        1 => Ok(chrono::Weekday::Mon),
        2 => Ok(chrono::Weekday::Tue),
        3 => Ok(chrono::Weekday::Wed),
        4 => Ok(chrono::Weekday::Thu),
        5 => Ok(chrono::Weekday::Fri),
        6 => Ok(chrono::Weekday::Sat),
        7 => Ok(chrono::Weekday::Sun),
        _ => Err(invalid("ISO weekday number is not in 1..=7".to_owned())),
    }
}

/// Resolve any of the three complete-date forms to a real [`chrono::NaiveDate`].
///
/// Calendar, ordinal, and week dates all resolve. Out-of-range components
/// return `Err`, never a silently clamped date.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(date)))]
pub(super) fn complete_date_descriptor_to_naive_date(
    date: CompleteDateDescriptor,
) -> Result<chrono::NaiveDate, TemporalError> {
    // Static messages, not `format!("{cal:?}/{ord:?}/{week:?} ...")`: a
    // `Debug` of these descriptors formats a signed, variable-width
    // `year` field, the same `Formatter::padding` cost confirmed
    // expensive elsewhere in this module for a symbolic value.
    match date {
        CompleteDateDescriptor::Calendar(cal) => chrono::NaiveDate::from_ymd_opt(
            cal.year(),
            u32::from(cal.month()),
            u32::from(cal.day()),
        )
        .ok_or_else(|| invalid("calendar date descriptor is not a real calendar date".to_owned())),
        CompleteDateDescriptor::Ordinal(ord) => {
            chrono::NaiveDate::from_yo_opt(ord.year(), u32::from(ord.day_of_year())).ok_or_else(
                || invalid("ordinal date descriptor is not a real ordinal date".to_owned()),
            )
        }
        CompleteDateDescriptor::Week(week) => chrono::NaiveDate::from_isoywd_opt(
            week.week_year(),
            u32::from(week.week()),
            iso_weekday(week.weekday())?,
        )
        .ok_or_else(|| invalid("week date descriptor is not a real ISO week date".to_owned())),
    }
}

/// Resolve a local date-time descriptor to a real [`chrono::NaiveDateTime`].
///
/// A leap second (second `60`) is rejected: chrono encodes a leap second as
/// nanoseconds past `999_999_999` on second `59`, and the descriptor form does
/// not map onto that. The rejection is an `Err`, not a silent adjustment.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn local_date_time_descriptor_to_naive(
    descriptor: &LocalDateTimeDescriptor,
) -> Result<chrono::NaiveDateTime, TemporalError> {
    let date = complete_date_descriptor_to_naive_date(descriptor.date())?;
    let time = descriptor.time();

    let nanos = match time.fractional_second() {
        None => 0,
        Some(fraction) => u32::try_from(fractional_seconds_digits_to_nanos(fraction.digits())?)
            .map_err(|_| invalid("fractional second does not fit chrono".to_owned()))?,
    };
    // Static messages, not `format!("...{err}/{time:?}...")`, for the
    // same reason as `complete_date_descriptor_to_naive_date` above.
    let naive_time = chrono::NaiveTime::from_hms_nano_opt(
        u32::from(time.hour()),
        u32::from(time.minute()),
        u32::from(time.second()),
        nanos,
    )
    .ok_or_else(|| {
        invalid("local time descriptor is not a real local time for chrono".to_owned())
    })?;

    Ok(chrono::NaiveDateTime::new(date, naive_time))
}

/// Reflect a [`chrono::NaiveDateTime`] as a local date-time descriptor, in
/// calendar form.
///
/// Leap seconds (chrono's nanoseconds past `999_999_999`) are rejected for the
/// same reason as the inverse direction.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(datetime)))]
pub fn naive_date_time_to_local_date_time_descriptor(
    datetime: chrono::NaiveDateTime,
) -> Result<LocalDateTimeDescriptor, TemporalError> {
    use chrono::{Datelike, Timelike};

    let nanos = datetime.nanosecond();
    if nanos >= 1_000_000_000 {
        return Err(TemporalError::new(
            TemporalErrorKind::LeapSecondNotRepresentable(LeapSecondNotRepresentableSource::new(
                nanos,
            )),
        ));
    }

    let date = datetime.date();
    // chrono's month/day/hour/minute/second are `u32`, always
    // 1..=12/1..=31/0..=23/0..=59/0..=59, so none of the narrowings or
    // builder calls below can genuinely fail for a real chrono value --
    // every `map_err` here is unreachable defensive code. Static
    // messages, not `format!("...{err}")`: `TryFromIntError`/the builder
    // errors carry no runtime-varying information beyond a fixed
    // description, so interpolating them was the real, confirmed cost
    // behind a CBMC timeout (a `Debug`-driven format recursion) while
    // adding nothing a static string doesn't already say.
    let calendar = CalendarDateDescriptor::new(
        date.year(),
        u8::try_from(date.month()).map_err(|_| invalid("month does not fit a u8".to_owned()))?,
        u8::try_from(date.day()).map_err(|_| invalid("day does not fit a u8".to_owned()))?,
    );

    let mut time_builder = LocalTimeDescriptorBuilder::default()
        .hour(
            u8::try_from(datetime.hour())
                .map_err(|_| invalid("hour does not fit a u8".to_owned()))?,
        )
        .minute(
            u8::try_from(datetime.minute())
                .map_err(|_| invalid("minute does not fit a u8".to_owned()))?,
        )
        .second(
            u8::try_from(datetime.second())
                .map_err(|_| invalid("second does not fit a u8".to_owned()))?,
        );
    if nanos != 0 {
        time_builder = time_builder.fractional_second(FractionalSecondDescriptor::new(
            nanos_to_fractional_seconds_digits(i64::from(nanos)),
        ));
    }
    let time = time_builder
        .build()
        .map_err(|_| invalid("could not build a local time descriptor".to_owned()))?;

    LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(calendar))
        .time(time)
        .build()
        .map_err(|_| invalid("could not build a local date-time descriptor".to_owned()))
}

// ── Civil native bridge ──────────────────────────────────────────────
//
// `TemporalCivilNativeBridge<ChronoVerifier>` is the `realize_local_date_time`
// / `reflect_local_date_time` inverse pair. Like the jiff backend's, the
// realize direction accepts all three complete-date forms, and the reflect
// direction always produces the calendar form.

impl Exchange<ReflectedLocalDateTime, ProvenLocalDateTimeCarrier<ChronoDateTime>, ChronoVerifier>
    for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ReflectedLocalDateTime,
    ) -> Result<ProvenLocalDateTimeCarrier<ChronoDateTime>, TemporalError> {
        let datetime = local_date_time_descriptor_to_naive(input.descriptor())?;
        let token = <ReflectedLocalDateTime as Sidecar<ChronoVerifier>>::sidecar(&input);
        Ok(ProvenLocalDateTimeCarrier::<ChronoDateTime>::new(
            ChronoDateTime::new(datetime),
            token,
        ))
    }
}

impl Exchange<ProvenLocalDateTimeCarrier<ChronoDateTime>, ReflectedLocalDateTime, ChronoVerifier>
    for ChronoTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenLocalDateTimeCarrier<ChronoDateTime>,
    ) -> Result<ReflectedLocalDateTime, TemporalError> {
        let descriptor = naive_date_time_to_local_date_time_descriptor(**input.carrier())?;
        let token =
            <ProvenLocalDateTimeCarrier<ChronoDateTime> as Sidecar<ChronoVerifier>>::sidecar(
                &input,
            );
        Ok(ReflectedLocalDateTime::new(descriptor, token))
    }
}
