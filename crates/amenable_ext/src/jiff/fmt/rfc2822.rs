//! `ExtType` registration for [`jiff::fmt::rfc2822`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::fmt::rfc2822::DateTimeParser,
    "jiff contributors",
    "jiff",
    "jiff::fmt::rfc2822",
    "https://docs.rs/jiff/latest/jiff/fmt/rfc2822/struct.DateTimeParser.html",
    "A parser for RFC 2822 datetimes, the format used in email headers \
     such as Date."
);

impl_ext_type!(
    jiff::fmt::rfc2822::DateTimePrinter,
    "jiff contributors",
    "jiff",
    "jiff::fmt::rfc2822",
    "https://docs.rs/jiff/latest/jiff/fmt/rfc2822/struct.DateTimePrinter.html",
    "A printer for RFC 2822 datetimes, the format used in email headers \
     such as Date."
);

register_ext_standard_evidence!(
    jiff::fmt::rfc2822::DateTimeParser,
    jiff::fmt::rfc2822::DateTimePrinter,
);
