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

use crate::{KaniAlreadyExists, KaniAlreadyLocked, KaniWriteHalfClosed};

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
    /// The modeled connection's write half is closed (see
    /// [`KaniWriteHalfClosed`]).
    #[display("{_0}")]
    #[from(KaniWriteHalfClosed)]
    WriteHalfClosed(KaniWriteHalfClosed),
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
    ///
    /// `Location::caller()` is itself an unsupported construct under
    /// Kani (see `fs_model::KaniAlreadyExists::new`'s doc comment for
    /// the confirming detail) -- a Kani-reachable panic is its own
    /// failure signal regardless of what file/line this carries.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(kind)))]
    pub fn new(kind: impl Into<KaniModelErrorKind>) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            kind: Box::new(kind.into()),
            line,
            file,
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

    /// Construct a [`KaniModelErrorKind::WriteHalfClosed`] error.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    #[must_use]
    pub fn write_half_closed() -> Self {
        Self::new(KaniWriteHalfClosed::new())
    }
}
