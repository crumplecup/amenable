//! Verus spec for `alloc::boxed::Box<i32>`.
//!
//! Covers the deref/write-through half of Kani's claim only — not a
//! weakened claim, a genuine category difference: Kani's harness also
//! checks Drop-glue timing (a displaced value drops exactly once on
//! reassignment through `DerefMut`, the replacement drops exactly once
//! when the box itself drops), which is a property about the ORDER and
//! COUNT of side-effecting `Drop::drop` invocations across statements —
//! not the kind of pre/postcondition Verus's specification style states
//! at all. Verus proves functional correctness of a call's inputs and
//! outputs, not a trace of destructor calls between them.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// The `Box` deref/write-through law this carrier establishes.
pub open spec fn box_derefs_and_writes_through(
    deref_value: int,
    observed_after_write: int,
    initial: int,
    updated: int,
) -> bool {
    deref_value == initial && observed_after_write == updated
}

/// `Box::new` derefs to the value it wraps, and a write through
/// `DerefMut` is visible on the next read — the deref/write-through half
/// of the claim the Kani harness checks.
pub fn verify_box_derefs_and_writes_through(value: i32, updated: i32) -> (result: (i32, i32))
    ensures
        box_derefs_and_writes_through(result.0, result.1, value, updated),
{
    let mut boxed = Box::new(value);
    let deref_value = *boxed;
    *boxed = updated;
    (deref_value, *boxed)
}

} // verus!
