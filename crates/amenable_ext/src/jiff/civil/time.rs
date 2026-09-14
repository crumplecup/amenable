//! `ExtType` registration for [`jiff::civil::Time`] and its supporting
//! types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::civil::Time,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.Time.html",
    "A representation of civil \"wall clock\" time: hours, minutes, \
     seconds, and fractional seconds, with no notion of time zone or \
     calendar date."
);

impl_ext_type!(
    jiff::civil::TimeArithmetic,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.TimeArithmetic.html",
    "Options for civil::Time::checked_add and civil::Time::checked_sub."
);

impl_ext_type!(
    jiff::civil::TimeDifference,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.TimeDifference.html",
    "Options for civil::Time::since and civil::Time::until."
);

impl_ext_type!(
    jiff::civil::TimeRound,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.TimeRound.html",
    "Options for civil::Time::round."
);

impl_ext_type!(
    jiff::civil::TimeSeries,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.TimeSeries.html",
    "An iterator over periodic times, created by civil::Time::series."
);

impl_ext_type!(
    jiff::civil::TimeWith,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.TimeWith.html",
    "A builder for setting the fields on a civil::Time, created by \
     civil::Time::with."
);

register_ext_standard_evidence!(
    jiff::civil::Time,
    jiff::civil::TimeArithmetic,
    jiff::civil::TimeDifference,
    jiff::civil::TimeRound,
    jiff::civil::TimeSeries,
    jiff::civil::TimeWith,
);
