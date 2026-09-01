use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::binary_heap_iterators::PeekRevealsTheStoredReference;
use super::linked_list_iterators::VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC;
use crate::CheckedProof;
use crate::KaniWitness;

/// The `#[cfg(kani)]` imports this file needs, consolidated into one gate
/// on this `mod` instead of one per item -- see
/// `amenable_creusot::stoplight::mirror`'s own doc comment for the
/// general rationale. Every name is re-exported: the `harness! { .. }`
/// block below needs all of them, unqualified, at this file's own top
/// level.
#[cfg(kani)]
mod mirror {
    pub(super) use amenable_core::{Ensures, Requires};
    pub(super) use std::collections::BinaryHeap;

    pub(super) use crate::DerefReflectsTheStoredValue;
    pub(super) use crate::FirstValueIsLessThanTheSecond;
    pub(super) use crate::PopRecoversTheStoredValue;
}
#[cfg(kani)]
use mirror::{
    BinaryHeap, DerefReflectsTheStoredValue, Ensures, FirstValueIsLessThanTheSecond,
    PopRecoversTheStoredValue, Requires,
};

impl<T> KaniWitness for PeekRevealsTheStoredReference<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_binary_heap_peek_mut_exposes_the_maximum".to_owned(),
            VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier> for PeekRevealsTheStoredReference<T> {
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for PeekRevealsTheStoredReference<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::PeekRevealsTheStoredReference",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::PeekRevealsTheStoredReference",
        "kani",
        || <PeekRevealsTheStoredReference<i32> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BINARY_HEAP_PEEK_MUT_EXPOSES_THE_MAXIMUM_SRC, {
        /// `.peek_mut()` derefs to the greatest element. Leaving it
        /// unmodified keeps it at the top, while lowering it through
        /// the guard re-establishes the heap invariant when the guard
        /// is released.
        #[kani::proof]
        fn verify_binary_heap_peek_mut_exposes_the_maximum() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();
            kani::assume(FirstValueIsLessThanTheSecond::requires((a, b)));

            let mut heap = BinaryHeap::new();
            heap.push(a);
            heap.push(b);
            {
                let peek = heap.peek_mut().unwrap();
                assert!(
                    DerefReflectsTheStoredValue::ensures((*peek, b)),
                    "peek_mut derefs to the greatest element"
                );
            }
            assert!(
                PeekRevealsTheStoredReference::ensures((heap.peek(), Some(&b))),
                "the maximum is still on top afterward"
            );
            {
                let mut peek = heap.peek_mut().unwrap();
                *peek = a;
                assert!(
                    DerefReflectsTheStoredValue::ensures((*peek, a)),
                    "peek_mut writes through to the guarded maximum"
                );
            }
            assert!(
                PeekRevealsTheStoredReference::ensures((heap.peek(), Some(&a))),
                "releasing a modified guard re-establishes the heap maximum"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((heap.pop(), Some(a))),
                "the re-heapified first value is available"
            );
            assert!(
                PopRecoversTheStoredValue::ensures((heap.pop(), Some(a))),
                "the re-heapified remaining value is available"
            );
        }
    }
}

/// An `(actual, expected)` pair of `.next()` results known to agree: an
/// iterator over shared references yields a reference to the exact
/// value known to be there.
///
/// Independently hand-written as `assert_eq!(it.next(), Some(&value),
/// ...)` at 7 real sites spanning `LinkedList::iter`, `VecDeque::iter`,
/// `Option::iter`, `Result::iter`, and `slice::iter` -- the identical
/// claim regardless of container kind. Generic over the whole
/// `Option<&value>` result type rather than just the referenced
/// element type: unlike this session's other generic contract types
/// (which vary over the *element* type across real sites, all fixed
/// `i32` here), this one has to vary over the *borrow's lifetime*,
/// which differs at every real call site (each borrows from its own
/// local container) -- a non-generic type has no way to name an
/// unconstrained lifetime, so `T` here is inferred as the full
/// `Option<&'a i32>` at each call site instead.
pub struct IteratorYieldsAReferenceToTheStoredValue<T>(std::marker::PhantomData<T>);

impl<T> amenable_core::Standard for IteratorYieldsAReferenceToTheStoredValue<T> {
    type Provenance = amenable_std::RustStdProvenance;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn provenance(&self) -> Self::Provenance {
        <i32 as amenable_std::RustStdType>::provenance()
    }
}

impl<T> Evidence for IteratorYieldsAReferenceToTheStoredValue<T> {
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

impl<T> KaniWitness for IteratorYieldsAReferenceToTheStoredValue<T> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_linked_list_iter_yields_references_in_order".to_owned(),
            VERIFY_LINKED_LIST_ITER_YIELDS_REFERENCES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

impl<T> amenable_core::Witness<crate::KaniVerifier>
    for IteratorYieldsAReferenceToTheStoredValue<T>
{
    type SupportingEvidence = <Self as KaniWitness>::SupportingEvidence;
    type ProofArtifact = <Self as KaniWitness>::ProofArtifact;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <Self as KaniWitness>::proof()
    }
}

impl<T: PartialEq> amenable_core::Ensures<crate::KaniVerifier>
    for IteratorYieldsAReferenceToTheStoredValue<T>
{
    type Input = (T, T);
    type Bound = bool;

    fn ensures((actual, expected): (T, T)) -> bool {
        actual == expected
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::IteratorYieldsAReferenceToTheStoredValue",
        "kani",
        "ensures",
        || stringify!(actual == expected),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IteratorYieldsAReferenceToTheStoredValue",
        "kani",
        || <IteratorYieldsAReferenceToTheStoredValue<i32> as KaniWitness>::proof()
            .to_string(),
    )
}
