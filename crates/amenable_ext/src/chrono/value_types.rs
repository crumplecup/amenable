//! `ExtType` registration and `ExtStandard<T>` evidence for chrono's value types.
//!
//! Phase 2 registers the seven value types here. Each one is a concrete type
//! with its own witness on Kani, Creusot, and Verus, so the registrations are
//! added as their witnesses land. Phase 3 adds `Local`: it shares the instant
//! bridge with `Utc`/`FixedOffset` (decision 12), but needs its own value
//! registration and witnesses, the same as any other type.
//!
//! `DateTime<T>` is registered below for `T` in {`Utc`, `FixedOffset`,
//! `Local`} (`chrono_tz::Tz`'s own instantiation lives in
//! `chrono_tz_value_types.rs`, gated on the `chrono-tz` feature).
//! `Date<T>` is deliberately NOT registered for any `T`: it has been
//! `#[deprecated]` since chrono 0.4.23, and rather than suppress that
//! lint at every call site (or crate-wide) just to force coverage of
//! a type chrono itself is steering users away from, it's excluded by
//! exception instead -- the user's direct call.

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

impl_ext_type!(
    chrono::Local,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.Local.html",
    "The local time zone: takes its offset rules from the host's timezone database."
);

impl_ext_type!(
    chrono::DateTime<chrono::Utc>,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.DateTime.html",
    "An offset-aware fixed instant, here always at the zero UTC offset."
);

impl_ext_type!(
    chrono::DateTime<chrono::FixedOffset>,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.DateTime.html",
    "An offset-aware fixed instant, paired with the fixed offset it was recorded at."
);

impl_ext_type!(
    chrono::DateTime<chrono::Local>,
    "chrono contributors",
    "chrono",
    "chrono",
    "https://docs.rs/chrono/latest/chrono/struct.DateTime.html",
    "An offset-aware fixed instant, paired with the host's local offset at the time it was recorded."
);

register_ext_standard_evidence!(
    chrono::NaiveDate,
    chrono::NaiveTime,
    chrono::NaiveDateTime,
    chrono::IsoWeek,
    chrono::NaiveWeek,
    chrono::Utc,
    chrono::FixedOffset,
    chrono::Local,
    chrono::DateTime<chrono::Utc>,
    chrono::DateTime<chrono::FixedOffset>,
    chrono::DateTime<chrono::Local>,
);
