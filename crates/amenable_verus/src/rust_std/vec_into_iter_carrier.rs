//! Verus spec for `alloc::vec::IntoIter<i32>`.
//!
//! Covers the order-preserving half of Kani's claim only — the same
//! genuine category difference `box_carrier.rs`/`vec_carrier.rs` document:
//! Kani's harness also checks Drop-glue timing (a separate drop-witness
//! type confirms `next()` transfers a yielded value without dropping it,
//! and that dropping the `IntoIter` drops every remaining value exactly
//! once), not the kind of pre/postcondition Verus states.
//!
//! `vstd` already gives `Vec::into_iter`/`IntoIter<T, A>` real, dedicated
//! spec support (`std_specs/vec.rs`), but only through the generic
//! `Iterator`/`FromIterator` trait-extension machinery — calling `.next()`
//! directly on a concrete `IntoIter<i32>` is itself `not supported`
//! (confirmed empirically: Verus's own diagnostic for the direct call
//! names a per-type `next` `assume_specification` as the missing piece,
//! which `vstd` doesn't declare for any concrete iterator type, including
//! its own `slice`/`VecDeque` iterators — this crate's own real Verus
//! test suite (`rust_verify_test/tests/iterators.rs`) only ever
//! exercises `Vec` iteration via `.into_iter().collect()`/`.rev()`, never
//! raw `.next()`). `.collect()` ties the returned collection back to the
//! consumed iterator's `remaining()` (which `axiom_spec_into_iter` ties
//! to the original vector's `@` view), giving the same order-preservation
//! claim through the one idiom `vstd` actually supports for this type.

use std::vec::Vec;

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `Vec::into_iter().collect()` round-trips the vector's elements by
/// value, in order — the order-preserving half of the claim the Kani
/// harness checks (not the Drop-glue-timing half; see the module doc
/// comment).
pub fn verify_vec_into_iter_round_trips_via_collect(a: i32, b: i32) -> (result: bool)
    ensures
        result,
{
    let v: Vec<i32> = vec![a, b];
    let ghost original_view = v@;

    let collected: Vec<i32> = v.into_iter().collect();
    assert(collected@ =~= original_view);

    collected.len() == 2 && collected[0] == a && collected[1] == b
}

} // verus!
