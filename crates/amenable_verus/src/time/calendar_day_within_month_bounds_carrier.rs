//! Verus spec for `amenable_time::CalendarDayWithinMonthBounds`.
//!
//! ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.2.1 — a calendar-date day component is within the valid day count for that month and year.

#[cfg(verus_keep_ghost)]
use crate::time::{days_in_month, gregorian_leap_year_holds, valid_calendar_day};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.2.1 — a calendar-date day component is within the valid day count for that month and year.
pub open spec fn calendar_day_within_month_bounds_holds(year: i32, month: u8, day: u8) -> bool {
    valid_calendar_day(year, month, day)
}

/// The exec bounds check matches the spec; a valid day is in `1..=31`; and
/// a valid February 29 forces a leap year.
pub fn verify_calendar_day_within_month_bounds(year: i32, month: u8, day: u8) -> (result: bool)
    requires
        1 <= month <= 12,
    ensures
        result == calendar_day_within_month_bounds_holds(year, month, day),
        result ==> 1 <= day <= 31,
        month == 2 && day == 29 && result ==> gregorian_leap_year_holds(year),
{
    let dim: u8 = if month == 2 {
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    };
    (1..=dim).contains(&day)
}

} // verus!
