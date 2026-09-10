//! Verus spec for `amenable_time::HourInRangeZeroToTwentyFour`.
//!
//! ISO 8601-1:2019/Amd 1:2022, 5.3.1.4 / 5.3.2 — an hour is 00 through 24 (24 reserved for end-of-day). The `0..=24` inclusive form (`hour <= 24`, the exec body)
//! agrees with the independently-written `hour < 25` (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=24` range for `HourInRangeZeroToTwentyFour`, stated as `hour < 25`.
pub open spec fn hour_in_range_zero_to_twenty_four_holds(hour: u8) -> bool {
    hour < 25
}

/// The `hour <= 24` inclusive form (exec body) satisfies the `hour < 25`
/// spec, for every `u8`.
pub fn verify_hour_in_range_zero_to_twenty_four(hour: u8) -> (result: bool)
    ensures
        result == hour_in_range_zero_to_twenty_four_holds(hour),
{
    hour <= 24
}

} // verus!
