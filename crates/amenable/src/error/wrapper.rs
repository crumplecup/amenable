//! Crate error wrapper with call-site location.
//!
//! Follows this repo's own documented Error Handling Pattern 5
//! (`CLAUDE.md`): `bridge_error!` chains a foreign type through its
//! `*Source` wrapper into [`AmenableErrorKind`]; `error_from!` completes
//! the chain from the foreign type all the way to [`AmenableError`], so
//! call sites just use `?`.

use std::path::PathBuf;

use crate::error::kind::AmenableErrorKind;
use crate::error::sources::{
    ChainSource, InvalidUtcDateSource, InvariantSource, IoSource, JsonLineSource, SerdeSource,
    SystemTimeSource, TimeComponentRangeSource, TimeFormatDescriptionSource, TimeFormatSource,
    TimeParseSource,
};

/// Crate-level result alias.
pub type AmenableResult<T> = Result<T, AmenableError>;

/// Wrapper error carrying kind + call site location.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("amenable: {kind} at {file}:{line}")]
pub struct AmenableError {
    /// The specific error kind, boxed to keep `AmenableError` itself
    /// small regardless of how large any one `AmenableErrorKind` variant
    /// grows. `derive_more::Error` only auto-detects a field literally
    /// named `source` on named structs (tuple variants get inference;
    /// named ones don't) -- without `#[error(source)]` here,
    /// `AmenableError::source()` would silently return `None` and the
    /// real io/serde/etc. error would never be reachable through
    /// `std::error::Error::source()`, breaking the exact chain this
    /// module exists to preserve.
    #[error(source)]
    pub kind: Box<AmenableErrorKind>,
    /// Source line of the call site that produced this error.
    pub line: u32,
    /// Source file of the call site that produced this error.
    pub file: &'static str,
}

impl AmenableError {
    /// Construct an error from an already-classified kind, recording the
    /// caller's location.
    #[track_caller]
    pub fn new(kind: AmenableErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file(),
        }
    }

    /// Construct an [`AmenableErrorKind::Invariant`] business-rule error.
    #[track_caller]
    pub fn invariant(detail: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Invariant(InvariantSource::new(detail)))
    }

    /// Construct an [`AmenableErrorKind::Io`] error naming the artifact
    /// path that failed.
    #[track_caller]
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::new(AmenableErrorKind::Io(IoSource::new(source, path)))
    }

    /// Construct an [`AmenableErrorKind::JsonLine`] error naming the file
    /// and 1-indexed line that failed to parse.
    #[track_caller]
    pub fn json_line(path: impl Into<PathBuf>, line: usize, source: serde_json::Error) -> Self {
        Self::new(AmenableErrorKind::JsonLine(JsonLineSource::new(
            source, path, line,
        )))
    }

    /// Construct an [`AmenableErrorKind::InvalidUtcDate`] error naming the
    /// rejected date string.
    #[track_caller]
    pub fn invalid_utc_date(value: impl Into<String>, source: time::error::Parse) -> Self {
        Self::new(AmenableErrorKind::InvalidUtcDate(
            InvalidUtcDateSource::new(source, value),
        ))
    }

    /// Construct an [`AmenableErrorKind::Chain`] error preserving a
    /// proof-chain lookup failure.
    #[track_caller]
    pub fn chain(source: crate::ChainError) -> Self {
        Self::new(AmenableErrorKind::Chain(ChainSource::new(source)))
    }
}

impl From<AmenableErrorKind> for AmenableError {
    #[track_caller]
    fn from(kind: AmenableErrorKind) -> Self {
        Self::new(kind)
    }
}

/// Bridge external errors through `*Source` wrappers into
/// [`AmenableErrorKind`].
macro_rules! bridge_error {
    ($external:ty => $wrapper:ty) => {
        impl From<$external> for AmenableErrorKind {
            #[track_caller]
            fn from(err: $external) -> Self {
                <$wrapper>::new(err).into()
            }
        }
    };
}

/// Complete the chain: external error → [`AmenableErrorKind`] →
/// [`AmenableError`].
macro_rules! error_from {
    ($source:ty) => {
        impl From<$source> for AmenableError {
            #[track_caller]
            fn from(err: $source) -> Self {
                Self::new(AmenableErrorKind::from(err))
            }
        }
    };
}

bridge_error!(serde_json::Error => SerdeSource);
bridge_error!(std::time::SystemTimeError => SystemTimeSource);
bridge_error!(time::error::InvalidFormatDescription => TimeFormatDescriptionSource);
bridge_error!(time::error::Parse => TimeParseSource);
bridge_error!(time::error::ComponentRange => TimeComponentRangeSource);
bridge_error!(time::error::Format => TimeFormatSource);

error_from!(serde_json::Error);
error_from!(std::time::SystemTimeError);
error_from!(time::error::InvalidFormatDescription);
error_from!(time::error::Parse);
error_from!(time::error::ComponentRange);
error_from!(time::error::Format);
