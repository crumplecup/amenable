//! `ExtType` registration for [`jiff::Span`] and its supporting types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::Span,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.Span.html",
    "A span of time represented via a mixture of calendar and clock \
     units (years, months, weeks, days, hours, minutes, seconds, \
     milliseconds, microseconds, and nanoseconds); used as inputs to \
     routines like Zoned::checked_add and as outputs from routines like \
     Timestamp::since."
);

impl_ext_type!(
    jiff::SpanArithmetic<'static>,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SpanArithmetic.html",
    "Options for Span::checked_add and Span::checked_sub, constructible \
     via its From impls for Span and, with a relative date, \
     civil::Date/civil::DateTime/Zoned/SpanRelativeTo."
);

impl_ext_type!(
    jiff::SpanCompare<'static>,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SpanCompare.html",
    "Options for Span::compare, which compares two spans based on their \
     actual elapsed time rather than their individual field values."
);

impl_ext_type!(
    jiff::SpanFieldwise,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SpanFieldwise.html",
    "A wrapper for Span that implements the Hash, Eq, and PartialEq \
     traits by comparing unit values directly, rather than the actual \
     elapsed time Span::compare uses."
);

impl_ext_type!(
    jiff::SpanRelativeTo<'static>,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SpanRelativeTo.html",
    "A relative datetime for use with Span APIs that need one to resolve \
     calendar units (years, months, weeks, or days) unambiguously."
);

impl_ext_type!(
    jiff::SpanRound<'static>,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SpanRound.html",
    "Options for Span::round."
);

impl_ext_type!(
    jiff::SpanTotal<'static>,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SpanTotal.html",
    "Options for Span::total, which computes the total duration of a \
     span in a single specified unit."
);

impl_ext_type!(
    jiff::Unit,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/enum.Unit.html",
    "A way to refer to a single calendar or clock unit, from years down \
     to nanoseconds."
);

register_ext_standard_evidence!(
    jiff::Span,
    jiff::SpanArithmetic<'static>,
    jiff::SpanCompare<'static>,
    jiff::SpanFieldwise,
    jiff::SpanRelativeTo<'static>,
    jiff::SpanRound<'static>,
    jiff::SpanTotal<'static>,
    jiff::Unit,
);
