//! [`UnsupportedCase`] -- the closed, enumerable set of backend-
//! capability gaps that carry no runtime-variable payload. Split out
//! of `capability.rs` once this enum grew past the point a human
//! reader could take it in alongside `UnsupportedDetail`/
//! `UnsupportedSource` in the same file.

use derive_more::Display;

/// A closed set of backend-capability gaps that carry no runtime-
/// variable payload -- the operation is unsupported for a fixed,
/// enumerable reason, not a situational one needing interpolated data.
/// Named so each gap is a real, matchable discriminant rather than an
/// ad-hoc string literal repeated at its call site.
///
/// Grouped by the real wrapper function each group used to flow
/// through before this fix (`unsupported_format`/`unsupported_parse`/
/// `unsupported_interval_endpoint_parse` in the jiff backend,
/// `unsupported_parse` in the std canary) -- every one of those took a
/// bare `&'static str` "edge" naming which closed case applied, which
/// is exactly the un-typed dispatch this project's own architecture
/// thesis asks named types to replace. The wrapper functions
/// themselves are gone now; each call site names its own variant here
/// directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum UnsupportedCase {
    /// `chrono::FixedOffset` cannot represent RFC 9557's unknown-local-
    /// offset case.
    #[display(
        "chrono::FixedOffset has no representation for the RFC 9557 unknown-local-offset case"
    )]
    ChronoUnknownLocalOffset,
    /// `jiff::tz::Offset` cannot represent RFC 9557's unknown-local-
    /// offset case.
    #[display(
        "jiff::tz::Offset has no representation for the RFC 9557 unknown-local-offset case"
    )]
    JiffUnknownLocalOffset,
    /// The jiff backend has no representation for an explicitly
    /// qualified (ISO 8601-2) interval-endpoint value.
    #[display(
        "jiff backend has no representation for an explicitly qualified (ISO 8601-2) interval-endpoint value"
    )]
    JiffQualifiedIntervalEndpoint,
    /// A `jiff::tz::TimeZone` has no IANA identifier to decompose into
    /// a named-zone descriptor (it is unknown, or a fixed offset).
    #[display(
        "this jiff::tz::TimeZone has no IANA identifier to decompose into a named-zone descriptor (it is unknown, or a fixed offset)"
    )]
    JiffUnidentifiedNamedZone,
    /// The `std::time` canary carries whole-second durations only.
    #[display("the std::time canary carries whole-second durations only")]
    StdCanaryWholeSecondDurationsOnly,
    /// The `std::time` canary resolves complete calendar dates only,
    /// not ordinal or week dates.
    #[display("std::time canary resolves complete calendar dates only, not ordinal or week dates")]
    StdCanaryCompleteCalendarDatesOnly,

    // ── jiff backend: cannot format (formerly `unsupported_format`) ──
    /// A CalConnect explicit date-with-shift value cannot be formatted.
    #[display(
        "jiff backend cannot format a CalConnect explicit date-with-shift value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatCalConnectDateWithShift,
    /// A CalConnect explicit time-of-day-with-shift value cannot be
    /// formatted.
    #[display(
        "jiff backend cannot format a CalConnect explicit time-of-day-with-shift value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatCalConnectTimeOfDayWithShift,
    /// An ISO 8601-2 extended year cannot be formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 extended year: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601ExtendedYear,
    /// A Gregorian decade cannot be formatted.
    #[display(
        "jiff backend cannot format a Gregorian decade: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatGregorianDecade,
    /// A Gregorian century cannot be formatted.
    #[display(
        "jiff backend cannot format a Gregorian century: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatGregorianCentury,
    /// An ISO 8601-2 explicitly-qualified temporal value cannot be
    /// formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 explicitly-qualified temporal value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601QualifiedTemporalValue,
    /// An ISO 8601-2 seasonal expression cannot be formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 seasonal expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601SeasonalExpression,
    /// An ISO 8601-2 sub-year grouping expression cannot be formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 sub-year grouping expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601SubYearGroupingExpression,
    /// An ISO 8601-2 unspecified-component expression cannot be
    /// formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 unspecified-component expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601UnspecifiedComponentExpression,
    /// An ISO 8601-2 temporal set cannot be formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 temporal set: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601TemporalSet,
    /// An ISO 8601-2 grouped time-scale unit expression cannot be
    /// formatted.
    #[display(
        "jiff backend cannot format an ISO 8601-2 grouped time-scale unit expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatIso8601GroupedTimeScaleUnit,
    /// A CalConnect date-time formula cannot be formatted.
    #[display(
        "jiff backend cannot format a CalConnect date-time formula: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatCalConnectDateTimeFormula,
    /// A reduced local time with a fractional hour/minute component
    /// cannot be formatted.
    #[display(
        "jiff backend cannot format a reduced local time with a fractional hour/minute component: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatReducedLocalTimeFractionalComponent,
    /// An explicitly qualified (ISO 8601-2) interval-endpoint value
    /// cannot be formatted.
    #[display(
        "jiff backend cannot format an explicitly qualified (ISO 8601-2) interval-endpoint value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatQualifiedIntervalEndpointValue,
    /// An `Open`/`Unknown` interval boundary cannot be formatted (no
    /// independently sourced textual convention).
    #[display(
        "jiff backend cannot format an Open/Unknown interval boundary (no independently sourced textual convention): outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    FormatOpenOrUnknownIntervalBoundary,

    // ── jiff backend: cannot parse (formerly `unsupported_parse`) ───
    /// An ISO 8601-2 extended year cannot be parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 extended year: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601ExtendedYear,
    /// A Gregorian decade cannot be parsed.
    #[display(
        "jiff backend cannot parse a Gregorian decade: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseGregorianDecade,
    /// A Gregorian century cannot be parsed.
    #[display(
        "jiff backend cannot parse a Gregorian century: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseGregorianCentury,
    /// An ISO 8601-2 explicitly-qualified temporal value cannot be
    /// parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 explicitly-qualified temporal value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601QualifiedTemporalValue,
    /// A CalConnect explicit date-with-shift value cannot be parsed.
    #[display(
        "jiff backend cannot parse a CalConnect explicit date-with-shift value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseCalConnectDateWithShift,
    /// A CalConnect explicit time-of-day-with-shift value cannot be
    /// parsed.
    #[display(
        "jiff backend cannot parse a CalConnect explicit time-of-day-with-shift value: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseCalConnectTimeOfDayWithShift,
    /// An ISO 8601-2 seasonal expression cannot be parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 seasonal expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601SeasonalExpression,
    /// An ISO 8601-2 sub-year grouping expression cannot be parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 sub-year grouping expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601SubYearGroupingExpression,
    /// An ISO 8601-2 unspecified-component expression cannot be
    /// parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 unspecified-component expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601UnspecifiedComponentExpression,
    /// An ISO 8601-2 temporal set cannot be parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 temporal set: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601TemporalSet,
    /// An ISO 8601-2 grouped time-scale unit expression cannot be
    /// parsed.
    #[display(
        "jiff backend cannot parse an ISO 8601-2 grouped time-scale unit expression: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseIso8601GroupedTimeScaleUnit,
    /// A CalConnect date-time formula cannot be parsed.
    #[display(
        "jiff backend cannot parse a CalConnect date-time formula: outside jiff's Level-1 Gregorian/IANA-zone model"
    )]
    ParseCalConnectDateTimeFormula,

    // ── jiff backend: interval-endpoint text parsing not yet built ──
    // (formerly `unsupported_interval_endpoint_parse`)
    /// Arbitrary interval-endpoint text parsing is not yet implemented
    /// for a recurring interval.
    #[display(
        "this backend cannot yet parse an arbitrary temporal value as an interval endpoint for a recurring interval: that requires TemporalParser (Phase 9), not yet built here"
    )]
    JiffIntervalEndpointParseRecurringInterval,
    /// Arbitrary interval-endpoint text parsing is not yet implemented
    /// for a time interval.
    #[display(
        "this backend cannot yet parse an arbitrary temporal value as an interval endpoint for a time interval: that requires TemporalParser (Phase 9), not yet built here"
    )]
    JiffIntervalEndpointParseTimeInterval,

    // ── std canary: no ISO 8601/RFC 3339 parser in the standard
    // library (formerly the std canary's own `unsupported_parse`) ───
    /// `std::time` has no ISO 8601 duration parser.
    #[display("std::time cannot parse an ISO 8601 duration: no ISO 8601 / RFC 3339 parser in the standard library")]
    StdParseIso8601Duration,
    /// `std::time` has no parser for a recurring interval.
    #[display("std::time cannot parse a recurring interval: no ISO 8601 / RFC 3339 parser in the standard library")]
    StdParseRecurringInterval,
    /// `std::time` has no parser for a time interval.
    #[display("std::time cannot parse a time interval: no ISO 8601 / RFC 3339 parser in the standard library")]
    StdParseTimeInterval,
}
