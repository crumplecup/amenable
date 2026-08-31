use std::iter::{Cloned, Enumerate, Rev, Zip};
use std::ops::Range;
use std::slice::Iter;

use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::{Ensures, Requires};
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
use crate::CheckedProof;
#[cfg(kani)]
use crate::FirstValueIsLessThanTheSecond;
#[cfg(kani)]
use crate::IteratorYieldsNoneWhenExhausted;
use crate::KaniWitness;
#[cfg(kani)]
use crate::PeekRevealsTheStoredReference;
#[cfg(kani)]
use crate::ValueIsWithinInclusiveRange;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_chain_sequences_two_iterators_end_to_end".to_owned(),
            VERIFY_CHAIN_SEQUENCES_TWO_ITERATORS_END_TO_END_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>>",
        "kani",
        || <RustStdStandard<std::iter::Chain<Range<i32>, Range<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX)));
            kani::assume(FirstValueIsLessThanTheSecond::requires((b, i32::MAX)));
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
        CheckedProof::new(
            "verify_zip_pairs_items_from_two_iterators".to_owned(),
            VERIFY_ZIP_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Zip<Range<i32>, Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Zip<Range<i32>, Range<i32>>>",
        "kani",
        || <RustStdStandard<Zip<Range<i32>, Range<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Zip<Range<i32>, Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Zip<Range<i32>, Range<i32>>>",
    (Option<(i32, i32)>, Option<(i32, i32)>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ZIP_PAIRS_ITEMS_FROM_TWO_ITERATORS_SRC, {
        /// `Zip` pairs up items from its two sources and stops as soon
        /// as either is exhausted.
        #[kani::proof]
        fn verify_zip_pairs_items_from_two_iterators() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX)));
            kani::assume(FirstValueIsLessThanTheSecond::requires((b, i32::MAX)));
            let mut z = (a..a + 1).zip(b..b + 1);
            assert!(
                RustStdStandard::<Zip<Range<i32>, Range<i32>>>::ensures((z.next(), Some((a, b)))),
                "zip pairs the two iterators' items"
            );
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
        CheckedProof::new(
            "verify_enumerate_pairs_each_item_with_its_index".to_owned(),
            VERIFY_ENUMERATE_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Enumerate<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Enumerate<Range<i32>>>",
        "kani",
        || <RustStdStandard<Enumerate<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Enumerate<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Enumerate<Range<i32>>>",
    (Option<(usize, i32)>, Option<(usize, i32)>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_ENUMERATE_PAIRS_EACH_ITEM_WITH_ITS_INDEX_SRC, {
        /// `Enumerate` pairs each item with a 0-based index that
        /// increments alongside the item.
        #[kani::proof]
        fn verify_enumerate_pairs_each_item_with_its_index() {
            let a: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 1)));
            let mut e = (a..a + 2).enumerate();
            assert!(
                RustStdStandard::<Enumerate<Range<i32>>>::ensures((e.next(), Some((0, a)))),
                "enumerate starts indexing at 0"
            );
            assert!(
                RustStdStandard::<Enumerate<Range<i32>>>::ensures((e.next(), Some((1, a + 1)))),
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
        CheckedProof::new(
            "verify_rev_reverses_iteration_order".to_owned(),
            VERIFY_REV_REVERSES_ITERATION_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Rev<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Rev<Range<i32>>>",
        "kani",
        || <RustStdStandard<Rev<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 1)));
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
        CheckedProof::new(
            "verify_cloned_clones_each_referenced_item".to_owned(),
            VERIFY_CLONED_CLONES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cloned<Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cloned<Iter<'static, i32>>>",
        "kani",
        || <RustStdStandard<Cloned<Iter<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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
