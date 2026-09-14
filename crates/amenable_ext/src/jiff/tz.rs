//! `ExtType` registration for [`jiff::tz`], split into one module per
//! real `jiff::tz` area (`TimeZone` here; `ambiguous`/`offset` in their
//! own sibling files).

mod ambiguous;
mod offset;

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    jiff::tz::TimeZone,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html",
    "A representation of a time zone, either a fixed offset, a named \
     IANA time zone with its own DST rules, or POSIX time zone string."
);

impl_ext_type!(
    jiff::tz::TimeZoneDatabase,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneDatabase.html",
    "A handle to an IANA Time Zone Database, used to look up TimeZone \
     values by name (e.g. via TimeZoneDatabase::get) and to enumerate \
     available names (via TimeZoneDatabase::available)."
);

impl_ext_type!(
    jiff::tz::TimeZoneFollowingTransitions<'static>,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneFollowingTransitions.html",
    "An iterator over time zone transitions going forward in time, \
     created by TimeZone::following."
);

impl_ext_type!(
    jiff::tz::TimeZoneName<'static>,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneName.html",
    "A time zone identifier name yielded by the TimeZoneNameIter \
     iterator."
);

impl_ext_type!(
    jiff::tz::TimeZoneNameIter<'static>,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneNameIter.html",
    "An iterator over the time zone identifiers in a TimeZoneDatabase, \
     created by TimeZoneDatabase::available."
);

impl_ext_type!(
    jiff::tz::TimeZoneOffsetInfo<'static>,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneOffsetInfo.html",
    "An offset along with its DST status and time zone abbreviation, \
     created by TimeZone::to_offset_info."
);

impl_ext_type!(
    jiff::tz::TimeZonePrecedingTransitions<'static>,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZonePrecedingTransitions.html",
    "An iterator over time zone transitions going backward in time, \
     created by TimeZone::preceding."
);

impl_ext_type!(
    jiff::tz::TimeZoneTransition<'static>,
    "jiff contributors",
    "jiff",
    "jiff::tz",
    "https://docs.rs/jiff/latest/jiff/tz/struct.TimeZoneTransition.html",
    "A representation of a single time zone transition, yielded by \
     TimeZoneFollowingTransitions/TimeZonePrecedingTransitions."
);

register_ext_standard_evidence!(
    jiff::tz::TimeZone,
    jiff::tz::TimeZoneDatabase,
    jiff::tz::TimeZoneFollowingTransitions<'static>,
    jiff::tz::TimeZoneName<'static>,
    jiff::tz::TimeZoneNameIter<'static>,
    jiff::tz::TimeZoneOffsetInfo<'static>,
    jiff::tz::TimeZonePrecedingTransitions<'static>,
    jiff::tz::TimeZoneTransition<'static>,
);
