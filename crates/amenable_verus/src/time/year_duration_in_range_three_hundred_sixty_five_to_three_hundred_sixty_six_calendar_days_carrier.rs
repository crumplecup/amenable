//! Verus spec for `amenable_time::YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays`.
//!
//! ISO 8601-1:2019, 2.2.14 — a year's duration is 365 or 366 calendar days. `days_in_year(y)` is always 365 or 366 — the model is well-formed —
//! for every non-negative `i32`.

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

/// ISO 8601-1:2019, 2.2.14 — a year's duration is 365 or 366 calendar days.
pub open spec fn year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds(year: i32) -> bool {
    365 <= days_in_year(year) && days_in_year(year) <= 366
}

/// The exec check's result matches the spec, named so the exec-to-spec
/// link is a citable fact.
pub open spec fn year_duration_result_matches(year: i32, result: bool) -> bool {
    result == year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days_holds(year)
}

/// The year-length model is well-formed: every year has 365 or 366
/// days — a standalone fact, named so it is citable.
pub open spec fn days_in_year_is_365_or_366(year: i32) -> bool {
    days_in_year(year) == 365 || days_in_year(year) == 366
}

/// The domain restriction every year-length check in this family
/// shares: the model is only defined for non-negative years.
pub open spec fn year_is_non_negative(year: i32) -> bool {
    year >= 0
}

/// The exec year-length check matches the spec.
pub fn verify_year_duration_in_range_three_hundred_sixty_five_to_three_hundred_sixty_six_calendar_days(year: i32) -> (result: bool)
    requires
        year_is_non_negative(year),
    ensures
        year_duration_result_matches(year, result),
        days_in_year_is_365_or_366(year),
{
    let days: i32 = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 366 } else { 365 };
    (365..=366).contains(&days)
}

} // verus!
