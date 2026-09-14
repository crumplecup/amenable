//! `ExtType` registration for [`jiff::tz::Offset`] and its supporting
//! types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::tz::Dst,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/enum.Dst.html",
    "An enum indicating whether a particular datetime is in daylight \
     saving time or not."
);

impl_ext_type!(
    jiff::tz::Offset,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.Offset.html",
    "Represents a fixed time zone offset from UTC, in seconds."
);

impl_ext_type!(
    jiff::tz::OffsetArithmetic,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.OffsetArithmetic.html",
    "Options for Offset::checked_add and Offset::checked_sub."
);

impl_ext_type!(
    jiff::tz::OffsetConflict,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/enum.OffsetConflict.html",
    "Configuration for resolving a conflict between an explicit offset \
     and a time zone attached to the same datetime."
);

impl_ext_type!(
    jiff::tz::OffsetRound,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.OffsetRound.html",
    "Options for Offset::round."
);

register_ext_standard_evidence!(
    jiff::tz::Dst,
    jiff::tz::Offset,
    jiff::tz::OffsetArithmetic,
    jiff::tz::OffsetConflict,
    jiff::tz::OffsetRound,
);
