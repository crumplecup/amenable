//! `ExtType` registration for [`jiff::Timestamp`].

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::Timestamp,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.Timestamp.html",
    "A Timestamp is an instant in time represented as a signed count of \
     nanoseconds since the Unix epoch, always in the Unix timescale at a \
     UTC offset of zero."
);

register_ext_standard_evidence!(jiff::Timestamp);
