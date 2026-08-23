//! Verus spec for `alloc::collections::TryReserveError`.
//!
//! Covers the content-preservation half of Kani's claim only — the same
//! genuine category difference other carriers in this crate document:
//! `vstd` already registers `Vec::try_reserve` (`std_specs/vec.rs`), but
//! its postcondition only states `final(vec)@ == old(vec)@` unconditionally
//! — it does not distinguish the `Ok`/`Err` outcome at all, since whether
//! a given capacity request actually overflows is a fact about real
//! allocator behavior that `TryReserveError` (`external_body`, fully
//! opaque — no variant, no `kind()` spec) doesn't expose to the ghost
//! model. Kani's harness additionally asserts
//! `.try_reserve(usize::MAX).is_err()`; that half stays Kani-only.

use std::vec::Vec;

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A `try_reserve` call — whether it succeeds or fails — never changes
/// values already stored in the vector, matching `vstd`'s own
/// unconditional `final(vec)@ == old(vec)@` postcondition for
/// `Vec::try_reserve` (not the "rejects an impossible capacity" half of
/// the claim the Kani harness checks; see the module doc comment).
pub fn verify_try_reserve_preserves_vec_contents_regardless_of_outcome(first: i32, second: i32) -> (result: bool)
    ensures
        result,
{
    let mut v: Vec<i32> = vec![first, second];

    let before0 = v[0];
    let before1 = v[1];

    let _ = v.try_reserve(usize::MAX);

    let after0 = v[0];
    let after1 = v[1];
    assert(after0 == before0);
    assert(after1 == before1);

    after0 == before0 && after1 == before1
}

} // verus!
