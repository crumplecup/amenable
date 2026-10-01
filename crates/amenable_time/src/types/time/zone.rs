//! UTC-offset descriptors.
//!
//! Time-of-day, UTC-offset, and time-scale descriptors: the wall-clock
//! half of the temporal accord, below the date/time-zone composites.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use strum::EnumIter;

use crate::LocalTimeDescriptor;

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
#[derive(
    Debug,
    Clone,
    Copy,
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
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumIter,
    derive_more::Display,
    Default,
)]
pub enum UtcTimeScaleDescriptor {
    /// Coordinated Universal Time.
    #[display("UTC")]
    #[default]
    Utc,
}
/// A UTC-of-day descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new, Default)]
pub struct UtcOfDayDescriptor {
    /// Time-of-day payload carried on the UTC time scale.
    time: LocalTimeDescriptor,
    /// Explicit UTC time-scale identity.
    #[getter(copy)]
    scale: UtcTimeScaleDescriptor,
}
