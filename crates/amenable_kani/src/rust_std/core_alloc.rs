//! `KaniWitness` impls for `core::alloc`.

use std::alloc::{Layout, LayoutError};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
#[cfg(kani)]
use crate::FallibleOperationReportsFailure;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Layout> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_layout_new_reports_the_types_size_and_alignment".to_owned(),
            VERIFY_LAYOUT_NEW_REPORTS_THE_TYPES_SIZE_AND_ALIGNMENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Layout>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Layout>",
        "kani",
        || <RustStdStandard<Layout> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_LAYOUT_NEW_REPORTS_THE_TYPES_SIZE_AND_ALIGNMENT_SRC, {
        /// `Layout::new::<T>()` reports `T`'s real size and alignment.
        #[kani::proof]
        fn verify_layout_new_reports_the_types_size_and_alignment() {
            let layout = Layout::new::<i32>();
            assert!(
                AccessorRecoversTheExpectedValue::ensures((layout.size(), 4)),
                "Layout::new::<i32>() reports i32's size"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((layout.align(), 4)),
                "Layout::new::<i32>() reports i32's alignment"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<LayoutError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_layout_from_size_align_rejects_a_non_power_of_two_alignment".to_owned(),
            VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<LayoutError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LayoutError>",
        "kani",
        || <RustStdStandard<LayoutError> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_LAYOUT_FROM_SIZE_ALIGN_REJECTS_A_NON_POWER_OF_TWO_ALIGNMENT_SRC, {
        /// `Layout::from_size_align` rejects an alignment that isn't a
        /// power of two.
        #[kani::proof]
        fn verify_layout_from_size_align_rejects_a_non_power_of_two_alignment() {
            let result = Layout::from_size_align(4, 3);
            assert!(
                FallibleOperationReportsFailure::ensures(result.is_err()),
                "a non-power-of-two alignment is rejected"
            );
        }
    }
}
