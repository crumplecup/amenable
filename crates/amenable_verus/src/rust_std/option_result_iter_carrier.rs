//! Verus accommodation model for `core::option::{IntoIter, Iter,
//! IterMut}` and `core::result::{IntoIter, Iter, IterMut}` (all
//! monomorphized on `i32`).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses check the real types
//! directly (no timeout concerns). `Option`'s and `Result`'s iterator
//! adapters check the identical shape per kind — `IntoIter` yields the
//! contained value (`Some`/`Ok`) once then stops, or nothing at all
//! (`None`/`Err`); `Iter` yields a shared reference to the contained
//! value; `IterMut` yields a mutable reference and writes through — so
//! each kind's `Option`/`Result` witnesses share one model function
//! here. None of these functions are the real types themselves — each
//! proof is conditional: sound if the real type refines the stated
//! law, which `amenable_kani`'s own harness for that exact type
//! (checking the real type directly) already confirms independently,
//! for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `.into_iter()`'s whole postcondition: when a value is present
/// (`Some(value)`/`Ok(value)`), it yields exactly that value once, then
/// stops; when absent (`None`/`Err(_)`), it yields nothing at all.
pub open spec fn into_iter_yields_zero_or_one_owned_value(present: bool, value: i32, result: (bool, i32)) -> bool {
    (present ==> result.0 && result.1 == value) && (!present ==> !result.0)
}

/// When a value is present (`Some(value)`/`Ok(value)`), `.into_iter()`
/// yields exactly that value once, then stops; when absent (`None`/
/// `Err(_)`), it yields nothing at all.
pub fn verify_into_iter_model_yields_zero_or_one_owned_value(value: i32, present: bool) -> (result: (bool, i32))
    ensures
        into_iter_yields_zero_or_one_owned_value(present, value, result),
{
    if present { (true, value) } else { (false, 0) }
}

/// `.iter()` borrows instead of consuming: it yields a shared reference
/// to the contained value, not the value itself.
pub fn verify_iter_model_yields_zero_or_one_reference(value: i32) -> (result: i32)
    ensures
        observed_value_matches_input(result as int, value as int),
{
    value
}

/// `.iter_mut()` yields a mutable reference to the contained value, and
/// a write through it is visible afterward.
pub fn verify_iter_mut_model_writes_through(value: i32, updated: i32) -> (result: (i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, value as int),
        observed_value_matches_input(result.1 as int, updated as int),
{
    (value, updated)
}

} // verus!
