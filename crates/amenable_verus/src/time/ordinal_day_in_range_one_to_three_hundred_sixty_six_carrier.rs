//! Verus spec for `amenable_time::OrdinalDayInRangeOneToThreeHundredSixtySix`.
//!
//! ISO/WD 8601-1:2016(E), 3.2.1 / 4.1.3.1 — an ordinal day-of-year is 001 through 365, or 366 in a leap year. The `1..=366` range form (`day` in `1..=366`, the exec body) agrees
//! with the `1 <= day && day < 367` bound conjunction (the postcondition),
//! for every `u16` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Lower bound of `OrdinalDayInRangeOneToThreeHundredSixtySix`: `day >= 1`.
pub open spec fn ordinal_day_in_range_one_to_three_hundred_sixty_six_at_least_1(day: u16) -> bool {
    day >= 1
}

/// Upper bound of `OrdinalDayInRangeOneToThreeHundredSixtySix`, stated as `day < 367`.
pub open spec fn ordinal_day_in_range_one_to_three_hundred_sixty_six_below_367(day: u16) -> bool {
    day < 367
}

/// The `1..=366` range for `OrdinalDayInRangeOneToThreeHundredSixtySix` as the conjunction of its bounds.
pub open spec fn ordinal_day_in_range_one_to_three_hundred_sixty_six_holds(day: u16) -> bool {
    ordinal_day_in_range_one_to_three_hundred_sixty_six_at_least_1(day) && ordinal_day_in_range_one_to_three_hundred_sixty_six_below_367(day)
}

/// The `1..=366` range form (exec body) satisfies the
/// bound-conjunction spec, for every `u16`.
pub fn verify_ordinal_day_in_range_one_to_three_hundred_sixty_six(day: u16) -> (result: bool)
    ensures
        result == ordinal_day_in_range_one_to_three_hundred_sixty_six_holds(day),
{
    (1..=366u16).contains(&day)
}

} // verus!
