//! `ExtType` registration for [`jiff::civil::Weekday`] and its
//! supporting iterator types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::civil::Weekday,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/enum.Weekday.html",
    "A representation for the day of the week."
);

impl_ext_type!(
    jiff::civil::WeekdaysForward,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.WeekdaysForward.html",
    "An unending iterator of the days of the week, created by \
     civil::Weekday::cycle_forward."
);

impl_ext_type!(
    jiff::civil::WeekdaysReverse,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.WeekdaysReverse.html",
    "An unending iterator of the days of the week in reverse, created by \
     civil::Weekday::cycle_reverse."
);

register_ext_standard_evidence!(
    jiff::civil::Weekday,
    jiff::civil::WeekdaysForward,
    jiff::civil::WeekdaysReverse,
);
