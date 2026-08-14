//! Live derived-witness canaries for the Verus export pipeline.

use amenable_core::{Provenance, Witness, WitnessSupportSummary};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use crate::{RustStdProvenance, RustStdStandard, RustStdType, VerusCheckedProof, VerusVerifier};

type ConcreteVerusExportCanaryEnum =
    VerusExportCanaryEnum<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;
type ConcreteVerusExportCanaryStruct = VerusExportCheckedPlusTrivialStruct<CheckedVerusExportLeaf>;
type ConcreteVerusExportTupleStruct =
    VerusExportTupleStruct<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;

crate::emit_verus_witnesses!(
    ConcreteVerusExportCanaryEnum,
    ConcreteVerusExportCanaryStruct,
    ConcreteVerusExportTupleStruct,
);

/// A leaf whose [`Witness<VerusVerifier>`] proof is real and
/// machine-checked — exercises the `Checked` slot of the derive-witness
/// composition canaries.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct CheckedVerusExportLeaf {
    label: String,
}

impl CheckedVerusExportLeaf {
    /// Wrap a label for [`VerusExportCanaryEnum`]'s checked-leaf slots.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
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

/// A leaf whose [`Witness<VerusVerifier>`] proof rests on explicit
/// provenance rather than a machine-checked spec — exercises the
/// `Trusted` slot of the derive-witness composition canaries.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct TrustedVerusExportLeaf {
    label: String,
}

impl TrustedVerusExportLeaf {
    /// Wrap a label for [`VerusExportCanaryEnum`]'s trusted-leaf slots.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_tuple_struct_witness"))]
struct VerusExportTupleStruct<
    TChecked: Provenance + Clone + Default,
    TTrusted: Provenance + Clone + Default,
>(
    TChecked,
    #[provenance(rename = "trusted")] TTrusted,
    #[provenance(rename = "marker")] TrivialVerusExportLeaf,
);

/// An enum-shaped derive-witness composition canary: one variant mixing
/// a checked and a trusted leaf, one variant carrying only a trusted
/// leaf (plus a skipped checked one), and one leaf-free variant —
/// exercises named-variant, tuple-variant, and unit-variant derive
/// codegen in a single registered export.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_canary_enum_witness"))]
pub enum VerusExportCanaryEnum<
    TChecked: Provenance + Clone + Default,
    TTrusted: Provenance + Clone + Default,
> {
    /// Mixes a generic checked leaf with a fixed trusted leaf.
    Balanced {
        /// The checked leaf.
        checked: TChecked,
        /// The trusted leaf.
        trusted: TrustedVerusExportLeaf,
    },
    /// Carries a generic trusted leaf and a fixed checked leaf (skipped
    /// from provenance metadata) — the fallback path.
    #[provenance(rename = "fallback")]
    Adjustment(
        #[provenance(rename = "trusted")] TTrusted,
        #[provenance(skip)] CheckedVerusExportLeaf,
    ),
    /// Carries no leaves at all.
    #[default]
    Closed,
}
