//! Verus spec for `amenable_time::DecadeOrdinalInRangeZeroToNineHundredNinetyNine`.
//!
//! ISO 8601-1:2019/Amd 1:2022, 4.3.11 — a Gregorian decade ordinal is 000 through 999. The `0..=999` inclusive form (`ordinal <= 999`, the exec body)
//! agrees with the independently-written `ordinal < 1000` (the postcondition),
//! for every `u16` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=999` range for `DecadeOrdinalInRangeZeroToNineHundredNinetyNine`, stated as `ordinal < 1000`.
pub open spec fn decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds(ordinal: u16) -> bool {
    ordinal < 1000
}

/// The `ordinal <= 999` inclusive form (exec body) satisfies the `ordinal < 1000`
/// spec, for every `u16`.
pub fn verify_decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine(ordinal: u16) -> (result: bool)
    ensures
        result == decade_ordinal_in_range_zero_to_nine_hundred_ninety_nine_holds(ordinal),
{
    ordinal <= 999
}

} // verus!
