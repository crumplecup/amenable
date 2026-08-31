//! Verus accommodation model for `core::sync::atomic::Ordering`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harness checks the real
//! `AtomicI32`/`Ordering::Relaxed` combination directly — every other
//! proof in this cluster uses `SeqCst`; this one confirms `Relaxed`
//! specifically, since the ordering variant only affects cross-thread
//! synchronization, not single-thread visibility. This carrier states
//! the same law directly: a `Relaxed` store is still observable via a
//! `Relaxed` load on the same atomic. Not `Ordering` itself — the proof
//! is conditional: sound if the real type refines this law, which
//! `amenable_kani`'s own
//! `verify_relaxed_ordering_still_makes_a_store_observable` harness
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A `Relaxed` store of `value` is observable via a `Relaxed` load on
/// the same atomic.
pub fn verify_atomic_ordering_model_relaxed_store_is_observable(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

} // verus!
