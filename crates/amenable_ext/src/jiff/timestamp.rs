//! `ExtType` registration for [`jiff::Timestamp`] and its supporting types.

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

impl_ext_type!(
    jiff::TimestampArithmetic,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.TimestampArithmetic.html",
    "Options for Timestamp::checked_add and Timestamp::checked_sub, \
     constructible via its From impls for Span, SignedDuration, and \
     std::time::Duration."
);

impl_ext_type!(
    jiff::TimestampDifference,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.TimestampDifference.html",
    "Options for Timestamp::since and Timestamp::until."
);

impl_ext_type!(
    jiff::TimestampDisplayWithOffset,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.TimestampDisplayWithOffset.html",
    "A type for formatting a Timestamp with a specific offset, returned by \
     Timestamp::display_with_offset."
);

impl_ext_type!(
    jiff::TimestampRound,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.TimestampRound.html",
    "Options for Timestamp::round."
);

impl_ext_type!(
    jiff::TimestampSeries,
    "jiff contributors",
    "jiff",
    "jiff",
    "https://docs.rs/jiff/latest/jiff/struct.TimestampSeries.html",
    "An iterator over periodic timestamps, created by Timestamp::series."
);

register_ext_standard_evidence!(
    jiff::Timestamp,
    jiff::TimestampArithmetic,
    jiff::TimestampDifference,
    jiff::TimestampDisplayWithOffset,
    jiff::TimestampRound,
    jiff::TimestampSeries,
);
