//! Local/UTC-of-day/time-of-day-form clock descriptors.
//!
//! Time-of-day, UTC-offset, and time-scale descriptors: the wall-clock
//! half of the temporal accord, below the date/time-zone composites.

use derive_builder::Builder;
use derive_getters::Getters;

use crate::{
    FractionalSecondDescriptor, StandardTimeOfDayDescriptor, TimeScaleUnitFractionDescriptor,
    UtcOfDayDescriptor,
};

/// A local time-of-day.
///
/// Rust uses the shorter `LocalTime` name for the ISO 8601
/// local-time-of-day family.
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
pub struct LocalTimeDescriptor {
    /// Hour of day.
    #[getter(copy)]
    hour: u8,
    /// Minute of hour.
    #[getter(copy)]
    minute: u8,
    /// Second of minute.
    #[getter(copy)]
    second: u8,
    /// Fractional-second suffix, when present.
    #[builder(default)]
    fractional_second: Option<FractionalSecondDescriptor>,
}
/// A reduced-accuracy local time-of-day.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub enum ReducedLocalTimeDescriptor {
    /// Hour-only local time form.
    Hour {
        /// Hour of day.
        hour: u8,
        /// Decimal fraction attached to the hour component, when present.
        fractional_component: Option<TimeScaleUnitFractionDescriptor>,
    },
    /// Hour-minute local time form.
    HourMinute {
        /// Hour of day.
        hour: u8,
        /// Minute of hour.
        minute: u8,
        /// Decimal fraction attached to the minute component, when present.
        fractional_component: Option<TimeScaleUnitFractionDescriptor>,
    },
}
impl core::default::Default for ReducedLocalTimeDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Hour {
            hour: 0,
            fractional_component: None,
        }
    }
}

/// The time-of-day representation family carried by the ISO 8601 exchange
/// surface.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeDescriptor {
    /// Local clock time.
    Local(LocalTimeDescriptor),
    /// Reduced-accuracy local clock time.
    ReducedLocal(ReducedLocalTimeDescriptor),
    /// UTC-of-day form.
    Utc(UtcOfDayDescriptor),
    /// Standard-time-of-day form.
    Standard(StandardTimeOfDayDescriptor),
}
impl core::default::Default for TimeDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Local(core::default::Default::default())
    }
}
