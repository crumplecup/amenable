//! Verus accommodation model for `jiff::fmt::strtime::
//! BrokenDownTime`'s 12 numeric setter/getter round trips.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! fmt_strtime_broken_down_time`'s own harness for the identical
//! claim, checked directly against jiff's real API, independently
//! confirms (see that module's own doc comment for the real bounds
//! and the scoping rationale: exactly these 12 numeric fields, not
//! the 5 reference-type ones).
//!
//! Genuinely new territory for this checklist: every prior Verus
//! model in `ext::jiff` is a pure function building a fresh value;
//! this one models a real `&mut self` setter, using Verus's own
//! `old(self)`/final-`self` convention (no `mem::forget`/`ManuallyDrop`
//! workaround needed at all — Verus has no CBMC-style Drop-glue cost,
//! confirmed by this model verifying instantly).

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
            result == Self::new_spec(),
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

    /// Models `BrokenDownTime::set_year`: valid range `-9999..=9999`.
    pub fn set_year(&mut self, year: i16) -> (result: bool)
        ensures
            result == (-9999 <= year && year <= 9999),
            result ==> final(self).year == Some(year),
            !result ==> final(self).year == old(self).year,
    {
        if (-9999..=9999).contains(&year) {
            self.year = Some(year);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_month`: valid range `1..=12`.
    pub fn set_month(&mut self, month: i8) -> (result: bool)
        ensures
            result == (1 <= month && month <= 12),
            result ==> final(self).month == Some(month),
            !result ==> final(self).month == old(self).month,
    {
        if (1..=12).contains(&month) {
            self.month = Some(month);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_day`: valid range `1..=31`,
    /// unconditional (no month cross-check).
    pub fn set_day(&mut self, day: i8) -> (result: bool)
        ensures
            result == (1 <= day && day <= 31),
            result ==> final(self).day == Some(day),
            !result ==> final(self).day == old(self).day,
    {
        if (1..=31).contains(&day) {
            self.day = Some(day);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_day_of_year`: valid range `1..=366`.
    pub fn set_day_of_year(&mut self, day_of_year: i16) -> (result: bool)
        ensures
            result == (1 <= day_of_year && day_of_year <= 366),
            result ==> final(self).day_of_year == Some(day_of_year),
            !result ==> final(self).day_of_year == old(self).day_of_year,
    {
        if (1..=366).contains(&day_of_year) {
            self.day_of_year = Some(day_of_year);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_iso_week_year`: valid range
    /// `-9999..=9999`.
    pub fn set_iso_week_year(&mut self, year: i16) -> (result: bool)
        ensures
            result == (-9999 <= year && year <= 9999),
            result ==> final(self).iso_week_year == Some(year),
            !result ==> final(self).iso_week_year == old(self).iso_week_year,
    {
        if (-9999..=9999).contains(&year) {
            self.iso_week_year = Some(year);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_iso_week`: valid range `1..=53`.
    pub fn set_iso_week(&mut self, week_number: i8) -> (result: bool)
        ensures
            result == (1 <= week_number && week_number <= 53),
            result ==> final(self).iso_week == Some(week_number),
            !result ==> final(self).iso_week == old(self).iso_week,
    {
        if (1..=53).contains(&week_number) {
            self.iso_week = Some(week_number);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_sunday_based_week`: valid range
    /// `0..=53`.
    pub fn set_sunday_based_week(&mut self, week_number: i8) -> (result: bool)
        ensures
            result == (0 <= week_number && week_number <= 53),
            result ==> final(self).week_sun == Some(week_number),
            !result ==> final(self).week_sun == old(self).week_sun,
    {
        if (0..=53).contains(&week_number) {
            self.week_sun = Some(week_number);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_monday_based_week`: valid range
    /// `0..=53`.
    pub fn set_monday_based_week(&mut self, week_number: i8) -> (result: bool)
        ensures
            result == (0 <= week_number && week_number <= 53),
            result ==> final(self).week_mon == Some(week_number),
            !result ==> final(self).week_mon == old(self).week_mon,
    {
        if (0..=53).contains(&week_number) {
            self.week_mon = Some(week_number);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_hour`: valid range `0..=23`.
    pub fn set_hour(&mut self, hour: i8) -> (result: bool)
        ensures
            result == (0 <= hour && hour <= 23),
            result ==> final(self).hour == Some(hour),
            !result ==> final(self).hour == old(self).hour,
    {
        if (0..=23).contains(&hour) {
            self.hour = Some(hour);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_minute`: valid range `0..=59`.
    pub fn set_minute(&mut self, minute: i8) -> (result: bool)
        ensures
            result == (0 <= minute && minute <= 59),
            result ==> final(self).minute == Some(minute),
            !result ==> final(self).minute == old(self).minute,
    {
        if (0..=59).contains(&minute) {
            self.minute = Some(minute);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_second`: valid range `0..=59` —
    /// genuinely different from `civil::Time`'s own `0..=60`
    /// leap-second allowance, confirmed against jiff's real bound.
    pub fn set_second(&mut self, second: i8) -> (result: bool)
        ensures
            result == (0 <= second && second <= 59),
            result ==> final(self).second == Some(second),
            !result ==> final(self).second == old(self).second,
    {
        if (0..=59).contains(&second) {
            self.second = Some(second);
            true
        } else {
            false
        }
    }

    /// Models `BrokenDownTime::set_subsec_nanosecond`: valid range
    /// `0..=999_999_999`.
    pub fn set_subsec_nanosecond(&mut self, subsec_nanosecond: i32) -> (result: bool)
        ensures
            result == (0 <= subsec_nanosecond && subsec_nanosecond <= 999999999),
            result ==> final(self).subsec_nanosecond == Some(subsec_nanosecond),
            !result ==> final(self).subsec_nanosecond == old(self).subsec_nanosecond,
    {
        if (0..=999999999).contains(&subsec_nanosecond) {
            self.subsec_nanosecond = Some(subsec_nanosecond);
            true
        } else {
            false
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
