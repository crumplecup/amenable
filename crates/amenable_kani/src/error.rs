//! Umbrella error type for `amenable_kani`'s own model-error surface.
//!
//! Not composed internally today — `KaniFsModel::create_new_file`/
//! `KaniCreateNewObservation::create_new`/`KaniLockObservation::try_lock`
//! return their specific native source directly (`KaniAlreadyExists`,
//! `KaniAlreadyLocked`), the narrower and more useful signature for a
//! caller who only cares about the one condition that method can fail
//! with. But a real, defined composition path needs to exist regardless
//! of whether this crate's own code exercises it: a consumer building
//! its own error type across `amenable_kani`'s error surface needs
//! somewhere real to `From`/`?` into, matching every other crate in
//! this workspace's own error architecture (`amenable::AmenableError`,
//! …) — the umbrella exists for that composability, not to replace the
//! narrower per-method signatures.

use crate::{KaniAlreadyExists, KaniAlreadyLocked};

/// Every distinct condition `amenable_kani`'s own model types fail
/// with. Every variant wraps a real, named, `Error`-implementing native
/// source — never a bare struct or `String` — so `KaniModelError::
/// source()`'s chain always reaches a genuine `Error`-implementing
/// value.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum KaniModelErrorKind {
    /// The modeled path already exists (see [`KaniAlreadyExists`]).
    #[display("{_0}")]
    #[from(KaniAlreadyExists)]
    AlreadyExists(KaniAlreadyExists),
    /// The modeled lock is already held (see [`KaniAlreadyLocked`]).
    #[display("{_0}")]
    #[from(KaniAlreadyLocked)]
    AlreadyLocked(KaniAlreadyLocked),
}

/// Umbrella error for `amenable_kani`'s own model-error surface — boxes
/// [`KaniModelErrorKind`] so a caller composing its own error type has
/// a real, single conversion target for every native source this crate
/// defines, matching `amenable::AmenableError`'s own shape.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error, derive_getters::Getters)]
#[display("amenable_kani: {kind} at {file}:{line}")]
pub struct KaniModelError {
    /// The specific error kind.
    #[error(source)]
    kind: Box<KaniModelErrorKind>,
    /// Source line of the call site that produced this error.
    #[getter(copy)]
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl KaniModelError {
    /// Construct an error from an already-classified kind, recording
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(kind)))]
    pub fn new(kind: impl Into<KaniModelErrorKind>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind.into()),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }

    /// Construct a [`KaniModelErrorKind::AlreadyExists`] error.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn already_exists() -> Self {
        Self::new(KaniAlreadyExists::new())
    }

    /// Construct a [`KaniModelErrorKind::AlreadyLocked`] error.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn already_locked() -> Self {
        Self::new(KaniAlreadyLocked::new())
    }
}
