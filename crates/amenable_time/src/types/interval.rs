//! ISO 8601 and CalConnect interval descriptors, plus the recurring-
//! interval wrapper.

use derive_builder::Builder;
use derive_getters::Getters;
use derive_new::new;

use crate::{
    DateWithShiftDescriptor, DurationDescriptor, ExplicitDateTimeDescriptor,
    ExplicitDateTimeWithShiftDescriptor, ExplicitDurationDescriptor,
    QualifiedOrBareTemporalValueDescriptor, TimeOfDayWithShiftDescriptor,
};

/// The CalConnect `[datetimeE]` endpoint family admitted at
/// explicit-interval boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExplicitTimeIntervalEndpointDescriptor {
    /// Explicit local date and time.
    DateTime(ExplicitDateTimeDescriptor),
    /// Explicit local date and time with time shift.
    DateTimeWithShift(ExplicitDateTimeWithShiftDescriptor),
    /// Explicit date with time shift.
    DateWithShift(DateWithShiftDescriptor),
    /// Explicit time of day with time shift.
    TimeOfDayWithShift(TimeOfDayWithShiftDescriptor),
}

/// The top-level CalConnect explicit time-interval representation form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ExplicitTimeIntervalRepresentation {
    /// Explicit start and end boundaries.
    StartEnd {
        /// Interval start boundary.
        start: ExplicitTimeIntervalEndpointDescriptor,
        /// Interval end boundary.
        end: ExplicitTimeIntervalEndpointDescriptor,
    },
    /// Explicit start boundary and an explicit duration.
    StartDuration {
        /// Interval start boundary.
        start: ExplicitTimeIntervalEndpointDescriptor,
        /// Interval duration.
        duration: ExplicitDurationDescriptor,
    },
    /// Explicit duration followed by an explicit end boundary.
    DurationEnd {
        /// Interval duration.
        duration: ExplicitDurationDescriptor,
        /// Interval end boundary.
        end: ExplicitTimeIntervalEndpointDescriptor,
    },
}

/// A neutral CalConnect explicit time-interval descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, new, Default)]
pub struct ExplicitTimeIntervalDescriptor {
    /// One of the legal CalConnect explicit interval top-level forms.
    representation: ExplicitTimeIntervalRepresentation,
}

/// An interval boundary representation, including explicit open or
/// unknown cases.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeIntervalEndpoint {
    /// A concrete temporal value.
    Value(QualifiedOrBareTemporalValueDescriptor),
    /// An explicit open boundary.
    Open,
    /// An explicit unknown boundary.
    Unknown,
}

/// The top-level interval representation form.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TimeIntervalRepresentation {
    /// Concrete start and end boundaries.
    StartEnd {
        /// Interval start boundary.
        start: TimeIntervalEndpoint,
        /// Interval end boundary.
        end: TimeIntervalEndpoint,
    },
    /// Concrete start boundary and a duration.
    StartDuration {
        /// Interval start boundary.
        start: TimeIntervalEndpoint,
        /// Interval duration.
        duration: DurationDescriptor,
    },
    /// Duration followed by a concrete end boundary.
    DurationEnd {
        /// Interval duration.
        duration: DurationDescriptor,
        /// Interval end boundary.
        end: TimeIntervalEndpoint,
    },
}

/// A neutral ISO 8601 interval descriptor.
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
pub struct TimeIntervalDescriptor {
    /// One of the legal ISO 8601 interval top-level forms.
    representation: TimeIntervalRepresentation,
}

/// A neutral ISO 8601 recurring interval descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Getters, Builder, Default)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct RecurringIntervalDescriptor {
    /// Bounded repetition count; `None` denotes unbounded recurrence.
    #[builder(default)]
    #[getter(copy)]
    repetitions: Option<u32>,
    /// Repeated interval payload.
    interval: TimeIntervalDescriptor,
}

impl core::default::Default for ExplicitTimeIntervalEndpointDescriptor {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::DateTime(core::default::Default::default())
    }
}

impl core::default::Default for ExplicitTimeIntervalRepresentation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::StartEnd {
            start: core::default::Default::default(),
            end: core::default::Default::default(),
        }
    }
}

impl core::default::Default for TimeIntervalEndpoint {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Value(core::default::Default::default())
    }
}

impl core::default::Default for TimeIntervalRepresentation {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::StartEnd {
            start: core::default::Default::default(),
            end: core::default::Default::default(),
        }
    }
}
