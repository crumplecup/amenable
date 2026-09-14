//! `ExtType` registration for [`jiff::tz::AmbiguousOffset`] and its
//! sibling ambiguity-handling types.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::tz::AmbiguousOffset,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/enum.AmbiguousOffset.html",
    "A possibly ambiguous Offset, corresponding to either a single \
     unambiguous offset, a fold (two valid offsets), or a gap (no valid \
     offset) around a DST transition."
);

impl_ext_type!(
    jiff::tz::AmbiguousTimestamp,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.AmbiguousTimestamp.html",
    "A possibly ambiguous Timestamp, created by \
     TimeZone::to_ambiguous_timestamp."
);

impl_ext_type!(
    jiff::tz::AmbiguousZoned,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.AmbiguousZoned.html",
    "A possibly ambiguous Zoned, created by TimeZone::to_ambiguous_zoned."
);

impl_ext_type!(
    jiff::tz::Disambiguation,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/enum.Disambiguation.html",
    "Configuration for resolving ambiguous datetimes (folds and gaps) in \
     a particular time zone."
);

register_ext_standard_evidence!(
    jiff::tz::AmbiguousOffset,
    jiff::tz::AmbiguousTimestamp,
    jiff::tz::AmbiguousZoned,
    jiff::tz::Disambiguation,
);
