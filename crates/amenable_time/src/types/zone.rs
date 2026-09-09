//! Named-time-zone, zone-transition-resolution, and RFC 9557 IXDTF
//! annotation descriptors.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;
use strum::EnumIter;

use crate::{OffsetDateTimeDescriptor, UtcOffsetDescriptor};

/// A named time-zone identity.
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
pub struct NamedTimeZoneDescriptor {
    /// IANA time-zone identifier.
    identifier: String,
    /// TZDB revision the producer associated with the interpretation, when present.
    #[builder(default)]
    tzdb_revision: Option<String>,
}

/// The declared authority for resolving an ambiguous repeated local
/// wall-clock time.
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
pub enum ZoneAmbiguityResolutionDescriptor {
    /// Prefer the earlier matching fixed instant.
    #[display("prefer-earlier")]
    #[default]
    PreferEarlier,
    /// Prefer the later matching fixed instant.
    #[display("prefer-later")]
    PreferLater,
}

/// The declared authority for resolving a skipped local wall-clock time
/// inside a gap.
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
pub enum ZoneGapResolutionDescriptor {
    /// Advance to the first valid instant after the gap.
    #[display("shift-forward")]
    #[default]
    ShiftForward,
    /// Retreat to the last valid instant before the gap.
    #[display("shift-backward")]
    ShiftBackward,
}

/// An explicit authority bundle for resolving local wall-clock timestamps
/// against a named zone.
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
    new,
    Default,
    amenable_derive::Evidence,
)]
#[evidence(basis = "Self")]
pub struct LocalTimeZoneResolutionAuthorityDescriptor {
    /// Authority for repeated local times during backward transitions.
    #[getter(copy)]
    ambiguity: ZoneAmbiguityResolutionDescriptor,
    /// Authority for skipped local times during forward transitions.
    #[getter(copy)]
    gap: ZoneGapResolutionDescriptor,
}

/// A zoned timestamp descriptor pairing a fixed-instant form with a named
/// zone.
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
#[builder(pattern = "owned", setter(into))]
pub struct ZonedDateTimeDescriptor {
    /// Offset timestamp representation.
    timestamp: OffsetDateTimeDescriptor,
    /// Named-zone identity.
    zone: NamedTimeZoneDescriptor,
}

/// An additional IXDTF annotation carried after the base RFC 3339
/// timestamp.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct IxdtfAnnotationDescriptor {
    /// Annotation key.
    key: String,
    /// One or more hyphen-delimited annotation values.
    #[builder(default)]
    values: Vec<String>,
    /// Whether the annotation is marked critical with a leading `!`.
    #[builder(default)]
    #[getter(copy)]
    critical: bool,
}

/// A preferred-presentation calendar annotation carried by RFC 9557
/// `u-ca`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct IxdtfCalendarAnnotationDescriptor {
    /// Unicode calendar identifier carried by the `u-ca` suffix key.
    identifier: String,
    /// Whether the annotation is marked critical with a leading `!`.
    #[builder(default)]
    #[getter(copy)]
    critical: bool,
}

/// The RFC 9557 time-zone annotation payload carried in an IXDTF suffix.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IxdtfTimeZoneAnnotationDescriptor {
    /// A named IANA time-zone identifier.
    Named(NamedTimeZoneDescriptor),
    /// An offset time-zone annotation used for compatibility.
    Offset(UtcOffsetDescriptor),
}

impl core::default::Default for IxdtfTimeZoneAnnotationDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Named(core::default::Default::default())
    }
}
