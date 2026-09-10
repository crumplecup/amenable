//! Verus spec for `amenable_time::GregorianLeapYearUsesDivisibleByFourAndFourHundredException`.
//!
//! ISO 8601-1:2019, 3.1.1.21 note 1 — a year is a leap year if divisible by 4, except a centennial year is a leap year only if also divisible by 400. The exec rule matches the spec, every leap year is divisible by
//! four, and the six dated anchors (2000/1600/2024 leap, 1900/2100/2023
//! not) hold.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The Gregorian leap-year rule: divisible by 4, but a centennial year
/// only when also divisible by 400.
pub open spec fn gregorian_leap_year_holds(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// The exec rule matches the spec, every leap year is divisible by four,
/// and the six dated anchors hold.
pub fn verify_gregorian_leap_year(year: i32) -> (result: bool)
    ensures
        result == gregorian_leap_year_holds(year),
        gregorian_leap_year_holds(year) ==> year % 4 == 0,
        gregorian_leap_year_holds(2000),
        gregorian_leap_year_holds(1600),
        gregorian_leap_year_holds(2024),
        !gregorian_leap_year_holds(1900),
        !gregorian_leap_year_holds(2100),
        !gregorian_leap_year_holds(2023),
{
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

} // verus!
