//! `amenable_time`'s trait-layer error — [`TemporalError`] +
//! [`TemporalErrorKind`], following CLAUDE.md error patterns 2 and 3
//! (owned `String` location, private fields with `derive_getters`,
//! `#[track_caller]` constructors, no `PartialEq`/`Eq` on the wrapper).
//!
//! Ported from `elicit_temporal::error`. `ParseRejected` names the
//! serialization profile as a `String` for now; it takes the real
//! `SerializationProfile` descriptor once that lands (plan Phase 2).

use derive_getters::Getters;
use derive_more::{Display, Error};

/// Specific error conditions for temporal trait operations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, Error)]
pub enum TemporalErrorKind {
    /// Input text does not conform to the requested serialization profile.
    #[display("parse rejected under {profile}: {detail}")]
    ParseRejected {
        /// The serialization profile the input was checked against.
        profile: String,
        /// Why the input was rejected.
        detail: String,
    },
    /// A neutral descriptor violates the required contract shape.
    #[display("invalid temporal descriptor: {_0}")]
    InvalidDescriptor(#[error(not(source))] String),
    /// The requested operation is unsupported by this backend.
    #[display("unsupported temporal operation: {_0}")]
    Unsupported(#[error(not(source))] String),
    /// The operation would require explicit authority for a lossy conversion.
    #[display("lossy conversion requires explicit authority")]
    LossyConversionRequiresAuthority,
    /// A local timestamp is ambiguous at a zone transition.
    #[display("ambiguous local timestamp: {_0}")]
    AmbiguousLocalTimestamp(#[error(not(source))] String),
    /// A named-zone annotation is inconsistent with the represented instant.
    #[display("named-zone inconsistency: {_0}")]
    NamedZoneInconsistency(#[error(not(source))] String),
}

/// Temporal trait-layer error with call-site location.
#[derive(Debug, Clone, Display, Error, Getters)]
#[display("{kind} at {file}:{line}")]
pub struct TemporalError {
    /// The specific error kind.
    #[error(source)]
    kind: TemporalErrorKind,
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
            kind,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Result type for temporal operations.
pub type TemporalResult<T> = Result<T, TemporalError>;
