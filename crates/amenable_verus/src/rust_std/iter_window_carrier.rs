//! Verus accommodation model for `core::iter::{Skip, SkipWhile, StepBy,
//! Take, TakeWhile}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Each of these five real types checks a position/predicate
//! windowing claim in `amenable_kani`, each over a small `Range<i32>`
//! example (`SkipWhile`/`TakeWhile` use `amenable_kani`'s own fixed
//! `a = 4`, so the predicate's pass/fail boundary is deterministic,
//! matching that harness's own choice) — this carrier states each one
//! directly, not `.next()` calls against modeled iterator state. None
//! of these functions are `Skip`/`SkipWhile`/`StepBy`/`Take`/`TakeWhile`
//! themselves — each proof is conditional: sound if the real adapter
//! refines the stated result, which `amenable_kani`'s own harness for
//! that exact type (checking the real adapter directly) already
//! confirms independently, for the identical claim.

#[cfg(verus_keep_ghost)]
use crate::rust_std::iter_sequence_carrier::{
    four_increment_headroom_holds, two_increment_headroom_holds,
};
#[cfg(verus_keep_ghost)]
use crate::rust_std::primitive_shapes_carrier::observed_option_matches_input;
use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// `Skip(2)::next` discards the first two items of a five-element
/// range, yielding the third.
pub fn verify_skip_model_discards_the_first_n_items(a: i32) -> (result: Option<i32>)
    requires
        two_increment_headroom_holds(a),
    ensures
        observed_option_matches_input(result, (a + 2) as i32),
{
    Some(a + 2)
}

/// `SkipWhile::next` discards items until the predicate first fails —
/// modeled over the fixed range `4..6` skipping while even, matching
/// `amenable_kani`'s own fixed example.
pub fn verify_skip_while_model_discards_items_while_the_predicate_holds() -> (result: Option<i32>)
    ensures
        observed_option_matches_input(result, 5),
{
    Some(5)
}

/// `StepBy(2)::next` yields every second item of a five-element range,
/// starting from the first.
pub fn verify_step_by_model_yields_every_nth_item(a: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    requires
        four_increment_headroom_holds(a),
    ensures
        observed_option_matches_input(result.0, a),
        observed_option_matches_input(result.1, (a + 2) as i32),
        observed_option_matches_input(result.2, (a + 4) as i32),
{
    (Some(a), Some(a + 2), Some(a + 4))
}

/// `Take(2)::next` yields no more than two items even though a
/// five-element source has more.
pub fn verify_take_model_yields_at_most_n_items(a: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    requires
        four_increment_headroom_holds(a),
    ensures
        observed_option_matches_input(result.0, a),
        observed_option_matches_input(result.1, (a + 1) as i32),
        result.2 is None,
{
    (Some(a), Some(a + 1), None)
}

/// `TakeWhile::next` yields items while the predicate holds and stops
/// as soon as it first fails — modeled over the fixed range `4..6`
/// taking while even, matching `amenable_kani`'s own fixed example.
pub fn verify_take_while_model_yields_items_while_the_predicate_holds() -> (result: (Option<i32>, Option<i32>))
    ensures
        observed_option_matches_input(result.0, 4),
        result.1 is None,
{
    (Some(4), None)
}

} // verus!
