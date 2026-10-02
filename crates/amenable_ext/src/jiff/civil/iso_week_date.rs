//! `ExtType` registration for [`jiff::civil::ISOWeekDate`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::civil::ISOWeekDate,
    "jiff contributors",
    "jiff",
    "jiff::civil",
    "https://docs.rs/jiff/latest/jiff/civil/struct.ISOWeekDate.html",
    "A type representing an ISO 8601 week date, an alternative \
     week-based calendar system tied to the Gregorian calendar."
);

register_ext_standard_evidence!(jiff::civil::ISOWeekDate);
