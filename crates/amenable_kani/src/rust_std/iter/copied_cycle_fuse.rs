use std::iter::{Copied, Cycle, Fuse, Inspect, Peekable};
use std::ops::Range;
use std::slice::Iter;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// blocks below need all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};
    pub(super) use std::cell::Cell;

    pub(super) use crate::FirstValueIsLessThanTheSecond;
    pub(super) use crate::IteratorYieldsNoneWhenExhausted;
    pub(super) use crate::PeekRevealsTheStoredReference;
}
#[cfg(kani)]
use mirror::{
    Cell, Ensures, FirstValueIsLessThanTheSecond, IteratorYieldsNoneWhenExhausted,
    PeekRevealsTheStoredReference, Requires,
};

impl KaniWitness for RustStdStandard<Copied<Iter<'static, i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_copied_copies_each_referenced_item".to_owned(),
            VERIFY_COPIED_COPIES_EACH_REFERENCED_ITEM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Copied<Iter<'static, i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Copied<Iter<'static, i32>>>",
        "kani",
        || <RustStdStandard<Copied<Iter<'static, i32>>> as KaniWitness>::proof()
            .to_string(),
    )
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cycle_repeats_its_sequence_forever".to_owned(),
            VERIFY_CYCLE_REPEATS_ITS_SEQUENCE_FOREVER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cycle<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cycle<Range<i32>>>",
        "kani",
        || <RustStdStandard<Cycle<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 1)));
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_fuse_keeps_returning_none_once_exhausted".to_owned(),
            VERIFY_FUSE_KEEPS_RETURNING_NONE_ONCE_EXHAUSTED_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Fuse<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Fuse<Range<i32>>>",
        "kani",
        || <RustStdStandard<Fuse<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX)));
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_inspect_calls_once_per_item_without_changing_values".to_owned(),
            VERIFY_INSPECT_CALLS_ONCE_PER_ITEM_WITHOUT_CHANGING_VALUES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Inspect<Range<i32>, fn(&i32)>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Inspect<Range<i32>, fn(&i32)>>",
        "kani",
        || <RustStdStandard<Inspect<Range<i32>, fn(&i32)>> as KaniWitness>::proof()
            .to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((value, i32::MAX)));
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

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_peekable_peek_does_not_consume".to_owned(),
            VERIFY_PEEKABLE_PEEK_DOES_NOT_CONSUME_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Peekable<Range<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Peekable<Range<i32>>>",
        "kani",
        || <RustStdStandard<Peekable<Range<i32>>> as KaniWitness>::proof().to_string(),
    )
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
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, i32::MAX - 1)));
            let mut p = (a..a + 2).peekable();
            assert!(
                PeekRevealsTheStoredReference::ensures((p.peek(), Some(&a))),
                "peek previews the next item"
            );
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
