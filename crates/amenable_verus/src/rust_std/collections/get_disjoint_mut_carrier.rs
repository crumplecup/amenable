//! Verus accommodation model for `core::slice::GetDisjointMutError`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for `get_disjoint_mut`'s
//! bounds-and-overlap check. `amenable_kani`'s own proof checks a
//! four-element array against three concrete index pairs directly (no
//! bounded observation needed here, since the real call doesn't time
//! out); this carrier states the same success/failure law as a boolean
//! function of a length and two candidate indices, generalizing the
//! three concrete cases the Kani proof checks. Not
//! `GetDisjointMutError`/`get_disjoint_mut` themselves — the proof is
//! conditional: sound if the real method refines this success rule,
//! which `amenable_kani`'s own harness (checking the real type directly)
//! already confirms independently, for the identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::values_are_equal;

verus! {

/// `get_disjoint_mut([i, j])` on a slice of length `len` succeeds
/// (`result == true`) exactly when `i` and `j` are distinct and both
/// in bounds; it fails — producing `GetDisjointMutError` — for either
/// overlapping (`i == j`) or out-of-bounds indices.
pub fn verify_get_disjoint_mut_model_rejects_overlap_and_out_of_bounds(len: u8, i: u8, j: u8) -> (result: bool)
    ensures
        values_are_equal(result, i != j && i < len && j < len),
{
    i != j && i < len && j < len
}

} // verus!
