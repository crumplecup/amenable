use std::iter::{MapWhile, OnceWith, TakeWhile};
use std::ops::Range;

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;

use crate::CheckedProof;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_take_while_yields_items_while_the_predicate_holds".to_owned(),
            VERIFY_TAKE_WHILE_YIELDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<TakeWhile<Range<i32>, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
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
        CheckedProof::new(
            "verify_map_while_maps_items_while_the_closure_returns_some".to_owned(),
            VERIFY_MAP_WHILE_MAPS_ITEMS_WHILE_THE_CLOSURE_RETURNS_SOME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>);

kani_ensures!(
    RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>,
    "amenable_std::rust_std::RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>",
        "kani",
        || <RustStdStandard<MapWhile<Range<i32>, fn(i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(ValueIsWithinInclusiveRange::requires((a, -1000, 999)));
            let expected = map_while_fn(a);
            assert!(
                RustStdStandard::<MapWhile<Range<i32>, fn(i32) -> Option<i32>>>::ensures((
                    (a..a + 1).map_while(map_while_fn).next(),
                    expected
                )),
                "map_while's result matches its closure applied to the item"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::iter::Once<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_yields_exactly_one_value".to_owned(),
            VERIFY_ONCE_YIELDS_EXACTLY_ONE_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Once<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Once<i32>>",
        "kani",
        || <RustStdStandard<std::iter::Once<i32>> as KaniWitness>::proof().to_string(),
    )
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
        CheckedProof::new(
            "verify_once_with_calls_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_WITH_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<OnceWith<fn() -> i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OnceWith<fn() -> i32>>",
        "kani",
        || <RustStdStandard<OnceWith<fn() -> i32>> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` an iterator's `.next().is_some()`
/// reports when it actually still had a value to yield -- the
/// positive counterpart to `IteratorYieldsNoneWhenExhausted`, named
/// separately since it's its own claim ("still has a value"), not
/// merely the boolean negation of exhaustion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct IteratorYieldsAValue;

impl KaniWitness for IteratorYieldsAValue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_once_with_calls_its_closure_exactly_once".to_owned(),
            VERIFY_ONCE_WITH_CALLS_ITS_CLOSURE_EXACTLY_ONCE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(IteratorYieldsAValue);

kani_ensures!(
    IteratorYieldsAValue,
    "amenable_kani::IteratorYieldsAValue",
    bool,
    |is_some| is_some
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IteratorYieldsAValue",
        "kani",
        || <IteratorYieldsAValue as KaniWitness>::proof().to_string(),
    )
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
            assert!(
                IteratorYieldsAValue::ensures(o.next().is_some()),
                "once_with yields one value"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(o.next()),
                "once_with yields nothing after its one value"
            );
        }
    }
}
