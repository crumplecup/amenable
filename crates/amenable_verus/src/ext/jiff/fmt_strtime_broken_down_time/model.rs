//! `BrokenDownTimeModel` itself: the struct, its ghost constructor,
//! the numeric-fields bundle, and the round-trip harness — split out
//! of `mod.rs`, which may only declare modules and re-exports.

#[cfg(verus_keep_ghost)]
use super::predicates::broken_down_time_new_matches_new_spec;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A model of `jiff::fmt::strtime::BrokenDownTime`'s 12 numeric
/// fields, using the exact same real bounds `amenable_kani::ext::jiff::
/// fmt_strtime_broken_down_time` and `amenable_creusot::ext_jiff::
/// fmt_strtime_broken_down_time` both check against jiff's real API.
pub struct BrokenDownTimeModel {
    /// Models `BrokenDownTime::year`.
    pub year: Option<i16>,
    /// Models `BrokenDownTime::month`.
    pub month: Option<i8>,
    /// Models `BrokenDownTime::day`.
    pub day: Option<i8>,
    /// Models `BrokenDownTime::day_of_year`.
    pub day_of_year: Option<i16>,
    /// Models `BrokenDownTime::iso_week_year`.
    pub iso_week_year: Option<i16>,
    /// Models `BrokenDownTime::iso_week`.
    pub iso_week: Option<i8>,
    /// Models `BrokenDownTime::sunday_based_week`.
    pub week_sun: Option<i8>,
    /// Models `BrokenDownTime::monday_based_week`.
    pub week_mon: Option<i8>,
    /// Models `BrokenDownTime::hour`.
    pub hour: Option<i8>,
    /// Models `BrokenDownTime::minute`.
    pub minute: Option<i8>,
    /// Models `BrokenDownTime::second`.
    pub second: Option<i8>,
    /// Models `BrokenDownTime::subsec_nanosecond`.
    pub subsec_nanosecond: Option<i32>,
}

impl BrokenDownTimeModel {
    /// The ghost value `new()` always produces: every field `None`.
    pub open spec fn new_spec() -> BrokenDownTimeModel {
        BrokenDownTimeModel {
            year: None,
            month: None,
            day: None,
            day_of_year: None,
            iso_week_year: None,
            iso_week: None,
            week_sun: None,
            week_mon: None,
            hour: None,
            minute: None,
            second: None,
            subsec_nanosecond: None,
        }
    }

    /// Models `BrokenDownTime::default()`: every field starts `None`.
    pub fn new() -> (result: BrokenDownTimeModel)
        ensures
            broken_down_time_new_matches_new_spec(result),
    {
        BrokenDownTimeModel {
            year: None,
            month: None,
            day: None,
            day_of_year: None,
            iso_week_year: None,
            iso_week: None,
            week_sun: None,
            week_mon: None,
            hour: None,
            minute: None,
            second: None,
            subsec_nanosecond: None,
        }
    }
}

/// The 12 numeric fields exercised, bundled into one parameter so
/// the verify function below stays under clippy's argument-count
/// limit — the same reason `amenable_kani::ext::jiff::
/// fmt_strtime_broken_down_time`'s own `BrokenDownTimeNumericFields`
/// exists.
pub struct BrokenDownTimeNumericFieldsModel {
    /// Value to try setting via `set_year`.
    pub year: i16,
    /// Value to try setting via `set_month`.
    pub month: i8,
    /// Value to try setting via `set_day`.
    pub day: i8,
    /// Value to try setting via `set_day_of_year`.
    pub day_of_year: i16,
    /// Value to try setting via `set_iso_week_year`.
    pub iso_week_year: i16,
    /// Value to try setting via `set_iso_week`.
    pub iso_week: i8,
    /// Value to try setting via `set_sunday_based_week`.
    pub week_sun: i8,
    /// Value to try setting via `set_monday_based_week`.
    pub week_mon: i8,
    /// Value to try setting via `set_hour`.
    pub hour: i8,
    /// Value to try setting via `set_minute`.
    pub minute: i8,
    /// Value to try setting via `set_second`.
    pub second: i8,
    /// Value to try setting via `set_subsec_nanosecond`.
    pub subsec_nanosecond: i32,
}

/// Exercises all 12 setters on a fresh model, confirming each one
/// round-trips exactly when in range — the same claim `amenable_kani`
/// and `amenable_creusot` both check against jiff's real API.
pub fn verify_fmt_strtime_broken_down_time_numeric_setters_round_trip_model(
    fields: BrokenDownTimeNumericFieldsModel,
) -> (result: bool)
    ensures
        result,
{
    let mut tm = BrokenDownTimeModel::new();
    let year_ok = !tm.set_year(fields.year) || tm.year == Some(fields.year);
    let month_ok = !tm.set_month(fields.month) || tm.month == Some(fields.month);
    let day_ok = !tm.set_day(fields.day) || tm.day == Some(fields.day);
    let day_of_year_ok =
        !tm.set_day_of_year(fields.day_of_year) || tm.day_of_year == Some(fields.day_of_year);
    let iso_week_year_ok = !tm.set_iso_week_year(fields.iso_week_year)
        || tm.iso_week_year == Some(fields.iso_week_year);
    let iso_week_ok = !tm.set_iso_week(fields.iso_week) || tm.iso_week == Some(fields.iso_week);
    let week_sun_ok =
        !tm.set_sunday_based_week(fields.week_sun) || tm.week_sun == Some(fields.week_sun);
    let week_mon_ok =
        !tm.set_monday_based_week(fields.week_mon) || tm.week_mon == Some(fields.week_mon);
    let hour_ok = !tm.set_hour(fields.hour) || tm.hour == Some(fields.hour);
    let minute_ok = !tm.set_minute(fields.minute) || tm.minute == Some(fields.minute);
    let second_ok = !tm.set_second(fields.second) || tm.second == Some(fields.second);
    let subsec_nanosecond_ok = !tm.set_subsec_nanosecond(fields.subsec_nanosecond)
        || tm.subsec_nanosecond == Some(fields.subsec_nanosecond);
    year_ok
        && month_ok
        && day_ok
        && day_of_year_ok
        && iso_week_year_ok
        && iso_week_ok
        && week_sun_ok
        && week_mon_ok
        && hour_ok
        && minute_ok
        && second_ok
        && subsec_nanosecond_ok
}

} // verus!

impl Default for BrokenDownTimeModel {
    fn default() -> Self {
        Self::new()
    }
}
