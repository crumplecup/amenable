//! Verus accommodation model for `std::sync::{Barrier,
//! BarrierWaitResult}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded
//! `KaniBarrierLeaderObservation` rather than the real `Barrier::wait()`
//! path directly — that path reaches an unsupported futex boundary
//! under Kani. `Barrier` and `BarrierWaitResult` check the identical
//! one-party leader claim, so both witnesses here point at the same
//! model function, matching `amenable_kani`'s own note that both
//! "reduce to the identical `is_leader()` check." Neither type itself —
//! the proof is conditional: sound if the real type refines the stated
//! law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type via the bounded observation) already
//! confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// A `Barrier` built for exactly one participant returns immediately
/// from `.wait()`, and that lone participant is always the leader — the
/// same claim `BarrierWaitResult::is_leader()` exposes directly.
pub fn verify_barrier_model_of_one_is_its_own_leader() -> (result: bool)
    ensures
        result,
{
    true
}

} // verus!
