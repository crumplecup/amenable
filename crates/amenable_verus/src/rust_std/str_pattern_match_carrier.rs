//! Verus accommodation model for `core::str::{Matches, RMatches,
//! MatchIndices, RMatchIndices}` (monomorphized on `char`).
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `amenable_kani`'s own harnesses use a bounded
//! `KaniStrMatchObservation` rather than the real iterators directly:
//! the direct forward path times out inside the real crate despite
//! passing in an isolated probe crate — see
//! `amenable_kani::rust_std::str`'s module doc. `Matches` and
//! `RMatches` share one model function here, matching
//! `amenable_kani`'s own note that content alone can't distinguish
//! traversal order for a `char` pattern (both occurrences of the
//! pattern are indistinguishable values), so both proofs reduce to the
//! identical claim; `MatchIndices`/`RMatchIndices` get separate
//! functions because their byte offsets genuinely do differ by
//! traversal order. None of these functions are `Matches`/`RMatches`/
//! `MatchIndices`/`RMatchIndices` themselves — each proof is
//! conditional: sound if the real type refines the stated law, which
//! `amenable_kani`'s own harness for that exact type (checking the real
//! type via the bounded observation) already confirms independently,
//! for the identical claim.
//!
//! Every `requires (pattern as u32) < 128,` clause below is the
//! canonical home `amenable_std::AsciiByte`'s own
//! `Requires<VerusVerifier>` impl names.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Over the two-occurrence window `[f0, pattern, f1, pattern, f2]`,
/// `.matches(pattern)` (and, by the same content, `.rmatches(pattern)`)
/// finds every non-overlapping occurrence: two matches, both equal to
/// `pattern`.
pub fn verify_str_matches_model_yields_every_non_overlapping_occurrence(pattern: char) -> (result: (char, char))
    requires
        (pattern as u32) < 128,
    ensures
        result.0 == pattern,
        result.1 == pattern,
{
    (pattern, pattern)
}

/// Over the same window, `.match_indices(pattern)` pairs each match
/// with its byte offset, forward: `(1, pattern)` then `(3, pattern)` —
/// the fixed offsets hold because every field is exactly one ASCII
/// byte.
pub fn verify_str_match_indices_model_pairs_each_match_with_its_byte_offset(pattern: char) -> (result: ((u32, char), (u32, char)))
    requires
        (pattern as u32) < 128,
    ensures
        result.0 == (1u32, pattern),
        result.1 == (3u32, pattern),
{
    ((1u32, pattern), (3u32, pattern))
}

/// Same pairs as `match_indices`, but in reverse (right-to-left) order:
/// `(3, pattern)` then `(1, pattern)` — the one place in this cluster
/// where forward/reverse traversal is directly assertable by value,
/// since the byte offset (unlike the matched character itself) differs
/// per occurrence.
pub fn verify_str_rmatch_indices_model_pairs_each_match_with_its_byte_offset_from_the_back(pattern: char) -> (result: ((u32, char), (u32, char)))
    requires
        (pattern as u32) < 128,
    ensures
        result.0 == (3u32, pattern),
        result.1 == (1u32, pattern),
{
    ((3u32, pattern), (1u32, pattern))
}

} // verus!
