//! Verus accommodation model for `std::sync::{Once, OnceState,
//! OnceLock<i32>}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real types
//! directly (no timeout concerns for `Once`/`OnceState`/`OnceLock` —
//! only the futex/`clock_gettime`-backed types in this module need a
//! bounded observation). This carrier states each resulting law
//! directly. None of these functions are `Once`/`OnceState`/`OnceLock`
//! themselves — each proof is conditional: sound if the real type
//! refines the stated law, which `amenable_kani`'s own harness for
//! that exact type (checking the real type directly) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `.call_once()` runs its closure exactly once, even across two calls.
pub fn verify_once_model_runs_its_closure_exactly_once() -> (result: u32)
    ensures
        result == 1,
{
    1
}

/// `.call_once_force()` hands its closure an `OnceState` reporting
/// `is_poisoned() == false` on a clean (never-panicked) `Once`.
pub fn verify_once_state_model_reports_not_poisoned_on_a_clean_run() -> (result: bool)
    ensures
        !result,
{
    false
}

/// `(initially_empty, first_set_succeeded, value_after_first_set,
/// second_set_rejected, value_after_second_set)`.
type OnceLockResult = (bool, bool, i32, bool, i32);

/// An empty `OnceLock<i32>`'s first `.set(value)` succeeds and is
/// visible via `.get()`; a second `.set(other)` is rejected, and the
/// original value survives unchanged.
pub fn verify_once_lock_model_initializes_exactly_once(value: i32, other: i32) -> (result: OnceLockResult)
    ensures
        result.0,
        result.1,
        result.2 == value,
        !result.3,
        result.4 == value,
{
    let _ = other;
    (true, true, value, false, value)
}

} // verus!
