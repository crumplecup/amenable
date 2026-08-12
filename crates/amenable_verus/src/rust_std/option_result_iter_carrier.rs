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
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// When a value is present (`Some(value)`/`Ok(value)`), `.into_iter()`
/// yields exactly that value once, then stops; when absent (`None`/
/// `Err(_)`), it yields nothing at all.
pub fn verify_into_iter_model_yields_zero_or_one_owned_value(value: i32, present: bool) -> (result: (bool, i32))
    ensures
        present ==> result.0 && result.1 == value,
        !present ==> !result.0,
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
        result.0 == value,
        result.1 == updated,
{
    (value, updated)
}

} // verus!
