//! `ExtType` registration and `ExtStandard<T>` evidence for chrono's value types.
//!
//! Phase 2 registers the seven value types here. Each one is a concrete type
//! with its own witness on Kani, Creusot, and Verus, so the registrations are
//! added as their witnesses land.

use crate::macros::{impl_ext_type, register_ext_standard_evidence};

impl_ext_type!(
    chrono::NaiveDate,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html",
    "A calendar date without a time zone, in the proleptic Gregorian calendar."
);

impl_ext_type!(
    chrono::NaiveTime,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.NaiveTime.html",
    "A time of day without a date or time zone, with nanosecond precision and leap-second support."
);

impl_ext_type!(
    chrono::NaiveDateTime,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.NaiveDateTime.html",
    "A date and time of day without a time zone: a NaiveDate paired with a NaiveTime."
);

impl_ext_type!(
    chrono::NaiveWeek,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.NaiveWeek.html",
    "A span of seven days from a chosen week-start day, containing one calendar date."
);

impl_ext_type!(
    chrono::IsoWeek,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.IsoWeek.html",
    "An ISO 8601 week: its year and week number, obtained from a calendar date."
);

impl_ext_type!(
    chrono::Utc,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.Utc.html",
    "The UTC time zone: a zero-sized TimeZone whose offset is always zero."
);

impl_ext_type!(
    chrono::FixedOffset,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.FixedOffset.html",
    "A time zone with a fixed offset from UTC, from -23:59:59 to +23:59:59."
);

register_ext_standard_evidence!(
    chrono::NaiveDate,
    chrono::NaiveTime,
    chrono::NaiveDateTime,
    chrono::IsoWeek,
    chrono::NaiveWeek,
    chrono::Utc,
    chrono::FixedOffset,
);
