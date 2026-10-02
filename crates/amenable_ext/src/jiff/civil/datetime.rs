//! `ExtType` registration for [`jiff::civil::DateTime`] and its
//! supporting types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::civil::DateTime,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html",
    "A civil DateTime is a representation of a datetime in the Gregorian \
     calendar as a pair of a Date and a Time, guaranteed to be valid, \
     and behaves without regard to daylight saving time or time zones."
);

impl_ext_type!(
    jiff::civil::DateTimeArithmetic,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeArithmetic.html",
    "Options for civil::DateTime::checked_add and \
     civil::DateTime::checked_sub."
);

impl_ext_type!(
    jiff::civil::DateTimeDifference,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeDifference.html",
    "Options for civil::DateTime::since and civil::DateTime::until."
);

impl_ext_type!(
    jiff::civil::DateTimeRound,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeRound.html",
    "Options for civil::DateTime::round."
);

impl_ext_type!(
    jiff::civil::DateTimeSeries,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeSeries.html",
    "An iterator over periodic civil datetimes, created by \
     civil::DateTime::series."
);

impl_ext_type!(
    jiff::civil::DateTimeWith,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeWith.html",
    "A builder for setting the fields on a civil::DateTime, created by \
     civil::DateTime::with."
);

register_ext_standard_evidence!(
    jiff::civil::DateTime,
    jiff::civil::DateTimeArithmetic,
    jiff::civil::DateTimeDifference,
    jiff::civil::DateTimeRound,
    jiff::civil::DateTimeSeries,
    jiff::civil::DateTimeWith,
);
