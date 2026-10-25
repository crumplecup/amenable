//! Error sources specific to the jiff backend -- only compiled when
//! the `jiff` feature is on, so `amenable_time` never depends on the
//! `jiff` crate unless some backend actually needs it to.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// Input text was rejected by jiff's own real parser for a requested
/// profile, preserving the real `jiff::Error` as `source` rather than
/// stringifying it -- jiff's own error *is* the genuine cause, so
/// wrapping it natively is normalizing, not the stringifying antipattern
/// `InvalidDescriptorSource` used to commit.
#[derive(Debug, Display, Error, Getters)]
#[display("parse rejected under {profile}: {source}")]
pub struct JiffParseRejectedSource {
    /// The serialization profile the input was checked against.
    #[error(ignore)]
    profile: String,
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffParseRejectedSource {
    /// Preserve a jiff parse rejection, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(profile, source)))]
    pub fn new(profile: impl Into<String>, source: jiff::Error) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            profile: profile.into(),
            source,
            line,
            file,
        }
    }
}

/// A named numeric component, as it crosses into or out of a backend's
/// own narrower integer types during bridging -- the closed vocabulary
/// a [`ComponentNarrowingFailedSource`] names, distinct from
/// [`crate::TemporalComponent`] (scoped to ISO 8601 precision rules
/// only) and covering both point-in-time fields and plural
/// duration-span counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum NarrowingComponent {
    /// Calendar year.
    #[display("year")]
    Year,
    /// Calendar month.
    #[display("month")]
    Month,
    /// Calendar day.
    #[display("day")]
    Day,
    /// Ordinal day-of-year.
    #[display("day of year")]
    DayOfYear,
    /// ISO week-numbering year.
    #[display("week year")]
    WeekYear,
    /// ISO week number.
    #[display("week")]
    Week,
    /// ISO weekday number.
    #[display("weekday")]
    Weekday,
    /// Clock hour.
    #[display("hour")]
    Hour,
    /// Clock minute.
    #[display("minute")]
    Minute,
    /// Clock second.
    #[display("second")]
    Second,
    /// Sub-second fraction.
    #[display("fractional second")]
    FractionalSecond,
    /// A UTC offset's own hour component.
    #[display("offset hour")]
    OffsetHour,
    /// A UTC offset's own minute component.
    #[display("offset minute")]
    OffsetMinute,
    /// A duration's whole-year count.
    #[display("years")]
    Years,
    /// A duration's whole-month count.
    #[display("months")]
    Months,
    /// A duration's whole-week count.
    #[display("weeks")]
    Weeks,
    /// A duration's whole-day count.
    #[display("days")]
    Days,
    /// A duration's whole-hour count.
    #[display("hours")]
    Hours,
    /// A duration's whole-minute count.
    #[display("minutes")]
    Minutes,
    /// A duration's whole-second count.
    #[display("seconds")]
    Seconds,
    /// A duration's whole-nanosecond count.
    #[display("nanoseconds")]
    Nanoseconds,
}

/// A backend-native numeric component does not fit the target integer
/// width, preserving both the real rejected value and the real
/// `std::num::TryFromIntError` as `source` -- none of this struct's
/// call sites are Kani-reachable (confirmed directly), so there is no
/// `Formatter::padding` cost to avoid the way `ComponentOutOfRangeSource`
/// must; keeping the real source here is the honest default.
#[derive(Debug, Display, Error, Getters)]
#[display("{component} value {value} does not fit the target integer width: {source}")]
pub struct ComponentNarrowingFailedSource {
    #[error(ignore)]
    component: NarrowingComponent,
    #[error(ignore)]
    value: i64,
    source: std::num::TryFromIntError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ComponentNarrowingFailedSource {
    /// Record a component-narrowing failure, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(component, source)))]
    pub fn new(component: NarrowingComponent, value: i64, source: std::num::TryFromIntError) -> Self {
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
            source,
            line,
            file,
        }
    }
}

/// A `jiff::tz::Offset` could not be constructed from a raw seconds
/// value, preserving the real `jiff::Error` as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("offset out of jiff::tz::Offset's representable range: {source}")]
pub struct JiffOffsetConstructionFailedSource {
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffOffsetConstructionFailedSource {
    /// Preserve a jiff offset-construction failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: jiff::Error) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { source, line, file }
    }
}

/// A `jiff::civil::Date` could not be constructed from its real
/// components (via any of the calendar, ordinal, or week-date
/// cross-representations), preserving the real `jiff::Error` as
/// `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("date out of jiff::civil::Date's representable range: {source}")]
pub struct JiffDateConstructionFailedSource {
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffDateConstructionFailedSource {
    /// Preserve a jiff date-construction failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: jiff::Error) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { source, line, file }
    }
}

/// A `jiff::civil::Time` could not be constructed from its real
/// components, preserving the real `jiff::Error` as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("local time out of jiff::civil::Time's representable range: {source}")]
pub struct JiffTimeConstructionFailedSource {
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffTimeConstructionFailedSource {
    /// Preserve a jiff time-construction failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: jiff::Error) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { source, line, file }
    }
}

