//! Verus spec for `amenable_ext::BoundedStepWalk`.
//!
//! One step of the real stepping formula `next = (prev + delta).min(hi)`
//! never moves backward and never advances by more than `delta`, under
//! the precondition `delta > 0 && prev <= hi`. Pure `i64` arithmetic, no
//! real chrono/chrono-tz dependency at all -- the same reason this file
//! carries no feature gate, matching every other file in this module.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `BoundedStepWalk`: `next` lies in `[prev, prev + delta]`.
pub open spec fn bounded_step_walk_holds(prev: i64, delta: i64, hi: i64, next: i64) -> bool {
    next >= prev && next - prev <= delta
}

/// The exec body's result matches the predicate, named so the
/// exec-to-spec link is a citable fact.
pub open spec fn bounded_step_walk_result_matches(prev: i64, delta: i64, hi: i64, result: i64) -> bool {
    bounded_step_walk_holds(prev, delta, hi, result)
}

/// The precondition this claim is stated under: `delta > 0`, `prev <=
/// hi`, and `prev + delta` does not overflow `i64` -- named so
/// `requires` calls it rather than restating three inline clauses.
pub open spec fn bounded_step_walk_requires(prev: i64, delta: i64, hi: i64) -> bool {
    delta > 0 && prev <= hi && prev + delta <= i64::MAX
}

/// `(prev + delta).min(hi)` (exec body) matches the predicate, for every
/// `i64` triple respecting the precondition. The overflow guard is
/// load-bearing, mirroring the real Kani finding: `prev + delta`
/// genuinely overflows for `prev`/`delta` near `i64::MAX` without it.
pub fn verify_bounded_step_walk(prev: i64, delta: i64, hi: i64) -> (result: i64)
    requires
        bounded_step_walk_requires(prev, delta, hi),
    ensures
        bounded_step_walk_result_matches(prev, delta, hi, result),
{
    if prev + delta < hi {
        prev + delta
    } else {
        hi
    }
}

} // verus!
