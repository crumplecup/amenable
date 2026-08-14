#![cfg(feature = "verus")]

use amenable_core::{
    Provenance, Witness, WitnessArtifactShape, WitnessExportRecord, WitnessModulePath,
    WitnessSupportKind, WitnessSupportSummary,
};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use amenable_std::{
    RustStdProvenance, RustStdStandard, RustStdType, VerusCheckedProof, VerusVerifier,
};

type ConcreteDerivedWitnessEnum = DerivedWitnessGenericEnum<CheckedVerusLeaf, TrustedVerusLeaf>;
type ConcreteDerivedCheckedPlusTrivialStruct =
    DerivedWitnessCheckedPlusTrivialStruct<CheckedVerusLeaf>;

amenable_std::emit_verus_witnesses!(
    DerivedWitnessGenericEnum<CheckedVerusLeaf, TrustedVerusLeaf>,
    DerivedWitnessCheckedPlusTrivialStruct<CheckedVerusLeaf>,
);

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct CheckedVerusLeaf {
    label: String,
}

impl CheckedVerusLeaf {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Witness<VerusVerifier> for CheckedVerusLeaf {
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
struct TrustedVerusLeaf {
    label: String,
}

impl TrustedVerusLeaf {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Witness<VerusVerifier> for TrustedVerusLeaf {
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
struct TrivialVerusLeaf;

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct DerivedWitnessCheckedPlusTrivialStruct<TChecked: Provenance + Clone + Default> {
    checked: TChecked,
    marker: TrivialVerusLeaf,
}

impl<TChecked: Provenance + Clone + Default> DerivedWitnessCheckedPlusTrivialStruct<TChecked> {
    fn new(checked: TChecked) -> Self {
        Self {
            checked,
            marker: TrivialVerusLeaf,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
enum DerivedWitnessGenericEnum<TChecked: Provenance + Clone, TTrusted: Provenance + Clone> {
    Balanced {
        checked: TChecked,
        trusted: TrustedVerusLeaf,
    },
    #[provenance(rename = "fallback")]
    Adjustment(
        #[provenance(rename = "trusted")] TTrusted,
        #[provenance(skip)] CheckedVerusLeaf,
    ),
    #[default]
    Closed,
}

impl<TChecked: Provenance + Clone, TTrusted: Provenance + Clone>
    DerivedWitnessGenericEnum<TChecked, TTrusted>
{
    fn balanced(checked: TChecked, trusted: TrustedVerusLeaf) -> Self {
        Self::Balanced { checked, trusted }
    }

    fn adjustment(trusted: TTrusted, skipped_checked: CheckedVerusLeaf) -> Self {
        Self::Adjustment(trusted, skipped_checked)
    }
}

fn concrete_variants() -> (ConcreteDerivedWitnessEnum, ConcreteDerivedWitnessEnum) {
    (
        ConcreteDerivedWitnessEnum::balanced(
            CheckedVerusLeaf::new("unicode scalar"),
            TrustedVerusLeaf::new("rust bool docs"),
        ),
        ConcreteDerivedWitnessEnum::adjustment(
            TrustedVerusLeaf::new("fallback bool docs"),
            CheckedVerusLeaf::new("skipped char proof"),
        ),
    )
}

fn concrete_checked_plus_trivial() -> ConcreteDerivedCheckedPlusTrivialStruct {
    ConcreteDerivedCheckedPlusTrivialStruct::new(CheckedVerusLeaf::new("unicode scalar"))
}

#[test]
fn derive_witness_supports_concrete_generic_enums_for_verus() {
    let _ = concrete_variants();
    let mixed_support = WitnessSupportSummary::compose(&[
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::trusted_leaf(),
        ]),
        WitnessSupportSummary::trusted_leaf(),
        WitnessSupportSummary::trivial_leaf(),
    ]);

    let proof = <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::proof();
    let proof_type = std::any::type_name::<
        <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::ProofArtifact,
    >();
    let report = proof.to_string();

    assert!(
        proof_type.contains("DerivedWitnessGenericEnumWitnessProof"),
        "{proof_type}"
    );
    assert!(report.contains("verifier: verus"), "{report}");
    assert!(report.contains("shape: enum"), "{report}");
    assert!(
        report.contains(&format!("support: {mixed_support}")),
        "{report}"
    );
    assert!(report.contains("tag: entry_kind"), "{report}");
    assert!(
        report.contains("variant Balanced: shape: named_variant"),
        "{report}"
    );
    assert!(
        report.contains("member checked: harness: verify_char_roundtrip"),
        "{report}"
    );
    assert!(
        report.contains("variant fallback: shape: tuple_variant"),
        "{report}"
    );
    assert!(
        proof
            .variant_closed
            .to_string()
            .contains("shape: unit_variant"),
        "{}",
        proof.variant_closed
    );
    assert_eq!(
        <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::support(),
        mixed_support
    );
    assert_eq!(proof.support, mixed_support);
    assert_eq!(
        proof.variant_balanced.support,
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::trusted_leaf(),
        ])
    );
    assert_eq!(
        proof.variant_adjustment.support,
        WitnessSupportSummary::trusted_leaf()
    );
    assert_eq!(
        proof.variant_closed.support,
        WitnessSupportSummary::trivial_leaf()
    );

    let VerusCheckedProof {
        harness,
        claim,
        provenance,
    } = proof.variant_balanced.checked;
    assert_eq!(harness, "verify_char_roundtrip");
    assert_eq!(
        claim,
        include_str!("../../amenable_verus/src/rust_std/char_carrier.rs")
    );
    assert_eq!(provenance, <char as RustStdType>::provenance());
    assert_eq!(
        proof.variant_balanced.trusted,
        <bool as RustStdType>::provenance()
    );
    assert_eq!(
        proof.variant_adjustment.field_0,
        <bool as RustStdType>::provenance()
    );
}

#[test]
fn derive_witness_keeps_trivial_members_neutral_for_verus() {
    let _ = concrete_checked_plus_trivial();
    let expected_support = WitnessSupportSummary::compose(&[
        WitnessSupportSummary::checked_leaf(),
        WitnessSupportSummary::trivial_leaf(),
    ]);

    let proof = <ConcreteDerivedCheckedPlusTrivialStruct as Witness<VerusVerifier>>::proof();
    let report = proof.to_string();

    assert!(report.contains("verifier: verus"), "{report}");
    assert!(report.contains("shape: named_struct"), "{report}");
    assert!(
        report.contains(&format!("support: {expected_support}")),
        "{report}"
    );
    assert!(
        report.contains("member checked: harness: verify_char_roundtrip"),
        "{report}"
    );
    assert!(
        report.contains("member marker: verifier: verus"),
        "{report}"
    );
    assert_eq!(
        <ConcreteDerivedCheckedPlusTrivialStruct as Witness<VerusVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
    assert_eq!(proof.marker.support, WitnessSupportSummary::trivial_leaf());
}

#[test]
fn explicit_verus_witness_exports_register_concrete_instantiations() {
    let exports = inventory::iter::<WitnessExportRecord>()
        .filter(|record| (record.verifier)() == "verus")
        .map(|record| {
            (
                (record.evidence)().to_string(),
                (record.destination_module)().to_string(),
                (record.describe)(),
                (record.support)(),
            )
        })
        .filter(|(evidence, _, _, _)| {
            evidence.contains("DerivedWitnessGenericEnum<")
                || evidence.contains("DerivedWitnessCheckedPlusTrivialStruct<")
        })
        .collect::<Vec<_>>();

    assert_eq!(exports.len(), 2);

    let enum_export = exports
        .iter()
        .find(|(evidence, _, _, _)| evidence.contains("DerivedWitnessGenericEnum<"))
        .expect("expected explicit export for the concrete generic enum");
    let struct_export = exports
        .iter()
        .find(|(evidence, _, _, _)| evidence.contains("DerivedWitnessCheckedPlusTrivialStruct<"))
        .expect("expected explicit export for the checked-plus-trivial struct");

    assert_eq!(
        enum_export.1,
        <<ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::ProofArtifact as WitnessModulePath>::MODULE_PATH
    );
    assert!(
        enum_export.2.contains("verifier: verus"),
        "{}",
        enum_export.2
    );
    assert!(enum_export.2.contains("shape: enum"), "{}", enum_export.2);
    assert_eq!(
        enum_export.3,
        <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::support()
    );

    assert_eq!(
        struct_export.1,
        <<ConcreteDerivedCheckedPlusTrivialStruct as Witness<VerusVerifier>>::ProofArtifact as WitnessModulePath>::MODULE_PATH
    );
    assert!(
        struct_export.2.contains("verifier: verus"),
        "{}",
        struct_export.2
    );
    assert!(
        struct_export.2.contains("shape: named_struct"),
        "{}",
        struct_export.2
    );
    assert_eq!(
        struct_export.3,
        <ConcreteDerivedCheckedPlusTrivialStruct as Witness<VerusVerifier>>::support()
    );

    let structured_exports = amenable_core::witness_exports();
    let enum_artifact = structured_exports
        .iter()
        .find(|record| record.evidence.contains("DerivedWitnessGenericEnum<"))
        .expect("expected structured export for the concrete generic enum")
        .artifact
        .clone();
    let struct_artifact = structured_exports
        .iter()
        .find(|record| {
            record
                .evidence
                .contains("DerivedWitnessCheckedPlusTrivialStruct<")
        })
        .expect("expected structured export for the checked-plus-trivial struct")
        .artifact
        .clone();

    assert_eq!(enum_artifact.shape, WitnessArtifactShape::Enum);
    assert_eq!(enum_artifact.kind, WitnessSupportKind::Mixed);
    assert_eq!(enum_artifact.tag.as_deref(), Some("entry_kind"));
    assert_eq!(enum_artifact.variants.len(), 3);
    assert_eq!(
        enum_artifact.variants[0].artifact.shape,
        WitnessArtifactShape::NamedVariant
    );

    assert_eq!(struct_artifact.shape, WitnessArtifactShape::NamedStruct);
    assert_eq!(struct_artifact.kind, WitnessSupportKind::Checked);
    assert_eq!(struct_artifact.members.len(), 2);
    assert_eq!(
        struct_artifact.members[0].artifact.shape,
        WitnessArtifactShape::Leaf
    );
}
