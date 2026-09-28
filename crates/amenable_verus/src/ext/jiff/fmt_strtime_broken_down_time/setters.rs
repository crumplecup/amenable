//! `BrokenDownTimeModel`'s 12 numeric setters — split out of `mod.rs`
//! to keep that file under the modularity line limit (`cordial`'s own
//! "peel... until this file is under the file size limit" suggestion).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use super::BrokenDownTimeModel;
#[cfg(verus_keep_ghost)]
use super::predicates::{
    broken_down_time_day_in_range, broken_down_time_day_of_year_in_range,
    broken_down_time_day_of_year_set_or_preserved, broken_down_time_day_set_or_preserved,
    broken_down_time_hour_in_range, broken_down_time_hour_set_or_preserved,
    broken_down_time_iso_week_in_range, broken_down_time_iso_week_set_or_preserved,
    broken_down_time_iso_week_year_set_or_preserved, broken_down_time_minute_or_second_in_range,
    broken_down_time_minute_set_or_preserved, broken_down_time_month_in_range,
    broken_down_time_month_set_or_preserved, broken_down_time_second_set_or_preserved,
    broken_down_time_subsec_nanosecond_in_range,
    broken_down_time_subsec_nanosecond_set_or_preserved,
    broken_down_time_week_mon_set_or_preserved, broken_down_time_week_number_in_range,
    broken_down_time_week_sun_set_or_preserved, broken_down_time_year_in_range,
    broken_down_time_year_set_or_preserved,
};

verus! {

impl BrokenDownTimeModel {
    /// Models `BrokenDownTime::set_year`: valid range `-9999..=9999`.
    pub fn set_year(&mut self, year: i16) -> (result: bool)
        ensures
            broken_down_time_year_in_range(year, result),
            broken_down_time_year_set_or_preserved(*old(self), *final(self), year, result),
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
            broken_down_time_month_in_range(month, result),
            broken_down_time_month_set_or_preserved(*old(self), *final(self), month, result),
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
            broken_down_time_day_in_range(day, result),
            broken_down_time_day_set_or_preserved(*old(self), *final(self), day, result),
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
            broken_down_time_day_of_year_in_range(day_of_year, result),
            broken_down_time_day_of_year_set_or_preserved(
                *old(self),
                *final(self),
                day_of_year,
                result,
            ),
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
            broken_down_time_year_in_range(year, result),
            broken_down_time_iso_week_year_set_or_preserved(*old(self), *final(self), year, result),
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
            broken_down_time_iso_week_in_range(week_number, result),
            broken_down_time_iso_week_set_or_preserved(
                *old(self),
                *final(self),
                week_number,
                result,
            ),
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
            broken_down_time_week_number_in_range(week_number, result),
            broken_down_time_week_sun_set_or_preserved(*old(self), *final(self), week_number, result),
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
            broken_down_time_week_number_in_range(week_number, result),
            broken_down_time_week_mon_set_or_preserved(*old(self), *final(self), week_number, result),
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
            broken_down_time_hour_in_range(hour, result),
            broken_down_time_hour_set_or_preserved(*old(self), *final(self), hour, result),
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
            broken_down_time_minute_or_second_in_range(minute, result),
            broken_down_time_minute_set_or_preserved(*old(self), *final(self), minute, result),
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
            broken_down_time_minute_or_second_in_range(second, result),
            broken_down_time_second_set_or_preserved(*old(self), *final(self), second, result),
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
            broken_down_time_subsec_nanosecond_in_range(subsec_nanosecond, result),
            broken_down_time_subsec_nanosecond_set_or_preserved(
                *old(self),
                *final(self),
                subsec_nanosecond,
                result,
            ),
    {
        if (0..=999999999).contains(&subsec_nanosecond) {
            self.subsec_nanosecond = Some(subsec_nanosecond);
            true
        } else {
            false
        }
    }
}

} // verus!
