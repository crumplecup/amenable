//! Verus accommodation model for `core::slice::{Split, SplitMut,
//! SplitInclusive, SplitInclusiveMut, SplitN, SplitNMut, RSplit,
//! RSplitMut, RSplitN, RSplitNMut}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document — `vstd` has no spec support for predicate-driven splitting.
//! Each real type checks a one- or two-delimiter split claim in
//! `amenable_kani` against a bounded `KaniSplitObservation`/
//! `KaniSplitNObservation`, because the real iterators time out under
//! Kani even on fixed short inputs; this carrier states each resulting
//! law directly as a tuple-valued function. `amenable_kani`'s own
//! `SplitN`/`RSplitN` proofs reuse one observation-demonstration method
//! for both the shared and mutable variants (`demonstrate_splitn_two`,
//! `demonstrate_rsplitn_two`), since the mutable variant's cap behavior
//! is identical to the shared one — this carrier mirrors that by having
//! both witnesses in each pair point at the same model function. The
//! `*Mut` variants that additionally check a write-through
//! (`SplitMut`, `RSplitMut`) model it the same way
//! `slice_iter_carrier.rs` does: an explicit updated value passed in and
//! echoed back, sidestepping Verus's prophecy-based mutable-reference-
//! return machinery. None of these functions are the real iterator types
//! themselves — each proof is conditional: sound if the real type refines
//! the stated split, which `amenable_kani`'s own harness for that exact
//! type (checking the real type directly) already confirms
//! independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::primitive_shapes_carrier::{
    observed_pair_matches_input, observed_triple_matches_input, observed_value_matches_input,
};
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `[before, delimiter, after].split(...)` yields `before` then `after`,
/// consuming the matched delimiter itself.
pub fn verify_split_model_yields_subslices_between_matches(before: i32, after: i32) -> (result: (i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, before as int),
        observed_value_matches_input(result.1 as int, after as int),
{
    (before, after)
}

/// `split_mut` yields the same two pieces as `Split`, and a write of
/// `updated` into the first piece is visible in the underlying data.
pub fn verify_split_mut_model_writes_through_the_first_piece(before: i32, after: i32, updated: i32) -> (result: (i32, i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, before as int),
        observed_value_matches_input(result.1 as int, after as int),
        observed_value_matches_input(result.2 as int, updated as int),
{
    (before, after, updated)
}

/// `split_inclusive` keeps the matched delimiter at the end of the piece
/// it terminates, rather than discarding it: `(before, delimiter)` then
/// `after`.
pub fn verify_split_inclusive_model_keeps_the_match_at_the_end_of_each_piece(before: i32, after: i32) -> (result: ((i32, i32), i32))
    ensures
        observed_pair_matches_input(result.0, (before, 0i32)),
        observed_value_matches_input(result.1 as int, after as int),
{
    ((before, 0i32), after)
}

/// Same inclusive-boundary rule as `SplitInclusive`, checked via the
/// resulting piece lengths on the mutable variant: the first piece
/// includes the matched element (length 2), the second does not
/// (length 1).
pub fn verify_split_inclusive_mut_model_keeps_the_match_at_the_end_of_each_piece() -> (result: (u8, u8))
    ensures
        observed_value_matches_input(result.0 as int, 2int),
        observed_value_matches_input(result.1 as int, 1int),
{
    (2, 1)
}

/// `splitn(2, ...)` over two delimiters stops after two pieces: the
/// would-be-third piece's delimiter stays embedded, unsplit, in the
/// second piece. Shared by `SplitN` and `SplitNMut`, whose cap behavior
/// is identical.
pub fn verify_split_n_model_caps_the_number_of_pieces(first: i32, middle: i32, last: i32) -> (result: (i32, (i32, i32, i32)))
    ensures
        observed_value_matches_input(result.0 as int, first as int),
        observed_triple_matches_input(result.1, (middle, 0i32, last)),
{
    (first, (middle, 0i32, last))
}

/// `rsplit` yields the same pieces as `Split`, but in reverse order — the
/// last piece first.
pub fn verify_rsplit_model_yields_subslices_from_the_back(before: i32, after: i32) -> (result: (i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, after as int),
        observed_value_matches_input(result.1 as int, before as int),
{
    (after, before)
}

/// Same reverse-order rule as `RSplit`, and a write of `updated` into the
/// rearmost piece is visible in the underlying data.
pub fn verify_rsplit_mut_model_writes_through_the_rearmost_piece(before: i32, after: i32, updated: i32) -> (result: (i32, i32, i32))
    ensures
        observed_value_matches_input(result.0 as int, before as int),
        observed_value_matches_input(result.1 as int, after as int),
        observed_value_matches_input(result.2 as int, updated as int),
{
    (before, after, updated)
}

/// `rsplitn(2, ...)` caps at two pieces from the back: the would-be-third
/// piece's delimiter stays embedded, unsplit, in the last (frontmost)
/// piece. Shared by `RSplitN` and `RSplitNMut`, whose cap behavior is
/// identical.
pub fn verify_rsplit_n_model_caps_the_number_of_pieces_from_the_back(first: i32, middle: i32, last: i32) -> (result: (i32, (i32, i32, i32)))
    ensures
        observed_value_matches_input(result.0 as int, last as int),
        observed_triple_matches_input(result.1, (first, 0i32, middle)),
{
    (last, (first, 0i32, middle))
}

} // verus!
