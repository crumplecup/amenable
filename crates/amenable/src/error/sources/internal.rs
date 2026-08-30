//! Business-rule invariants and this workspace's own cross-crate errors.

use tracing::instrument;

/// A business-rule invariant violation (bad input, unknown ID, mismatched
/// counts, and similar non-foreign validation failures) -- a real,
/// `Error`-implementing leaf, not the bare `String`
/// `AmenableErrorKind::Invariant` used to carry directly. No foreign
/// source to preserve, but still carries its own owned `file`/`line`
/// like every other source in this module -- uniform, not a special
/// case, even though `AmenableError`'s own `file`/`line` usually
/// coincides with it today.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{detail}")]
pub struct InvariantSource {
    /// Human-readable description of the violated invariant.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl InvariantSource {
    /// Wrap a human-readable description of the violated invariant,
    /// recording the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(detail))]
    pub fn new(detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `amenable_core::ChainError` source -- a proof-chain lookup
/// failure (`ChainError::NotFound`/`Incomplete`), kept as the real typed
/// value rather than stringified. `main::run_audit` used to discard this
/// via `AmenableError::invariant(error.to_string())`, losing the ability
/// to downcast back to the real `ChainError` the way every other real
/// error in this crate can be.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct ChainSource {
    /// The preserved `amenable_core::ChainError`.
    source: crate::ChainError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ChainSource {
    /// Preserve a proof-chain lookup failure alongside the caller's
    /// location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: crate::ChainError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `amenable_std::AmenableStdError` source -- `amenable_std`
/// has its own umbrella error, the same as every other crate ("each
/// crate gets its own umbrella error, that's how it works"); `amenable`
/// wraps it here rather than re-deriving a fresh error from its own
/// `std::io::Error` chain.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct StdSource {
    /// The preserved `amenable_std::AmenableStdError`.
    source: amenable_std::AmenableStdError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl StdSource {
    /// Preserve an `amenable_std` error alongside the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: amenable_std::AmenableStdError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
