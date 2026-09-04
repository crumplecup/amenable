use std::iter::{Filter, FilterMap, FlatMap, Flatten, Map};
use std::ops::Range;
use std::vec::IntoIter;

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
#[cfg(kani)]
use crate::FirstValueIsLessThanTheSecond;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Map<Range<i32>, fn(i32) -> i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_map_applies_its_closure_to_each_item".to_owned(),
            VERIFY_MAP_APPLIES_ITS_CLOSURE_TO_EACH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>);

kani_ensures!(
    RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>,
    "amenable_std::rust_std::RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Map<Range<i32>, fn(i32) -> i32>>",
        "kani",
        || <RustStdStandard<Map<Range<i32>, fn(i32) -> i32>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((x, i32::MAX)));
            assert!(
                RustStdStandard::<Map<Range<i32>, fn(i32) -> i32>>::ensures((
                    (x..x + 1).map(add_one).next(),
                    Some(add_one(x))
                )),
                "map applies its closure to the underlying item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_filter_yields_only_items_matching_the_predicate".to_owned(),
            VERIFY_FILTER_YIELDS_ONLY_ITEMS_MATCHING_THE_PREDICATE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>);

kani_ensures!(
    RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(
                RustStdStandard::<Filter<std::array::IntoIter<i32, 1>, fn(&i32) -> bool>>::ensures((
                    [x].into_iter().filter(is_even).next(),
                    expected
                )),
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_filter_map_applies_and_filters_in_one_step".to_owned(),
            VERIFY_FILTER_MAP_APPLIES_AND_FILTERS_IN_ONE_STEP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(
    RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>
);

kani_ensures!(
    RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>",
        "kani",
        || <RustStdStandard<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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
            assert!(
                RustStdStandard::<FilterMap<std::array::IntoIter<i32, 1>, fn(i32) -> Option<i32>>>::ensures((
                    [x].into_iter().filter_map(filter_map_fn).next(),
                    filter_map_fn(x)
                )),
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_flat_map_flattens_each_generated_iterator".to_owned(),
            VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(
    RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>>",
        "kani",
        || <RustStdStandard<FlatMap<std::array::IntoIter<i32, 1>, Range<i32>, fn(i32) -> Range<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(ValueIsWithinInclusiveRange::requires((x, 0, 4)));
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IteratorMatchesReferenceStepByStep<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as amenable_std::RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for IteratorMatchesReferenceStepByStep<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_flat_map_flattens_each_generated_iterator".to_owned(),
            VERIFY_FLAT_MAP_FLATTENS_EACH_GENERATED_ITERATOR_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for IteratorMatchesReferenceStepByStep<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
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
    ::amenable_core::ContractRecord::new(
        "amenable_kani::IteratorMatchesReferenceStepByStep",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IteratorMatchesReferenceStepByStep",
        "kani",
        || <IteratorMatchesReferenceStepByStep<i32> as KaniWitness>::proof().to_string(),
    )
}

impl KaniWitness for RustStdStandard<Flatten<IntoIter<Range<i32>>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_flatten_concatenates_the_inner_iterators".to_owned(),
            VERIFY_FLATTEN_CONCATENATES_THE_INNER_ITERATORS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Flatten<IntoIter<Range<i32>>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Flatten<IntoIter<Range<i32>>>>",
        "kani",
        || <RustStdStandard<Flatten<IntoIter<Range<i32>>>> as KaniWitness>::proof()
            .to_string(),
    )
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
