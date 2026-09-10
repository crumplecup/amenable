//! Verus spec for `amenable_time::CalendarMonthInRangeOneToTwelve`.
//!
//! Matches the Kani harness's claim scope exactly: both check, over every
//! possible `u8`, that the `1..=12` range check agrees with the
//! independently-written twelve-way enumeration of the legal calendar
//! months (ISO 8601-1:2019, 3.1.1.2).

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The twelve-way enumeration of the legal calendar months — one of
/// twelve named intervals within a calendar year.
pub open spec fn calendar_month_is_enumerated(month: u8) -> bool {
    month == 1 || month == 2 || month == 3 || month == 4 || month == 5 || month == 6
        || month == 7 || month == 8 || month == 9 || month == 10 || month == 11 || month == 12
}

/// The `1..=12` range check (the exec body) agrees with the twelve-way
/// enumeration of the legal calendar months (the postcondition), for
/// every `u8` — the same claim the Kani harness checks.
pub fn verify_calendar_month_in_range(month: u8) -> (result: bool)
    ensures
        result == calendar_month_is_enumerated(month),
{
    (1..=12u8).contains(&month)
}

} // verus!
