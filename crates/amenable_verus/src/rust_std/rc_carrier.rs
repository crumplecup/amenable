//! Verus spec for `alloc::rc::Rc<i32>`.
//!
//! Not the `strong_count`-tracking half of Kani's claim (clone
//! increments it, dropping the clone decrements it back) — the same
//! structural blocker `amenable_std::verus_gallery`'s
//! `cell_hidden_state_unreachable_via_plain_assume_specification`
//! finding already establishes for `Cell<T>`: `strong_count`'s value
//! after a `clone`/`drop` sequence is state shared across separate
//! calls (here, across separate `Rc` handles sharing one allocation, an
//! even harder case than `Cell`'s single shared reference), and
//! `assume_specification` has no mechanism to relate one call's effect
//! to a later, different call's observation. Real, narrower coverage
//! lands on the deref half instead — the same category split
//! `box_carrier.rs` already uses for `Box<i32>`.
//!
//! No local `assume_specification` here at all: `vstd`'s own
//! `std_specs/smart_ptrs.rs` already specifies `Rc::<T>::new` with
//! exactly the postcondition needed (`*v == t`, the deref operator
//! directly), found by reading vstd's source rather than guessed — a
//! second, local declaration for the same function would hit the same
//! duplicate-assume_specification wall the `TryFromIntError` finding
//! already documents (there, it crashed `verus` outright; here `verus`
//! reports a real, diagnosed "duplicate specification" error instead —
//! confirmed directly, not assumed to be the same failure mode).

use std::rc::Rc;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `Rc::new` derefs to the value it wraps — the deref half of the claim
/// the Kani harness checks.
pub fn verify_rc_derefs_to_the_wrapped_value(value: i32) -> (result: i32)
    ensures
        result == value,
{
    let rc = Rc::new(value);
    *rc
}

} // verus!
