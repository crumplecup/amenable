//! `Vec<i32>`'s core `KaniWitness` impl and the push/pop round-trip harness,
//! plus the two contract types that harness reuses:
//! `VecLengthTracksPushesAndPops` (element count) and the generic
//! `PopRecoversTheStoredValue` (owned-value pop accessor).

use std::vec::Vec;

use amenable_core::Evidence;
use amenable_derive::Standard;
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};

#[cfg(kani)]
use amenable_core::Ensures;
#[cfg(kani)]
use std::cell::Cell;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};
#[cfg(kani)]
use crate::{
    EmptiedContainerReportsEmpty, IndexRecoversTheStoredElement, IteratorYieldsNoneWhenExhausted,
};

impl KaniWitness for RustStdStandard<Vec<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_push_pop_round_trips".to_owned(),
            VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Vec<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
        "kani",
        || <RustStdStandard<Vec<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Vec<i32>>,
    "amenable_std::rust_std::RustStdStandard<Vec<i32>>",
    (Vec<i32>, Vec<i32>),
    |(actual, expected)| actual == expected
);

/// A `Vec`'s length known to match the count of pushes and pops applied
/// to it -- `RustStdStandard<Vec<i32>>`'s own `Ensures<KaniVerifier>`
/// slot is already occupied by content-equality (just above), so this
/// distinct claim (element count, not content) needs its own type per
/// the associated-type-uniqueness rule.
///
/// A derived claim about `usize`, not a fresh root authority — its
/// evidence chain rests on `usize`'s own already-registered standard-
/// library provenance ([`RustStdStandard<usize>`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Standard)]
#[standard(
    basis = "RustStdStandard<usize>",
    basis_ctor = "RustStdStandard::<usize>::new()",
    provenance = "<usize as RustStdType>::provenance()",
    provenance_type = "RustStdProvenance"
)]
pub struct VecLengthTracksPushesAndPops {
    value: usize,
}

impl VecLengthTracksPushesAndPops {
    /// Wrap a length already known to match the count of pushes and pops.
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// The wrapped length.
    pub const fn value(&self) -> usize {
        self.value
    }
}

impl KaniWitness for VecLengthTracksPushesAndPops {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_push_pop_round_trips".to_owned(),
            VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(VecLengthTracksPushesAndPops);

kani_ensures!(
    VecLengthTracksPushesAndPops,
    "amenable_kani::rust_std::alloc_vec::VecLengthTracksPushesAndPops",
    (usize, usize),
    |(actual, expected)| actual == expected
);

/// An `(actual, expected)` pair of `.pop()` results known to agree: a
/// container's pop accessor recovers exactly the owned value known to
/// be there, transferring ownership out -- the owned-value counterpart
/// to `PeekRevealsTheStoredReference` (`.peek()`, borrows without
/// consuming), same reasoning for keeping the two separate despite an
/// identical `Ensures` impl body.
///
/// Independently hand-written as `assert_eq!(container.pop(),
/// Some(value), ...)` at 3 real sites spanning `Vec::pop()` and
/// `BinaryHeap::pop()`.
pub struct PopRecoversTheStoredValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for PopRecoversTheStoredValue<T> {
    type Provenance = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as RustStdType>::provenance()
    }
}

impl<T> Evidence for PopRecoversTheStoredValue<T> {
    type Basis = RustStdStandard<i32>;
    type Audit = RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn basis() -> Self::Basis {
        RustStdStandard::<i32>::new()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn audit(&self) -> Self::Audit {
        <i32 as RustStdType>::provenance()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", ret))]
    fn is_root() -> bool {
        false
    }
}

impl<T> KaniWitness for PopRecoversTheStoredValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_vec_push_pop_round_trips".to_owned(),
            VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for PopRecoversTheStoredValue<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier> for PopRecoversTheStoredValue<T> {
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::PopRecoversTheStoredValue",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PopRecoversTheStoredValue",
        "kani",
        || <PopRecoversTheStoredValue<i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_VEC_PUSH_POP_ROUND_TRIPS_SRC, {
        /// `push` appends and is indexable, and `pop` removes and
        /// returns the last element, leaving the Vec empty. Also
        /// checked with a drop-instrumented, non-`Copy` witness type:
        /// `pop()` transfers ownership out *without* dropping the
        /// value, and dropping the Vec drops every remaining element
        /// exactly once — `i32` alone has no drop glue to distinguish
        /// "moved out cleanly" from "dropped early" or "leaked".
        #[kani::proof]
        fn verify_vec_push_pop_round_trips() {
            let value = <i32 as crate::KaniCompose>::kani_any();
            let mut v = <Vec<i32> as crate::KaniCompose>::kani_depth0();
            v.push(value);
            assert!(VecLengthTracksPushesAndPops::ensures((v.len(), 1)));
            assert!(
                IndexRecoversTheStoredElement::ensures((v[0], value)),
                "the pushed value is indexable"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((v.pop(), Some(value))),
                "pop returns the last pushed value"
            );
            assert!(
                EmptiedContainerReportsEmpty::ensures(v.is_empty()),
                "pop leaves the Vec empty"
            );
            assert!(
                IteratorYieldsNoneWhenExhausted::ensures(v.pop()),
                "popping an exhausted Vec returns None"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witnesses = Vec::new();
            witnesses.push(DropWitness { drop_count: drop_count.clone() });
            witnesses.push(DropWitness { drop_count: drop_count.clone() });
            let popped = witnesses.pop().unwrap();
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "pop does not drop the returned value"
            );
            drop(popped);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "the popped value drops once its owner drops it"
            );
            drop(witnesses);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 2)),
                "dropping the Vec drops the remaining element"
            );
        }
    }
}
