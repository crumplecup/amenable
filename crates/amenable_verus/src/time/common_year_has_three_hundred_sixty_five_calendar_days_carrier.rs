//! Verus spec for `amenable_time::CommonYearHasThreeHundredSixtyFiveCalendarDays`.
//!
//! ISO 8601-1:2019, 3.1.1.20 — a common year contains 365 calendar days. `days_in_year(y) == 365` agrees with `!leap(y)`, and the count is
//! always 365 or 366, for every non-negative `i32`.

#[cfg(verus_keep_ghost)]
use crate::time::gregorian_leap_year_holds;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A Gregorian calendar year is 366 days when a leap year, else 365.
pub open spec fn days_in_year(year: i32) -> i32 {
    if gregorian_leap_year_holds(year) { 366 } else { 365 }
}

/// ISO 8601-1:2019, 3.1.1.20 — a common year contains 365 calendar days.
pub open spec fn common_year_has_three_hundred_sixty_five_calendar_days_holds(year: i32) -> bool {
    days_in_year(year) == 365
}

/// The exec year-length check matches the spec.
pub fn verify_common_year_has_three_hundred_sixty_five_calendar_days(year: i32) -> (result: bool)
    requires
        year >= 0,
    ensures
        result == common_year_has_three_hundred_sixty_five_calendar_days_holds(year),
        common_year_has_three_hundred_sixty_five_calendar_days_holds(year) == !gregorian_leap_year_holds(year),
        days_in_year(year) == 365 || days_in_year(year) == 366,
{
    let days: i32 = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
    days == 365
}

} // verus!
