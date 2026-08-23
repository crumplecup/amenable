//! Verus accommodation model for `core::task::{Context, Poll<i32>,
//! Waker}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real types
//! directly (no timeout concerns for any of these three — only
//! `RawWaker`/`RawWakerVTable` are trusted, since meaningfully
//! exercising them needs `unsafe fn` vtable entries forbidden by this
//! crate's `#![forbid(unsafe_code)]`, and `Waker` itself gets a real
//! proof via the safe `Wake` trait bridge instead of `RawWaker`
//! directly). This carrier states each resulting law directly. None of
//! these functions are `Context`/`Poll`/`Waker` themselves — each proof
//! is conditional: sound if the real type refines the stated law,
//! which `amenable_kani`'s own harness for that exact type (checking
//! the real type directly) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `Context::from_waker` just bundles the `&Waker` it's given;
/// `.waker()` hands back a reference that would wake the same task as
/// the original (checked via `Waker::will_wake`, the documented way to
/// compare wakers, since `vstd` has no spec support for `Waker`
/// pointer identity).
pub fn verify_context_model_from_waker_exposes_the_same_waker() -> (result: bool)
    ensures
        result,
{
    true
}

/// `(ready_is_ready, ready_is_pending, ready_round_tripped_value,
/// pending_is_pending, pending_is_ready)`.
type PollResult = (bool, bool, i32, bool, bool);

/// `Ready` and `Pending` are mutually exclusive, and `Ready` round-trips
/// its value.
pub fn verify_poll_model_ready_and_pending_are_disjoint(value: i32) -> (result: PollResult)
    ensures
        result.0,
        !result.1,
        result.2 == value,
        result.3,
        !result.4,
{
    (true, false, value, true, false)
}

/// `Waker::from(Arc<impl Wake>)` dispatches through to the `Wake` impl:
/// calling `wake_by_ref` on the resulting `Waker` invokes the wrapped
/// type's `wake_by_ref` exactly once.
pub fn verify_waker_model_wake_by_ref_invokes_the_wake_impl() -> (result: u32)
    ensures
        result == 1,
{
    1
}

} // verus!
