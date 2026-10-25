//! Parse-rejection error sources shared across backends.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// Input text does not match the requested profile's lexical shape at
/// all -- a fresh, synthesized diagnostic, not a discarded real error:
/// several of this profile's textual forms have no parser of their own
/// at all to delegate to (jiff's own `FromStr` doesn't accept them), so
/// there is no foreign error object here to preserve in the first
/// place. `detail` is legitimate exactly because nothing real is being
/// thrown away, the same reason [`super::UnsupportedSource`]'s own
/// `detail` is. [`ParseIntRejectedSource`]/`JiffParseRejectedSource`
/// (the latter only under the `jiff` feature) are the sibling variants
/// for when a real foreign parse error does exist.
#[derive(Debug, Display, Error, Getters)]
#[display("parse rejected under {profile}: {detail}")]
pub struct ParseRejectedSource {
    /// The serialization profile the input was checked against.
    #[error(ignore)]
    profile: String,
    /// Why the input was rejected.
    #[error(ignore)]
    detail: String,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ParseRejectedSource {
    /// Record a rejected parse attempt, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(profile, detail)))]
    pub fn new(profile: impl Into<String>, detail: impl Into<String>) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            profile: profile.into(),
            detail: detail.into(),
            line,
            file,
        }
    }
}

/// Input text failed to parse as an integer sub-component of a
/// requested profile's textual grammar, preserving the real
/// `std::num::ParseIntError` as `source`.
#[derive(Debug, Display, Error, Getters)]
#[display("parse rejected under {profile}: {source}")]
pub struct ParseIntRejectedSource {
    /// The serialization profile the input was checked against.
    #[error(ignore)]
    profile: String,
    source: std::num::ParseIntError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ParseIntRejectedSource {
    /// Preserve a rejected integer sub-component, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(profile, source)))]
    pub fn new(profile: impl Into<String>, source: std::num::ParseIntError) -> Self {
        #[cfg(kani)]
        let (line, file) = (0, String::new());
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            profile: profile.into(),
            source,
            line,
            file,
        }
    }
}
