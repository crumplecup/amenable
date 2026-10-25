//! Error sources specific to the chrono-tz backend -- only compiled
//! when the `chrono-tz` feature is on, so `amenable_time` never
//! depends on the `chrono-tz` crate unless some backend actually needs
//! it to.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// A chrono-tz IANA zone identifier failed to resolve, preserving the
/// real `chrono_tz::ParseError` as `source` alongside the rejected
/// identifier -- the builder error *is* the genuine cause, so wrapping
/// it natively is normalizing, not the stringifying antipattern
/// `InvalidDescriptorSource` used to commit.
#[derive(Debug, Display, Error, Getters)]
#[display("could not resolve IANA time zone {identifier:?}: {source}")]
pub struct NamedZoneResolutionFailedSource {
    source: chrono_tz::ParseError,
    identifier: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NamedZoneResolutionFailedSource {
    /// Preserve an IANA zone resolution failure, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source, identifier)))]
    pub fn new(source: chrono_tz::ParseError, identifier: impl Into<String>) -> Self {
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

/// A computed offset does not match any of a named zone's own rules for
/// a given local time -- a fresh invariant violation, not a foreign
/// error being converted, so there is no `source` to preserve; the real
/// `chrono` values that describe the mismatch are the native payload
/// instead of a hand-written description of them.
#[derive(Debug, Display, Error, Getters)]
#[display(
    "offset {offset:?} is not one of the real offsets {zone}'s own rules produce for local time {local}"
)]
pub struct OffsetInconsistentWithNamedZoneSource {
    #[error(ignore)]
    offset: chrono::FixedOffset,
    #[error(ignore)]
    local: chrono::NaiveDateTime,
    #[error(ignore)]
    zone: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl OffsetInconsistentWithNamedZoneSource {
    /// Record an offset/zone-rule mismatch, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(offset, local, zone)))]
    pub fn new(
        offset: chrono::FixedOffset,
        local: chrono::NaiveDateTime,
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
