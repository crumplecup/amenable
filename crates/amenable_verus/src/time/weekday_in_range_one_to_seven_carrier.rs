//! Verus spec for `amenable_time::WeekdayInRangeOneToSeven`.
//!
//! ISO/WD 8601-1:2016(E), 4.1.4.1 — a weekday is 1 (Monday) through 7 (Sunday). The `1..=7` range form (`weekday` in `1..=7`, the exec body) agrees
//! with the seven-way enumeration of the ISO weekdays (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The seven ISO weekdays (Mon..Sun) for `WeekdayInRangeOneToSeven`.
pub open spec fn weekday_in_range_one_to_seven_holds(weekday: u8) -> bool {
    weekday == 1 || weekday == 2 || weekday == 3 || weekday == 4 || weekday == 5 || weekday == 6 || weekday == 7
}

/// The `1..=7` range form (exec body) satisfies the seven-way
/// enumeration, for every `u8`.
pub fn verify_weekday_in_range_one_to_seven(weekday: u8) -> (result: bool)
    ensures
        result == weekday_in_range_one_to_seven_holds(weekday),
{
    (1..=7u8).contains(&weekday)
}

} // verus!
