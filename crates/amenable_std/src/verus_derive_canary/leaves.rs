//! The atomic building-block leaves the composite canaries in
//! [`super::composites`] are built from.

use amenable_core::{ClassifiedWitness, Witness, WitnessSupportSummary};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use crate::{RustStdProvenance, RustStdStandard, RustStdType, VerusCheckedProof, VerusVerifier};

/// A leaf whose [`Witness<VerusVerifier>`] proof is real and
/// machine-checked — exercises the `Checked` slot of the derive-witness
/// composition canaries.
#[derive(
    Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, derive_new::new,
)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct CheckedVerusExportLeaf {
    /// Label for [`super::composites::VerusExportCanaryEnum`]'s
    /// checked-leaf slots.
    #[new(into)]
    label: String,
}

impl Witness<VerusVerifier> for CheckedVerusExportLeaf {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        <RustStdStandard<char> as Witness<VerusVerifier>>::proof()
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for CheckedVerusExportLeaf {}

/// A leaf whose [`Witness<VerusVerifier>`] proof rests on explicit
/// provenance rather than a machine-checked spec — exercises the
/// `Trusted` slot of the derive-witness composition canaries.
#[derive(
    Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, derive_new::new,
)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct TrustedVerusExportLeaf {
    /// Label for [`super::composites::VerusExportCanaryEnum`]'s
    /// trusted-leaf slots.
    #[new(into)]
    label: String,
}

impl Witness<VerusVerifier> for TrustedVerusExportLeaf {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <bool as RustStdType>::provenance()
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::trusted_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for TrustedVerusExportLeaf {}

/// A leaf whose real Verus harness has a real `requires` precondition —
/// exercises the derive-witness composition renderer's `requires`-
/// propagation path (reuses `EscapeAscii`'s real harness/call shape,
/// registered in `verus_witness`).
#[derive(
    Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, derive_new::new,
)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct RequiresVerusExportLeaf {
    /// Label for the `requires`-propagation canary's leaf slot.
    #[new(into)]
    label: String,
}

impl Witness<VerusVerifier> for RequiresVerusExportLeaf {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    fn proof() -> Self::ProofArtifact {
        <RustStdStandard<std::slice::EscapeAscii<'static>> as Witness<VerusVerifier>>::proof()
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for RequiresVerusExportLeaf {}

/// A leaf whose real Verus harness's own `ensures` mixes raw tuple-field
/// projections with a named-predicate citation whose own argument is a
/// projection-and-cast, not a bare call — exercises the derive-witness
/// composition renderer's `$placeholder`-template path (reuses
/// `RefCell`'s real harness/call shape, registered in `verus_witness`).
#[derive(
    Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, derive_new::new,
)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct RawTemplateVerusExportLeaf {
    /// Label for the raw-template canary's leaf slot.
    #[new(into)]
    label: String,
}

impl Witness<VerusVerifier> for RawTemplateVerusExportLeaf {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        <RustStdStandard<std::cell::RefCell<i32>> as Witness<VerusVerifier>>::proof()
    }

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for RawTemplateVerusExportLeaf {}

/// A leaf-free marker: exercises a composite's `Trivial` slot, the leaf
/// kind with nothing to check or trust.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub(super) struct TrivialVerusExportLeaf;
