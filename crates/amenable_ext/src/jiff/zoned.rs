//! `ExtType` registration for [`jiff::Zoned`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::Zoned,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.Zoned.html",
    "A Zoned is a time zone aware instant in time, combining a Timestamp \
     for the precise instant, a civil DateTime for the calendar date and \
     clock time, and a TimeZone for how to apply time zone transitions \
     during arithmetic."
);

register_ext_standard_evidence!(jiff::Zoned);
