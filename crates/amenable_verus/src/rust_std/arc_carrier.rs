//! Verus spec for `alloc::sync::Arc<i32>`.
//!
//! Same shape and same real blocker as `rc_carrier.rs`: not the
//! `strong_count`-tracking half of Kani's claim (cross-call shared
//! state, the same structural wall `Cell<T>`'s gallery finding already
//! establishes), just the deref half. No local `assume_specification`
//! here either — `vstd`'s `std_specs/smart_ptrs.rs` already specifies
//! `Arc::<T>::new` with `*v == t` directly.

use std::sync::Arc;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `Arc::new` derefs to the value it wraps — the deref half of the
/// claim the Kani harness checks.
pub fn verify_arc_derefs_to_the_wrapped_value(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    let arc = Arc::new(value);
    *arc
}

} // verus!
