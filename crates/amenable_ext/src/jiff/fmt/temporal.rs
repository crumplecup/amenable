//! `ExtType` registration for [`jiff::fmt::temporal`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::fmt::temporal::DateTimeParser,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.DateTimeParser.html",
    "A parser for Temporal (RFC 9557 / ISO 8601) datetimes."
);

impl_ext_type!(
    jiff::fmt::temporal::DateTimePrinter,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.DateTimePrinter.html",
    "A printer for Temporal (RFC 9557 / ISO 8601) datetimes."
);

impl_ext_type!(
    jiff::fmt::temporal::Pieces<'static>,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.Pieces.html",
    "A low level representation of a parsed Temporal ISO 8601 datetime \
     string, preserving distinctions (like a -00:00 offset) that a fully \
     resolved Timestamp/Zoned would lose."
);

impl_ext_type!(
    jiff::fmt::temporal::PiecesNumericOffset,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.PiecesNumericOffset.html",
    "A specific numeric offset, including its sign, for use with Pieces."
);

impl_ext_type!(
    jiff::fmt::temporal::PiecesOffset,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/enum.PiecesOffset.html",
    "An offset parsed from a Temporal ISO 8601 datetime string, for use \
     with Pieces."
);

impl_ext_type!(
    jiff::fmt::temporal::SpanParser,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.SpanParser.html",
    "A parser for Temporal (ISO 8601) durations."
);

impl_ext_type!(
    jiff::fmt::temporal::SpanPrinter,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.SpanPrinter.html",
    "A printer for Temporal (ISO 8601) durations."
);

impl_ext_type!(
    jiff::fmt::temporal::TimeZoneAnnotation<'static>,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.TimeZoneAnnotation.html",
    "An RFC 9557 time zone annotation, for use with Pieces."
);

impl_ext_type!(
    jiff::fmt::temporal::TimeZoneAnnotationKind<'static>,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/enum.TimeZoneAnnotationKind.html",
    "The kind of time zone found in an RFC 9557 timestamp, for use with \
     Pieces."
);

impl_ext_type!(
    jiff::fmt::temporal::TimeZoneAnnotationName<'static>,
    "jiff contributors",
    "jiff",
    "jiff::fmt::temporal",
    "https://docs.rs/jiff/latest/jiff/fmt/temporal/struct.TimeZoneAnnotationName.html",
    "A time zone annotation name (an IANA identifier or a fixed offset) \
     parsed from a datetime string."
);

register_ext_standard_evidence!(
    jiff::fmt::temporal::DateTimeParser,
    jiff::fmt::temporal::DateTimePrinter,
    jiff::fmt::temporal::Pieces<'static>,
    jiff::fmt::temporal::PiecesNumericOffset,
    jiff::fmt::temporal::PiecesOffset,
    jiff::fmt::temporal::SpanParser,
    jiff::fmt::temporal::SpanPrinter,
    jiff::fmt::temporal::TimeZoneAnnotation<'static>,
    jiff::fmt::temporal::TimeZoneAnnotationKind<'static>,
    jiff::fmt::temporal::TimeZoneAnnotationName<'static>,
);
