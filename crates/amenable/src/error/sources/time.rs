//! Foreign errors from the `time` crate and the system clock.

use tracing::instrument;
/// Preserved `std::time::SystemTimeError` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct SystemTimeSource {
    /// The preserved `std::time::SystemTimeError`.
    source: std::time::SystemTimeError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl SystemTimeSource {
    /// Preserve a system-clock error alongside the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: std::time::SystemTimeError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `time::error::InvalidFormatDescription` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeFormatDescriptionSource {
    /// The preserved `time::error::InvalidFormatDescription`.
    source: time::error::InvalidFormatDescription,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TimeFormatDescriptionSource {
    /// Preserve a format-description error alongside the caller's
    /// location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: time::error::InvalidFormatDescription) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `time::error::Parse` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeParseSource {
    /// The preserved `time::error::Parse`.
    source: time::error::Parse,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TimeParseSource {
    /// Preserve a date/time parse error alongside the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: time::error::Parse) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `time::error::Parse` source for a rejected UTC date string,
/// naming the value that was rejected. Distinct from [`TimeParseSource`]
/// (used where there's no meaningful rejected-value string to preserve):
/// the two variants this backs, `AmenableErrorKind::TimeParse`/
/// `InvalidUtcDate`, are genuinely different claims.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid date {value:?}; expected YYYY-MM-DD: {source}")]
pub struct InvalidUtcDateSource {
    /// The preserved `time::error::Parse`.
    source: time::error::Parse,
    /// The rejected date string, verbatim.
    value: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidUtcDateSource {
    /// Preserve a date/time parse error alongside the rejected value and
    /// the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, value))]
    pub fn new(source: time::error::Parse, value: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            value: value.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `time::error::ComponentRange` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeComponentRangeSource {
    /// The preserved `time::error::ComponentRange`.
    source: time::error::ComponentRange,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TimeComponentRangeSource {
    /// Preserve a date/time component-range error alongside the caller's
    /// location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: time::error::ComponentRange) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `time::error::Format` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeFormatSource {
    /// The preserved `time::error::Format`.
    source: time::error::Format,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TimeFormatSource {
    /// Preserve a date/time format error alongside the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: time::error::Format) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
