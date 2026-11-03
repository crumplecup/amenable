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
#[cfg_attr(
    not(kani),
    display("duration component out of jiff::Span's representable range: {source}")
)]
#[cfg_attr(kani, display("duration component out of jiff::Span's representable range"))]
pub struct JiffSpanConstructionFailedSource {
    /// The real jiff construction failure. Dropped entirely under
    /// `cfg(kani)`: confirmed directly that a bare `jiff::Error`
    /// value, with no wrapping type at all, times out under Kani on
    /// its own (`gallery::temporal_error_construction::
    /// check_bare_jiff_error_leaf`).
    #[cfg(not(kani))]
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl JiffSpanConstructionFailedSource {
    /// Preserve a jiff span-construction failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: jiff::Error) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(kani)]
        let _ = &source;
        Self {
            #[cfg(not(kani))]
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A local civil date-time could not be pinned to a concrete instant
/// under a given offset or zone, preserving the real `jiff::Error` as
/// `source`.
#[derive(Debug, Display, Error, Getters)]
#[cfg_attr(
    not(kani),
    display("could not pin the offset date-time to a fixed instant: {source}")
)]
#[cfg_attr(kani, display("could not pin the offset date-time to a fixed instant"))]
pub struct JiffInstantPinFailedSource {
    /// The real jiff construction failure. Dropped entirely under
    /// `cfg(kani)`: confirmed directly that a bare `jiff::Error`
    /// value, with no wrapping type at all, times out under Kani on
    /// its own (`gallery::temporal_error_construction::
    /// check_bare_jiff_error_leaf`).
    #[cfg(not(kani))]
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl JiffInstantPinFailedSource {
    /// Preserve a jiff instant-pinning failure, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: jiff::Error) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(kani)]
        let _ = &source;
        Self {
            #[cfg(not(kani))]
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
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
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl IntervalEndpointsReversedSource {
    /// Record reversed interval endpoints, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(start, end)))]
    pub fn new(start: jiff::Timestamp, end: jiff::Timestamp) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            start,
            end,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A jiff IANA zone identifier failed to resolve, preserving the real
/// `jiff::Error` as `source` alongside the rejected identifier.
#[derive(Debug, Display, Error, Getters)]
#[cfg_attr(not(kani), display("could not resolve IANA time zone {identifier:?}: {source}"))]
#[cfg_attr(kani, display("could not resolve IANA time zone"))]
pub struct JiffNamedZoneResolutionFailedSource {
    /// The real jiff resolution failure. Dropped entirely under
    /// `cfg(kani)`: confirmed directly that a bare `jiff::Error`
    /// value, with no wrapping type at all, times out under Kani on
    /// its own (`gallery::temporal_error_construction::
    /// check_bare_jiff_error_leaf`).
    #[cfg(not(kani))]
    source: jiff::Error,
    /// The attempted-but-invalid zone identifier. Dropped entirely
    /// under `cfg(kani)`: it's exactly the text that failed to
    /// resolve, so there is no closed set to type it as, and nothing
    /// in this codebase reads it under Kani.
    #[cfg(not(kani))]
    identifier: String,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl JiffNamedZoneResolutionFailedSource {
    /// Preserve a jiff IANA zone resolution failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source, identifier)))]
    pub fn new(source: jiff::Error, identifier: impl Into<String>) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(not(kani))]
        let identifier = identifier.into();
        #[cfg(kani)]
        let _ = identifier;
        #[cfg(kani)]
        let _ = &source;
        Self {
            #[cfg(not(kani))]
            source,
            #[cfg(not(kani))]
            identifier,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
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
#[cfg_attr(
    not(kani),
    display(
        "offset {offset:?} is not one of the real offsets {zone}'s own rules produce for local time {local}"
    )
)]
#[cfg_attr(
    kani,
    display("offset {offset:?} is not one of the real offsets this zone's own rules produce for local time {local}")
)]
pub struct JiffOffsetInconsistentWithNamedZoneSource {
    #[error(ignore)]
    offset: jiff::tz::Offset,
    #[error(ignore)]
    local: jiff::civil::DateTime,
    /// The real, already-resolved zone's own IANA name. Dropped
    /// entirely under `cfg(kani)`, same reason the chrono-tz sibling
    /// (`OffsetInconsistentWithNamedZoneSource`) is.
    #[error(ignore)]
    #[cfg(not(kani))]
    zone: String,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
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
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(not(kani))]
        let zone = zone.into();
        #[cfg(kani)]
        let _ = zone;
        Self {
            offset,
            local,
            #[cfg(not(kani))]
            zone,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
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
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl LosslessPrecisionAdjustmentRejectedSource {
    /// Record a rejected lossless precision adjustment, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(digits, original_nanos)))]
    pub fn new(digits: u8, original_nanos: i32) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            digits,
            original_nanos,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}

/// A `jiff::civil::DateTime` could not be rebuilt from its own `.with()`
/// builder, preserving the real `jiff::Error` as `source`.
#[derive(Debug, Display, Error, Getters)]
#[cfg_attr(
    not(kani),
    display("could not rebuild the precision-adjusted local time: {source}")
)]
#[cfg_attr(kani, display("could not rebuild the precision-adjusted local time"))]
pub struct JiffDateTimeConstructionFailedSource {
    /// The real jiff construction failure. Dropped entirely under
    /// `cfg(kani)`: confirmed directly that a bare `jiff::Error`
    /// value, with no wrapping type at all, times out under Kani on
    /// its own (`gallery::temporal_error_construction::
    /// check_bare_jiff_error_leaf`).
    #[cfg(not(kani))]
    source: jiff::Error,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl JiffDateTimeConstructionFailedSource {
    /// Preserve a jiff date-time rebuild failure, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source)))]
    pub fn new(source: jiff::Error) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(kani)]
        let _ = &source;
        Self {
            #[cfg(not(kani))]
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}
