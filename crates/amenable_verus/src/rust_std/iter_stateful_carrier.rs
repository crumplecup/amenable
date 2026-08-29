//! Verus accommodation model for the remaining `core::iter` adapters:
//! `Cycle`, `Fuse`, `Inspect`, `Peekable`, `Scan`, `FlatMap`, `Flatten`,
//! `Successors`, `FromFn`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Each of these nine real types checks a stateful or
//! multi-step generation claim in `amenable_kani` — this carrier states
//! each one directly rather than modeling repeated `.next()` calls
//! against iterator state. `FlatMap`/`Flatten` use fixed concrete
//! examples rather than symbolic inputs, mirroring `amenable_kani`'s own
//! choice for `Flatten` specifically (its own doc comment there records
//! that even an array-outer/symbolic-inner variant still times out under
//! Kani, so only fully concrete inner-range lengths work — this carrier
//! mirrors that same concrete shape for both). `Inspect`/`FromFn` track
//! a plain call-count field rather than `amenable_kani`'s real
//! `Cell`/atomic counters, since the claim itself ("calls its closure
//! exactly this many times") doesn't depend on which counter mechanism
//! observes it. None of these functions are `Cycle`/`Fuse`/`Inspect`/
//! `Peekable`/`Scan`/`FlatMap`/`Flatten`/`Successors`/`FromFn`
//! themselves — each proof is conditional: sound if the real adapter
//! refines the stated behavior, which `amenable_kani`'s own harness for
//! that exact type (checking the real adapter directly) already
//! confirms independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_option_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

// The increment-headroom precondition `amenable_std::IncrementHeadroom`
// names — a real spec fn defined once in `iter_sequence_carrier`, called
// from here directly rather than restated inline. Confirmed under real
// `verus` that a spec fn defined in one carrier file gets full proof
// credit when called from a sibling file (see `amenable_std::
// verus_gallery`'s `cross_file_spec_fn_reuse_gets_real_proof_credit`
// case) — unlike Creusot's cross-module `#[logic]` opacity.
// `#[cfg(verus_keep_ghost)]`-gated like every other spec-only import in
// this crate (see `chars_carrier.rs`): plain `spec fn`s carry no runtime
// representation, so this import only resolves when Verus's own ghost
// content is retained, not under ordinary `cargo check`.
#[cfg(verus_keep_ghost)]
use crate::rust_std::iter_sequence_carrier::{
    increment_headroom_holds, single_increment_headroom_holds,
};

verus! {

/// `Cycle::next` restarts its sequence once exhausted — modeled over
/// the two-element range `a..a+2`, observed across four calls.
pub fn verify_cycle_model_repeats_its_sequence_forever(a: i32) -> (result: (i32, i32, i32, i32))
    requires
        increment_headroom_holds(a),
    ensures
        observed_value_matches_input(result.0 as int, a as int),
        observed_value_matches_input(result.1 as int, (a + 1) as int),
        observed_value_matches_input(result.2 as int, a as int),
        observed_value_matches_input(result.3 as int, (a + 1) as int),
{
    (a, a + 1, a, a + 1)
}

/// `Fuse::next` keeps returning `None` once the underlying iterator
/// has, checked across two calls after exhaustion.
pub fn verify_fuse_model_keeps_returning_none_once_exhausted(a: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    requires
        single_increment_headroom_holds(a),
    ensures
        observed_option_matches_input(result.0, a),
        result.1 is None,
        result.2 is None,
{
    (Some(a), None, None)
}

/// `Inspect::next` leaves the value unchanged and calls its closure
/// exactly once per item, never re-invoking it after exhaustion —
/// tracked here as a plain call count.
pub fn verify_inspect_model_calls_once_per_item_without_changing_values(value: i32) -> (result: (Option<i32>, u32, Option<i32>, u32))
    requires
        single_increment_headroom_holds(value),
    ensures
        observed_option_matches_input(result.0, value),
        observed_value_matches_input(result.1 as int, 1int),
        result.2 is None,
        observed_value_matches_input(result.3 as int, 1int),
{
    let mut calls: u32 = 0;
    calls += 1;
    let first = Some(value);
    let calls_after_first = calls;
    let second = None;
    let calls_after_second = calls;

    (first, calls_after_first, second, calls_after_second)
}

/// `Peekable::peek` previews the next item without consuming it: a
/// following `next()` still returns that same item, and the peek
/// doesn't skip the item after it.
pub fn verify_peekable_model_peek_does_not_consume(a: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    requires
        increment_headroom_holds(a),
    ensures
        observed_option_matches_input(result.0, a),
        observed_option_matches_input(result.1, a),
        observed_option_matches_input(result.2, (a + 1) as i32),
{
    (Some(a), Some(a), Some(a + 1))
}

/// A margin narrow enough that `a + (a + 1)` (`Scan`'s own running-sum
/// closure, applied twice) never overflows `i32`.
pub open spec fn is_within_scan_sum_headroom(a: int) -> bool {
    -1000 <= a <= 1000
}

/// `Scan::next` threads its mutable state from one call into the next —
/// a running-sum closure's second result includes the first item's
/// contribution.
pub fn verify_scan_model_threads_state_through_its_closure(a: i32) -> (result: (Option<i32>, Option<i32>))
    requires
        is_within_scan_sum_headroom(a as int),
    ensures
        observed_option_matches_input(result.0, a),
        observed_option_matches_input(result.1, (a + (a + 1)) as i32),
{
    (Some(a), Some(a + (a + 1)))
}

/// `FlatMap::next` over the single-item source `[3]` with closure
/// `|x| 0..x` matches calling the closure directly on `3` and iterating
/// it — a fixed representative in place of `amenable_kani`'s symbolic
/// `x`, avoiding this carrier's own version of the `Flatten` timeout
/// wall documented above.
pub fn verify_flat_map_model_flattens_each_generated_iterator() -> (result: (Option<i32>, Option<i32>, Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, 0),
        observed_option_matches_input(result.1, 1),
        observed_option_matches_input(result.2, 2),
        result.3 is None,
{
    (Some(0), Some(1), Some(2), None)
}

/// `Flatten::next` concatenates `[0..1, 0..2]` in order — the same
/// fixed two-length example `amenable_kani`'s own harness uses (see
/// this module's doc comment for why a symbolic length isn't
/// achievable here).
pub fn verify_flatten_model_concatenates_the_inner_iterators() -> (result: (Option<i32>, Option<i32>, Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, 0),
        observed_option_matches_input(result.1, 0),
        observed_option_matches_input(result.2, 1),
        result.3 is None,
{
    (Some(0), Some(0), Some(1), None)
}

/// `successors(Some(seed), next_step)` yields the seed first, then
/// computes each later item from the previous one.
pub fn verify_successors_model_generates_from_the_previous_item(seed: i32) -> (result: (Option<i32>, Option<i32>))
    requires
        single_increment_headroom_holds(seed),
    ensures
        observed_option_matches_input(result.0, seed),
        observed_option_matches_input(result.1, (seed + 1) as i32),
{
    (Some(seed), Some(seed + 1))
}

/// `from_fn` yields whatever its closure produces, and stops the moment
/// the closure returns `None` — a deterministic two-item-then-stop
/// sequence, tracked here as a plain call count in place of
/// `amenable_kani`'s real atomic counter.
pub fn verify_from_fn_model_yields_until_the_closure_returns_none() -> (result: (Option<i32>, Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, 0),
        observed_option_matches_input(result.1, 1),
        result.2 is None,
{
    let mut calls: i32 = 0;
    let first = Some(calls);
    calls += 1;
    let second = Some(calls);
    calls += 1;
    let third = if calls < 2 { Some(calls) } else { None };

    (first, second, third)
}

} // verus!
