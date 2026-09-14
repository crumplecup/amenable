//! `ExtType` registration for [`jiff::Zoned`] and its supporting types.

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

impl_ext_type!(
    jiff::ZonedArithmetic,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.ZonedArithmetic.html",
    "Options for Zoned::checked_add and Zoned::checked_sub, constructible \
     via its From impls for Span, SignedDuration, and std::time::Duration."
);

impl_ext_type!(
    jiff::ZonedDifference<'static>,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.ZonedDifference.html",
    "Options for Zoned::since and Zoned::until."
);

impl_ext_type!(
    jiff::ZonedRound,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.ZonedRound.html",
    "Options for Zoned::round."
);

impl_ext_type!(
    jiff::ZonedSeries,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.ZonedSeries.html",
    "An iterator over periodic zoned datetimes, created by Zoned::series."
);

impl_ext_type!(
    jiff::ZonedWith,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.ZonedWith.html",
    "A builder for setting the fields on a Zoned, created by Zoned::with."
);

register_ext_standard_evidence!(
    jiff::Zoned,
    jiff::ZonedArithmetic,
    jiff::ZonedDifference<'static>,
    jiff::ZonedRound,
    jiff::ZonedSeries,
    jiff::ZonedWith,
);
