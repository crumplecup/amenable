//! Precision, rounding, and serialization-profile descriptors for
//! temporal emitters.

use derive_builder::Builder;
use derive_getters::Getters;
use strum::EnumIter;

use crate::{
    IxdtfAnnotationDescriptor, IxdtfCalendarAnnotationDescriptor,
    IxdtfTimeZoneAnnotationDescriptor, OffsetDateTimeDescriptor, TemporalComponent,
};

/// The declared rounding mode for precision-reducing conversions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display)]
pub enum RoundingModeDescriptor {
    /// Truncate the discarded digits.
    #[display("truncate")]
    Truncate,
    /// Round to nearest, ties away from zero.
    #[display("half-up")]
    HalfUp,
    /// Round to nearest, ties to even.
    #[display("half-even")]
    HalfEven,
    /// Round toward positive infinity.
    #[display("ceiling")]
    Ceiling,
    /// Round toward negative infinity.
    #[display("floor")]
    Floor,
    /// Another implementation-defined rounding mode.
    #[display("other({_0})")]
    Other(String),
}

/// The declared precision target for temporal conversions or
/// serializations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct PrecisionDescriptor {
    /// Smallest retained temporal component.
    #[getter(copy)]
    smallest_component: TemporalComponent,
    /// Fractional-digit precision within the smallest component, when applicable.
    #[builder(default)]
    #[getter(copy)]
    fractional_digits: Option<u8>,
    /// Declared rounding mode for precision reduction, when present.
    #[builder(default)]
    rounding_mode: Option<RoundingModeDescriptor>,
}

/// The named serialization profiles exposed by the branch-level temporal
/// accord.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, EnumIter, derive_more::Display,
)]
pub enum SerializationProfile {
    /// ISO 8601 basic form, without separators.
    #[display("ISO 8601 basic")]
    Iso8601Basic,
    /// ISO 8601 extended form, with separators.
    #[display("ISO 8601 extended")]
    Iso8601Extended,
    /// The RFC 3339 Internet timestamp profile.
    #[display("RFC 3339")]
    Rfc3339,
    /// The RFC 9557 IXDTF profile.
    #[display("RFC 9557 IXDTF")]
    Ixdtf,
}

/// A neutral serialization descriptor for temporal emitters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct TemporalSerializationDescriptor {
    /// Governing serialization profile.
    #[getter(copy)]
    profile: SerializationProfile,
    /// Whether the representation uses basic rather than extended separators.
    #[builder(default)]
    #[getter(copy)]
    uses_basic_form: bool,
    /// Whether the representation uses an uppercase `T` between date and time.
    #[builder(default)]
    #[getter(copy)]
    uppercase_time_designator: bool,
    /// Whether the representation uses an uppercase `Z` for UTC.
    #[builder(default)]
    #[getter(copy)]
    uppercase_utc_designator: bool,
    /// Fractional-second digit count when a fractional second is present.
    #[builder(default)]
    #[getter(copy)]
    fractional_second_digits: Option<u8>,
    /// RFC 9557 time-zone annotation when the profile preserves one.
    #[builder(default)]
    time_zone_annotation: Option<IxdtfTimeZoneAnnotationDescriptor>,
    /// Preferred presentation calendar annotation when the profile preserves one.
    #[builder(default)]
    calendar_annotation: Option<IxdtfCalendarAnnotationDescriptor>,
    /// Additional IXDTF annotations carried by the serialization.
    #[builder(default)]
    additional_annotations: Vec<IxdtfAnnotationDescriptor>,
}

/// A neutral RFC 9557 IXDTF timestamp descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct IxdtfTimestampDescriptor {
    /// Base RFC 3339 timestamp payload.
    timestamp: OffsetDateTimeDescriptor,
    /// RFC 9557 time-zone annotation, when present.
    #[builder(default)]
    time_zone_annotation: Option<IxdtfTimeZoneAnnotationDescriptor>,
    /// Additional IXDTF suffix annotations in source order.
    #[builder(default)]
    additional_annotations: Vec<IxdtfAnnotationDescriptor>,
}
