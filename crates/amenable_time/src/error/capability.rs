//! Backend-capability and ambiguity error sources, shared across
//! backends.

use derive_getters::Getters;
use derive_more::{Display, Error};

use super::unsupported_case::UnsupportedCase;
use crate::{RoundingModeDescriptor, TemporalComponent, TemporalValueDescriptor};

/// Either a fixed, enumerable capability gap ([`UnsupportedCase`]), a
/// real already-typed value this backend cannot represent, or (only
/// where no closed set is genuinely available) a situational case
/// carrying its own interpolated, runtime-variable text --
/// `UnsupportedSource` covers all three without forcing a real typed
/// value's identity into a premature, unmatchable string.
#[derive(Debug, Display)]
pub enum UnsupportedDetail {
    /// A fixed, enumerable capability gap.
    #[display("{_0}")]
    Closed(UnsupportedCase),
    /// A sub-second precision-adjustment edge requires its target to
    /// anchor at the second; this is the non-`Second` component that
    /// was supplied instead.
    #[display(
        "jiff backend's precision-adjustment edges anchor at the second; smallest_component = {_0} is not supported"
    )]
    PrecisionAdjustmentRequiresSecond(TemporalComponent),
    /// A fractional-second digit count exceeds jiff's nanosecond
    /// representable range (9 digits).
    #[display(
        "jiff's civil time is nanosecond-precision (9 fractional digits); {_0} fractional digits exceeds its representable range"
    )]
    FractionalDigitsOutOfRange(u8),
    /// A duration's fractional component is anchored somewhere other
    /// than the second; this backend only supports a fractional-second
    /// component.
    #[display(
        "jiff backend supports a fractional-second component only, not a fraction on {_0}"
    )]
    FractionComponentNotSecond(TemporalComponent),
    /// A declared rounding mode other than truncation was requested
    /// for a lossy sub-second precision reduction; this backend only
    /// implements truncation.
    #[display(
        "jiff backend only implements truncation for sub-second precision reduction, not rounding mode {_0}"
    )]
    UnsupportedRoundingMode(RoundingModeDescriptor),
    /// A bare temporal value could not be formatted -- it is one of
    /// the CalConnect/ISO 8601-2 extension forms, out of scope for
    /// this backend's formatter.
    #[display(
        "jiff backend cannot format the {_0:?} interval-endpoint form (ISO 8601-2 / CalConnect extension family, out of scope)"
    )]
    UnformattableTemporalValue(TemporalValueDescriptor),
    /// A jiff time-interval endpoint has no representation for a bare
    /// temporal value -- it is one of the CalConnect/ISO 8601-2
    /// extension forms, out of scope for this backend.
    #[display(
        "jiff backend has no representation for the {_0:?} interval-endpoint form (ISO 8601-2 / CalConnect extension family, out of scope)"
    )]
    UnrepresentableIntervalEndpointValue(TemporalValueDescriptor),
    /// A reduced-precision local-time string carries a fractional
    /// hour/minute component (e.g. `"12,5"`); jiff's civil time has no
    /// representation for it at all. Carries the real offending text,
    /// since the set of possible inputs here is the user's own, not a
    /// closed one this backend controls.
    #[display(
        "jiff's civil time has no representation for a fractional hour/minute component, as in {_0:?}"
    )]
    FractionalHourMinuteComponent(String),
    /// An IXDTF time-zone annotation names a kind this backend does
    /// not recognize. Carries the real, owned `jiff::fmt::temporal::
    /// TimeZoneAnnotationKind` (via its own `into_owned`), not a
    /// stringified copy -- `amenable_time` already carries an optional
    /// `jiff` dependency for exactly this (naming a backend's real
    /// foreign type on a native source, never its conversion logic),
    /// gated the same way every other jiff-specific source in this
    /// crate is.
    #[cfg(feature = "jiff")]
    #[display("jiff backend does not recognize this time-zone annotation kind: {_0:?}")]
    UnrecognizedTimeZoneAnnotationKind(jiff::fmt::temporal::TimeZoneAnnotationKind<'static>),
    /// A situational capability gap carrying its own free text, for
    /// the rare case with no real typed value or closed set available.
    #[display("{_0}")]
    FreeText(String),
}

/// The requested operation is unsupported by this backend.
#[derive(Debug, Display, Error, Getters)]
#[display("unsupported temporal operation: {detail}")]
pub struct UnsupportedSource {
    /// Which capability gap this is -- a closed, named case or
    /// situational free text.
    #[error(ignore)]
    detail: UnsupportedDetail,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl UnsupportedSource {
    /// Record a situational unsupported operation carrying its own
    /// free text, capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            detail: UnsupportedDetail::FreeText(detail.into()),
            line,
            file,
        }
    }

    /// Record a fixed, enumerable capability gap, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(case)))]
    pub fn closed(case: UnsupportedCase) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            detail: UnsupportedDetail::Closed(case),
            line,
            file,
        }
    }

    /// Record an already-typed [`UnsupportedDetail`] (a real value
    /// this backend cannot represent, not a closed case and not free
    /// text), capturing the caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn from_detail(detail: UnsupportedDetail) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            detail,
            line,
            file,
        }
    }
}

/// The operation would require explicit authority for a lossy
/// conversion.
#[derive(Debug, Display, Error, Getters)]
#[display("lossy conversion requires explicit authority")]
pub struct LossyConversionRequiresAuthoritySource {
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl LossyConversionRequiresAuthoritySource {
    /// Record a missing lossy-conversion authority, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    pub fn new() -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self { line, file }
    }
}

impl Default for LossyConversionRequiresAuthoritySource {
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
    fn default() -> Self {
        Self::new()
    }
}

/// A local timestamp is ambiguous at a zone transition.
#[derive(Debug, Display, Error, Getters)]
#[display("ambiguous local timestamp: {detail}")]
pub struct AmbiguousLocalTimestampSource {
    /// Human-readable description of the ambiguity.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl AmbiguousLocalTimestampSource {
    /// Record an ambiguous local timestamp, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            detail: detail.into(),
            line,
            file,
        }
    }
}

/// A named-zone annotation is inconsistent with the represented
/// instant.
#[derive(Debug, Display, Error, Getters)]
#[display("named-zone inconsistency: {detail}")]
pub struct NamedZoneInconsistencySource {
    /// Human-readable description of the inconsistency.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl NamedZoneInconsistencySource {
    /// Record a named-zone inconsistency, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(detail)))]
    pub fn new(detail: impl Into<String>) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            detail: detail.into(),
            line,
            file,
        }
    }
}
