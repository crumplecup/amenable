//! Parse-rejection error sources shared across backends.

use derive_getters::Getters;
use derive_more::{Display, Error};

/// Which textual serialization form input was checked against.
///
/// Every real call site in this workspace passes one of these twelve
/// literal names -- confirmed directly, not a guess: `profile` was a
/// bare `String` until this enum replaced it, and every one of its
/// construction sites (`ParseRejectedSource::new`/
/// `JiffParseRejectedSource::new`, directly or through
/// `parse_temporal_pieces`/`pieces_to_offset_date_time`/
/// `reject_zone_annotation`'s own `profile: &'static str` parameters)
/// passed a compile-time string literal, never runtime text. A closed
/// set like this belongs in a typed enum, not a stringified dumping
/// ground.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum ParseProfile {
    /// ISO 8601 duration.
    #[display("ISO 8601 duration")]
    Iso8601Duration,
    /// ISO 8601 calendar date.
    #[display("ISO 8601 calendar date")]
    Iso8601CalendarDate,
    /// ISO 8601 local time.
    #[display("ISO 8601 local time")]
    Iso8601LocalTime,
    /// ISO 8601 local date-time.
    #[display("ISO 8601 local date-time")]
    Iso8601LocalDateTime,
    /// ISO 8601 ordinal date.
    #[display("ISO 8601 ordinal date")]
    Iso8601OrdinalDate,
    /// ISO 8601 week date.
    #[display("ISO 8601 week date")]
    Iso8601WeekDate,
    /// ISO 8601 / RFC 9557 UTC offset.
    #[display("ISO 8601 / RFC 9557 UTC offset")]
    Iso8601Rfc9557UtcOffset,
    /// ISO 8601 reduced-precision calendar date.
    #[display("ISO 8601 reduced-precision calendar date")]
    Iso8601ReducedPrecisionCalendarDate,
    /// ISO 8601 reduced-precision local time.
    #[display("ISO 8601 reduced-precision local time")]
    Iso8601ReducedPrecisionLocalTime,
    /// ISO 8601 offset date-time.
    #[display("ISO 8601 offset date-time")]
    Iso8601OffsetDateTime,
    /// RFC 3339 timestamp.
    #[display("RFC 3339 timestamp")]
    Rfc3339Timestamp,
    /// RFC 9557 IXDTF timestamp.
    #[display("RFC 9557 IXDTF timestamp")]
    Rfc9557IxdtfTimestamp,
}

/// Why a parse attempt was rejected, closed per profile's own grammar
/// checks.
///
/// Confirmed directly, not assumed: every real call site that used to
/// build a `detail: String` via `format!()` constructed one of these
/// exact phrases, never genuinely free text -- the varying part was
/// always a small, closed set of grammar-check failures, each applied
/// to whatever input text was being checked. `MonthOutOfRange` is the
/// one case that also carries its own extra value (the real rejected
/// month); every other reason needs no payload of its own, since the
/// offending text itself lives on `ParseRejectedSource` separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum ParseRejectionReason {
    /// Input text is not ASCII.
    #[display("is not ASCII")]
    NotAscii,
    /// Not a 7-digit or 8-character (`YYYY-DDD`) ordinal date.
    #[display("is not a 7-digit or 8-character (YYYY-DDD) ordinal date")]
    NotSevenOrEightCharOrdinalDate,
    /// The year slice could not be sliced out of the input text.
    #[display("has an invalid year slice")]
    InvalidYearSlice,
    /// The day-of-year slice could not be sliced out of the input text.
    #[display("has an invalid day-of-year slice")]
    InvalidDayOfYearSlice,
    /// Not an 8-character (`YYYYWwwD`) or 10-character (`YYYY-Www-D`)
    /// ISO week date.
    #[display("is not an 8-character (YYYYWwwD) or 10-character (YYYY-Www-D) ISO week date")]
    NotEightOrTenCharWeekDate,
    /// The week-year slice could not be sliced out of the input text.
    #[display("has an invalid week-year slice")]
    InvalidWeekYearSlice,
    /// The week-number slice could not be sliced out of the input text.
    #[display("has an invalid week-number slice")]
    InvalidWeekNumberSlice,
    /// The weekday slice could not be sliced out of the input text.
    #[display("has an invalid weekday slice")]
    InvalidWeekdaySlice,
    /// Too short to be a UTC offset.
    #[display("is too short to be a UTC offset")]
    TooShortForUtcOffset,
    /// Does not start with `+`, `-`, or `Z`.
    #[display("must start with '+', '-', or 'Z'")]
    MustStartWithSignOrZ,
    /// The offset-hours slice could not be sliced out of the input text.
    #[display("has an invalid offset-hours slice")]
    InvalidOffsetHoursSlice,
    /// The offset-minutes slice could not be sliced out of the input text.
    #[display("has an invalid offset-minutes slice")]
    InvalidOffsetMinutesSlice,
    /// The real rejected month, out of range `1..=12`.
    #[display("is out of range 1..=12 for month {_0}")]
    MonthOutOfRange(u8),
    /// The hour slice could not be sliced out of the input text.
    #[display("has an invalid hour slice")]
    InvalidHourSlice,
    /// The minute slice could not be sliced out of the input text.
    #[display("has an invalid minute slice")]
    InvalidMinuteSlice,
    /// jiff's own decomposed `Pieces` carried no time-of-day component.
    #[display("has no time-of-day component")]
    NoTimeOfDayComponent,
    /// jiff's own decomposed `Pieces` carried no UTC offset.
    #[display("has no UTC offset")]
    NoUtcOffset,
    /// Carries an RFC 9557 zone annotation, not valid under this profile.
    #[display("carries an RFC 9557 zone annotation, not valid under this profile")]
    CarriesZoneAnnotation,
}

/// Input text does not match the requested profile's lexical shape at
/// all -- a fresh, synthesized diagnostic, not a discarded real error:
/// several of this profile's textual forms have no parser of their own
/// at all to delegate to (jiff's own `FromStr` doesn't accept them), so
/// there is no foreign error object here to preserve in the first
/// place. [`ParseIntRejectedSource`]/`JiffParseRejectedSource` (the
/// latter only under the `jiff` feature) are the sibling variants for
/// when a real foreign parse error does exist.
#[derive(Debug, Display, Error, Getters)]
#[cfg_attr(
    not(kani),
    display("parse rejected under {profile}: {offending_text:?} {reason}")
)]
#[cfg_attr(kani, display("parse rejected under {profile}: {reason}"))]
pub struct ParseRejectedSource {
    /// The serialization profile the input was checked against.
    #[error(ignore)]
    profile: ParseProfile,
    /// The closed grammar-check failure this input tripped.
    #[error(ignore)]
    reason: ParseRejectionReason,
    /// The real input text that was rejected. Dropped entirely under
    /// `cfg(kani)`, same reason `line`/`file` are: nothing in this
    /// codebase ever reads it under Kani, there is no closed set to
    /// type it as (it's genuinely arbitrary input), and a
    /// `String`-typed field is the exact shape `gallery::
    /// boxed_error_variant_count` already confirmed costly to model
    /// through `Box<TemporalErrorKind>`, regardless of which variant a
    /// given call site actually constructs.
    #[cfg(not(kani))]
    #[error(ignore)]
    offending_text: String,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl ParseRejectedSource {
    /// Record a rejected parse attempt, capturing the caller's
    /// location.
    #[track_caller]
    #[cfg_attr(
        not(kani),
        tracing::instrument(level = "debug", skip(profile, reason, offending_text))
    )]
    pub fn new(
        profile: ParseProfile,
        reason: ParseRejectionReason,
        offending_text: impl Into<String>,
    ) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        #[cfg(not(kani))]
        let offending_text = offending_text.into();
        #[cfg(kani)]
        let _ = offending_text;
        Self {
            profile,
            reason,
            #[cfg(not(kani))]
            offending_text,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
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
    profile: ParseProfile,
    source: std::num::ParseIntError,
    /// Source line of the call site that produced this error.
    #[cfg(not(kani))]
    line: u32,
    /// Source file of the call site that produced this error.
    #[cfg(not(kani))]
    file: String,
}

impl ParseIntRejectedSource {
    /// Preserve a rejected integer sub-component, capturing the
    /// caller's location.
    #[track_caller]
    #[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(profile, source)))]
    pub fn new(profile: ParseProfile, source: std::num::ParseIntError) -> Self {
        #[cfg(not(kani))]
        let (line, file) = {
            let loc = std::panic::Location::caller();
            (loc.line(), loc.file().to_string())
        };
        Self {
            profile,
            source,
            #[cfg(not(kani))]
            line,
            #[cfg(not(kani))]
            file,
        }
    }
}
