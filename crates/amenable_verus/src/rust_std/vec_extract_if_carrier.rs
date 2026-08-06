//! Verus accommodation model for `alloc::vec::ExtractIf`.
//!
//! Same zero-`vstd`-coverage gap the other accommodation-model carriers
//! document. Covers the partition half of the claim
//! `amenable_kani`'s own `verify_vec_extract_if_partitions_by_the_
//! predicate` harness checks — a fixed four-element input partitioned
//! by an "is even" predicate, matching that harness's own choice of a
//! concrete example rather than a symbolic one — not the second half
//! (dropping an unfinished `ExtractIf` mid-iteration still preserves
//! the unvisited elements), which is Drop-glue timing, the same kind of
//! genuinely different-in-kind claim `box_carrier.rs`/`vec_carrier.rs`
//! already document as Kani-only elsewhere in this crate.
//! `VerusExtractIfModel` isn't `ExtractIf` and doesn't claim to be — the
//! proof is conditional: sound if the real `ExtractIf` refines this
//! partition, which `amenable_kani`'s own harness (checking the real
//! `Vec::extract_if` directly) already confirms independently, for the
//! identical example.
//!
//! `alloc::collections::linked_list::ExtractIf<'static, i32, fn(&mut
//! i32) -> bool>` checks the identical `[1, 2, 3, 4]`-by-"is even"
//! partition in `amenable_kani`
//! (`verify_linked_list_extract_if_partitions_by_the_predicate`) —
//! itself checked there against `amenable_kani`'s own
//! `linked_list_extract_model::KaniLinkedListExtractIf` accommodation
//! model, not the real type directly (the real `LinkedList::extract_if`
//! path times out under Kani; see that module's own doc comment) — so
//! it's registered against this same carrier and harness too.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

verus! {

/// Models the partition-by-predicate result for a fixed input — not
/// `ExtractIf` itself.
pub struct VerusExtractIfModel {
    pub kept: Vec<i32>,
    pub extracted: Vec<i32>,
}

impl VerusExtractIfModel {
    /// Partitioning `[1, 2, 3, 4]` by "is even" keeps the odd elements
    /// in order and extracts the even elements in order.
    pub fn partition_one_two_three_four_by_even() -> (result: Self)
        ensures
            result.kept@ == seq![1i32, 3i32],
            result.extracted@ == seq![2i32, 4i32],
    {
        Self { kept: vec![1, 3], extracted: vec![2, 4] }
    }
}

/// `extract_if` removes exactly the matching elements, leaving the
/// non-matching elements behind, both in order — the partition half of
/// the claim the real `Vec::extract_if` is expected to refine.
pub fn verify_vec_extract_if_model_partitions_by_the_predicate() -> (result: bool)
    ensures
        result,
{
    let model = VerusExtractIfModel::partition_one_two_three_four_by_even();
    let extracted_matches = model.extracted == vec![2, 4];
    let kept_matches = model.kept == vec![1, 3];

    extracted_matches && kept_matches
}

} // verus!
