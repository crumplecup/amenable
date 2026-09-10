//! Verus spec for `amenable_time::CenturyOrdinalInRangeZeroToNinetyNine`.
//!
//! ISO 8601-1:2019/Amd 1:2022, 4.3.12 — a Gregorian century ordinal is 00 through 99. The `0..=99` inclusive form (`ordinal <= 99`, the exec body)
//! agrees with the independently-written `ordinal < 100` (the postcondition),
//! for every `u8` — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The `0..=99` range for `CenturyOrdinalInRangeZeroToNinetyNine`, stated as `ordinal < 100`.
pub open spec fn century_ordinal_in_range_zero_to_ninety_nine_holds(ordinal: u8) -> bool {
    ordinal < 100
}

/// The `ordinal <= 99` inclusive form (exec body) satisfies the `ordinal < 100`
/// spec, for every `u8`.
pub fn verify_century_ordinal_in_range_zero_to_ninety_nine(ordinal: u8) -> (result: bool)
    ensures
        result == century_ordinal_in_range_zero_to_ninety_nine_holds(ordinal),
{
    ordinal <= 99
}

} // verus!
