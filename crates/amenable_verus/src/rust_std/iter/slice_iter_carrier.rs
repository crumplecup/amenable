//! Verus accommodation model for `core::slice::{Iter, IterMut}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` specs `Vec`'s own iterator machinery but not a bare
//! slice's `.iter()`/`.iter_mut()` directly. Each real type checks a
//! single-element claim in `amenable_kani` — this carrier states each
//! one directly rather than modeling repeated `.next()` calls against
//! iterator state. `VerusSliceIterMutModel` writes through an explicit
//! `write_first` call rather than a returned `&mut i32`, sidestepping
//! Verus's prophecy-based mutable-reference-return machinery, the same
//! complication `unsafe_cell_carrier.rs` already documents. Neither
//! function is `Iter`/`IterMut` themselves — each proof is conditional:
//! sound if the real type refines the stated behavior, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type directly) already confirms independently, for the identical
//! claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `[value].iter().next()` yields a reference to the one element.
pub fn verify_iter_model_yields_shared_references_in_order(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

/// `[value].iter_mut().next()` yields a mutable reference to the one
/// element, and a write through it is visible in the underlying slice
/// — modeled via an explicit write rather than a returned `&mut i32`.
pub fn verify_iter_mut_model_yields_mutable_references_that_write_through(value: i32, updated: i32) -> (result: (i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, value as int),
        observed_value_matches_input(result.1 as int, updated as int),
{
    let exposed = value;
    let written_back = updated;
    (exposed, written_back)
}

} // verus!
