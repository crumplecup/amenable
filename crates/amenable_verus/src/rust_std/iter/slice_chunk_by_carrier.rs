//! Verus accommodation model for `core::slice::{ChunkBy, ChunkByMut}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for predicate-driven adjacent
//! grouping. Both real types check the identical grouping rule in
//! `amenable_kani` (`ChunkByMut`'s own harness checks only the chunk
//! length, a strict subset of what `ChunkBy`'s harness checks), so both
//! witnesses below point at the same model function here, mirroring how
//! `amenable_kani` itself lets `ChunkByMut`'s claim ride on the stronger
//! `ChunkBy` observation shape. Not `ChunkBy`/`ChunkByMut` themselves —
//! each proof is conditional: sound if the real type refines the stated
//! grouping, which `amenable_kani`'s own harness for that exact type
//! (checking the real type directly) already confirms independently, for
//! the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `ChunkBy`'s whole postcondition: two adjacent elements are grouped
/// into one two-element chunk exactly when `grouped_together` holds
/// (standing in for the predicate); when it doesn't, they split into a
/// one-element leading chunk and a one-element trailing chunk.
pub open spec fn chunk_by_result_matches_grouping(grouped_together: bool, result: (u8, bool)) -> bool {
    (grouped_together ==> result.0 == 2 && !result.1)
        && (!grouped_together ==> result.0 == 1 && result.1)
}

/// Two adjacent elements are grouped into one two-element chunk exactly
/// when `grouped_together` holds (standing in for the predicate); when it
/// doesn't, they split into a one-element leading chunk and a
/// one-element trailing chunk.
pub fn verify_chunk_by_model_groups_adjacent_elements_matching_the_predicate(grouped_together: bool) -> (result: (u8, bool))
    ensures
        chunk_by_result_matches_grouping(grouped_together, result),
{
    if grouped_together { (2, false) } else { (1, true) }
}

} // verus!
