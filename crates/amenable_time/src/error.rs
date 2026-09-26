//! `amenable_time`'s trait-layer error -- [`TemporalError`] +
//! [`TemporalErrorKind`], following CLAUDE.md error patterns 2 and 3:
//! the parent boxes an umbrella `Kind` enum, and every `Kind` variant
//! is a clean 1-tuple wrapping a real, named, `Error`-implementing
//! native source -- never a bare `String` or struct-literal payload --
//! so `TemporalError::source()`'s chain always reaches a genuine
//! `Error`-implementing value at each hop, matching the real,
//! already-compliant precedent in `amenable::error::sources::internal`
//! (`InvariantSource`, etc).
//!
//! Ported from `elicit_temporal::error`. `ParseRejectedSource` names
//! the serialization profile as a `String` for now; it takes the real
//! `SerializationProfile` descriptor once that lands (plan Phase 2).

use derive_getters::Getters;
use derive_more::{Display, Error};

/// Input text does not conform to the requested serialization profile.
#[derive(Debug, Display, Error, Getters)]
#[display("parse rejected under {profile}: {detail}")]
pub struct ParseRejectedSource {
    /// The serialization profile the input was checked against.
    #[error(ignore)]
    profile: String,
    /// Why the input was rejected.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ParseRejectedSource {
    /// Record a rejected parse attempt, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(profile, detail)))]
    pub fn new(profile: impl Into<String>, detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            profile: profile.into(),
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// A neutral descriptor violates the required contract shape.
#[derive(Debug, Display, Error, Getters)]
#[display("invalid temporal descriptor: {detail}")]
pub struct InvalidDescriptorSource {
    /// Human-readable description of the violated shape.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvalidDescriptorSource {
    /// Record an invalid descriptor, capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// The requested operation is unsupported by this backend.
#[derive(Debug, Display, Error, Getters)]
#[display("unsupported temporal operation: {detail}")]
pub struct UnsupportedSource {
    /// Human-readable description of what isn't supported.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl UnsupportedSource {
    /// Record an unsupported operation, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// The operation would require explicit authority for a lossy
/// conversion.
#[derive(Debug, Display, Error, Getters)]
#[display("lossy conversion requires explicit authority")]
pub struct LossyConversionRequiresAuthoritySource {
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl LossyConversionRequiresAuthoritySource {
    /// Record a missing lossy-conversion authority, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new() -> Self {
        let loc = std::panic::Location::caller();
        Self {
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

impl Default for LossyConversionRequiresAuthoritySource {
    #[track_caller]
    fn default() -> Self {
        Self::new()
    }
}

/// A local timestamp is ambiguous at a zone transition.
#[derive(Debug, Display, Error, Getters)]
#[display("ambiguous local timestamp: {detail}")]
pub struct AmbiguousLocalTimestampSource {
    /// Human-readable description of the ambiguity.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl AmbiguousLocalTimestampSource {
    /// Record an ambiguous local timestamp, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// A named-zone annotation is inconsistent with the represented
/// instant.
#[derive(Debug, Display, Error, Getters)]
#[display("named-zone inconsistency: {detail}")]
pub struct NamedZoneInconsistencySource {
    /// Human-readable description of the inconsistency.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NamedZoneInconsistencySource {
    /// Record a named-zone inconsistency, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Specific error conditions for temporal trait operations. Every
/// variant is a clean 1-tuple wrapping a real, named,
/// `Error`-implementing source type.
#[derive(Debug, Display, Error)]
pub enum TemporalErrorKind {
    /// Input text does not conform to the requested serialization profile.
    #[display("{_0}")]
    ParseRejected(ParseRejectedSource),
    /// A neutral descriptor violates the required contract shape.
    #[display("{_0}")]
    InvalidDescriptor(InvalidDescriptorSource),
    /// The requested operation is unsupported by this backend.
    #[display("{_0}")]
    Unsupported(UnsupportedSource),
    /// The operation would require explicit authority for a lossy conversion.
    #[display("{_0}")]
    LossyConversionRequiresAuthority(LossyConversionRequiresAuthoritySource),
    /// A local timestamp is ambiguous at a zone transition.
    #[display("{_0}")]
    AmbiguousLocalTimestamp(AmbiguousLocalTimestampSource),
    /// A named-zone annotation is inconsistent with the represented instant.
    #[display("{_0}")]
    NamedZoneInconsistency(NamedZoneInconsistencySource),
}

/// Temporal trait-layer error with call-site location.
#[derive(Debug, Display, Error, Getters)]
#[display("{kind} at {file}:{line}")]
pub struct TemporalError {
    /// The specific error kind, boxed to keep `TemporalError` itself
    /// small regardless of how large any one `TemporalErrorKind`
    /// variant grows.
    #[error(source)]
    kind: Box<TemporalErrorKind>,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl TemporalError {
    /// Create a [`TemporalError`] capturing the call-site location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(kind)))]
    pub fn new(kind: TemporalErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Result type for temporal operations.
pub type TemporalResult<T> = Result<T, TemporalError>;
