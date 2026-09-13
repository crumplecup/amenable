//! `ExtType` registration for [`jiff::civil::DateTime`].

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

register_ext_standard_evidence!(jiff::civil::DateTime);
