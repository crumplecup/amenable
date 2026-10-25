//! Declared-interval-endpoint and fraction/leap-second validation error
//! sources, split out of [`super::validation`] to stay under this
//! crate's per-file line threshold.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// Which declared-but-valueless time-interval endpoint was asked to
/// decompose into a bare temporal value.
///
/// Lives here, not in `amenable_ext`'s jiff backend, because
/// `amenable_time`'s error types can't depend on a backend crate;
/// this names just the two real causes, not the full
/// backend-specific endpoint enum.
/// Not named `...Kind`: unlike `TemporalErrorKind`, this has no
/// native-source-wrapping variants of its own -- it's a bare
/// discriminant payload, not an umbrella Kind enum, and naming it
/// `DeclaredEndpointKind` falsely tripped cordial's parent/Kind
/// boxing-convention detector (`ERROR-CHAIN-COMPLIANCE-ARCH-KIND-
/// BOX-001`), which (correctly) expects anything shaped like a real
/// Kind enum to be boxed the same way `TemporalError` boxes its own
/// `Kind`. Boxing a 2-variant `Copy` enum just to satisfy that
/// pattern would be ceremony with no referent; renaming out of the
/// pattern is the honest fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum DeclaredEndpoint {
    /// An explicitly open boundary.
    #[display("open")]
    Open,
    /// An explicitly unknown boundary.
    #[display("unknown")]
    Unknown,
}

/// A declared-but-valueless time-interval endpoint (open or unknown)
/// carries no temporal value to decompose.
#[derive(Debug, Display, Error, Getters)]
#[display("a {endpoint} endpoint carries no temporal value to decompose")]
pub struct EndpointHasNoTemporalValueSource {
    /// Which declared endpoint was asked to decompose.
    #[error(ignore)]
    endpoint: DeclaredEndpoint,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl EndpointHasNoTemporalValueSource {
    /// Record a valueless-endpoint decomposition attempt, capturing
    /// the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(endpoint)))]
    pub fn new(endpoint: DeclaredEndpoint) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            endpoint,
            line,
            file,
        }
    }
}

/// A fractional-second digit string contains a non-digit byte.
#[derive(Debug, Display, Error, Getters)]
#[display("fractional-second digit string contains the non-digit byte {byte:?}")]
pub struct NonDigitFractionByteSource {
    /// The real offending byte.
    #[error(ignore)]
    byte: u8,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NonDigitFractionByteSource {
    /// Record a non-digit fractional-second byte, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(byte: u8) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { byte, line, file }
    }
}

/// A duration descriptor carries calendar-variable (`years`/`months`)
/// components, which have no fixed `std::time::Duration`.
#[derive(Debug, Display, Error, Getters)]
#[display(
    "a duration with {years} year(s) and {months} month(s) has no fixed std::time::Duration"
)]
pub struct CalendarComponentsPresentSource {
    /// The real `years` component present.
    #[error(ignore)]
    years: u32,
    /// The real `months` component present.
    #[error(ignore)]
    months: u32,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl CalendarComponentsPresentSource {
    /// Record a calendar-variable duration rejection, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(years: u32, months: u32) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            years,
            months,
            line,
            file,
        }
    }
}

/// A local date-time's sub-second precision represents a leap second
/// (`nanosecond` in `1_000_000_000..2_000_000_000`), which the
/// descriptor form has no representation for.
///
/// Holds the raw `nanosecond` count rather than a pre-formatted
/// `detail: String`: the backend value this wraps (e.g. a
/// `chrono::NaiveDateTime`) is often still fully symbolic at the call
/// site (a Kani proof exercising the surrounding `Exchange`), and
/// `Display`-formatting it eagerly at construction time -- rather
/// than lazily, only if something later actually displays this error
/// -- was confirmed to be the real cost behind a CBMC timeout
/// (`format!("{datetime}")`'s variable-width signed-year formatting
/// goes through `Formatter::padding`, expensive for a symbolic
/// value, even on a branch CBMC cannot cheaply prove dead).
#[derive(Debug, Display, Error, Getters)]
#[display("leap-second nanosecond value {nanosecond} is not representable by the descriptor form")]
pub struct LeapSecondNotRepresentableSource {
    /// The sub-second nanosecond count that marked this as a leap
    /// second.
    #[error(ignore)]
    nanosecond: u32,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl LeapSecondNotRepresentableSource {
    /// Record an unrepresentable leap second, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new(nanosecond: u32) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            nanosecond,
            line,
            file,
        }
    }
}
