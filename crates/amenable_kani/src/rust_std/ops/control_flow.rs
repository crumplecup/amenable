//! `ControlFlow<i32, i32>`'s `KaniWitness` impl and harness, plus the
//! `IsContinueVariantReportsTrue` / `IsBreakVariantReportsTrue` raw-boolean
//! claim types its variant checks reuse.

use std::ops::ControlFlow;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<ControlFlow<i32, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_control_flow_continue_and_break_are_disjoint".to_owned(),
            VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ControlFlow<i32, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ControlFlow<i32, i32>>",
        "kani",
        || <RustStdStandard<ControlFlow<i32, i32>> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` `ControlFlow::is_continue()` reports
/// when the flow is actually the `Continue` variant -- following
/// `EmptiedContainerReportsEmpty`'s established shape for a raw
/// boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct IsContinueVariantReportsTrue;

impl KaniWitness for IsContinueVariantReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_control_flow_continue_and_break_are_disjoint".to_owned(),
            VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(IsContinueVariantReportsTrue);

kani_ensures!(
    IsContinueVariantReportsTrue,
    "amenable_kani::IsContinueVariantReportsTrue",
    bool,
    |is_continue| is_continue
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IsContinueVariantReportsTrue",
        "kani",
        || <IsContinueVariantReportsTrue as KaniWitness>::proof().to_string(),
    )
}

/// The `.is_break()` sibling of [`IsContinueVariantReportsTrue`], same
/// reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct IsBreakVariantReportsTrue;

impl KaniWitness for IsBreakVariantReportsTrue {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_control_flow_continue_and_break_are_disjoint".to_owned(),
            VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(IsBreakVariantReportsTrue);

kani_ensures!(
    IsBreakVariantReportsTrue,
    "amenable_kani::IsBreakVariantReportsTrue",
    bool,
    |is_break| is_break
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::IsBreakVariantReportsTrue",
        "kani",
        || <IsBreakVariantReportsTrue as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_CONTROL_FLOW_CONTINUE_AND_BREAK_ARE_DISJOINT_SRC, {
        /// `Continue` and `Break` are mutually exclusive, and each
        /// accessor round-trips the value the other variant lacks.
        #[kani::proof]
        fn verify_control_flow_continue_and_break_are_disjoint() {
            let c: i32 = kani::any();
            let b: i32 = kani::any();

            let flow: ControlFlow<i32, i32> = ControlFlow::Continue(c);
            assert!(
                IsContinueVariantReportsTrue::ensures(flow.is_continue()),
                "Continue reports is_continue"
            );
            assert!(
                !IsBreakVariantReportsTrue::ensures(flow.is_break()),
                "Continue reports !is_break"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((flow.continue_value(), Some(c))),
                "Continue round-trips its value"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((flow.break_value(), None)),
                "Continue has no break value"
            );

            let flow: ControlFlow<i32, i32> = ControlFlow::Break(b);
            assert!(
                IsBreakVariantReportsTrue::ensures(flow.is_break()),
                "Break reports is_break"
            );
            assert!(
                !IsContinueVariantReportsTrue::ensures(flow.is_continue()),
                "Break reports !is_continue"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((flow.break_value(), Some(b))),
                "Break round-trips its value"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((flow.continue_value(), None)),
                "Break has no continue value"
            );
        }
    }
}
