//! Temporal-value umbrella descriptors: the families that range over
//! every concrete date/time/interval form, plus the explicit-form and
//! grouped-unit value carriers.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    CalendarDateDescriptor, CenturyDescriptor, DateWithShiftDescriptor, DecadeDescriptor,
    ExplicitDateTimeDescriptor, ExplicitDateTimeWithShiftDescriptor, ExplicitDurationDescriptor,
    ExplicitTimeOfDayDescriptor, ExplicitTimeShiftDescriptor, ExtendedYearDescriptor,
    LocalDateTimeDescriptor, OffsetDateTimeDescriptor, OrdinalDateDescriptor,
    QualifiedTemporalExpressionDescriptor, ReducedCalendarDateDescriptor,
    SeasonalTemporalExpressionDescriptor, SubYearGroupingExpressionDescriptor,
    TimeOfDayWithShiftDescriptor, TimeScaleUnitDescriptor, TimeScaleUnitValueDescriptor,
    UnspecifiedComponentExpressionDescriptor, WeekDateDescriptor, ZonedDateTimeDescriptor,
};

/// The temporal-value forms that may appear at interval boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TemporalValueDescriptor {
    /// Calendar date form.
    CalendarDate(CalendarDateDescriptor),
    /// Reduced-precision calendar date form.
    ReducedCalendarDate(ReducedCalendarDateDescriptor),
    /// Gregorian calendar decade form.
    Decade(DecadeDescriptor),
    /// Gregorian calendar century form.
    Century(CenturyDescriptor),
    /// ISO 8601-2 extended year form.
    ExtendedYear(ExtendedYearDescriptor),
    /// Ordinal date form.
    OrdinalDate(OrdinalDateDescriptor),
    /// Week date form.
    WeekDate(WeekDateDescriptor),
    /// Date with time shift form.
    DateWithShift(DateWithShiftDescriptor),
    /// Time-of-day with time shift form.
    TimeOfDayWithShift(TimeOfDayWithShiftDescriptor),
    /// Local date-time form.
    LocalDateTime(LocalDateTimeDescriptor),
    /// Offset date-time form.
    OffsetDateTime(OffsetDateTimeDescriptor),
    /// Zoned date-time form.
    ZonedDateTime(ZonedDateTimeDescriptor),
    /// ISO 8601-2 Level 2 sub-year grouping expression.
    SubYearGrouping(SubYearGroupingExpressionDescriptor),
    /// Seasonal temporal expression.
    Seasonal(SeasonalTemporalExpressionDescriptor),
    /// Unspecified-component temporal expression.
    Unspecified(UnspecifiedComponentExpressionDescriptor),
}

/// A top-level temporal value plus an explicit ISO 8601-2 qualification
/// sidecar.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Getters,
    new,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
pub struct QualifiedTemporalValueDescriptor {
    /// Underlying temporal value.
    value: TemporalValueDescriptor,
    /// Explicit qualification metadata carried alongside the value.
    #[getter(copy)]
    qualification: QualifiedTemporalExpressionDescriptor,
}

/// A temporal-value carrier used where the standards permit either a bare
/// or an explicitly qualified value.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QualifiedOrBareTemporalValueDescriptor {
    /// A bare temporal value with no explicit qualification sidecar.
    Bare(TemporalValueDescriptor),
    /// A temporal value carrying an explicit qualification sidecar.
    Qualified(QualifiedTemporalValueDescriptor),
}

/// The explicit temporal values governed by CalConnect explicit-form
/// rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExplicitTemporalValueDescriptor {
    /// Explicit calendar date.
    CalendarDate(CalendarDateDescriptor),
    /// Explicit ordinal date.
    OrdinalDate(OrdinalDateDescriptor),
    /// Explicit week date.
    WeekDate(WeekDateDescriptor),
    /// Explicit Gregorian calendar decade.
    Decade(DecadeDescriptor),
    /// Explicit Gregorian calendar century.
    Century(CenturyDescriptor),
    /// Explicit duration representation.
    Duration(ExplicitDurationDescriptor),
    /// Explicit local time of day.
    TimeOfDay(ExplicitTimeOfDayDescriptor),
    /// Explicit time shift.
    TimeShift(ExplicitTimeShiftDescriptor),
    /// Explicit local date and time.
    DateTime(ExplicitDateTimeDescriptor),
    /// Explicit local date and time with time shift.
    DateTimeWithShift(ExplicitDateTimeWithShiftDescriptor),
    /// Explicit date with time shift.
    DateWithShift(DateWithShiftDescriptor),
    /// Explicit time of day with time shift.
    TimeOfDayWithShift(TimeOfDayWithShiftDescriptor),
}

/// Explicit temporal form metadata preserved across CalConnect exchanges.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitTemporalFormDescriptor {
    /// The explicit temporal value carried by the representation.
    value: ExplicitTemporalValueDescriptor,
    /// The lowest denoted component, which declares explicit precision.
    #[getter(copy)]
    precision: TimeScaleUnitDescriptor,
    /// Zero-valued components intentionally omitted from the lexical form.
    #[builder(default)]
    omitted_zero_components: Vec<TimeScaleUnitDescriptor>,
}

/// A grouped time-scale unit expression, including its coefficient and
/// trailing lower-order units.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Getters,
    Builder,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct GroupedTimeScaleUnitDescriptor {
    /// Coefficient attached to the grouped unit value.
    #[getter(copy)]
    coefficient: u32,
    /// One or more units inside the grouped `G...U` definition.
    #[builder(default)]
    grouped_units: Vec<TimeScaleUnitValueDescriptor>,
    /// Lower-order units that apply within the grouped unit.
    #[builder(default)]
    lower_order_units: Vec<TimeScaleUnitValueDescriptor>,
}

impl core::default::Default for TemporalValueDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CalendarDate(core::default::Default::default())
    }
}

impl core::default::Default for QualifiedOrBareTemporalValueDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Bare(core::default::Default::default())
    }
}

impl core::default::Default for ExplicitTemporalValueDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CalendarDate(core::default::Default::default())
    }
}
