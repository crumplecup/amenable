//! Verus spec for `amenable_time::UtcOffsetMinuteInRangeZeroToFiftyNine`.
//!
//! ISO/WD 8601-1:2016(E), 4.2.5.1 — a UTC-offset minute is 00 through 59. The `0..=59` inclusive form (`minute <= 59`, the exec body)
//! agrees with the independently-written `minute < 60` (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=59` range for `UtcOffsetMinuteInRangeZeroToFiftyNine`, stated as `minute < 60`.
pub open spec fn utc_offset_minute_in_range_zero_to_fifty_nine_holds(minute: u8) -> bool {
    minute < 60
}

/// The `minute <= 59` inclusive form (exec body) satisfies the `minute < 60`
/// spec, for every `u8`.
pub fn verify_utc_offset_minute_in_range_zero_to_fifty_nine(minute: u8) -> (result: bool)
    ensures
        result == utc_offset_minute_in_range_zero_to_fifty_nine_holds(minute),
{
    minute <= 59
}

} // verus!
