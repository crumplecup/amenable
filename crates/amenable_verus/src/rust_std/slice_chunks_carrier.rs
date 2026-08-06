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

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// `[a, b, c].chunks(2)` yields `(a, b)` then the short last chunk `c`.
pub fn verify_chunks_model_yields_non_overlapping_groups_with_a_short_last_chunk(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        result.0 == (a, b),
        result.1 == c,
{
    ((a, b), c)
}

/// `[a, b, c].chunks_exact(2)` yields only the full chunk `(a, b)`; the
/// short remainder `c` is dropped from iteration but reachable via
/// `.remainder()`.
pub fn verify_chunks_exact_model_discards_a_short_remainder(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        result.0 == (a, b),
        result.1 == c,
{
    ((a, b), c)
}

/// `[a, b].chunks_mut(2)` writes `+10` through the single full chunk.
pub fn verify_chunks_mut_model_writes_through_every_chunk(a: i32, b: i32) -> (result: (i32, i32))
    requires
        a <= i32::MAX - 10,
        b <= i32::MAX - 10,
    ensures
        result.0 == a + 10,
        result.1 == b + 10,
{
    (a + 10, b + 10)
}

/// `[a, b, c].chunks_exact_mut(2)` writes `+10` through the full chunk
/// `(a, b)`; the short remainder `c` is never visited.
pub fn verify_chunks_exact_mut_model_leaves_the_remainder_untouched(a: i32, b: i32, c: i32) -> (result: (i32, i32, i32))
    requires
        a <= i32::MAX - 10,
        b <= i32::MAX - 10,
    ensures
        result.0 == a + 10,
        result.1 == b + 10,
        result.2 == c,
{
    (a + 10, b + 10, c)
}

/// `[a, b, c].rchunks(2)` groups from the back: the full chunk `(b, c)`
/// first, then the short chunk `a` at the front.
pub fn verify_rchunks_model_groups_from_the_back(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        result.0 == (b, c),
        result.1 == a,
{
    ((b, c), a)
}

/// `[a, b, c].rchunks_exact(2)` yields only the full chunk `(b, c)` from
/// the back; the short remainder `a` sits at the front, reachable via
/// `.remainder()`.
pub fn verify_rchunks_exact_model_discards_a_short_remainder_at_the_front(a: i32, b: i32, c: i32) -> (result: ((i32, i32), i32))
    ensures
        result.0 == (b, c),
        result.1 == a,
{
    ((b, c), a)
}

/// `[a, b, c].rchunks_exact_mut(2)` writes `+10` through the full chunk
/// `(b, c)`; the front remainder `a` is never visited.
pub fn verify_rchunks_exact_mut_model_leaves_the_front_remainder_untouched(a: i32, b: i32, c: i32) -> (result: (i32, i32, i32))
    requires
        b <= i32::MAX - 10,
        c <= i32::MAX - 10,
    ensures
        result.0 == a,
        result.1 == b + 10,
        result.2 == c + 10,
{
    (a, b + 10, c + 10)
}

/// `[a, b].rchunks_mut(2)` writes `+10` through the single full chunk
/// (grouped from the back).
pub fn verify_rchunks_mut_model_writes_through_every_chunk(a: i32, b: i32) -> (result: (i32, i32))
    requires
        a <= i32::MAX - 10,
        b <= i32::MAX - 10,
    ensures
        result.0 == a + 10,
        result.1 == b + 10,
{
    (a + 10, b + 10)
}

/// `[a, b, c].windows(2)` yields overlapping pairs `(a, b)` then `(b, c)`
/// — consecutive windows share an element, unlike `Chunks`.
pub fn verify_windows_model_yields_overlapping_slices(a: i32, b: i32, c: i32) -> (result: ((i32, i32), (i32, i32)))
    ensures
        result.0 == (a, b),
        result.1 == (b, c),
{
    ((a, b), (b, c))
}

} // verus!
