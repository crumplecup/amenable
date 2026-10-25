//! std-only numeric-narrowing and reversed-endpoint error sources,
//! shared across backends.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// A `std::time::Duration`'s whole-day count does not fit a `u32`,
/// preserving the real `std::num::TryFromIntError` as `source` alongside
/// the real rejected day count -- a genuine std conversion failure, not
/// a hand-written description of one.
#[derive(Debug, Display, Error, Getters)]
#[display("duration's {days}-day span exceeds the descriptor's u32 day component: {source}")]
pub struct DurationDayOverflowSource {
    source: std::num::TryFromIntError,
    days: u64,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl DurationDayOverflowSource {
    /// Preserve a day-count overflow, capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: std::num::TryFromIntError, days: u64) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            days,
            line,
            file,
        }
    }
}

/// A time interval's two offset endpoints, as raw Unix epoch seconds,
/// are reversed (start after end) -- a fresh invariant violation, not a
/// foreign error, so there is no `source` to preserve; the real
/// rejected values are the native payload instead of a hand-written
/// description of them.
#[derive(Debug, Display, Error, Getters)]
#[display("interval start ({start}s) is after its end ({end}s)")]
pub struct EpochSecondsEndpointsReversedSource {
    #[error(ignore)]
    start: i64,
    #[error(ignore)]
    end: i64,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl EpochSecondsEndpointsReversedSource {
    /// Record reversed epoch-second interval endpoints, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(start, end)))]
    pub fn new(start: i64, end: i64) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            start,
            end,
            line,
            file,
        }
    }
}
