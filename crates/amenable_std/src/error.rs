//! `amenable_std`'s own umbrella error -- each crate gets its own,
//! matching `amenable`'s own `error/{kind,sources,wrapper}.rs`
//! (`docs/CONTRACT_BOUND_NAMING_WORKFLOW.md`-adjacent `cordial` policy:
//! every Kind variant is a clean 1-tuple wrapping a real, named,
//! `Error`-implementing source type, boxed under one parent struct).
//! Scoped to exactly what this crate's own fallible operations need
//! today -- writing certificate artifacts to disk, the only place this
//! crate currently produces a real error at all -- not speculatively
//! built out for hypothetical future variants.

use std::path::PathBuf;

/// Preserved `std::io::Error` source, naming the artifact path that
/// failed. Carries its own owned `file`/`line`, captured from
/// `Location::caller()` by a `#[track_caller] fn new` -- not `&'static
/// Location`, and not passed as constructor arguments, so the location
/// can never silently drift from the real call site.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("IO error at {}: {source}", path.display())]
pub struct IoSource {
    /// The preserved `std::io::Error`.
    source: std::io::Error,
    /// The artifact path the failing operation targeted.
    path: PathBuf,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl IoSource {
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(source, path)))]
    fn new(source: std::io::Error, path: impl Into<PathBuf>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            path: path.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Error kind for `amenable_std` operations. Every variant is a clean
/// 1-tuple wrapping a real, named, `Error`-implementing source type.
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum AmenableStdErrorKind {
    /// IO error naming the specific artifact path that failed.
    #[display("{_0}")]
    Io(IoSource),
}

/// Crate-level result alias.
pub type AmenableStdResult<T> = Result<T, AmenableStdError>;

/// Wrapper error carrying kind + call site location -- the same shape
/// `amenable::AmenableError`/`amenable_core::ChainError` use.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("amenable_std: {kind} at {file}:{line}")]
pub struct AmenableStdError {
    /// The specific error kind, boxed to keep `AmenableStdError` itself
    /// small.
    #[error(source)]
    kind: Box<AmenableStdErrorKind>,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl AmenableStdError {
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(kind)))]
    fn new(kind: AmenableStdErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }

    /// Construct an [`AmenableStdErrorKind::Io`] error naming the
    /// artifact path that failed.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(path, source)))]
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::new(AmenableStdErrorKind::Io(IoSource::new(source, path)))
    }
}
