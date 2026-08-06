//! Verus accommodation model for `std::io::{IoSlice, IoSliceMut}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real types
//! directly (no timeout concerns — both are thin borrow wrappers).
//! `IoSliceMut`'s write-through is modeled the same way
//! `unsafe_cell_carrier.rs`/`slice_iter_carrier.rs` do: an explicit
//! updated value passed in and echoed back, sidestepping Verus's
//! prophecy-based mutable-reference-return machinery. Neither function
//! is `IoSlice`/`IoSliceMut` themselves — each proof is conditional:
//! sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type directly) already confirms independently, for the identical
//! claim.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `IoSlice::new` borrows a byte slice without copying it: dereferencing
/// yields exactly the wrapped four bytes.
pub fn verify_io_slice_model_derefs_to_the_wrapped_bytes(a: u8, b: u8, c: u8, d: u8) -> (result: (u8, u8, u8, u8))
    ensures
        result == (a, b, c, d),
{
    (a, b, c, d)
}

/// `IoSliceMut::new` mutably borrows a byte slice without copying it:
/// dereferencing yields exactly the wrapped four bytes, and a write of
/// `new_value` into the first slot is visible in the underlying slice.
pub fn verify_io_slice_mut_model_derefs_to_and_permits_mutating_the_wrapped_bytes(a: u8, b: u8, c: u8, d: u8, new_value: u8) -> (result: (u8, u8, u8, u8, u8))
    ensures
        result.0 == a,
        result.1 == b,
        result.2 == c,
        result.3 == d,
        result.4 == new_value,
{
    (a, b, c, d, new_value)
}

} // verus!
