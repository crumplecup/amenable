//! Error kind enumeration for `amenable`'s CLI-facing operations.

use std::path::PathBuf;

use crate::error::sources::{
    IoSource, SerdeSource, SystemTimeSource, TimeComponentRangeSource, TimeFormatDescriptionSource,
    TimeFormatSource, TimeParseSource,
};

/// Error kind for `amenable` CLI operations.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum AmenableErrorKind {
    /// IO error naming the specific artifact path that failed. Not
    /// derived through the generic `bridge_error!`/`error_from!` chain
    /// (see `wrapper.rs`) because every IO call site in this crate
    /// already has a path in scope worth preserving in the error itself,
    /// not just at the `AmenableError`-level `file`/`line` (which name
    /// the Rust source, not the data path).
    #[display("IO error at {}: {}", path.display(), source)]
    Io {
        /// The artifact path the failing operation targeted.
        path: PathBuf,
        /// The preserved `std::io::Error`.
        source: IoSource,
    },

    /// Failed to parse one line of a JSON Lines artifact.
    #[display("invalid JSON on line {line} in {}: {}", path.display(), source)]
    JsonLine {
        /// The JSON Lines artifact path.
        path: PathBuf,
        /// The 1-indexed line number that failed to parse.
        line: usize,
        /// The preserved `serde_json::Error`.
        source: SerdeSource,
    },

    /// Serde JSON error (assessment/registry-dump serialization).
    #[display("JSON error: {}", _0)]
    #[from(SerdeSource)]
    Serde(SerdeSource),

    /// System clock error (timestamps before the Unix epoch).
    #[display("system clock error: {}", _0)]
    #[from(SystemTimeSource)]
    SystemTime(SystemTimeSource),

    /// Failed to build a `time` format description.
    #[display("date/time format description error: {}", _0)]
    #[from(TimeFormatDescriptionSource)]
    TimeFormatDescription(TimeFormatDescriptionSource),

    /// Failed to parse a date/time value.
    #[display("date/time parse error: {}", _0)]
    #[from(TimeParseSource)]
    TimeParse(TimeParseSource),

    /// A date string didn't match the expected `YYYY-MM-DD` format.
    #[display("invalid date {value:?}; expected YYYY-MM-DD: {source}")]
    InvalidUtcDate {
        /// The rejected date string, verbatim.
        value: String,
        /// The preserved `time::error::Parse`.
        source: TimeParseSource,
    },

    /// A date/time value was out of its valid component range.
    #[display("date/time component range error: {}", _0)]
    #[from(TimeComponentRangeSource)]
    TimeComponentRange(TimeComponentRangeSource),

    /// Failed to format a date/time value.
    #[display("date/time format error: {}", _0)]
    #[from(TimeFormatSource)]
    TimeFormat(TimeFormatSource),

    /// A business-rule invariant was violated (bad input, unknown ID,
    /// mismatched counts, and similar non-foreign validation failures).
    #[display("{detail}")]
    Invariant {
        /// Human-readable description of the violated invariant.
        detail: String,
    },
}
