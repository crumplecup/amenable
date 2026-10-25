//! Jiff-backend error sources for span/instant/interval/named-zone
//! construction, split out of [`super::jiff_sources`] to stay under
//! this crate's per-file line threshold -- only compiled when the
//! `jiff` feature is on, so `amenable_time` never depends on the
//! `jiff` crate unless some backend actually needs it to.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// A `jiff::Span` could not be constructed from a duration descriptor's
/// real components, preserving the real `jiff::Error` as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("duration component out of jiff::Span's representable range: {source}")]
pub struct JiffSpanConstructionFailedSource {
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffSpanConstructionFailedSource {
    /// Preserve a jiff span-construction failure, capturing the
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

/// A local civil date-time could not be pinned to a concrete instant
/// under a given offset or zone, preserving the real `jiff::Error` as
/// `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not pin the offset date-time to a fixed instant: {source}")]
pub struct JiffInstantPinFailedSource {
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffInstantPinFailedSource {
    /// Preserve a jiff instant-pinning failure, capturing the caller's
    /// location.
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

/// A recurring interval's two offset endpoints are reversed (start
/// after end) -- a fresh invariant violation, not a foreign error, so
/// there is no `source` to preserve; the real `jiff::Timestamp` values
/// are the native payload instead of a hand-written description.
#[derive(Debug, Display, Error, Getters)]
#[display("interval start ({start}) is after its end ({end})")]
pub struct IntervalEndpointsReversedSource {
    #[error(ignore)]
    start: jiff::Timestamp,
    #[error(ignore)]
    end: jiff::Timestamp,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl IntervalEndpointsReversedSource {
    /// Record reversed interval endpoints, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(start, end)))]
    pub fn new(start: jiff::Timestamp, end: jiff::Timestamp) -> Self {
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

/// A jiff IANA zone identifier failed to resolve, preserving the real
/// `jiff::Error` as `source` alongside the rejected identifier.
#[derive(Debug, Display, Error, Getters)]
#[display("could not resolve IANA time zone {identifier:?}: {source}")]
pub struct JiffNamedZoneResolutionFailedSource {
    source: jiff::Error,
    identifier: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffNamedZoneResolutionFailedSource {
    /// Preserve a jiff IANA zone resolution failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source, identifier)))]
    pub fn new(source: jiff::Error, identifier: impl Into<String>) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            source,
            identifier: identifier.into(),
            line,
            file,
        }
    }
}

/// A computed jiff offset does not match any of a named zone's own
/// rules for a given local time -- a fresh invariant violation, not a
/// foreign error being converted, so there is no `source` to preserve;
/// the real jiff values that describe the mismatch are the native
/// payload instead of a hand-written description of them.
#[derive(Debug, Display, Error, Getters)]
#[display(
    "offset {offset:?} is not one of the real offsets {zone}'s own rules produce for local time {local}"
)]
pub struct JiffOffsetInconsistentWithNamedZoneSource {
    #[error(ignore)]
    offset: jiff::tz::Offset,
    #[error(ignore)]
    local: jiff::civil::DateTime,
    #[error(ignore)]
    zone: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffOffsetInconsistentWithNamedZoneSource {
    /// Record a jiff offset/zone-rule mismatch, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset, local, zone)))]
    pub fn new(
        offset: jiff::tz::Offset,
        local: jiff::civil::DateTime,
        zone: impl Into<String>,
    ) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            offset,
            local,
            zone: zone.into(),
            line,
            file,
        }
    }
}

/// A lossless precision adjustment would discard real, non-zero
/// sub-second precision -- a fresh invariant violation (the request
/// itself declared "lossless", and the real nanosecond value shows
/// that promise can't be kept), not a foreign error, so there is no
/// `source` to preserve; the real rejected values are the native
/// payload instead of a hand-written description of them.
#[derive(Debug, Display, Error, Getters)]
#[display(
    "adjusting to {digits} fractional digits would discard non-zero sub-second precision \
     ({original_nanos} nanoseconds); not lossless"
)]
pub struct LosslessPrecisionAdjustmentRejectedSource {
    #[error(ignore)]
    digits: u8,
    #[error(ignore)]
    original_nanos: i32,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl LosslessPrecisionAdjustmentRejectedSource {
    /// Record a rejected lossless precision adjustment, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(digits, original_nanos)))]
    pub fn new(digits: u8, original_nanos: i32) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            digits,
            original_nanos,
            line,
            file,
        }
    }
}

/// A `jiff::civil::DateTime` could not be rebuilt from its own `.with()`
/// builder, preserving the real `jiff::Error` as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("could not rebuild the precision-adjusted local time: {source}")]
pub struct JiffDateTimeConstructionFailedSource {
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl JiffDateTimeConstructionFailedSource {
    /// Preserve a jiff date-time rebuild failure, capturing the
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
