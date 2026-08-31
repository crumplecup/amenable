//! Verus accommodation model for `core::iter::{Chain, Zip, Enumerate, Rev}`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. `vstd` does give `Rev<I>` real spec support generically
//! (`std_specs/iter.rs`'s `ExRev`/`rev_postcondition`, for any `I:
//! DoubleEndedIterator + DoubleEndedIteratorSpec`), but `Range<i32>` —
//! the real type this carrier's sibling types (and `amenable_kani`'s own
//! harnesses) all use as the underlying source — has no
//! `DoubleEndedIteratorSpec` impl in `vstd` at all (only the forward
//! `IteratorSpec`), so `Rev<Range<i32>>` specifically still can't reach
//! that real support. Each of these four real types checks a two-step
//! sequencing claim in `amenable_kani`, each over a small
//! `Range<i32>`-based example — this carrier states each one directly:
//! two single-element ranges concatenated end to end (`Chain`), paired
//! up (`Zip`), or index-paired (`Enumerate`), and a two-element range
//! walked back to front (`Rev`). None of these functions are
//! `Chain`/`Zip`/`Enumerate`/`Rev` themselves — each proof is
//! conditional: sound if the real adapter refines the stated sequence,
//! which `amenable_kani`'s own harness for that exact type (checking
//! the real adapter directly) already confirms independently, for the
//! identical claim.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

#[cfg(verus_keep_ghost)]
use crate::rust_std::misc::{observed_option_matches_input, values_are_equal};

verus! {

/// Two zipped pairs' worth of `next()` results: the paired items, then
/// `None`.
type ZipResult = (Option<(i32, i32)>, Option<(i32, i32)>);

/// Three enumerated pairs' worth of `next()` results: two
/// index/item pairs, then `None`.
type EnumerateResult = (Option<(usize, i32)>, Option<(usize, i32)>, Option<(usize, i32)>);

/// `Chain::next` yields the first source's item, then the second
/// source's item, then `None` — modeled here as chaining the
/// single-element ranges `a..a+1` and `b..b+1`.
pub fn verify_chain_model_sequences_two_iterators_end_to_end(a: i32, b: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    requires
        single_increment_headroom_holds(a),
        single_increment_headroom_holds(b),
    ensures
        observed_option_matches_input(result.0, a),
        observed_option_matches_input(result.1, b),
        result.2 is None,
{
    (Some(a), Some(b), None)
}

/// `Zip::next` pairs the two sources' items, then stops once either is
/// exhausted — modeled here as zipping the single-element ranges
/// `a..a+1` and `b..b+1`.
pub fn verify_zip_model_pairs_items_from_two_iterators(a: i32, b: i32) -> (result: ZipResult)
    requires
        single_increment_headroom_holds(a),
        single_increment_headroom_holds(b),
    ensures
        values_are_equal(result.0, Some((a, b))),
        result.1 is None,
{
    (Some((a, b)), None)
}

/// The increment-headroom precondition `amenable_std::IncrementHeadroom`
/// names — called directly from `requires` clauses (in this file and,
/// confirmed under real `verus`, from other carrier files too — see
/// `amenable_std::verus_gallery`'s `cross_file_spec_fn_reuse_gets_real_
/// proof_credit` case) rather than restated inline.
pub open spec fn increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX - 1
}

/// The single-increment-headroom precondition -- the same
/// `amenable_std::IncrementHeadroom` bound at its loosest margin (only
/// one more increment needs to stay in range, not two), named once here
/// and called from every carrier file that needs exactly this margin
/// rather than restating `x < i32::MAX` inline at each site.
pub open spec fn single_increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX
}

/// The ten-increment-headroom precondition -- still the same
/// `amenable_std::IncrementHeadroom` bound, but at the wider margin the
/// `slice_chunks` write-through models need before they add `10`.
pub open spec fn ten_increment_headroom_holds(a: i32) -> bool {
    a <= i32::MAX - 10
}

/// The two-increment-headroom precondition -- still the same
/// `amenable_std::IncrementHeadroom` bound, at the margin
/// `iter_window_carrier`'s own `Skip(2)` model needs before it adds `2`.
pub open spec fn two_increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX - 2
}

/// The four-increment-headroom precondition -- still the same
/// `amenable_std::IncrementHeadroom` bound, at the margin
/// `iter_window_carrier`'s own `StepBy(2)`/`Take(2)` models need before
/// they add up to `4`.
pub open spec fn four_increment_headroom_holds(a: i32) -> bool {
    a < i32::MAX - 4
}

/// `Enumerate::next` pairs each item with its index, starting at `0` —
/// modeled here over the two-element range `a..a+2`.
pub fn verify_enumerate_model_pairs_each_item_with_its_index(a: i32) -> (result: EnumerateResult)
    requires
        increment_headroom_holds(a),
    ensures
        values_are_equal(result.0, Some((0usize, a))),
        values_are_equal(result.1, Some((1usize, (a + 1) as i32))),
        result.2 is None,
{
    (Some((0usize, a)), Some((1usize, a + 1)), None)
}

/// `Rev::next` walks the source back to front — modeled here over the
/// two-element range `a..a+2`.
pub fn verify_rev_model_reverses_iteration_order(a: i32) -> (result: (Option<i32>, Option<i32>, Option<i32>))
    requires
        increment_headroom_holds(a),
    ensures
        values_are_equal(result.0, Some((a + 1) as i32)),
        observed_option_matches_input(result.1, a),
        result.2 is None,
{
    (Some(a + 1), Some(a), None)
}

} // verus!
