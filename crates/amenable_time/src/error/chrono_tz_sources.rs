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
#[cfg_attr(not(kani), display("could not resolve IANA time zone {identifier:?}: {source}"))]
#[cfg_attr(kani, display("could not resolve IANA time zone: {source}"))]
pub struct NamedZoneResolutionFailedSource {
    source: chrono_tz::ParseError,
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

impl NamedZoneResolutionFailedSource {
    /// Preserve an IANA zone resolution failure, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source, identifier)))]
    pub fn new(source: chrono_tz::ParseError, identifier: impl Into<String>) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(not(kani))]
        let identifier = identifier.into();
        #[cfg(kani)]
        let _ = identifier;
        Self {
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

/// A computed offset does not match any of a named zone's own rules for
/// a given local time -- a fresh invariant violation, not a foreign
/// error being converted, so there is no `source` to preserve; the real
/// `chrono` values that describe the mismatch are the native payload
/// instead of a hand-written description of them.
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
pub struct OffsetInconsistentWithNamedZoneSource {
    #[error(ignore)]
    offset: chrono::FixedOffset,
    #[error(ignore)]
    local: chrono::NaiveDateTime,
    /// The real, already-resolved zone's own IANA name. Dropped
    /// entirely under `cfg(kani)`: this could in principle hold the
    /// real zone type instead of its stringified name (matching this
    /// file's own "preserve the real source" convention for
    /// `source` fields elsewhere), but that's a separate, larger
    /// change than this pass makes; for now it's just gated the same
    /// way `identifier`/`detail` are.
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
