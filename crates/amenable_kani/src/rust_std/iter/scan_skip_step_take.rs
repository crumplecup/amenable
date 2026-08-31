use std::iter::{Scan, Skip, SkipWhile, StepBy};
use std::ops::Range;

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

impl KaniWitness for RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_scan_threads_state_through_its_closure".to_owned(),
            VERIFY_SCAN_THREADS_STATE_THROUGH_ITS_CLOSURE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>>",
        "kani",
        || <RustStdStandard<Scan<Range<i32>, i32, fn(&mut i32, i32) -> Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(ValueIsWithinInclusiveRange::requires((a, -1000, 1000)));
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
        CheckedProof::new(
            "verify_skip_discards_the_first_n_items".to_owned(),
            VERIFY_SKIP_DISCARDS_THE_FIRST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Skip<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Skip<Range<i32>>>",
        "kani",
        || <RustStdStandard<Skip<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Skip<Range<i32>>>,
    "amenable_std::rust_std::RustStdStandard<Skip<Range<i32>>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_SKIP_DISCARDS_THE_FIRST_N_ITEMS_SRC, {
        /// `Skip(n)` discards exactly the first `n` items.
        #[kani::proof]
        fn verify_skip_discards_the_first_n_items() {
            let a: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 2)));
            let mut s = (a..a + 3).skip(2);
            assert!(
                RustStdStandard::<Skip<Range<i32>>>::ensures((s.next(), Some(a + 2))),
                "skip discards exactly the first n items"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_skip_while_discards_items_while_the_predicate_holds".to_owned(),
            VERIFY_SKIP_WHILE_DISCARDS_ITEMS_WHILE_THE_PREDICATE_HOLDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>",
        "kani",
        || <RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>> as KaniWitness>::proof()
            .to_string(),
    )
}

kani_ensures!(
    RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>,
    "amenable_std::rust_std::RustStdStandard<SkipWhile<Range<i32>, fn(&i32) -> bool>>",
    (Option<i32>, Option<i32>),
    |(actual, expected)| actual == expected
);

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
            assert!(
                RustStdStandard::<SkipWhile<Range<i32>, fn(&i32) -> bool>>::ensures((
                    s.next(),
                    Some(a + 1)
                )),
                "skip_while discards items until the predicate first fails"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<StepBy<Range<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_step_by_yields_every_nth_item".to_owned(),
            VERIFY_STEP_BY_YIELDS_EVERY_NTH_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<StepBy<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<StepBy<Range<i32>>>",
        "kani",
        || <RustStdStandard<StepBy<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 4)));
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
        CheckedProof::new(
            "verify_take_yields_at_most_n_items".to_owned(),
            VERIFY_TAKE_YIELDS_AT_MOST_N_ITEMS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::iter::Take<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::iter::Take<Range<i32>>>",
        "kani",
        || <RustStdStandard<std::iter::Take<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 4)));
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
