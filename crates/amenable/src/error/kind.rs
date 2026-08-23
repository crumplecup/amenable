//! Error kind enumeration for `amenable`'s CLI-facing operations.

use crate::error::sources::{
    ChainSource, InvalidScoreSource, InvalidUtcDateSource, InvariantSource, IoSource,
    JsonLineSource, PreEpochDateSource, SerdeSource, StdSource, SystemTimeSource,
    TimeComponentRangeSource, TimeFormatDescriptionSource, TimeFormatSource, TimeParseSource,
    TimestampTooLargeSource,
};

/// Error kind for `amenable` CLI operations. Every variant is a clean
/// 1-tuple wrapping a real, named, `Error`-implementing source type --
/// never a bare struct or `String` -- so `AmenableError::source()`'s
/// chain always reaches a genuine `Error`-implementing value at each
/// hop, downcastable all the way to the real foreign error, not a
/// stringified copy or an untyped dead end.
#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum AmenableErrorKind {
    /// IO error naming the specific artifact path that failed. Not
    /// derived through the generic `bridge_error!`/`error_from!` chain
    /// (see `wrapper.rs`) because every IO call site in this crate
    /// already has a path in scope worth preserving in the error itself,
    /// not just at the `AmenableError`-level `file`/`line` (which name
    /// the Rust source, not the data path) -- [`IoSource`] itself now
    /// carries that path.
    #[display("{_0}")]
    Io(IoSource),

    /// Failed to parse one line of a JSON Lines artifact -- [`JsonLineSource`]
    /// carries the artifact path and the 1-indexed line that failed.
    #[display("{_0}")]
    JsonLine(JsonLineSource),

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

    /// A date string didn't match the expected `YYYY-MM-DD` format --
    /// [`InvalidUtcDateSource`] carries the rejected string.
    #[display("{_0}")]
    InvalidUtcDate(InvalidUtcDateSource),

    /// A date/time value was out of its valid component range.
    #[display("date/time component range error: {}", _0)]
    #[from(TimeComponentRangeSource)]
    TimeComponentRange(TimeComponentRangeSource),

    /// Failed to format a date/time value.
    #[display("date/time format error: {}", _0)]
    #[from(TimeFormatSource)]
    TimeFormat(TimeFormatSource),

    /// A business-rule invariant was violated (bad input, unknown ID,
    /// mismatched counts, and similar non-foreign validation failures) --
    /// [`InvariantSource`] is a real `Error`-implementing leaf, not a
    /// bare `String`.
    #[display("{_0}")]
    Invariant(InvariantSource),

    /// A proof-chain lookup failed (`amenable_core::ChainError::NotFound`/
    /// `Incomplete`), preserved as the real typed value.
    #[display("{_0}")]
    Chain(ChainSource),

    /// An `amenable_std` operation failed, preserved as the real typed
    /// `amenable_std::AmenableStdError`.
    #[display("{_0}")]
    Std(StdSource),

    /// A `--score` argument failed to parse as an integer.
    #[display("{_0}")]
    InvalidScore(InvalidScoreSource),

    /// A `--since` date's Unix timestamp is negative (before the epoch).
    #[display("{_0}")]
    PreEpochDate(PreEpochDateSource),

    /// An assessment timestamp is too large to format.
    #[display("{_0}")]
    TimestampTooLarge(TimestampTooLargeSource),
}
