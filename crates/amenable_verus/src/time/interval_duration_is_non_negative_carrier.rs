//! Verus spec for `amenable_time::IntervalDurationIsNonNegative`.
//!
//! ISO 8601-1:2019, 3.1.1.8 — the span between an interval's endpoints is zero or positive, never negative. The non-negative-`int`-span form agrees with `start <= end`, and the
//! span is zero exactly when the endpoints coincide,
//! for every `i32` pair — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `IntervalDurationIsNonNegative`: the `end - start` span, taken in `int`, is >= 0.
pub open spec fn interval_duration_is_non_negative_holds(start: i32, end: i32) -> bool {
    (end as int) - (start as int) >= 0
}

/// `start <= end` (exec body) is the panic-free witness that the span is
/// non-negative; it also pins the zero-span case to endpoint equality.
pub fn verify_interval_duration_is_non_negative(start: i32, end: i32) -> (result: bool)
    ensures
        result == interval_duration_is_non_negative_holds(start, end),
        interval_duration_is_non_negative_holds(start, end) == (start <= end),
        ((end as int) - (start as int) == 0) == (start == end),
{
    start <= end
}

} // verus!
