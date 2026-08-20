//! Foreign error wrappers preserving the underlying `source` chain.
//!
//! Each wrapper holds exactly one foreign error in a field named `source`,
//! which `derive_more::Error` recognizes as the chain-preserving source
//! without any extra attribute. Every wrapper also carries its own owned
//! `file`/`line`, captured from `Location::caller()` by a `#[track_caller]
//! fn new` -- not `&'static Location`, so the wrapper stays `'static`-free
//! and comparable/clonable the same way the rest of this module's types
//! are, and not passed as constructor arguments, so the location can never
//! silently drift from the real call site the way a hand-typed `file!()`/
//! `line!()` pair at the wrong call frame could.
//!
//! [`JsonLineSource`] and [`InvalidUtcDateSource`] additionally carry the
//! domain context (`path`/`json_line`, `value`) that used to live directly
//! on `AmenableErrorKind`'s own `JsonLine`/`InvalidUtcDate` variants --
//! moved here so every `AmenableErrorKind` variant can be a clean 1-tuple
//! wrapping a real, named, `Error`-implementing type (`docs/
//! CONTRACT_BOUND_NAMING_WORKFLOW.md`-adjacent `cordial` policy: every Kind
//! variant is a native source, never a bare struct/String), without losing
//! any of the context those variants used to carry directly.

use std::path::PathBuf;

/// Preserved `std::io::Error` source, naming the artifact path that failed.
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
    file: &'static str,
}

impl IoSource {
    /// Preserve an IO error alongside the artifact path it failed on and
    /// the caller's location.
    #[track_caller]
    pub fn new(source: std::io::Error, path: impl Into<PathBuf>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            path: path.into(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `serde_json::Error` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct SerdeSource {
    /// The preserved `serde_json::Error`.
    source: serde_json::Error,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl SerdeSource {
    /// Preserve a serde JSON error alongside the caller's location.
    #[track_caller]
    pub fn new(source: serde_json::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `serde_json::Error` source for one line of a JSON Lines
/// artifact, naming the artifact path and the 1-indexed line that failed
/// to parse. Distinct from [`SerdeSource`] (used for JSON errors with no
/// meaningful path/line, e.g. `assessment::print_json`): the two variants
/// this backs, `AmenableErrorKind::Serde`/`JsonLine`, are genuinely
/// different claims, not the same shape restated.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid JSON on line {json_line} in {}: {source}", path.display())]
pub struct JsonLineSource {
    /// The preserved `serde_json::Error`.
    source: serde_json::Error,
    /// The JSON Lines artifact path.
    path: PathBuf,
    /// The 1-indexed line number that failed to parse.
    json_line: usize,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl JsonLineSource {
    /// Preserve a serde JSON error alongside the artifact path, the
    /// 1-indexed JSON line that failed, and the caller's location.
    #[track_caller]
    pub fn new(source: serde_json::Error, path: impl Into<PathBuf>, json_line: usize) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            path: path.into(),
            json_line,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `std::time::SystemTimeError` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct SystemTimeSource {
    /// The preserved `std::time::SystemTimeError`.
    source: std::time::SystemTimeError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl SystemTimeSource {
    /// Preserve a system-clock error alongside the caller's location.
    #[track_caller]
    pub fn new(source: std::time::SystemTimeError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `time::error::InvalidFormatDescription` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeFormatDescriptionSource {
    /// The preserved `time::error::InvalidFormatDescription`.
    source: time::error::InvalidFormatDescription,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl TimeFormatDescriptionSource {
    /// Preserve a format-description error alongside the caller's
    /// location.
    #[track_caller]
    pub fn new(source: time::error::InvalidFormatDescription) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `time::error::Parse` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeParseSource {
    /// The preserved `time::error::Parse`.
    source: time::error::Parse,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl TimeParseSource {
    /// Preserve a date/time parse error alongside the caller's location.
    #[track_caller]
    pub fn new(source: time::error::Parse) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `time::error::Parse` source for a rejected UTC date string,
/// naming the value that was rejected. Distinct from [`TimeParseSource`]
/// (used where there's no meaningful rejected-value string to preserve):
/// the two variants this backs, `AmenableErrorKind::TimeParse`/
/// `InvalidUtcDate`, are genuinely different claims.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid date {value:?}; expected YYYY-MM-DD: {source}")]
pub struct InvalidUtcDateSource {
    /// The preserved `time::error::Parse`.
    source: time::error::Parse,
    /// The rejected date string, verbatim.
    value: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl InvalidUtcDateSource {
    /// Preserve a date/time parse error alongside the rejected value and
    /// the caller's location.
    #[track_caller]
    pub fn new(source: time::error::Parse, value: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            value: value.into(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `time::error::ComponentRange` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeComponentRangeSource {
    /// The preserved `time::error::ComponentRange`.
    source: time::error::ComponentRange,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl TimeComponentRangeSource {
    /// Preserve a date/time component-range error alongside the caller's
    /// location.
    #[track_caller]
    pub fn new(source: time::error::ComponentRange) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

/// Preserved `time::error::Format` source.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct TimeFormatSource {
    /// The preserved `time::error::Format`.
    source: time::error::Format,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: &'static str,
}

impl TimeFormatSource {
    /// Preserve a date/time format error alongside the caller's location.
    #[track_caller]
    pub fn new(source: time::error::Format) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}

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
    file: &'static str,
}

impl InvariantSource {
    /// Wrap a human-readable description of the violated invariant,
    /// recording the caller's location.
    #[track_caller]
    pub fn new(detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            detail: detail.into(),
            line: loc.line(),
            file: loc.file(),
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
    file: &'static str,
}

impl ChainSource {
    /// Preserve a proof-chain lookup failure alongside the caller's
    /// location.
    #[track_caller]
    pub fn new(source: crate::ChainError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
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
    file: &'static str,
}

impl StdSource {
    /// Preserve an `amenable_std` error alongside the caller's location.
    #[track_caller]
    pub fn new(source: amenable_std::AmenableStdError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file(),
        }
    }
}
