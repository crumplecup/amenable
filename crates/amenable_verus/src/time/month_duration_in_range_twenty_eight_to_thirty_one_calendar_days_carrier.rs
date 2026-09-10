//! Verus spec for `amenable_time::MonthDurationInRangeTwentyEightToThirtyOneCalendarDays`.
//!
//! ISO 8601-1:2019, 2.2.12 — a month's duration is 28, 29, 30, or 31 calendar days according to the month and year.

#[cfg(verus_keep_ghost)]
use crate::time::{days_in_year, gregorian_leap_year_holds};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The number of calendar days in month `m` of year `y` (m in 1..=12).
pub open spec fn days_in_month(year: i32, month: u8) -> int {
    if month == 2 {
        if gregorian_leap_year_holds(year) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    }
}

/// A day component `d` is within month `m`'s bounds: `1 <= d <= days_in_month`.
pub open spec fn valid_calendar_day(year: i32, month: u8, day: u8) -> bool {
    1 <= day && day <= days_in_month(year, month)
}

/// ISO 8601-1:2019, 2.2.12 — a month's duration is 28, 29, 30, or 31 calendar days according to the month and year.
pub open spec fn month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds(
    year: i32,
    month: u8,
) -> bool {
    28 <= days_in_month(year, month) && days_in_month(year, month) <= 31
}

/// The exec duration check matches the spec, and the twelve months of a
/// year sum to that year's day count.
pub fn verify_month_duration_in_range_twenty_eight_to_thirty_one_calendar_days(
    year: i32,
    month: u8,
) -> (result: bool)
    requires
        1 <= month <= 12,
    ensures
        result == month_duration_in_range_twenty_eight_to_thirty_one_calendar_days_holds(year, month),
        days_in_month(year, 1) + days_in_month(year, 2) + days_in_month(year, 3)
            + days_in_month(year, 4) + days_in_month(year, 5) + days_in_month(year, 6)
            + days_in_month(year, 7) + days_in_month(year, 8) + days_in_month(year, 9)
            + days_in_month(year, 10) + days_in_month(year, 11) + days_in_month(year, 12)
            == days_in_year(year),
{
    let dim: u8 = if month == 2 {
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else {
        31
    };
    (28..=31).contains(&dim)
}

} // verus!
