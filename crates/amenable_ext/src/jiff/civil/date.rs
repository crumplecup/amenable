//! `ExtType` registration for [`jiff::civil::Date`] and its supporting
//! types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::civil::Date,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.Date.html",
    "A representation of a civil date in the Gregorian calendar: a year, \
     month, and day, with no notion of time zone or clock time."
);

impl_ext_type!(
    jiff::civil::DateArithmetic,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateArithmetic.html",
    "Options for civil::Date::checked_add and civil::Date::checked_sub."
);

impl_ext_type!(
    jiff::civil::DateDifference,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateDifference.html",
    "Options for civil::Date::since and civil::Date::until."
);

impl_ext_type!(
    jiff::civil::DateSeries,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateSeries.html",
    "An iterator over periodic dates, created by civil::Date::series."
);

impl_ext_type!(
    jiff::civil::DateWith,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.DateWith.html",
    "A builder for setting the fields on a civil::Date, created by \
     civil::Date::with."
);

register_ext_standard_evidence!(
    jiff::civil::Date,
    jiff::civil::DateArithmetic,
    jiff::civil::DateDifference,
    jiff::civil::DateSeries,
    jiff::civil::DateWith,
);
