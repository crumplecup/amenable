//! Verus spec for `amenable_time::WeekNumberInRangeOneToFiftyThree`.
//!
//! ISO/WD 8601-1:2016(E), 4.1.4.1 — a calendar-week number is 01 through 53. The `1..=53` range form (`week` in `1..=53`, the exec body) agrees
//! with the `1 <= week && week < 54` bound conjunction (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Lower bound of `WeekNumberInRangeOneToFiftyThree`: `week >= 1`.
pub open spec fn week_number_in_range_one_to_fifty_three_at_least_1(week: u8) -> bool {
    week >= 1
}

/// Upper bound of `WeekNumberInRangeOneToFiftyThree`, stated as `week < 54`.
pub open spec fn week_number_in_range_one_to_fifty_three_below_54(week: u8) -> bool {
    week < 54
}

/// The `1..=53` range for `WeekNumberInRangeOneToFiftyThree` as the conjunction of its bounds.
pub open spec fn week_number_in_range_one_to_fifty_three_holds(week: u8) -> bool {
    week_number_in_range_one_to_fifty_three_at_least_1(week) && week_number_in_range_one_to_fifty_three_below_54(week)
}

/// The `1..=53` range form (exec body) satisfies the
/// bound-conjunction spec, for every `u8`.
pub fn verify_week_number_in_range_one_to_fifty_three(week: u8) -> (result: bool)
    ensures
        result == week_number_in_range_one_to_fifty_three_holds(week),
{
    (1..=53u8).contains(&week)
}

} // verus!
