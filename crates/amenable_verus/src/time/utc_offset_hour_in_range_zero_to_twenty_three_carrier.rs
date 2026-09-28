//! Verus spec for `amenable_time::UtcOffsetHourInRangeZeroToTwentyThree`.
//!
//! ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset hour is 00 through 23. The `0..=23` inclusive form (`hour <= 23`, the exec body)
//! agrees with the independently-written `hour < 24` (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=23` range for `UtcOffsetHourInRangeZeroToTwentyThree`, stated as `hour < 24`.
pub open spec fn utc_offset_hour_in_range_zero_to_twenty_three_holds(hour: u8) -> bool {
    hour < 24
}

/// The `hour <= 23` inclusive form's result matches the `hour < 24`
/// spec, named so the exec-to-spec link is a citable fact.
pub open spec fn utc_offset_hour_in_range_zero_to_twenty_three_result_matches(hour: u8, result: bool) -> bool {
    result == utc_offset_hour_in_range_zero_to_twenty_three_holds(hour)
}

/// The `hour <= 23` inclusive form (exec body) satisfies the `hour < 24`
/// spec, for every `u8`.
pub fn verify_utc_offset_hour_in_range_zero_to_twenty_three(hour: u8) -> (result: bool)
    ensures
        utc_offset_hour_in_range_zero_to_twenty_three_result_matches(hour, result),
{
    hour <= 23
}

} // verus!
