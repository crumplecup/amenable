//! Live derived-witness canaries for the Verus export pipeline.

use amenable_core::{Provenance, Witness, WitnessSupportSummary};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use crate::{RustStdProvenance, RustStdStandard, RustStdType, VerusCheckedProof, VerusVerifier};

type ConcreteVerusExportCanaryEnum =
    VerusExportCanaryEnum<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;
type ConcreteVerusExportCanaryStruct = VerusExportCheckedPlusTrivialStruct<CheckedVerusExportLeaf>;

crate::emit_verus_witnesses!(
    ConcreteVerusExportCanaryEnum,
    ConcreteVerusExportCanaryStruct,
);

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct CheckedVerusExportLeaf {
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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct TrustedVerusExportLeaf {
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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct TrivialVerusExportLeaf;

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(
    module = "crate::derived_witness::verus_export_checked_plus_trivial_struct_witness"
))]
struct VerusExportCheckedPlusTrivialStruct<TChecked: Provenance + Clone + Default> {
    checked: TChecked,
    marker: TrivialVerusExportLeaf,
}

#[expect(
    dead_code,
    reason = "the canary exists to register and export a concrete derived proof shape; no runtime value construction is required"
)]
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_canary_enum_witness"))]
enum VerusExportCanaryEnum<
    TChecked: Provenance + Clone + Default,
    TTrusted: Provenance + Clone + Default,
> {
    Balanced {
        checked: TChecked,
        trusted: TrustedVerusExportLeaf,
    },
    #[provenance(rename = "fallback")]
    Adjustment(
        #[provenance(rename = "trusted")] TTrusted,
        #[provenance(skip)] CheckedVerusExportLeaf,
    ),
    #[default]
    Closed,
}
