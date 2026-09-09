//! Combined date-and-time descriptors: local, offset, and the CalConnect
//! explicit-form composites that pair a date or time with a time shift.

use derive_builder::Builder;
use derive_getters::Getters;

use crate::{
    CompleteDateDescriptor, ExplicitTimeOfDayDescriptor, ExplicitTimeShiftDescriptor,
    LocalTimeDescriptor, UtcOffsetDescriptor,
};

/// A combined complete ISO date and local clock time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct LocalDateTimeDescriptor {
    /// Complete date component.
    #[getter(copy)]
    date: CompleteDateDescriptor,
    /// Local clock-time component.
    time: LocalTimeDescriptor,
}

/// An offset-aware timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct OffsetDateTimeDescriptor {
    /// Local civil timestamp fields.
    local: LocalDateTimeDescriptor,
    /// UTC relationship carried by the timestamp.
    #[getter(copy)]
    offset: UtcOffsetDescriptor,
}

/// An explicit-form complete date plus explicit-form local time of day.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ExplicitDateTimeDescriptor {
    /// Complete date component.
    #[getter(copy)]
    date: CompleteDateDescriptor,
    /// Local time-of-day component, which may carry reduced precision.
    time: ExplicitTimeOfDayDescriptor,
}

/// An explicit-form complete date plus local time of day and time shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct ExplicitDateTimeWithShiftDescriptor {
    /// Complete local date-and-time component.
    local: ExplicitDateTimeDescriptor,
    /// Explicit time-shift component.
    shift: ExplicitTimeShiftDescriptor,
}

/// A complete explicit-form date carrying a time shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct DateWithShiftDescriptor {
    /// Complete date component.
    #[getter(copy)]
    date: CompleteDateDescriptor,
    /// Explicit time shift carried alongside the date.
    shift: ExplicitTimeShiftDescriptor,
}

/// A complete explicit-form local time of day carrying a time shift.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into))]
pub struct TimeOfDayWithShiftDescriptor {
    /// Explicit local time-of-day component.
    time: ExplicitTimeOfDayDescriptor,
    /// Explicit time shift carried alongside the time of day.
    shift: ExplicitTimeShiftDescriptor,
}
