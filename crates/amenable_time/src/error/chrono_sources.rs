//! Error sources specific to chrono-tz offset discovery -- only
//! compiled when the `chrono` feature is on, so `amenable_time` never
//! depends on the `chrono` crate unless some backend actually needs it
//! to.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// A chrono-tz offset-discovery request's step size is not positive,
/// preserving the real rejected `chrono::Duration` rather than a
/// hand-written description of it.
#[derive(Debug, Display, Error, Getters)]
#[display("delta must be positive, got {delta:?}")]
pub struct NonPositiveDeltaSource {
    #[error(ignore)]
    delta: chrono::Duration,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NonPositiveDeltaSource {
    /// Record a non-positive step size, capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(delta)))]
    pub fn new(delta: chrono::Duration) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { delta, line, file }
    }
}

/// A chrono-tz offset-discovery request's range is reversed (`lo` after
/// `hi`), preserving the real rejected `chrono::NaiveDateTime` bounds
/// rather than a hand-written description of them.
#[derive(Debug, Display, Error, Getters)]
#[display("lo {lo:?} must not be after hi {hi:?}")]
pub struct OffsetDiscoveryRangeReversedSource {
    #[error(ignore)]
    lo: chrono::NaiveDateTime,
    #[error(ignore)]
    hi: chrono::NaiveDateTime,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl OffsetDiscoveryRangeReversedSource {
    /// Record a reversed discovery range, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(lo, hi)))]
    pub fn new(lo: chrono::NaiveDateTime, hi: chrono::NaiveDateTime) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { lo, hi, line, file }
    }
}
