//! Verus accommodation model for `core::slice::{Chunks, ChunksExact,
//! ChunksMut, ChunksExactMut, RChunks, RChunksExact, RChunksExactMut,
//! RChunksMut, Windows}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for fixed-size or windowed
//! grouping over a slice. Each real type checks a fixed three- or
//! two-element grouping claim in `amenable_kani`, using either a `[i32]`
//! literal comparison or a mutating write-through — this carrier states
//! each one directly as a tuple-valued function rather than modeling
//! repeated `.next()` calls against iterator state. The `*Mut` variants
//! model the write-through the same way `unsafe_cell_carrier.rs` and
//! `slice_iter_carrier.rs` do: an explicit updated value passed in and
//! echoed back, sidestepping Verus's prophecy-based mutable-reference-
//! return machinery. None of these functions are the real iterator types
//! themselves — each proof is conditional: sound if the real type refines
//! the stated grouping, which `amenable_kani`'s own harness for that
//! exact type (checking the real type directly) already confirms
//! independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::{
    observed_pair_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

// The wider increment-headroom precondition `amenable_std::
// IncrementHeadroom` names -- a real spec fn defined once in
// `iter_sequence_carrier`, called from here directly rather than
// restated inline. `#[cfg(verus_keep_ghost)]`-gated like every other
// spec-only import in this crate (see `chars_carrier.rs`): plain
// `spec fn`s carry no runtime representation, so this import only
// resolves when Verus's own ghost content is retained, not under
// ordinary `cargo check`.
#[cfg(verus_keep_ghost)]
use crate::rust_std::iter_sequence_carrier::ten_increment_headroom_holds;

verus! {

/// The shared write-through law for the mutable slice-chunk models: the
/// observed post-state element is exactly the pre-state element plus the
/// fixed `+10` update the model applies.
pub open spec fn ten_increment_write_through(before: int, after: int) -> bool {
    after == before + 10
}

/// `[a, b, c].chunks(2)` yields `(a, b)` then the short last chunk `c`.
pub fn verify_chunks_model_yields_non_overlapping_groups_with_a_short_last_chunk(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        observed_pair_matches_input(result.0, (a, b)),
        observed_value_matches_input(result.1 as int, c as int),
{
    ((a, b), c)
}

/// `[a, b, c].chunks_exact(2)` yields only the full chunk `(a, b)`; the
/// short remainder `c` is dropped from iteration but reachable via
/// `.remainder()`.
pub fn verify_chunks_exact_model_discards_a_short_remainder(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        observed_pair_matches_input(result.0, (a, b)),
        observed_value_matches_input(result.1 as int, c as int),
{
    ((a, b), c)
}

/// `[a, b].chunks_mut(2)` writes `+10` through the single full chunk.
pub fn verify_chunks_mut_model_writes_through_every_chunk(a: i32, b: i32) -> (result: (i32, i32))
    requires
        ten_increment_headroom_holds(a),
        ten_increment_headroom_holds(b),
    ensures
        ten_increment_write_through(a as int, result.0 as int),
        ten_increment_write_through(b as int, result.1 as int),
{
    (a + 10, b + 10)
}

/// `[a, b, c].chunks_exact_mut(2)` writes `+10` through the full chunk
/// `(a, b)`; the short remainder `c` is never visited.
pub fn verify_chunks_exact_mut_model_leaves_the_remainder_untouched(a: i32, b: i32, c: i32) -> (result: (i32, i32, i32))
    requires
        ten_increment_headroom_holds(a),
        ten_increment_headroom_holds(b),
    ensures
        ten_increment_write_through(a as int, result.0 as int),
        ten_increment_write_through(b as int, result.1 as int),
        observed_value_matches_input(result.2 as int, c as int),
{
    (a + 10, b + 10, c)
}

/// `[a, b, c].rchunks(2)` groups from the back: the full chunk `(b, c)`
/// first, then the short chunk `a` at the front.
pub fn verify_rchunks_model_groups_from_the_back(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        observed_pair_matches_input(result.0, (b, c)),
        observed_value_matches_input(result.1 as int, a as int),
{
    ((b, c), a)
}

/// `[a, b, c].rchunks_exact(2)` yields only the full chunk `(b, c)` from
/// the back; the short remainder `a` sits at the front, reachable via
/// `.remainder()`.
pub fn verify_rchunks_exact_model_discards_a_short_remainder_at_the_front(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        observed_pair_matches_input(result.0, (b, c)),
        observed_value_matches_input(result.1 as int, a as int),
{
    ((b, c), a)
}

/// `[a, b, c].rchunks_exact_mut(2)` writes `+10` through the full chunk
/// `(b, c)`; the front remainder `a` is never visited.
pub fn verify_rchunks_exact_mut_model_leaves_the_front_remainder_untouched(a: i32, b: i32, c: i32) -> (result: (i32, i32, i32))
    requires
        ten_increment_headroom_holds(b),
        ten_increment_headroom_holds(c),
    ensures
        observed_value_matches_input(result.0 as int, a as int),
        ten_increment_write_through(b as int, result.1 as int),
        ten_increment_write_through(c as int, result.2 as int),
{
    (a, b + 10, c + 10)
}

/// `[a, b].rchunks_mut(2)` writes `+10` through the single full chunk
/// (grouped from the back).
pub fn verify_rchunks_mut_model_writes_through_every_chunk(a: i32, b: i32) -> (result: (i32, i32))
    requires
        ten_increment_headroom_holds(a),
        ten_increment_headroom_holds(b),
    ensures
        ten_increment_write_through(a as int, result.0 as int),
        ten_increment_write_through(b as int, result.1 as int),
{
    (a + 10, b + 10)
}

/// `[a, b, c].windows(2)` yields overlapping pairs `(a, b)` then `(b, c)`
/// — consecutive windows share an element, unlike `Chunks`.
pub fn verify_windows_model_yields_overlapping_slices(a: i32, b: i32, c: i32) -> (result: ((i32, i32), (i32, i32)))
    ensures
        observed_pair_matches_input(result.0, (a, b)),
        observed_pair_matches_input(result.1, (b, c)),
{
    ((a, b), (b, c))
}

} // verus!
