//! Verus spec for `amenable_time::IntervalStartPrecedesEnd`.
//!
//! ISO 8601-1:2019, 3.1.1.6 / 3.1.1.8 — an interval's first endpoint is no later than its second on the relevant timeline. The `start <= end` form agrees with `!(end < start)` and with a
//! non-negative `int` span, for every `i32` pair,
//! for every `i32` pair — the same claim the Kani harness checks.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `IntervalStartPrecedesEnd`: the first endpoint is no later than the second.
pub open spec fn interval_start_precedes_end_holds(start: i32, end: i32) -> bool {
    start <= end
}

/// `start <= end` (exec body) matches the predicate, its negation form, and
/// the non-negative-span form, for every `i32` pair.
pub fn verify_interval_start_precedes_end(start: i32, end: i32) -> (result: bool)
    ensures
        result == interval_start_precedes_end_holds(start, end),
        interval_start_precedes_end_holds(start, end) == !(end < start),
        interval_start_precedes_end_holds(start, end) == ((end as int) - (start as int) >= 0),
{
    start <= end
}

} // verus!
