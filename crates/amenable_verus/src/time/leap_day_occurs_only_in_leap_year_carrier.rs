//! Verus spec for `amenable_time::LeapDayOccursOnlyInLeapYear`.
//!
//! ISO 8601-1:2019, 3.1.1.21 note 1 — the 29th of February is a valid calendar date only when the year is a leap year.

#[cfg(verus_keep_ghost)]
use crate::time::{gregorian_leap_year_holds, valid_calendar_day};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// ISO 8601-1:2019, 3.1.1.21 note 1 — the 29th of February is a valid calendar date only when the year is a leap year.
pub open spec fn leap_day_occurs_only_in_leap_year_holds(year: i32, month: u8, day: u8) -> bool {
    !(month == 2 && day == 29) || gregorian_leap_year_holds(year)
}

/// The exec predicate matches the spec, and February 29 is a valid
/// calendar date exactly when the year is a leap year.
pub fn verify_leap_day_occurs_only_in_leap_year(year: i32, month: u8, day: u8) -> (result: bool)
    ensures
        result == leap_day_occurs_only_in_leap_year_holds(year, month, day),
        valid_calendar_day(year, 2, 29) == gregorian_leap_year_holds(year),
{
    !(month == 2 && day == 29) || (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0))
}

} // verus!
