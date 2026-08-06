//! Verus spec for `alloc::sync::Arc<i32>`.
//!
//! Same shape and same real blocker as `rc_carrier.rs`: not the
//! `strong_count`-tracking half of Kani's claim (cross-call shared
//! state, the same structural wall `Cell<T>`'s gallery finding already
//! establishes), just the deref half. No local `assume_specification`
//! here either — `vstd`'s `std_specs/smart_ptrs.rs` already specifies
//! `Arc::<T>::new` with `*v == t` directly.

use std::sync::Arc;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `Arc::new` derefs to the value it wraps — the deref half of the
/// claim the Kani harness checks.
pub fn verify_arc_derefs_to_the_wrapped_value(value: i32) -> (result: i32)
    ensures
        result == value,
{
    let arc = Arc::new(value);
    *arc
}

} // verus!
