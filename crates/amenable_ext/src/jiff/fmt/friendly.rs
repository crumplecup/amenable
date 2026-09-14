//! `ExtType` registration for [`jiff::fmt::friendly`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::fmt::friendly::Designator,
    "jiff contributors",
    "jiff",
    "jiff::fmt::friendly",
    "https://docs.rs/jiff/latest/jiff/fmt/friendly/enum.Designator.html",
    "Configuration for SpanPrinter::designator, controlling whether unit \
     designators in the \"friendly\" duration format are verbose, \
     compact, or short."
);

impl_ext_type!(
    jiff::fmt::friendly::Direction,
    "jiff contributors",
    "jiff",
    "jiff::fmt::friendly",
    "https://docs.rs/jiff/latest/jiff/fmt/friendly/enum.Direction.html",
    "Configuration for SpanPrinter::direction, controlling whether a \
     sign appears before or after a \"friendly\" formatted span."
);

impl_ext_type!(
    jiff::fmt::friendly::FractionalUnit,
    "jiff contributors",
    "jiff",
    "jiff::fmt::friendly",
    "https://docs.rs/jiff/latest/jiff/fmt/friendly/enum.FractionalUnit.html",
    "Configuration for SpanPrinter::fractional, controlling which unit \
     (if any) may be printed with a fractional component."
);

impl_ext_type!(
    jiff::fmt::friendly::Spacing,
    "jiff contributors",
    "jiff",
    "jiff::fmt::friendly",
    "https://docs.rs/jiff/latest/jiff/fmt/friendly/enum.Spacing.html",
    "Configuration for SpanPrinter::spacing, controlling whitespace \
     between components of a \"friendly\" formatted span."
);

impl_ext_type!(
    jiff::fmt::friendly::SpanParser,
    "jiff contributors",
    "jiff",
    "jiff::fmt::friendly",
    "https://docs.rs/jiff/latest/jiff/fmt/friendly/struct.SpanParser.html",
    "A parser for jiff's \"friendly\" (human-readable) duration format, \
     e.g. \"1 year, 2 months\"."
);

impl_ext_type!(
    jiff::fmt::friendly::SpanPrinter,
    "jiff contributors",
    "jiff",
    "jiff::fmt::friendly",
    "https://docs.rs/jiff/latest/jiff/fmt/friendly/struct.SpanPrinter.html",
    "A printer for jiff's \"friendly\" (human-readable) duration format, \
     e.g. \"1 year, 2 months\"."
);

register_ext_standard_evidence!(
    jiff::fmt::friendly::Designator,
    jiff::fmt::friendly::Direction,
    jiff::fmt::friendly::FractionalUnit,
    jiff::fmt::friendly::Spacing,
    jiff::fmt::friendly::SpanParser,
    jiff::fmt::friendly::SpanPrinter,
);
