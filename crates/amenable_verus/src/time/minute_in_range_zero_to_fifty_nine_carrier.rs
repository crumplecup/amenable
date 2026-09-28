//! Verus spec for `amenable_time::MinuteInRangeZeroToFiftyNine`.
//!
//! ISO/WD 8601-1:2016(E), 4.2.1 — a minute is 00 through 59. The `0..=59` inclusive form (`minute <= 59`, the exec body)
//! agrees with the independently-written `minute < 60` (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=59` range for `MinuteInRangeZeroToFiftyNine`, stated as `minute < 60`.
pub open spec fn minute_in_range_zero_to_fifty_nine_holds(minute: u8) -> bool {
    minute < 60
}

/// The `minute <= 59` inclusive form's result matches the `minute < 60`
/// spec, named so the exec-to-spec link is a citable fact.
pub open spec fn minute_in_range_zero_to_fifty_nine_result_matches(minute: u8, result: bool) -> bool {
    result == minute_in_range_zero_to_fifty_nine_holds(minute)
}

/// The `minute <= 59` inclusive form (exec body) satisfies the `minute < 60`
/// spec, for every `u8`.
pub fn verify_minute_in_range_zero_to_fifty_nine(minute: u8) -> (result: bool)
    ensures
        minute_in_range_zero_to_fifty_nine_result_matches(minute, result),
{
    minute <= 59
}

} // verus!
