//! Verus spec for `amenable_time::SecondInRangeZeroToSixty`.
//!
//! ISO/WD 8601-1:2016(E), 4.2.1 — a second is 00 through 60 (60 admits a leap second). The `0..=60` inclusive form (`second <= 60`, the exec body)
//! agrees with the independently-written `second < 61` (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=60` range for `SecondInRangeZeroToSixty`, stated as `second < 61`.
pub open spec fn second_in_range_zero_to_sixty_holds(second: u8) -> bool {
    second < 61
}

/// The `second <= 60` inclusive form (exec body) satisfies the `second < 61`
/// spec, for every `u8`.
pub fn verify_second_in_range_zero_to_sixty(second: u8) -> (result: bool)
    ensures
        result == second_in_range_zero_to_sixty_holds(second),
{
    second <= 60
}

} // verus!
