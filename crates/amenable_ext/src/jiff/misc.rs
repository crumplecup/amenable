//! `ExtType` registration for jiff's root-level error and duration types
//! not otherwise grouped with `Span`/`Timestamp`/`Zoned`.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::Error,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.Error.html",
    "The single error type used throughout this crate for overflow, time \
     zone database lookup failure, configuration problems, I/O errors, \
     and parse errors."
);

impl_ext_type!(
    jiff::RoundMode,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/enum.RoundMode.html",
    "The mode for dealing with the remainder when rounding datetimes or \
     spans."
);

impl_ext_type!(
    jiff::SignedDuration,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html",
    "A signed duration of time represented as a 96-bit integer of \
     nanoseconds, unattached to any calendar or time zone, unlike Span."
);

impl_ext_type!(
    jiff::SignedDurationRound,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.SignedDurationRound.html",
    "Options for SignedDuration::round."
);

register_ext_standard_evidence!(
    jiff::Error,
    jiff::RoundMode,
    jiff::SignedDuration,
    jiff::SignedDurationRound,
);
