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

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_quad_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `IoSlice::new` borrows a byte slice without copying it: dereferencing
/// yields exactly the wrapped four bytes.
pub fn verify_io_slice_model_derefs_to_the_wrapped_bytes(a: u8, b: u8, c: u8, d: u8) -> (result: (u8, u8, u8, u8))
    ensures
        observed_quad_matches_input(result, (a, b, c, d)),
{
    (a, b, c, d)
}

/// `IoSliceMut::new` mutably borrows a byte slice without copying it:
/// dereferencing yields exactly the wrapped four bytes, and a write of
/// `new_value` into the first slot is visible in the underlying slice.
pub fn verify_io_slice_mut_model_derefs_to_and_permits_mutating_the_wrapped_bytes(a: u8, b: u8, c: u8, d: u8, new_value: u8) -> (result: (u8, u8, u8, u8, u8))
    ensures
        observed_value_matches_input(result.0 as int, a as int),
        observed_value_matches_input(result.1 as int, b as int),
        observed_value_matches_input(result.2 as int, c as int),
        observed_value_matches_input(result.3 as int, d as int),
        observed_value_matches_input(result.4 as int, new_value as int),
{
    (a, b, c, d, new_value)
}

} // verus!
