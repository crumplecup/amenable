//! Neutral-descriptor validation error sources, shared across backends.

use derive_getters::Getters;
use derive_more::{Display, Error};

use crate::{CalendarDateDescriptor, OrdinalDateDescriptor, TemporalComponent, WeekDateDescriptor};

/// A temporal component's real value does not fit in a `u8`.
///
/// Holds the real `component` and `value` rather than a pre-formatted
/// `detail: String`: this is the same closed, small set of causes
/// (`TemporalComponent::{Month, Day, Hour, Minute, Second}`) that
/// `InvalidDescriptorSource`'s free-text `detail` field used to paper
/// over at five separate call sites with five hand-written strings
/// that didn't even carry the offending value. One variant, real
/// structured data, matching `LeapSecondNotRepresentableSource`'s own
/// precedent just above.
#[derive(Debug, Display, Error, Getters)]
#[display("{component} value {value} does not fit a u8")]
pub struct ComponentOutOfRangeSource {
    /// Which temporal component overflowed.
    #[error(ignore)]
    component: TemporalComponent,
    /// The real value that didn't fit.
    #[error(ignore)]
    value: u32,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ComponentOutOfRangeSource {
    /// Record a component-range overflow, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(component)))]
    pub fn new(component: TemporalComponent, value: u32) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            component,
            value,
            line,
            file,
        }
    }
}

/// A raw ISO weekday number is outside `1..=7` (`1` = Monday, `7` =
/// Sunday).
#[derive(Debug, Display, Error, Getters)]
#[display("ISO weekday number {number} is not in 1..=7")]
pub struct InvalidIsoWeekdaySource {
    /// The out-of-range number that was given.
    #[error(ignore)]
    number: u8,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidIsoWeekdaySource {
    /// Record an out-of-range ISO weekday number, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(number: u8) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { number, line, file }
    }
}

/// A calendar-date descriptor's year, month, and day do not combine
/// into a real calendar date.
///
/// Holds the rejected [`CalendarDateDescriptor`] itself rather than a
/// `detail: String`: the descriptor is already a real, named, `Copy`
/// value, so there is no reason to re-describe it in prose.
#[derive(Debug, Display, Error, Getters)]
#[display("calendar date descriptor {descriptor:?} is not a real calendar date")]
pub struct InvalidCalendarDateSource {
    /// The rejected descriptor.
    #[error(ignore)]
    descriptor: CalendarDateDescriptor,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidCalendarDateSource {
    /// Record a rejected calendar date descriptor, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
    pub fn new(descriptor: CalendarDateDescriptor) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            descriptor,
            line,
            file,
        }
    }
}

/// An ordinal-date descriptor's year and day-of-year do not combine
/// into a real calendar date. Holds the rejected
/// [`OrdinalDateDescriptor`] itself, same rationale as
/// [`InvalidCalendarDateSource`].
#[derive(Debug, Display, Error, Getters)]
#[display("ordinal date descriptor {descriptor:?} is not a real ordinal date")]
pub struct InvalidOrdinalDateSource {
    /// The rejected descriptor.
    #[error(ignore)]
    descriptor: OrdinalDateDescriptor,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidOrdinalDateSource {
    /// Record a rejected ordinal date descriptor, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
    pub fn new(descriptor: OrdinalDateDescriptor) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            descriptor,
            line,
            file,
        }
    }
}

/// A week-date descriptor's week-year, week, and weekday do not
/// combine into a real ISO week date. Holds the rejected
/// [`WeekDateDescriptor`] itself, same rationale as
/// [`InvalidCalendarDateSource`].
#[derive(Debug, Display, Error, Getters)]
#[display("week date descriptor {descriptor:?} is not a real ISO week date")]
pub struct InvalidWeekDateSource {
    /// The rejected descriptor.
    #[error(ignore)]
    descriptor: WeekDateDescriptor,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidWeekDateSource {
    /// Record a rejected week date descriptor, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
    pub fn new(descriptor: WeekDateDescriptor) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            descriptor,
            line,
            file,
        }
    }
}

/// A fractional-second nanosecond count does not fit a `u32`.
///
/// Distinct from [`ComponentOutOfRangeSource`]: `TemporalComponent`
/// has no nanosecond-level member, and this narrows an `i64`, not a
/// `u32`.
#[derive(Debug, Display, Error, Getters)]
#[display("fractional-second nanosecond count {nanos} does not fit a u32")]
pub struct NanosecondOutOfRangeSource {
    /// The real nanosecond count that didn't fit.
    #[error(ignore)]
    nanos: i64,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NanosecondOutOfRangeSource {
    /// Record a nanosecond-count overflow, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(nanos: i64) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { nanos, line, file }
    }
}

/// An hour/minute/second/nanosecond combination does not form a real
/// local time of day.
#[derive(Debug, Display, Error, Getters)]
#[display("local time {hour:02}:{minute:02}:{second:02}.{nanos:09} is not a real local time")]
pub struct InvalidLocalTimeSource {
    /// Hour of day, as given.
    #[error(ignore)]
    hour: u8,
    /// Minute of hour, as given.
    #[error(ignore)]
    minute: u8,
    /// Second of minute, as given.
    #[error(ignore)]
    second: u8,
    /// Nanosecond of second, as given.
    #[error(ignore)]
    nanos: u32,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidLocalTimeSource {
    /// Record a rejected local time, capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(hour: u8, minute: u8, second: u8, nanos: u32) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            hour,
            minute,
            second,
            nanos,
            line,
            file,
        }
    }
}

/// A UTC-offset magnitude is outside `chrono::FixedOffset`'s
/// representable range.
///
/// Carries the real signed `magnitude` as a getter-accessible field,
/// but deliberately leaves it out of `Display`: interpolating a
/// symbolic signed, variable-width value here was confirmed
/// Kani-expensive (`Formatter::padding` recursion) at this exact call
/// site (`utc_offset_descriptor_to_fixed_offset`, which real Kani
/// harnesses exercise directly, unlike this file's other new source
/// types above). A `derive_more::Display` template only touches the
/// fields it names, so the field itself costs nothing until something
/// actually formats it.
#[derive(Debug, Display, Error, Getters)]
#[display("offset out of chrono::FixedOffset's representable range")]
pub struct FixedOffsetOutOfRangeSource {
    /// The real signed offset magnitude, in seconds, that was out of
    /// range.
    #[error(ignore)]
    magnitude: i32,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl FixedOffsetOutOfRangeSource {
    /// Record an out-of-range offset magnitude, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(magnitude: i32) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            magnitude,
            line,
            file,
        }
    }
}

/// A local date-time has no single, unique mapping under a given
/// fixed offset.
///
/// No payload beyond provenance: confirmed unreachable for
/// `chrono::FixedOffset` specifically (a fixed offset has no
/// ambiguity or gap to report; see
/// `offset_date_time_descriptor_to_chrono`'s own doc comment), so
/// there is no real diagnostic value to add, and chrono's own
/// `NaiveDateTime` Debug formatting was separately confirmed
/// Kani-expensive elsewhere in this crate's bridge.
#[derive(Debug, Display, Error, Getters)]
#[display("local date-time has no unique mapping under this offset")]
pub struct NoUniqueLocalMappingSource {
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NoUniqueLocalMappingSource {
    /// Record an unmappable local date-time, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new() -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { line, file }
    }
}

impl Default for NoUniqueLocalMappingSource {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::new()
    }
}

