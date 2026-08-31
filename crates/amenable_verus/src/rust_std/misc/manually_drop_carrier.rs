//! Verus spec for `std::mem::ManuallyDrop<i32>`.

use std::mem::ManuallyDrop;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::observed_value_matches_input;

verus! {

/// `ManuallyDrop::new(value)` derefs to exactly `value`, and
/// `ManuallyDrop::into_inner` returns exactly `value` — the same claim
/// the Kani/Creusot harnesses check.
pub fn verify_manually_drop_derefs_and_into_inner_round_trip(value: i32) -> (result: (i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, value as int),
        observed_value_matches_input(result.1 as int, value as int),
{
    let wrapped = ManuallyDrop::new(value);
    let deref_value = *wrapped;
    (deref_value, ManuallyDrop::into_inner(wrapped))
}

} // verus!
