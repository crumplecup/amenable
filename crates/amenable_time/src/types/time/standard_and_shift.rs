//! Standard-time and explicit-form time-of-day/time-shift descriptors.
//!
//! Time-of-day, UTC-offset, and time-scale descriptors: the wall-clock
//! half of the temporal accord, below the date/time-zone composites.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    FractionalSecondDescriptor, LocalTimeDescriptor, TimeScaleUnitDescriptor, UtcOffsetDescriptor,
    UtcOffsetSign, UtcTimeScaleDescriptor,
};

/// An explicit-form local time-of-day used by the CalConnect `timeE` family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitTimeOfDayDescriptor {
    /// Hour of day.
    #[getter(copy)]
    hour: u8,
    /// Minute of hour, when the representation is at least minute precision.
    #[builder(default)]
    #[getter(copy)]
    minute: Option<u8>,
    /// Second of minute, when the representation is at least second precision.
    #[builder(default)]
    #[getter(copy)]
    second: Option<u8>,
    /// Fractional-second suffix, when present.
    #[builder(default)]
    fractional_second: Option<FractionalSecondDescriptor>,
    /// Lowest denoted time-scale unit, which declares the explicit-form precision.
    #[getter(copy)]
    precision: TimeScaleUnitDescriptor,
    /// Zero-valued time units intentionally omitted from the lexical representation.
    #[builder(default)]
    omitted_zero_components: Vec<TimeScaleUnitDescriptor>,
}
/// A standard-time descriptor derived from UTC by a local shift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new, Default)]
pub struct StandardTimeDescriptor {
    /// Explicit UTC reference time scale from which the standard time is derived.
    #[getter(copy)]
    reference: UtcTimeScaleDescriptor,
    /// Constant local shift from UTC.
    #[getter(copy)]
    shift: UtcOffsetDescriptor,
}
/// A standard-time-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new, Default)]
pub struct StandardTimeOfDayDescriptor {
    /// Local wall-clock time of day.
    time: LocalTimeDescriptor,
    /// Standard-time scale carried alongside that wall-clock time.
    #[getter(copy)]
    scale: StandardTimeDescriptor,
}
/// The locally applicable time-scale family for a local wall-clock time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LocalTimeScaleDescriptor {
    /// Standard time derived from UTC by a local shift.
    Standard(StandardTimeDescriptor),
    /// Another locally applicable time scale not defined as a UTC-derived standard time.
    NonUtcBased,
}
impl core::default::Default for LocalTimeScaleDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Standard(core::default::Default::default())
    }
}

/// An explicit-form time-shift descriptor used by the CalConnect `shiftE`
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct ExplicitTimeShiftDescriptor {
    /// Shift sign. Positive or zero shifts omit an explicit plus sign in lexical form.
    #[builder(default)]
    #[getter(copy)]
    sign: UtcOffsetSign,
    /// Explicit time-of-day payload after the leading `Z` designator, when
    /// present. `None` denotes the bare `Z` form, which represents UTC
    /// with zero shift.
    #[builder(default)]
    time: Option<ExplicitTimeOfDayDescriptor>,
}
