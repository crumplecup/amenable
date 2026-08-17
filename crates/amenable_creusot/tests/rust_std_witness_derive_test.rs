mod support;

use amenable_core::{Witness, WitnessSupportSummary};
use amenable_creusot::{CheckedProof, CreusotVerifier, VERIFY_CHAR_ROUNDTRIP_SRC};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use amenable_std::{RustStdProvenance, RustStdStandard, RustStdType};
use support::derive_witness::{
    DerivedWitnessCheckedPlusTrivialStruct as SharedDerivedWitnessCheckedPlusTrivialStruct,
    DerivedWitnessGenericEnum as SharedDerivedWitnessGenericEnum,
    DerivedWitnessTupleStruct as SharedDerivedWitnessTupleStruct,
    assert_checked_plus_trivial_report, assert_generic_enum_report, assert_tuple_struct_report,
    balanced_variant_support, checked_plus_trivial_support, mixed_support, tuple_struct_support,
};

type ConcreteDerivedWitnessEnum =
    SharedDerivedWitnessGenericEnum<CheckedCreusotLeaf, TrustedCreusotLeaf, CheckedCreusotLeaf>;
type ConcreteDerivedCheckedPlusTrivialStruct =
    SharedDerivedWitnessCheckedPlusTrivialStruct<CheckedCreusotLeaf, TrivialCreusotLeaf>;
type ConcreteDerivedTupleStruct =
    SharedDerivedWitnessTupleStruct<CheckedCreusotLeaf, TrustedCreusotLeaf, TrivialCreusotLeaf>;

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct CheckedCreusotLeaf {
    label: String,
}

impl CheckedCreusotLeaf {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Witness<CreusotVerifier> for CheckedCreusotLeaf {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        <RustStdStandard<char> as Witness<CreusotVerifier>>::proof()
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct TrustedCreusotLeaf {
    label: String,
}

impl TrustedCreusotLeaf {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl Witness<CreusotVerifier> for TrustedCreusotLeaf {
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
struct TrivialCreusotLeaf;

fn concrete_variants() -> (ConcreteDerivedWitnessEnum, ConcreteDerivedWitnessEnum) {
    (
        ConcreteDerivedWitnessEnum::balanced(
            CheckedCreusotLeaf::new("unicode scalar"),
            TrustedCreusotLeaf::new("rust bool docs"),
        ),
        ConcreteDerivedWitnessEnum::adjustment(
            TrustedCreusotLeaf::new("fallback bool docs"),
            CheckedCreusotLeaf::new("skipped char proof"),
        ),
    )
}

fn concrete_checked_plus_trivial() -> ConcreteDerivedCheckedPlusTrivialStruct {
    ConcreteDerivedCheckedPlusTrivialStruct::new(CheckedCreusotLeaf::new("unicode scalar"))
}

fn concrete_tuple_struct() -> ConcreteDerivedTupleStruct {
    ConcreteDerivedTupleStruct::new(
        CheckedCreusotLeaf::new("unicode scalar"),
        TrustedCreusotLeaf::new("rust bool docs"),
    )
}

#[test]
fn derive_witness_supports_concrete_generic_enums_for_creusot() {
    let _ = concrete_variants();
    let expected_support = mixed_support();

    let proof = <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::proof();
    let proof_type = std::any::type_name::<
        <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::ProofArtifact,
    >();
    let report = proof.to_string();

    assert_generic_enum_report(
        proof_type,
        &report,
        &proof.variant_closed.to_string(),
        "creusot",
        expected_support,
    );
    assert_eq!(
        <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::support(),
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

    let CheckedProof {
        harness,
        claim,
        provenance,
    } = proof.variant_balanced.checked;
    assert_eq!(harness, "verify_char_roundtrip");
    assert_eq!(claim, VERIFY_CHAR_ROUNDTRIP_SRC);
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
fn derive_witness_keeps_trivial_members_neutral_for_creusot() {
    let _ = concrete_checked_plus_trivial();
    let expected_support = checked_plus_trivial_support();

    let proof = <ConcreteDerivedCheckedPlusTrivialStruct as Witness<CreusotVerifier>>::proof();
    let report = proof.to_string();

    assert_checked_plus_trivial_report(&report, "creusot", expected_support);
    assert_eq!(
        <ConcreteDerivedCheckedPlusTrivialStruct as Witness<CreusotVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
    assert_eq!(proof.marker.support, WitnessSupportSummary::trivial_leaf());
}

#[test]
fn derive_witness_supports_tuple_structs_for_creusot() {
    let _ = concrete_tuple_struct();
    let expected_support = tuple_struct_support();

    let proof = <ConcreteDerivedTupleStruct as Witness<CreusotVerifier>>::proof();
    let report = proof.to_string();

    assert_tuple_struct_report(&report, "creusot", expected_support);
    assert_eq!(
        <ConcreteDerivedTupleStruct as Witness<CreusotVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
}
