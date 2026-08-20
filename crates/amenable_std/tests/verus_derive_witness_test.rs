#![cfg(feature = "verus")]

mod support;

use amenable_core::{
    ClassifiedWitness, Witness, WitnessArtifactShape, WitnessExportRecord, WitnessModulePath,
    WitnessSupportKind, WitnessSupportSummary,
};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use amenable_std::{
    CheckedVerusExportLeaf, RawTemplateVerusExportLeaf, RequiresVerusExportLeaf, RustStdProvenance,
    RustStdStandard, RustStdType, TrustedVerusExportLeaf, VerusCheckedProof, VerusExportCanaryEnum,
    VerusExportMultiCheckedEnum, VerusVerifier,
};
use support::derive_witness::{
    DerivedWitnessCheckedPlusTrivialStruct as SharedDerivedWitnessCheckedPlusTrivialStruct,
    DerivedWitnessGenericEnum as SharedDerivedWitnessGenericEnum,
    DerivedWitnessTupleStruct as SharedDerivedWitnessTupleStruct,
    assert_checked_plus_trivial_report, assert_generic_enum_report, assert_tuple_struct_report,
    balanced_variant_support, checked_plus_trivial_support, mixed_support, tuple_struct_support,
};

type ConcreteDerivedWitnessEnum =
    SharedDerivedWitnessGenericEnum<CheckedVerusLeaf, TrustedVerusLeaf, CheckedVerusLeaf>;
type ConcreteDerivedCheckedPlusTrivialStruct =
    SharedDerivedWitnessCheckedPlusTrivialStruct<CheckedVerusLeaf, TrivialVerusLeaf>;
type ConcreteDerivedTupleStruct =
    SharedDerivedWitnessTupleStruct<CheckedVerusLeaf, TrustedVerusLeaf, TrivialVerusLeaf>;

amenable_std::emit_verus_witnesses!(
    SharedDerivedWitnessGenericEnum<CheckedVerusLeaf, TrustedVerusLeaf, CheckedVerusLeaf>,
    SharedDerivedWitnessCheckedPlusTrivialStruct<CheckedVerusLeaf, TrivialVerusLeaf>,
    SharedDerivedWitnessTupleStruct<CheckedVerusLeaf, TrustedVerusLeaf, TrivialVerusLeaf>,
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

impl ClassifiedWitness<VerusVerifier> for CheckedVerusLeaf {}

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

impl ClassifiedWitness<VerusVerifier> for TrustedVerusLeaf {}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct TrivialVerusLeaf;

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

fn concrete_tuple_struct() -> ConcreteDerivedTupleStruct {
    ConcreteDerivedTupleStruct::new(
        CheckedVerusLeaf::new("unicode scalar"),
        TrustedVerusLeaf::new("rust bool docs"),
    )
}

#[test]
fn derive_witness_supports_concrete_generic_enums_for_verus() {
    let _ = concrete_variants();
    let expected_support = mixed_support();

    let proof = <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::proof();
    let proof_type = std::any::type_name::<
        <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::ProofArtifact,
    >();
    let report = proof.to_string();

    assert_generic_enum_report(
        proof_type,
        &report,
        &proof.variant_closed.to_string(),
        "verus",
        expected_support,
    );
    assert_eq!(
        <ConcreteDerivedWitnessEnum as Witness<VerusVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
    assert_eq!(proof.variant_balanced.support, balanced_variant_support());
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
    let expected_support = checked_plus_trivial_support();

    let proof = <ConcreteDerivedCheckedPlusTrivialStruct as Witness<VerusVerifier>>::proof();
    let report = proof.to_string();

    assert_checked_plus_trivial_report(&report, "verus", expected_support);
    assert_eq!(
        <ConcreteDerivedCheckedPlusTrivialStruct as Witness<VerusVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
    assert_eq!(proof.marker.support, WitnessSupportSummary::trivial_leaf());
}

#[test]
fn derive_witness_supports_tuple_structs_for_verus() {
    let _ = concrete_tuple_struct();
    let expected_support = tuple_struct_support();

    let proof = <ConcreteDerivedTupleStruct as Witness<VerusVerifier>>::proof();
    let report = proof.to_string();

    assert_tuple_struct_report(&report, "verus", expected_support);
    assert_eq!(
        <ConcreteDerivedTupleStruct as Witness<VerusVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
}

#[test]
fn explicit_verus_witness_exports_register_concrete_instantiations() -> miette::Result<()> {
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
                || evidence.contains("DerivedWitnessTupleStruct<")
        })
        .collect::<Vec<_>>();

    assert_eq!(exports.len(), 3);

    let enum_export = exports
        .iter()
        .find(|(evidence, _, _, _)| evidence.contains("DerivedWitnessGenericEnum<"))
        .ok_or_else(|| miette::miette!("expected explicit export for the concrete generic enum"))?;
    let struct_export = exports
        .iter()
        .find(|(evidence, _, _, _)| evidence.contains("DerivedWitnessCheckedPlusTrivialStruct<"))
        .ok_or_else(|| {
            miette::miette!("expected explicit export for the checked-plus-trivial struct")
        })?;
    let tuple_export = exports
        .iter()
        .find(|(evidence, _, _, _)| evidence.contains("DerivedWitnessTupleStruct<"))
        .ok_or_else(|| miette::miette!("expected explicit export for the tuple struct"))?;

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

    assert_eq!(
        tuple_export.1,
        <<ConcreteDerivedTupleStruct as Witness<VerusVerifier>>::ProofArtifact as WitnessModulePath>::MODULE_PATH
    );
    assert!(
        tuple_export.2.contains("verifier: verus"),
        "{}",
        tuple_export.2
    );
    assert!(
        tuple_export.2.contains("shape: tuple_struct"),
        "{}",
        tuple_export.2
    );
    assert_eq!(
        tuple_export.3,
        <ConcreteDerivedTupleStruct as Witness<VerusVerifier>>::support()
    );

    let structured_exports = amenable_core::witness_exports();
    let enum_artifact = structured_exports
        .iter()
        .find(|record| record.evidence.contains("DerivedWitnessGenericEnum<"))
        .ok_or_else(|| miette::miette!("expected structured export for the concrete generic enum"))?
        .artifact
        .clone();
    let struct_artifact = structured_exports
        .iter()
        .find(|record| {
            record
                .evidence
                .contains("DerivedWitnessCheckedPlusTrivialStruct<")
        })
        .ok_or_else(|| {
            miette::miette!("expected structured export for the checked-plus-trivial struct")
        })?
        .artifact
        .clone();
    let tuple_artifact = structured_exports
        .iter()
        .find(|record| record.evidence.contains("DerivedWitnessTupleStruct<"))
        .ok_or_else(|| miette::miette!("expected structured export for the tuple struct"))?
        .artifact
        .clone();

    assert_eq!(enum_artifact.shape, WitnessArtifactShape::Enum);
    assert_eq!(enum_artifact.kind, WitnessSupportKind::Mixed);
    assert_eq!(enum_artifact.tag.as_deref(), Some("entry_kind"));
    assert_eq!(enum_artifact.variants.len(), 3);
    let balanced_variant = enum_artifact
        .variants
        .iter()
        .find(|variant| variant.name == "Balanced")
        .ok_or_else(|| miette::miette!("expected Balanced variant artifact"))?;
    assert_eq!(
        balanced_variant.artifact.shape,
        WitnessArtifactShape::NamedVariant
    );
    let checked_leaf_metadata = &balanced_variant.artifact.members[0].artifact.metadata;
    assert_eq!(checked_leaf_metadata.len(), 3);
    assert_eq!(checked_leaf_metadata[0].key(), "verifier");
    assert_eq!(checked_leaf_metadata[0].value(), "verus");
    assert_eq!(checked_leaf_metadata[1].key(), "harness");
    assert_eq!(checked_leaf_metadata[1].value(), "verify_char_roundtrip");
    assert_eq!(checked_leaf_metadata[2].key(), "claim");
    assert!(
        checked_leaf_metadata[2]
            .value()
            .contains("pub fn verify_char_roundtrip(c: char)"),
        "{}",
        checked_leaf_metadata[2].value()
    );
    let trusted_leaf_metadata = &balanced_variant.artifact.members[1].artifact.metadata;
    assert!(
        trusted_leaf_metadata.iter().any(|entry| {
            entry.key() == "rust.authority" && entry.value() == "Rust Project Developers"
        }),
        "{trusted_leaf_metadata:?}"
    );

    assert_eq!(struct_artifact.shape, WitnessArtifactShape::NamedStruct);
    assert_eq!(struct_artifact.kind, WitnessSupportKind::Checked);
    assert_eq!(struct_artifact.members.len(), 2);
    assert_eq!(
        struct_artifact.members[0].artifact.shape,
        WitnessArtifactShape::Leaf
    );

    assert_eq!(tuple_artifact.shape, WitnessArtifactShape::TupleStruct);
    assert_eq!(tuple_artifact.kind, WitnessSupportKind::Mixed);
    assert_eq!(tuple_artifact.members.len(), 3);
    assert_eq!(
        tuple_artifact.members[0].artifact.shape,
        WitnessArtifactShape::Leaf
    );
    Ok(())
}

#[test]
fn live_verus_canary_exports_include_tuple_struct_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_tuple_struct_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus tuple-struct export canary"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::TupleStruct);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Mixed);
    assert_eq!(export.support, tuple_struct_support());
    assert_eq!(export.artifact.members.len(), 3);
    assert_eq!(export.artifact.members[0].label, "0");
    assert_eq!(export.artifact.members[1].label, "trusted");
    assert_eq!(export.artifact.members[2].label, "marker");
    Ok(())
}

type CanaryEnum = VerusExportCanaryEnum<CheckedVerusExportLeaf, TrustedVerusExportLeaf>;

#[test]
fn verus_export_canary_enum_variants_are_constructible() {
    let balanced = CanaryEnum::Balanced {
        checked: CheckedVerusExportLeaf::new("checked"),
        trusted: TrustedVerusExportLeaf::new("trusted"),
    };
    let adjustment = CanaryEnum::Adjustment(
        TrustedVerusExportLeaf::new("trusted"),
        CheckedVerusExportLeaf::new("checked"),
    );
    let closed = CanaryEnum::Closed;

    assert!(format!("{balanced:?}").starts_with("Balanced"));
    assert!(format!("{adjustment:?}").starts_with("Adjustment"));
    assert!(format!("{closed:?}").starts_with("Closed"));
}

#[test]
fn live_verus_canary_exports_include_canary_enum_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_canary_enum_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus canary-enum export"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::Enum);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Mixed);
    assert_eq!(export.artifact.variants.len(), 3);
    assert_eq!(export.artifact.variants[0].name, "Balanced");
    assert_eq!(
        export.artifact.variants[0].artifact.shape,
        WitnessArtifactShape::NamedVariant
    );
    assert_eq!(export.artifact.variants[1].name, "fallback");
    assert_eq!(
        export.artifact.variants[1].artifact.shape,
        WitnessArtifactShape::TupleVariant
    );
    assert_eq!(export.artifact.variants[1].artifact.members.len(), 1);
    assert_eq!(export.artifact.variants[2].name, "Closed");
    assert_eq!(
        export.artifact.variants[2].artifact.shape,
        WitnessArtifactShape::UnitVariant
    );
    Ok(())
}

#[test]
fn verus_export_multi_checked_enum_variants_are_constructible() {
    let active = VerusExportMultiCheckedEnum::Active {
        first: CheckedVerusExportLeaf::new("checked"),
        second: RequiresVerusExportLeaf::new("printable"),
    };
    let idle = VerusExportMultiCheckedEnum::Idle;

    assert!(format!("{active:?}").starts_with("Active"));
    assert!(format!("{idle:?}").starts_with("Idle"));
}

#[test]
fn live_verus_canary_exports_include_multi_checked_enum_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_multi_checked_enum_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus multi-checked-enum export"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::Enum);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Checked);
    assert_eq!(export.artifact.variants.len(), 2);
    assert_eq!(export.artifact.variants[0].name, "Active");
    assert_eq!(
        export.artifact.variants[0].artifact.shape,
        WitnessArtifactShape::NamedVariant
    );
    assert_eq!(export.artifact.variants[0].artifact.members.len(), 2);
    assert_eq!(
        export.artifact.variants[0].artifact.members[0].label,
        "first"
    );
    assert_eq!(
        export.artifact.variants[0].artifact.members[1].label,
        "second"
    );
    assert_eq!(export.artifact.variants[1].name, "Idle");
    assert_eq!(
        export.artifact.variants[1].artifact.shape,
        WitnessArtifactShape::UnitVariant
    );
    Ok(())
}

#[test]
fn requires_verus_export_leaf_is_constructible_and_checked() {
    let leaf = RequiresVerusExportLeaf::new("printable");

    assert_eq!(
        <RequiresVerusExportLeaf as Witness<VerusVerifier>>::support(),
        WitnessSupportSummary::checked_leaf()
    );
    assert!(format!("{leaf:?}").contains("printable"));
}

#[test]
fn live_verus_canary_exports_include_requires_struct_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_requires_struct_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus requires-struct export"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::NamedStruct);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Checked);
    Ok(())
}

#[test]
fn raw_template_verus_export_leaf_is_constructible_and_checked() {
    let leaf = RawTemplateVerusExportLeaf::new("borrow-state");

    assert_eq!(
        <RawTemplateVerusExportLeaf as Witness<VerusVerifier>>::support(),
        WitnessSupportSummary::checked_leaf()
    );
    assert!(format!("{leaf:?}").contains("borrow-state"));
}

#[test]
fn live_verus_canary_exports_include_raw_template_struct_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_raw_template_struct_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus raw-template-struct export"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::NamedStruct);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Checked);
    Ok(())
}

#[test]
fn live_verus_canary_exports_include_multi_checked_struct_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_multi_checked_struct_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus multi-checked-struct export"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::NamedStruct);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Checked);
    assert_eq!(export.artifact.members.len(), 2);
    assert_eq!(export.artifact.members[0].label, "first");
    assert_eq!(export.artifact.members[1].label, "second");
    Ok(())
}

#[test]
fn live_verus_canary_exports_include_nested_struct_shape() -> miette::Result<()> {
    let export = amenable_core::witness_exports()
        .into_iter()
        .find(|record| {
            record.verifier == "verus"
                && record.destination_module
                    == "crate::derived_witness::verus_export_nested_struct_witness"
        })
        .ok_or_else(|| miette::miette!("expected library Verus nested-struct export"))?;

    assert_eq!(export.artifact.shape, WitnessArtifactShape::NamedStruct);
    assert_eq!(export.artifact.kind, WitnessSupportKind::Mixed);
    assert_eq!(export.artifact.members.len(), 2);
    assert_eq!(export.artifact.members[0].label, "inner");
    assert_eq!(
        export.artifact.members[0].artifact.shape,
        WitnessArtifactShape::NamedStruct
    );
    assert_eq!(export.artifact.members[0].artifact.members.len(), 1);
    assert_eq!(
        export.artifact.members[0].artifact.members[0].label,
        "checked"
    );
    assert_eq!(export.artifact.members[1].label, "trusted");
    Ok(())
}
