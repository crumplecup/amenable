use crate::jiff::backend::instant::{
    jiff_civil_datetime_to_local_date_time_descriptor,
    local_date_time_descriptor_to_jiff_civil_datetime,
};
use crate::{JiffTimeBackend, JiffVerifier};
use amenable_core::{Exchange, Sidecar};
use amenable_time::{
    ProvenLocalDateTimeCarrier, ReflectedLocalDateTime, TemporalCivilProps, TemporalError,
};

// ── Phase 3: Civil ───────────────────────────────────────────────────

/// A [`jiff::civil::Date`] as a calendar (or ordinal) date carrier.
///
/// Reused for BOTH `TemporalCivilProps::CalendarDate` and `::OrdinalDate`
/// — the same real jiff type honestly backs both once resolved to a
/// concrete value, the same "one carrier, several associated-type
/// slots" pattern `amenable_std::StdSystemTime` already uses for its
/// own `Instant`/`OffsetDateTime`.
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
pub struct JiffDate(
    /// The wrapped date.
    jiff::civil::Date,
);

/// A [`jiff::civil::Time`] as a local time-of-day carrier.
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
pub struct JiffTime(
    /// The wrapped time.
    jiff::civil::Time,
);

/// A [`jiff::civil::DateTime`] as a local date-time carrier.
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
pub struct JiffDateTime(
    /// The wrapped date-time.
    jiff::civil::DateTime,
);

/// A [`jiff::civil::ISOWeekDate`] as a week-date carrier.
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
pub struct JiffISOWeekDate(
    /// The wrapped ISO week date.
    jiff::civil::ISOWeekDate,
);

/// A reduced-precision calendar date carrier.
///
/// `jiff::civil::Date` has no partial (year-only / year-month-only)
/// representation of its own — this is a real, meaningful shape
/// (mirroring `amenable_time::ReducedCalendarDateDescriptor`'s own two
/// forms exactly, using jiff's own `i16`/`i8` component widths), but its
/// conversion logic isn't wired to any `Exchange` edge yet: no
/// `Temporal*NativeBridge` trait in Phases 1-3's scope needs a
/// `ReducedCalendarDate` realize/reflect pair — only `TemporalParser`'s
/// own `RawInput -> ParsedReducedCalendarDate` edge (Phase 9) will.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffReducedCalendarDate {
    /// Year-only calendar date form.
    Year {
        /// Signed calendar year.
        year: i16,
    },
    /// Year-month calendar date form.
    YearMonth {
        /// Signed calendar year.
        year: i16,
        /// Calendar month number.
        month: i8,
    },
}

impl Default for JiffReducedCalendarDate {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Year { year: 0 }
    }
}

/// A reduced-precision local time-of-day carrier.
///
/// Same "real shape, not yet wired to an `Exchange` edge" status as
/// [`JiffReducedCalendarDate`] — and a genuinely harder case besides:
/// `amenable_time::ReducedLocalTimeDescriptor` allows a fractional
/// component on its own hour/minute field, which `jiff::civil::Time`
/// cannot represent at all (its finest fractional unit is the second).
/// This carrier's own conversion logic, whenever Phase 9 needs it, will
/// have to reject that case the same way `duration_descriptor_to_jiff_span`
/// already rejects a duration fraction on a coarser-than-seconds unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum JiffReducedLocalTime {
    /// Hour-only local time form.
    Hour {
        /// Hour of day.
        hour: i8,
    },
    /// Hour-minute local time form.
    HourMinute {
        /// Hour of day.
        hour: i8,
        /// Minute of hour.
        minute: i8,
    },
}

impl Default for JiffReducedLocalTime {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Hour { hour: 0 }
    }
}

impl TemporalCivilProps for JiffTimeBackend {
    type CalendarDate = JiffDate;
    type ReducedCalendarDate = JiffReducedCalendarDate;
    type OrdinalDate = JiffDate;
    type WeekDate = JiffISOWeekDate;
    type LocalTime = JiffTime;
    type ReducedLocalTime = JiffReducedLocalTime;
    type LocalDateTime = JiffDateTime;
}

// ── Civil native bridge ──────────────────────────────────────────────
//
// `TemporalCivilNativeBridge<JiffVerifier>` is the `realize_local_date_
// time` / `reflect_local_date_time` inverse pair. Both are real, and
// both now cover all three complete-date forms (calendar, ordinal,
// week) via `complete_date_descriptor_to_jiff_date` (in `instant`) — a
// genuine widening of Phase 2's own calendar-only scope, not a
// duplicate of it.

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
