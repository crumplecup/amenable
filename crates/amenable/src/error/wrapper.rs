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
    AssessmentCountSource, ChainSource, InvalidScoreSource, InvalidUtcDateSource, InvariantSource,
    IoSource, JsonLineSource, PreEpochDateSource, SerdeSource, StdSource, SystemTimeSource,
    TimeComponentRangeSource, TimeFormatDescriptionSource, TimeFormatSource, TimeParseSource,
    TimestampTooLargeSource,
};

use tracing::instrument;
/// Crate-level result alias.
pub type AmenableResult<T> = Result<T, AmenableError>;

/// Wrapper error carrying kind + call site location.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_getters::Getters)]
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
    kind: Box<AmenableErrorKind>,
    /// Source line of the call site that produced this error.
    #[getter(copy)]
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl AmenableError {
    /// Construct an error from an already-classified kind, recording the
    /// caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(kind))]
    pub fn new(kind: AmenableErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }

    /// Construct an [`AmenableErrorKind::Invariant`] business-rule error.
    #[track_caller]
    #[instrument(level = "debug", skip(detail))]
    pub fn invariant(detail: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Invariant(InvariantSource::new(detail)))
    }

    /// Construct an [`AmenableErrorKind::Io`] error naming the artifact
    /// path that failed.
    #[track_caller]
    #[instrument(level = "debug", skip(path, source))]
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::new(AmenableErrorKind::Io(IoSource::new(source, path)))
    }

    /// Construct an [`AmenableErrorKind::JsonLine`] error naming the file
    /// and 1-indexed line that failed to parse.
    #[track_caller]
    #[instrument(level = "debug", skip(path, source))]
    pub fn json_line(path: impl Into<PathBuf>, line: usize, source: serde_json::Error) -> Self {
        Self::new(AmenableErrorKind::JsonLine(JsonLineSource::new(
            source, path, line,
        )))
    }

    /// Construct an [`AmenableErrorKind::InvalidUtcDate`] error naming the
    /// rejected date string.
    #[track_caller]
    #[instrument(level = "debug", skip(value, source))]
    pub fn invalid_utc_date(value: impl Into<String>, source: time::error::Parse) -> Self {
        Self::new(AmenableErrorKind::InvalidUtcDate(
            InvalidUtcDateSource::new(source, value),
        ))
    }

    /// Construct an [`AmenableErrorKind::Chain`] error preserving a
    /// proof-chain lookup failure.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn chain(source: crate::ChainError) -> Self {
        Self::new(AmenableErrorKind::Chain(ChainSource::new(source)))
    }

    /// Construct an [`AmenableErrorKind::Std`] error preserving an
    /// `amenable_std` operation failure.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn std(source: amenable_std::AmenableStdError) -> Self {
        Self::new(AmenableErrorKind::Std(StdSource::new(source)))
    }

    /// Construct an [`AmenableErrorKind::InvalidScore`] error naming the
    /// rejected score string.
    #[track_caller]
    #[instrument(level = "debug", skip(value, source))]
    pub fn invalid_score(value: impl Into<String>, source: std::num::ParseIntError) -> Self {
        Self::new(AmenableErrorKind::InvalidScore(InvalidScoreSource::new(
            source, value,
        )))
    }

    /// Construct an [`AmenableErrorKind::PreEpochDate`] error naming the
    /// rejected date.
    #[track_caller]
    #[instrument(level = "debug", skip(date, source))]
    pub fn pre_epoch_date(date: impl Into<String>, source: std::num::TryFromIntError) -> Self {
        Self::new(AmenableErrorKind::PreEpochDate(PreEpochDateSource::new(
            source, date,
        )))
    }

    /// Construct an [`AmenableErrorKind::TimestampTooLarge`] error naming
    /// the rejected timestamp.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn timestamp_too_large(timestamp: u64, source: std::num::TryFromIntError) -> Self {
        Self::new(AmenableErrorKind::TimestampTooLarge(
            TimestampTooLargeSource::new(source, timestamp),
        ))
    }

    /// Construct an [`AmenableErrorKind::AssessmentCount`] error naming
    /// the rejected count.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn assessment_count(count: usize, source: std::num::TryFromIntError) -> Self {
        Self::new(AmenableErrorKind::AssessmentCount(
            AssessmentCountSource::new(source, count),
        ))
    }
}

impl From<AmenableErrorKind> for AmenableError {
    #[track_caller]
    #[instrument(level = "debug", skip(kind))]
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
