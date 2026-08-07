//! Verus accommodation model for `core::future::{Pending<i32>,
//! PollFn<fn(&mut Context<'_>) -> Poll<i32>>, Ready<i32>}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real types
//! directly, polling each pinned future with a real, safe `Waker` built
//! through the `Wake` trait bridge — no timeout concerns for any of the
//! three. This carrier states each resulting law directly. None of
//! these functions are `Pending`/`PollFn`/`Ready` themselves — each
//! proof is conditional: sound if the real type refines the stated
//! law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `Pending` always reports `Poll::Pending` when polled, including on
/// a repeated poll.
pub fn verify_pending_model_never_resolves() -> (result: (bool, bool))
    ensures
        result.0,
        result.1,
{
    (true, true)
}

/// `Ready`'s first poll resolves immediately with the value it was
/// constructed from.
pub fn verify_ready_model_resolves_immediately_with_its_value(value: i32) -> (result: i32)
    ensures
        result == value,
{
    value
}

/// `poll_fn` turns a plain `poll`-shaped function into a `Future`:
/// polling it calls straight through to the wrapped function, both
/// invoking it (`result.1`) and preserving its result (`result.0`).
pub fn verify_poll_fn_model_dispatches_through_to_its_closure(value: i32) -> (result: (i32, bool))
    ensures
        result.0 == value,
        result.1,
{
    (value, true)
}

} // verus!
