//! Foreign integer-conversion errors, each naming the rejected value.

use tracing::instrument;
/// Preserved `std::num::ParseIntError` source for a rejected assessment
/// score string, naming the value that was rejected.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid score {value:?}; expected an integer from 0 to 4: {source}")]
pub struct InvalidScoreSource {
    /// The preserved `std::num::ParseIntError`.
    source: std::num::ParseIntError,
    /// The rejected score string, verbatim.
    value: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidScoreSource {
    /// Preserve an integer parse error alongside the rejected value and
    /// the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, value))]
    pub fn new(source: std::num::ParseIntError, value: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            value: value.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `std::num::TryFromIntError` source for a UTC date whose Unix
/// timestamp is negative (before the epoch), naming the rejected date.
/// Distinct from [`TimestampTooLargeSource`] (also a `TryFromIntError`
/// over a timestamp, but for the opposite direction -- too large, not
/// negative): the two variants this backs are genuinely different claims,
/// the same reason the `time`-domain sources stay separate too.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "date {date} is before the Unix epoch; expected YYYY-MM-DD on or after 1970-01-01: {source}"
)]
pub struct PreEpochDateSource {
    /// The preserved `std::num::TryFromIntError`.
    source: std::num::TryFromIntError,
    /// The rejected date, formatted.
    date: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl PreEpochDateSource {
    /// Preserve an int-conversion error alongside the rejected date and
    /// the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, date))]
    pub fn new(source: std::num::TryFromIntError, date: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            date: date.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `std::num::TryFromIntError` source for an assessment
/// timestamp too large to format. See [`PreEpochDateSource`]'s own doc
/// comment for why this stays a separate type rather than a shared one.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("assessment timestamp {timestamp} is too large to format: {source}")]
pub struct TimestampTooLargeSource {
    /// The preserved `std::num::TryFromIntError`.
    source: std::num::TryFromIntError,
    /// The rejected timestamp.
    timestamp: u64,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TimestampTooLargeSource {
    /// Preserve an int-conversion error alongside the rejected timestamp
    /// and the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: std::num::TryFromIntError, timestamp: u64) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            timestamp,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `std::num::TryFromIntError` source for an assessment count
/// too large to convert to `u32` when computing a rubric score mean.
/// See [`PreEpochDateSource`]'s own doc comment for why this stays a
/// separate type rather than a shared one.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("assessment count {count} does not fit in u32: {source}")]
pub struct AssessmentCountSource {
    /// The preserved `std::num::TryFromIntError`.
    source: std::num::TryFromIntError,
    /// The rejected assessment count.
    count: usize,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl AssessmentCountSource {
    /// Preserve an int-conversion error alongside the rejected count and
    /// the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: std::num::TryFromIntError, count: usize) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            count,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
