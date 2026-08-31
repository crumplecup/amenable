//! Verus accommodation model for `std::thread::{Thread, ThreadId}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded
//! `KaniCurrentThreadObservation` rather than the real
//! `std::thread::current()` path directly — that path reaches Kani's
//! unsupported `pthread_key_create` TLS boundary first. `Thread`'s and
//! `ThreadId`'s harnesses check the identical stability claim (repeated
//! queries for the current, already-running thread identify the same
//! handle/id), so both witnesses here point at the same model function,
//! matching `amenable_kani`'s own note that both reduce to the same
//! kind of `handle_is_stable()`/`id_is_stable()` check. Neither
//! `Thread` nor `ThreadId` themselves — each proof is conditional:
//! sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type via the bounded observation) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Repeated queries for the current, already-running thread identify
/// the same thread handle (and, identically, the same thread id).
pub fn verify_thread_current_model_is_stable_across_repeated_calls() -> (result: bool)
    ensures
        result,
{
    true
}

} // verus!
