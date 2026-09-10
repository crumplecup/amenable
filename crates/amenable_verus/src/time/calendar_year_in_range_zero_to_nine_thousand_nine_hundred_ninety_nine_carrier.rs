//! Verus spec for `amenable_time::CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine`.
//!
//! ISO/WD 8601-1:2016(E), 4.1.2.1 — a non-expanded calendar year is 0000 through 9999. The `0..=9999` inclusive form (`year <= 9999`, the exec body)
//! agrees with the independently-written `year < 10000` (the postcondition),
//! for every `u16` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=9999` range for `CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine`, stated as `year < 10000`.
pub open spec fn calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds(year: u16) -> bool {
    year < 10000
}

/// The `year <= 9999` inclusive form (exec body) satisfies the `year < 10000`
/// spec, for every `u16`.
pub fn verify_calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine(year: u16) -> (result: bool)
    ensures
        result == calendar_year_in_range_zero_to_nine_thousand_nine_hundred_ninety_nine_holds(year),
{
    year <= 9999
}

} // verus!
