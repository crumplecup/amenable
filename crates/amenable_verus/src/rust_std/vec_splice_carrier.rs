//! Verus accommodation model for `alloc::vec::Splice`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Covers the replace-and-yield-removed half of the claim
//! `amenable_kani`'s own `verify_splice_replaces_a_range_and_yields_
//! what_it_removed` harness checks — splicing out the middle element of
//! a three-element sequence and replacing it with two new elements —
//! not the second half (dropping an unfinished `Splice` mid-iteration
//! still completes the replacement), which is Drop-glue timing, the
//! same kind of genuinely different-in-kind claim `box_carrier.rs`/
//! `vec_carrier.rs` already document as Kani-only elsewhere in this
//! crate. `VerusSpliceModel` isn't `Splice` and doesn't claim to be —
//! the proof is conditional: sound if the real `Splice` refines this
//! replace-and-yield-removed law, which `amenable_kani`'s own harness
//! (checking the real `Vec::splice` directly) already confirms
//! independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_value_matches_input;

verus! {

/// The spliced vector's final content: the middle element replaced by
/// `[x, y]`, the rest untouched.
pub open spec fn splice_result_matches(result: Seq<i32>, a: i32, x: i32, y: i32, c: i32) -> bool {
    result == seq![a, x, y, c]
}

/// Splicing out the middle element of `[a, b, c]` and replacing it with
/// `[x, y]` yields the removed element `b`, and leaves `[a, x, y, c]`
/// behind — the replace-and-yield-removed law the real `Splice` is
/// expected to refine.
pub fn verify_splice_model_replaces_a_range_and_yields_what_it_removed(a: i32, b: i32, c: i32, x: i32, y: i32) -> (result: (i32, Vec<i32>))
    ensures
        observed_value_matches_input(result.0 as int, b as int),
        splice_result_matches(result.1@, a, x, y, c),
{
    let removed = b;
    let spliced: Vec<i32> = vec![a, x, y, c];

    (removed, spliced)
}

} // verus!
