//! Live derived-witness canaries for the Verus export pipeline.

use amenable_core::{ClassifiedWitness, Provenance, Witness, WitnessSupportSummary};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};

use crate::{RustStdProvenance, RustStdStandard, RustStdType, VerusCheckedProof, VerusVerifier};

type ConcreteVerusExportCanaryStruct = VerusExportCheckedPlusTrivialStruct<CheckedVerusExportLeaf>;
type ConcreteVerusExportTupleStruct =
    VerusExportTupleStruct<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;
type ConcreteVerusExportCanaryEnum =
    VerusExportCanaryEnum<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;

crate::emit_verus_witnesses!(
    ConcreteVerusExportCanaryStruct,
    ConcreteVerusExportTupleStruct,
    VerusExportRequiresStruct,
    VerusExportRawTemplateStruct,
    ConcreteVerusExportCanaryEnum,
    VerusExportMultiCheckedStruct,
    VerusExportNestedStruct,
    VerusExportMultiCheckedEnum,
);

/// A leaf whose [`Witness<VerusVerifier>`] proof is real and
/// machine-checked — exercises the `Checked` slot of the derive-witness
/// composition canaries.
#[derive(
    Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, derive_new::new,
)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
pub struct CheckedVerusExportLeaf {
    /// Label for [`VerusExportCanaryEnum`]'s checked-leaf slots.
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
    /// Label for [`VerusExportCanaryEnum`]'s trusted-leaf slots.
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
/// registered in `verus_witness.rs`).
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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_requires_struct_witness"))]
struct VerusExportRequiresStruct {
    checked: RequiresVerusExportLeaf,
}

/// Two checked leaves with different real harnesses in one composite —
/// exercises the derive-witness composition renderer's multi-checked-
/// call tuple path (`-> (result: (T1, T2))`, `result.0`/`result.1`
/// citations) against the real `verus` tool for the first time: every
/// other checked-leaf canary so far has had exactly one checked field,
/// so this path was previously only exercised by `RefCell`'s own single
/// leaf, whose real return type already happens to be a tuple -- never
/// by two genuinely independent checked calls composed into one.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_multi_checked_struct_witness"))]
struct VerusExportMultiCheckedStruct {
    first: CheckedVerusExportLeaf,
    second: RequiresVerusExportLeaf,
}

/// A composite field that is itself another already-composed `Witness`
/// type, not a bare leaf -- confirms the derive macro and the
/// `amenable::verus_export` renderer both recurse correctly past depth
/// 1, which no canary so far has exercised (every existing composite's
/// own fields have all been leaves).
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_nested_struct_witness"))]
struct VerusExportNestedStruct {
    inner: VerusExportRequiresStruct,
    trusted: TrustedVerusExportLeaf,
}

/// A leaf whose real Verus harness's own `ensures` mixes raw tuple-field
/// projections with a named-predicate citation whose own argument is a
/// projection-and-cast, not a bare call — exercises the derive-witness
/// composition renderer's `$placeholder`-template path (reuses
/// `RefCell`'s real harness/call shape, registered in
/// `verus_witness.rs`).
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

    fn proof() -> Self::ProofArtifact {
        <RustStdStandard<std::cell::RefCell<i32>> as Witness<VerusVerifier>>::proof()
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<VerusVerifier> for RawTemplateVerusExportLeaf {}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_raw_template_struct_witness"))]
struct VerusExportRawTemplateStruct {
    checked: RawTemplateVerusExportLeaf,
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

/// One enum variant carrying two checked leaves with different real
/// harnesses — exercises `render_enum_module`'s multi-checked-call
/// bind-name path (`r0`/`r1`, not the single-`r` path) against the real
/// `verus` tool for the first time: `VerusExportCanaryEnum::Balanced`
/// above has always had exactly one checked leaf per variant.
#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "state")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::derived_witness::verus_export_multi_checked_enum_witness"))]
pub enum VerusExportMultiCheckedEnum {
    /// Two independent checked leaves in one variant.
    Active {
        /// The first checked leaf.
        first: CheckedVerusExportLeaf,
        /// The second checked leaf.
        second: RequiresVerusExportLeaf,
    },
    /// Carries no leaves at all.
    #[default]
    Idle,
}
