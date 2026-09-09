//! Time-of-day, UTC-offset, and time-scale descriptors: the wall-clock
//! half of the temporal accord, below the date/time-zone composites.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use strum::EnumIter;

use crate::{FractionalSecondDescriptor, TimeScaleUnitDescriptor, TimeScaleUnitFractionDescriptor};

/// A local time-of-day.
///
/// Rust uses the shorter `LocalTime` name for the ISO 8601
/// local-time-of-day family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// An explicit-form local time-of-day used by the CalConnect `timeE` family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
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

/// The sign of a numeric UTC offset.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    EnumIter,
    derive_more::Display,
)]
pub enum UtcOffsetSign {
    /// Positive or east-of-UTC offset.
    #[default]
    #[display("+")]
    Positive,
    /// Negative or west-of-UTC offset.
    #[display("-")]
    Negative,
}

/// The semantic interpretation of a UTC-relationship payload in an
/// offset-aware timestamp.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    EnumIter,
    derive_more::Display,
)]
pub enum UtcOffsetRelationship {
    /// The local offset is known.
    #[default]
    #[display("known")]
    Known,
    /// RFC 9557-updated RFC 3339 unknown-local-offset semantics.
    #[display("unknown-local-offset")]
    UnknownLocalOffset,
}

/// A UTC-offset relationship descriptor for an offset-aware timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct UtcOffsetDescriptor {
    /// Offset sign when the UTC relationship is carried numerically.
    #[getter(copy)]
    sign: UtcOffsetSign,
    /// Absolute hour component for numeric-offset forms.
    #[getter(copy)]
    hours: u8,
    /// Absolute minute component when the offset is represented with
    /// hour-minute precision. `None` denotes the integral-hour form
    /// permitted by ISO 8601.
    #[builder(default)]
    #[getter(copy)]
    minutes: Option<u8>,
    /// Whether the relationship is a known local offset or the RFC
    /// 9557-updated unknown-offset case.
    #[builder(default)]
    #[getter(copy)]
    relationship: UtcOffsetRelationship,
}

/// A descriptor for the UTC reference time scale.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum UtcTimeScaleDescriptor {
    /// Coordinated Universal Time.
    #[display("UTC")]
    Utc,
}

/// A UTC-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct UtcOfDayDescriptor {
    /// Time-of-day payload carried on the UTC time scale.
    time: LocalTimeDescriptor,
    /// Explicit UTC time-scale identity.
    #[getter(copy)]
    scale: UtcTimeScaleDescriptor,
}

/// A standard-time descriptor derived from UTC by a local shift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
pub struct StandardTimeDescriptor {
    /// Explicit UTC reference time scale from which the standard time is derived.
    #[getter(copy)]
    reference: UtcTimeScaleDescriptor,
    /// Constant local shift from UTC.
    #[getter(copy)]
    shift: UtcOffsetDescriptor,
}

/// A standard-time-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new)]
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

/// An explicit-form time-shift descriptor used by the CalConnect `shiftE`
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
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
