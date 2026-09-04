#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
#[cfg(kani)]
use crate::IndexRecoversTheStoredElement;
#[cfg(kani)]
use crate::IteratorYieldsAReferenceToTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::bridge_kani_witness;

impl KaniWitness for RustStdStandard<std::slice::Iter<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_iter_yields_shared_references_in_order".to_owned(),
            VERIFY_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::Iter<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::Iter<'static, i32>>",
        "kani",
        || <RustStdStandard<std::slice::Iter<'static, i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ITER_YIELDS_SHARED_REFERENCES_IN_ORDER_SRC, {
        /// `Iter::next` yields a shared reference to each element in
        /// order.
        #[kani::proof]
        fn verify_iter_yields_shared_references_in_order() {
            let value: i32 = kani::any();
            let data = [value];
            let mut it = data.iter();
            assert!(
                IteratorYieldsAReferenceToTheStoredValue::ensures((it.next(), Some(&value))),
                "iter yields a reference to the element"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::slice::IterMut<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_iter_mut_yields_mutable_references_that_write_through".to_owned(),
            VERIFY_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::slice::IterMut<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::slice::IterMut<'static, i32>>",
        "kani",
        || <RustStdStandard<std::slice::IterMut<'static, i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ITER_MUT_YIELDS_MUTABLE_REFERENCES_THAT_WRITE_THROUGH_SRC, {
        /// `IterMut::next` yields a mutable reference to each element,
        /// and a write through it is visible in the underlying slice.
        #[kani::proof]
        fn verify_iter_mut_yields_mutable_references_that_write_through() {
            let value: i32 = kani::any();
            let updated: i32 = kani::any();
            let mut data = [value];
            {
                let first = data.iter_mut().next().unwrap();
                assert!(
                    DerefReflectsTheStoredValue::ensures((*first, value)),
                    "iter_mut yields a reference to the element"
                );
                *first = updated;
            }
            assert!(
                IndexRecoversTheStoredElement::ensures((data[0], updated)),
                "a write through iter_mut's reference is visible"
            );
        }
    }
}
