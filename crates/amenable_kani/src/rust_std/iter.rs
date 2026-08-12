//! `KaniWitness` impls for `core::iter` adapters.
//!
//! Every harness here checks against a symbolic (`kani::any()`) element
//! value threaded through a single `.next()` step or a small fixed number
//! of steps, rather than a fixed literal example — the claim holds for
//! *any* value the adapter might see, not just one hand-picked case.
//! Closure parameters use bare `fn` pointer types rather than closures,
//! since closures have no nameable type to register evidence against (the
//! same pattern as `LazyCell<i32, fn() -> i32>` in `rust_std::cell`).
//! `Range<i32>` (or `Iter<'static, i32>`/`IntoIter<Range<i32>>` where an
//! adapter wraps an iterator of iterators or a borrowing source) is the one
//! representative source iterator this batch covers, matching the bare
//! names `amenable_std::rust_std::iter`'s evidence registration uses.
//!
//! `Filter`, `FilterMap`, and `FlatMap`'s outer source are the exceptions:
//! their `next()` routes through `Iterator::find`/`try_fold`, and
//! `Range<i32>::try_fold`'s loop bound depends on a runtime comparison
//! between symbolic endpoints that Kani's unwinder cannot conclude is
//! bounded — confirmed still timing out past 500 unwind iterations even for
//! an assumed single-item range. `std::array::IntoIter<i32, 1>` has the same
//! `find`-routed `next()` but a compile-time loop bound, so it resolves
//! immediately. See `gallery::iter_materialization` for the isolated
//! experiment and `gallery::replace_recommendations` for the direct
//! `Range<i32>`-sourced false trail this replaces.

use std::iter::{
    Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect, Map,
    MapWhile, OnceWith, Peekable, RepeatN, RepeatWith, Rev, Scan, Skip, SkipWhile, StepBy,
    Successors, TakeWhile, Zip,
};
use std::ops::Range;
use std::slice::Iter;
use std::vec::IntoIter;

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

use super::CheckedProof;
#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
#[cfg(kani)]
use crate::FirstValueIsLessThanTheSecond;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Map<Range<i32>, fn(i32) -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_map_applies_its_closure_to_each_item".to_owned(),
            claim: VERIFY_MAP_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Map<Range<i32>, fn(i32) -> i32>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_MAP_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC, {
        /// `Map::next` applies the closure to the underlying iterator's
        /// next item, for any item value.
        #[kani::proof]
        fn verify_map_applies_its_closure_to_each_item() {
            fn add_one(x: i32) -> i32 {
                x + 1
            }
            let x: i32 = kani::any();
            kani::assume(x < i32::MAX);
            assert_eq!(
                (x..x + 1).map(add_one).next(),
                Some(add_one(x)),
                "map applies its closure to the underlying item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_filter_yields_only_items_matching_the_predicate".to_owned(),
            claim: VERIFY_FILTER_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILTER_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC, {
        /// `Filter::next` yields the underlying item only when the
        /// predicate holds for it, for any item value.
        ///
        /// The source is a one-element array rather than `Range<i32>`:
        /// `Filter::next` is implemented in std via `Iterator::find`, which
        /// routes through the source's `try_fold`. `Range<i32>::try_fold`'s
        /// loop bound is a runtime comparison between two symbolic `i32`
        /// endpoints, which Kani's unwinder cannot conclude is bounded no
        /// matter how tightly a single-item range narrows it (confirmed:
        /// still times out past 500 unwind iterations). An array's
        /// `try_fold` loop bound is a compile-time length, so the same
        /// `find`-routed path resolves immediately. See
        /// `gallery::replace_recommendations` for the direct
        /// `Range<i32>`-sourced false trail this replaces.
        #[kani::proof]
        fn verify_filter_yields_only_items_matching_the_predicate() {
            fn is_even(x: &i32) -> bool {
                *x != 0
            }
            let x: i32 = kani::any();
            let expected = if is_even(&x) { Some(x) } else { None };
            assert_eq!(
                [x].into_iter().filter(is_even).next(),
                expected,
                "filter yields the item only when the predicate holds"
            );
        }
    }
}

impl KaniWitness
    for RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_filter_map_applies_and_filters_in_one_step".to_owned(),
            claim: VERIFY_FILTER_MAP_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(
    RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FILTER_MAP_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC, {
        /// `FilterMap::next` matches its closure applied directly to the
        /// item — for a single-item source, whether the closure returns
        /// `Some` or `None` fully determines the result.
        ///
        /// Same array-source reasoning as `Filter`: `FilterMap::next` also
        /// routes through `Iterator::find`/`try_fold`, and `Range<i32>` was
        /// confirmed to still time out even for a single-item range.
        #[kani::proof]
        fn verify_filter_map_applies_and_filters_in_one_step() {
            fn filter_map_fn(x: i32) -> Option<i32> {
                if x == 0 { None } else { Some(x) }
            }
            let x: i32 = kani::any();
            assert_eq!(
                [x].into_iter().filter_map(filter_map_fn).next(),
                filter_map_fn(x),
                "filter_map's result matches its closure applied to the item"
            );
        }
    }
}

impl KaniWitness
    for RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>
{
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_flat_map_flattens_each_generated_iterator".to_owned(),
            claim: VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(
    RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC, {
        /// `FlatMap` over a single outer item matches calling its closure
        /// directly on that item: the two ways of getting the inner
        /// sequence agree. `x` is bounded small so Kani can fully unroll
        /// the resulting sequence.
        ///
        /// The outer source is a one-element array rather than
        /// `Range<i32>`, for the same `try_fold`-unwinding reason as
        /// `Filter`/`FilterMap` above; the inner sequence (the actual
        /// subject of this claim) stays the original symbolic `Range<i32>`.
        #[kani::proof]
        fn verify_flat_map_flattens_each_generated_iterator() {
            fn flat_map_fn(x: i32) -> Range<i32> {
                0..x
            }
            let x: i32 = kani::any();
            kani::assume((0..=4).contains(&x));
            let mut flattened = [x].into_iter().flat_map(flat_map_fn);
            let mut direct = flat_map_fn(x);
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), direct.next())),
                "flat_map's first item matches the direct inner iterator"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), direct.next())),
                "flat_map's second item matches the direct inner iterator"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), direct.next())),
                "flat_map's third item matches the direct inner iterator"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), direct.next())),
                "flat_map's fourth item matches the direct inner iterator"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), direct.next())),
                "flat_map and the direct inner iterator exhaust together"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), direct.next())),
                "flat_map over one item matches calling its closure directly"
            );
        }
    }
}

/// An `(actual, expected)` pair of `.next()` results known to agree: an
/// iterator adapter's sequence matches a directly-constructed reference
/// iterator's sequence, step by step.
///
/// Independently hand-written as `assert_eq!(adapter.next(),
/// reference.next(), ...)` at 16 real sites split between
/// `verify_flat_map_flattens_each_generated_iterator` (comparing
/// `FlatMap` against calling its closure directly) and
/// `verify_flatten_concatenates_the_inner_iterators` (comparing
/// `Flatten` against a direct `.chain()` concatenation) -- the identical
/// claim regardless of which adapter or reference construction is being
/// checked. Generic over the item type rather than one registration per
/// adapter, the same reasoning (and the same reason it needs a hand-
/// written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros) as
/// `IteratorYieldsNoneWhenExhausted` just above.
pub struct IteratorMatchesReferenceStepByStep<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for IteratorMatchesReferenceStepByStep<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IteratorMatchesReferenceStepByStep<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for IteratorMatchesReferenceStepByStep<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_flat_map_flattens_each_generated_iterator".to_owned(),
            claim: VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for IteratorMatchesReferenceStepByStep<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for IteratorMatchesReferenceStepByStep<T>
{
    type Input = (Option<T>, Option<T>);
    type Bound = bool;

    fn ensures((actual, expected): (Option<T>, Option<T>)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_kani::IteratorMatchesReferenceStepByStep",
        verifier: "kani",
        kind: "ensures",
        fragment: || stringify!(actual == expected),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::IteratorMatchesReferenceStepByStep",
        verifier: "kani",
        describe: || <IteratorMatchesReferenceStepByStep<i32> as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<Flatten<IntoIter<Range<i32>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_flatten_concatenates_the_inner_iterators".to_owned(),
            claim: VERIFY_FLATTEN_CONCATENATES_THE_INNER_ITERATORS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Flatten<IntoIter<Range<i32>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Flatten<IntoIter<Range<i32>>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Flatten<IntoIter<Range<i32>>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_FLATTEN_CONCATENATES_THE_INNER_ITERATORS_SRC, {
        /// `Flatten` concatenates its inner iterators in order.
        ///
        /// Unlike `Filter`/`FilterMap`/`FlatMap` above, swapping only the
        /// outer source for an array does not resolve `Flatten`'s timeout:
        /// confirmed empirically that an array-outer/symbolic-inner
        /// variant still times out (see
        /// `gallery::iter_materialization::flatten_incremental_next_passes`,
        /// which documents that incremental observation over symbolic
        /// `0..=4`-bounded lengths times out regardless of outer source).
        /// Only fully concrete inner-range lengths avoid the blow-up
        /// (`gallery::iter_materialization::flatten_incremental_fixed_lengths_passes`).
        /// This production proof adopts that same concrete-length shape:
        /// a weaker but working two-length representative rather than a
        /// claim over every possible pair of lengths.
        #[kani::proof]
        fn verify_flatten_concatenates_the_inner_iterators() {
            let a: i32 = 1;
            let b: i32 = 2;
            let nested: Vec<Range<i32>> = vec![0..a, 0..b];
            let mut flattened = nested.into_iter().flatten();
            let mut expected = (0..a).chain(0..b);
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's first item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's second item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's third item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's fourth item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's fifth item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's sixth item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's seventh item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten's eighth item matches the direct concatenation"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten concatenates its inner iterators in order"
            );
            assert!(
                IteratorMatchesReferenceStepByStep::ensures((flattened.next(), expected.next())),
                "flatten and the direct concatenation exhaust together"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_chain_sequences_two_iterators_end_to_end".to_owned(),
            claim: VERIFY_CHAIN_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CHAIN_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC, {
        /// `Chain` yields the first iterator's items, then the second's,
        /// then stops.
        #[kani::proof]
        fn verify_chain_sequences_two_iterators_end_to_end() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a < i32::MAX && b < i32::MAX);
            let mut c = (a..a + 1).chain(b..b + 1);
            assert!(
                RustStdStandard::<std::iter::Chain<Range<i32>, Range<i32>>>::ensures((
                    c.next(),
                    Some(a)
                )),
                "chain yields the first iterator's items first"
            );
            assert!(
                RustStdStandard::<std::iter::Chain<Range<i32>, Range<i32>>>::ensures((
                    c.next(),
                    Some(b)
                )),
                "chain yields the second iterator's items once the first is exhausted"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(c.next()),
                "chain is exhausted once both inputs are"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Zip<Range<i32>, Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_zip_pairs_items_from_two_iterators".to_owned(),
            claim: VERIFY_ZIP_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Zip<Range<i32>, Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Zip<Range<i32>, Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Zip<Range<i32>, Range<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ZIP_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC, {
        /// `Zip` pairs up items from its two sources and stops as soon
        /// as either is exhausted.
        #[kani::proof]
        fn verify_zip_pairs_items_from_two_iterators() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(a < i32::MAX && b < i32::MAX);
            let mut z = (a..a + 1).zip(b..b + 1);
            assert_eq!(z.next(), Some((a, b)), "zip pairs the two iterators' items");
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(z.next()),
                "zip stops once either input is exhausted"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Enumerate<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_enumerate_pairs_each_item_with_its_index".to_owned(),
            claim: VERIFY_ENUMERATE_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Enumerate<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Enumerate<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Enumerate<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ENUMERATE_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC, {
        /// `Enumerate` pairs each item with a 0-based index that
        /// increments alongside the item.
        #[kani::proof]
        fn verify_enumerate_pairs_each_item_with_its_index() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 1);
            let mut e = (a..a + 2).enumerate();
            assert_eq!(e.next(), Some((0, a)), "enumerate starts indexing at 0");
            assert_eq!(
                e.next(),
                Some((1, a + 1)),
                "enumerate increments the index alongside the item"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(e.next()),
                "enumerate is exhausted once the underlying iterator is exhausted"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Rev<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rev_reverses_iteration_order".to_owned(),
            claim: VERIFY_REV_REVERSES_ITERATION_ORDER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Rev<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Rev<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Rev<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Rev<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Rev<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_REV_REVERSES_ITERATION_ORDER_SRC, {
        /// `Rev` yields a double-ended iterator's items back to front.
        #[kani::proof]
        fn verify_rev_reverses_iteration_order() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 1);
            let mut r = (a..a + 2).rev();
            assert!(
                RustStdStandard::<Rev<Range<i32>>>::ensures((r.next(), Some(a + 1))),
                "rev yields the last item first"
            );
            assert!(
                RustStdStandard::<Rev<Range<i32>>>::ensures((r.next(), Some(a))),
                "rev yields items in reverse order"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(r.next()),
                "rev is exhausted once every item has been yielded"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Cloned<Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cloned_clones_each_referenced_item".to_owned(),
            claim: VERIFY_CLONED_CLONES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Cloned<Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cloned<Iter<'static, i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Cloned<Iter<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Cloned<Iter<'static, i32>>>,
    "amenable_std::rust_std::RustStdStandard<Cloned<Iter<'static, i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CLONED_CLONES_EACH_REFERENCED_ITEM_SRC, {
        /// `Cloned` yields an owned clone of each referenced item. The
        /// slice here is not `'static` — the claim holds uniformly over
        /// every lifetime, so a local array is enough to check it.
        #[kani::proof]
        fn verify_cloned_clones_each_referenced_item() {
            let value: i32 = kani::any();
            let data = [value];
            let mut c = data.iter().cloned();
            assert!(
                RustStdStandard::<Cloned<Iter<'static, i32>>>::ensures((c.next(), Some(value))),
                "cloned yields an owned clone of each referenced item"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(c.next()),
                "cloned is exhausted after yielding the only referenced item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Copied<Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_copied_copies_each_referenced_item".to_owned(),
            claim: VERIFY_COPIED_COPIES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Copied<Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Copied<Iter<'static, i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Copied<Iter<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Copied<Iter<'static, i32>>>,
    "amenable_std::rust_std::RustStdStandard<Copied<Iter<'static, i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_COPIED_COPIES_EACH_REFERENCED_ITEM_SRC, {
        /// `Copied` yields an owned copy of each referenced item. Same
        /// non-`'static`-in-the-harness reasoning as `Cloned`.
        #[kani::proof]
        fn verify_copied_copies_each_referenced_item() {
            let value: i32 = kani::any();
            let data = [value];
            let mut c = data.iter().copied();
            assert!(
                RustStdStandard::<Copied<Iter<'static, i32>>>::ensures((c.next(), Some(value))),
                "copied yields an owned copy of each referenced item"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(c.next()),
                "copied is exhausted after yielding the only referenced item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Cycle<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cycle_repeats_its_sequence_forever".to_owned(),
            claim: VERIFY_CYCLE_REPEATS_ITS_SEQUENCE_FOREVER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Cycle<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cycle<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Cycle<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Cycle<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Cycle<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CYCLE_REPEATS_ITS_SEQUENCE_FOREVER_SRC, {
        /// `Cycle` restarts its underlying sequence once exhausted,
        /// checked across two full laps.
        #[kani::proof]
        fn verify_cycle_repeats_its_sequence_forever() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 1);
            let mut c = (a..a + 2).cycle();
            assert!(RustStdStandard::<Cycle<Range<i32>>>::ensures((c.next(), Some(a))));
            assert!(RustStdStandard::<Cycle<Range<i32>>>::ensures((c.next(), Some(a + 1))));
            assert!(
                RustStdStandard::<Cycle<Range<i32>>>::ensures((c.next(), Some(a))),
                "cycle restarts its sequence once exhausted"
            );
            assert!(RustStdStandard::<Cycle<Range<i32>>>::ensures((c.next(), Some(a + 1))));
        }
    }
}

impl KaniWitness for RustStdStandard<Fuse<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_fuse_keeps_returning_none_once_exhausted".to_owned(),
            claim: VERIFY_FUSE_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Fuse<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Fuse<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Fuse<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Fuse<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Fuse<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_FUSE_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC, {
        /// `Fuse` keeps returning `None` once the underlying iterator
        /// has, checked across two calls after exhaustion.
        #[kani::proof]
        fn verify_fuse_keeps_returning_none_once_exhausted() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX);
            let mut f = (a..a + 1).fuse();
            assert!(RustStdStandard::<Fuse<Range<i32>>>::ensures((f.next(), Some(a))));
            assert!(IteratorYieldsNoneWhenExhausted::ensures(f.next()));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(f.next()),
                "fuse keeps returning None once the underlying iterator has"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Inspect<Range<i32>, fn(&i32)>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_inspect_calls_once_per_item_without_changing_values".to_owned(),
            claim: VERIFY_INSPECT_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Inspect<Range<i32>, fn(&i32)>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Inspect<Range<i32>, fn(&i32)>>",
        verifier: "kani",
        describe: || <RustStdStandard<Inspect<Range<i32>, fn(&i32)>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Inspect<Range<i32>, fn(&i32)>>,
    "amenable_std::rust_std::RustStdStandard<Inspect<Range<i32>, fn(&i32)>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_INSPECT_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC, {
        /// `Inspect` leaves values unchanged and calls its closure
        /// exactly once per item.
        #[kani::proof]
        fn verify_inspect_calls_once_per_item_without_changing_values() {
            let value: i32 = kani::any();
            kani::assume(value < i32::MAX);
            let calls = Cell::new(0usize);
            let mut inspected = (value..value + 1).inspect(|_| calls.set(calls.get() + 1));

            assert!(
                RustStdStandard::<Inspect<Range<i32>, fn(&i32)>>::ensures((
                    inspected.next(),
                    Some(value)
                )),
                "inspect does not change the yielded value"
            );
            assert!(
                RustStdStandard::<Cell<usize>>::ensures((calls.get(), 1)),
                "inspect calls its closure exactly once per item"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(inspected.next()),
                "the one-item inspected iterator then exhausts"
            );
            assert!(
                RustStdStandard::<Cell<usize>>::ensures((calls.get(), 1)),
                "inspect does not re-invoke its closure after exhaustion"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Peekable<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_peekable_peek_does_not_consume".to_owned(),
            claim: VERIFY_PEEKABLE_PEEK_DOES_NOT_CONSUME_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Peekable<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Peekable<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Peekable<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Peekable<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Peekable<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_PEEKABLE_PEEK_DOES_NOT_CONSUME_SRC, {
        /// `Peekable::peek` previews the next item without consuming it:
        /// a following `next()` still returns that same item.
        #[kani::proof]
        fn verify_peekable_peek_does_not_consume() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 1);
            let mut p = (a..a + 2).peekable();
            assert_eq!(p.peek(), Some(&a), "peek previews the next item");
            assert!(
                RustStdStandard::<Peekable<Range<i32>>>::ensures((p.next(), Some(a))),
                "next still returns the peeked item"
            );
            assert!(
                RustStdStandard::<Peekable<Range<i32>>>::ensures((p.next(), Some(a + 1))),
                "peek did not consume an item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_scan_threads_state_through_its_closure".to_owned(),
            claim: VERIFY_SCAN_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SCAN_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC, {
        /// `Scan` threads its mutable state from one call into the next:
        /// a running-sum closure's second result includes the first
        /// item's contribution. `a` is bounded to keep the running sum
        /// itself free of overflow.
        #[kani::proof]
        fn verify_scan_threads_state_through_its_closure() {
            fn running_sum(acc: &mut i32, x: i32) -> Option<i32> {
                *acc += x;
                Some(*acc)
            }
            let a: i32 = kani::any();
            kani::assume((-1000..=1000).contains(&a));
            let mut s = (a..a + 2).scan(0i32, running_sum);
            assert!(
                RustStdStandard::<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>::ensures((
                    s.next(),
                    Some(a)
                )),
                "scan's first item is the closure applied to the initial state and first item"
            );
            assert!(
                RustStdStandard::<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>::ensures((
                    s.next(),
                    Some(a + (a + 1))
                )),
                "scan threads its updated state into the next call"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Skip<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_skip_discards_the_first_n_items".to_owned(),
            claim: VERIFY_SKIP_DISCARDS_THE_FIRST_N_ITEMS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Skip<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Skip<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Skip<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SKIP_DISCARDS_THE_FIRST_N_ITEMS_SRC, {
        /// `Skip(n)` discards exactly the first `n` items.
        #[kani::proof]
        fn verify_skip_discards_the_first_n_items() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 2);
            let mut s = (a..a + 3).skip(2);
            assert_eq!(s.next(), Some(a + 2), "skip discards exactly the first n items");
        }
    }
}

impl KaniWitness for RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_skip_while_discards_items_while_the_predicate_holds".to_owned(),
            claim: VERIFY_SKIP_WHILE_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SKIP_WHILE_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC, {
        /// `SkipWhile` discards items until the predicate first fails,
        /// then yields the rest unchanged. `a` is fixed even so the
        /// predicate's pass/fail boundary is deterministic.
        #[kani::proof]
        fn verify_skip_while_discards_items_while_the_predicate_holds() {
            fn is_even(x: &i32) -> bool {
                *x % 2 == 0
            }
            let a: i32 = 4;
            let mut s = (a..a + 2).skip_while(is_even);
            assert_eq!(
                s.next(),
                Some(a + 1),
                "skip_while discards items until the predicate first fails"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<StepBy<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_step_by_yields_every_nth_item".to_owned(),
            claim: VERIFY_STEP_BY_YIELDS_EVERY_NTH_ITEM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<StepBy<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<StepBy<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<StepBy<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<StepBy<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<StepBy<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_STEP_BY_YIELDS_EVERY_NTH_ITEM_SRC, {
        /// `StepBy(n)` yields every nth item starting from the first.
        #[kani::proof]
        fn verify_step_by_yields_every_nth_item() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 4);
            let mut s = (a..a + 5).step_by(2);
            assert!(RustStdStandard::<StepBy<Range<i32>>>::ensures((s.next(), Some(a))));
            assert!(RustStdStandard::<StepBy<Range<i32>>>::ensures((s.next(), Some(a + 2))));
            assert!(
                RustStdStandard::<StepBy<Range<i32>>>::ensures((s.next(), Some(a + 4))),
                "step_by yields every nth item from the start"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Take<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_take_yields_at_most_n_items".to_owned(),
            claim: VERIFY_TAKE_YIELDS_AT_MOST_N_ITEMS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Take<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Take<Range<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::iter::Take<Range<i32>>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::iter::Take<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Take<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_TAKE_YIELDS_AT_MOST_N_ITEMS_SRC, {
        /// `Take(n)` yields no more than `n` items even when the source
        /// has more.
        #[kani::proof]
        fn verify_take_yields_at_most_n_items() {
            let a: i32 = kani::any();
            kani::assume(a < i32::MAX - 4);
            let mut t = (a..a + 5).take(2);
            assert!(RustStdStandard::<std::iter::Take<Range<i32>>>::ensures((t.next(), Some(a))));
            assert!(RustStdStandard::<std::iter::Take<Range<i32>>>::ensures((
                t.next(),
                Some(a + 1)
            )));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(t.next()),
                "take yields no more than n items even though the source has more"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_take_while_yields_items_while_the_predicate_holds".to_owned(),
            claim: VERIFY_TAKE_WHILE_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>",
        verifier: "kani",
        describe: || <RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_TAKE_WHILE_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC, {
        /// `TakeWhile` yields items while the predicate holds and stops
        /// as soon as it first fails. `a` is fixed even so the
        /// predicate's pass/fail boundary is deterministic.
        #[kani::proof]
        fn verify_take_while_yields_items_while_the_predicate_holds() {
            fn is_even(x: &i32) -> bool {
                *x % 2 == 0
            }
            let a: i32 = 4;
            let mut t = (a..a + 2).take_while(is_even);
            assert!(
                RustStdStandard::<TakeWhile<Range<i32>, fn(&i32) -> bool>>::ensures((
                    t.next(),
                    Some(a)
                )),
                "take_while yields items while the predicate holds"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(t.next()),
                "take_while stops as soon as the predicate first fails"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_map_while_maps_items_while_the_closure_returns_some".to_owned(),
            claim: VERIFY_MAP_WHILE_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_MAP_WHILE_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC, {
        /// `MapWhile::next` matches its closure applied directly to the
        /// item — for a single-item source, whether the closure returns
        /// `Some` or `None` fully determines the result, same reasoning
        /// as `FilterMap`.
        #[kani::proof]
        fn verify_map_while_maps_items_while_the_closure_returns_some() {
            fn map_while_fn(x: i32) -> Option<i32> {
                if x % 2 == 0 { Some(x * 2) } else { None }
            }
            let a: i32 = kani::any();
            kani::assume((-1000..1000).contains(&a));
            let expected = map_while_fn(a);
            assert_eq!(
                (a..a + 1).map_while(map_while_fn).next(),
                expected,
                "map_while's result matches its closure applied to the item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Once<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_once_yields_exactly_one_value".to_owned(),
            claim: VERIFY_ONCE_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Once<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::iter::Once<i32>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::iter::Once<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ONCE_YIELDS_EXACTLY_ONE_VALUE_SRC, {
        /// `once` yields its value once, then stops.
        #[kani::proof]
        fn verify_once_yields_exactly_one_value() {
            let value: i32 = kani::any();
            let mut o = std::iter::once(value);
            assert!(
                RustStdStandard::<std::iter::Once<i32>>::ensures((o.next(), Some(value))),
                "once yields its value"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(o.next()),
                "once yields nothing after its one value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<OnceWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_once_with_calls_its_closure_exactly_once".to_owned(),
            claim: VERIFY_ONCE_WITH_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<OnceWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<OnceWith<fn() -> i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<OnceWith<fn() -> i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ONCE_WITH_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC, {
        /// `once_with` calls its closure to produce exactly one value,
        /// then stops.
        #[kani::proof]
        fn verify_once_with_calls_its_closure_exactly_once() {
            fn produce() -> i32 {
                kani::any()
            }
            let mut o = std::iter::once_with(produce as fn() -> i32);
            assert!(o.next().is_some(), "once_with yields one value");
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(o.next()),
                "once_with yields nothing after its one value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Repeat<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_repeat_yields_the_same_value_forever".to_owned(),
            claim: VERIFY_REPEAT_YIELDS_THE_SAME_VALUE_FOREVER_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Repeat<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::iter::Repeat<i32>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::iter::Repeat<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::Repeat<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_REPEAT_YIELDS_THE_SAME_VALUE_FOREVER_SRC, {
        /// `repeat` yields the same value on every call, checked across
        /// three calls.
        #[kani::proof]
        fn verify_repeat_yields_the_same_value_forever() {
            let value: i32 = kani::any();
            let mut r = std::iter::repeat(value);
            assert!(RustStdStandard::<std::iter::Repeat<i32>>::ensures((r.next(), Some(value))));
            assert!(RustStdStandard::<std::iter::Repeat<i32>>::ensures((r.next(), Some(value))));
            assert!(
                RustStdStandard::<std::iter::Repeat<i32>>::ensures((r.next(), Some(value))),
                "repeat yields the same value on every call"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RepeatWith<fn() -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_repeat_with_calls_its_closure_once_per_item".to_owned(),
            claim: VERIFY_REPEAT_WITH_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RepeatWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RepeatWith<fn() -> i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RepeatWith<fn() -> i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_REPEAT_WITH_CALLS_ITS_CLOSURE_ONCE_PER_ITEM_SRC, {
        /// Unlike `LazyCell`, `RepeatWith` never caches: it calls its
        /// closure once per item, every time, observed through a shared
        /// counter across three calls.
        #[kani::proof]
        fn verify_repeat_with_calls_its_closure_once_per_item() {
            static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn produce() -> i32 {
                CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                0
            }

            let mut r = std::iter::repeat_with(produce as fn() -> i32);
            r.next();
            r.next();
            r.next();
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    CALLS.load(std::sync::atomic::Ordering::SeqCst),
                    3
                )),
                "repeat_with calls its closure once per item, never caching"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RepeatN<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_repeat_n_yields_the_value_exactly_n_times".to_owned(),
            claim: VERIFY_REPEAT_N_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RepeatN<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RepeatN<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RepeatN<i32>> as KaniWitness>::proof().to_string(),
    }
}

kani_ensures!(
    RustStdStandard<RepeatN<i32>>,
    "amenable_std::rust_std::RustStdStandard<RepeatN<i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_REPEAT_N_YIELDS_THE_VALUE_EXACTLY_N_TIMES_SRC, {
        /// `repeat_n(value, n)` yields `value` exactly `n` times, then
        /// stops — the bounded counterpart to `repeat`.
        #[kani::proof]
        fn verify_repeat_n_yields_the_value_exactly_n_times() {
            let value: i32 = kani::any();
            let mut r = std::iter::repeat_n(value, 2);
            assert!(RustStdStandard::<RepeatN<i32>>::ensures((r.next(), Some(value))));
            assert!(RustStdStandard::<RepeatN<i32>>::ensures((r.next(), Some(value))));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(r.next()),
                "repeat_n stops after exactly n items"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Empty<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_empty_yields_nothing".to_owned(),
            claim: VERIFY_EMPTY_YIELDS_NOTHING_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Empty<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::Empty<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::iter::Empty<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_EMPTY_YIELDS_NOTHING_SRC, {
        /// `empty` never yields a value. Both assertions call
        /// `IteratorYieldsNoneWhenExhausted::ensures` directly rather than
        /// restating the comparison -- see that type for why this is the
        /// one harness its registration reuses as a witness.
        #[kani::proof]
        fn verify_empty_yields_nothing() {
            let mut e: std::iter::Empty<i32> = std::iter::empty();
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(e.next()),
                "empty never yields a value"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(e.next()),
                "empty continues yielding nothing on repeated next calls"
            );
        }
    }
}

/// An `Option<T>` known to be the `None` an exhausted iterator adapter
/// keeps returning on every subsequent `.next()` call.
///
/// Independently hand-written as `assert_eq!(iter.next(), None, ...)` at
/// dozens of real proof sites across nearly every iterator adapter this
/// crate proves, spanning this file plus `alloc_collections`, `alloc_vec`,
/// `array`, `option_result`, `path`, `slice`, and `str` -- the identical
/// claim regardless of the adapter's item type. A single contract type
/// generic over the item, rather than one registration per concrete
/// adapter, resolves every site at once.
///
/// Unlike every other named bound in this crate, this one needs a
/// hand-written `Witness`/`Ensures` impl instead of the
/// `bridge_kani_witness!`/`kani_ensures!` macros: both macros emit a
/// non-generic `impl Trait for $ty`, which can't carry the `impl<T>`
/// generic parameter list this type's single, item-type-agnostic
/// registration depends on. Call sites never construct an instance --
/// `ensures` is a static associated function, resolved by unifying
/// `Self::Input = Option<T>` against the argument's type -- so this type
/// carries no fields beyond the `PhantomData<T>` Rust requires to use `T`
/// at all, and every call site writes the bare type name with no
/// turbofish, letting inference pick `T`.
pub struct IteratorYieldsNoneWhenExhausted<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for IteratorYieldsNoneWhenExhausted<T> {
    type Provenance = amenable_std::RustStdProvenance;

    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IteratorYieldsNoneWhenExhausted<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for IteratorYieldsNoneWhenExhausted<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_empty_yields_nothing".to_owned(),
            claim: VERIFY_EMPTY_YIELDS_NOTHING_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for IteratorYieldsNoneWhenExhausted<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T> amenable_core::Ensures<crate::KaniVerifier> for IteratorYieldsNoneWhenExhausted<T> {
    type Input = Option<T>;
    type Bound = bool;

    fn ensures(input: Option<T>) -> bool {
        input.is_none()
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord {
        evidence: "amenable_kani::IteratorYieldsNoneWhenExhausted",
        verifier: "kani",
        kind: "ensures",
        fragment: || stringify!(input.is_none()),
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_kani::IteratorYieldsNoneWhenExhausted",
        verifier: "kani",
        describe: || <IteratorYieldsNoneWhenExhausted<i32> as KaniWitness>::proof().to_string(),
    }
}

impl KaniWitness for RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_successors_generates_from_the_previous_item".to_owned(),
            claim: VERIFY_SUCCESSORS_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Successors<i32, fn(&i32) -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SUCCESSORS_GENERATES_FROM_THE_PREVIOUS_ITEM_SRC, {
        /// `successors` yields the seed first, then computes each next
        /// item from the previous one via its closure.
        #[kani::proof]
        fn verify_successors_generates_from_the_previous_item() {
            fn next_step(x: &i32) -> Option<i32> {
                if *x < 100 { Some(x + 1) } else { None }
            }
            let seed: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((seed, 100)));
            let mut s = std::iter::successors(Some(seed), next_step);
            assert!(
                RustStdStandard::<Successors<i32, fn(&i32) -> Option<i32>>>::ensures((
                    s.next(),
                    Some(seed)
                )),
                "successors yields the seed first"
            );
            assert!(
                RustStdStandard::<Successors<i32, fn(&i32) -> Option<i32>>>::ensures((
                    s.next(),
                    Some(seed + 1)
                )),
                "successors computes the next item from the previous one"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_from_fn_yields_until_the_closure_returns_none".to_owned(),
            claim: VERIFY_FROM_FN_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    }
}

kani_ensures!(
    RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<std::iter::FromFn<fn() -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_FROM_FN_YIELDS_UNTIL_THE_CLOSURE_RETURNS_NONE_SRC, {
        /// `from_fn` yields whatever its closure produces, and stops the
        /// moment the closure returns `None` — a shared counter drives a
        /// deterministic two-item-then-stop sequence, since the closure
        /// itself is a bare `fn` and can't capture state directly.
        #[kani::proof]
        fn verify_from_fn_yields_until_the_closure_returns_none() {
            static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            fn produce() -> Option<i32> {
                let n = CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if n < 2 { Some(n as i32) } else { None }
            }

            let mut f = std::iter::from_fn(produce as fn() -> Option<i32>);
            assert!(RustStdStandard::<std::iter::FromFn<fn() -> Option<i32>>>::ensures((
                f.next(),
                Some(0)
            )));
            assert!(RustStdStandard::<std::iter::FromFn<fn() -> Option<i32>>>::ensures((
                f.next(),
                Some(1)
            )));
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(f.next()),
                "from_fn stops once its closure returns None"
            );
        }
    }
}
