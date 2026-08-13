#![cfg(feature = "creusot")]

use amenable_core::{Provenance, Witness};
use amenable_creusot::{CreusotVerifier, VERIFY_CHAR_ROUNDTRIP_SRC};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use amenable_std::{CheckedProof, RustStdProvenance, RustStdStandard, RustStdType};

type ConcreteDerivedWitnessEnum =
    DerivedWitnessGenericEnum<CheckedCreusotLeaf, TrustedCreusotLeaf>;

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
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "entry_kind")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
enum DerivedWitnessGenericEnum<TChecked: Provenance + Clone, TTrusted: Provenance + Clone> {
    Balanced {
        checked: TChecked,
        trusted: TrustedCreusotLeaf,
    },
    #[provenance(rename = "fallback")]
    Adjustment(
        #[provenance(rename = "trusted")] TTrusted,
        #[provenance(skip)] CheckedCreusotLeaf,
    ),
    #[default]
    Closed,
}

impl<TChecked: Provenance + Clone, TTrusted: Provenance + Clone>
    DerivedWitnessGenericEnum<TChecked, TTrusted>
{
    fn balanced(checked: TChecked, trusted: TrustedCreusotLeaf) -> Self {
        Self::Balanced { checked, trusted }
    }

    fn adjustment(trusted: TTrusted, skipped_checked: CheckedCreusotLeaf) -> Self {
        Self::Adjustment(trusted, skipped_checked)
    }
}

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

#[test]
fn derive_witness_supports_concrete_generic_enums_for_creusot() {
    let _ = concrete_variants();

    let proof = <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::proof();
    let proof_type = std::any::type_name::<<ConcreteDerivedWitnessEnum as Witness<
        CreusotVerifier,
    >>::ProofArtifact>();
    let report = proof.to_string();

    assert!(
        proof_type.contains("DerivedWitnessGenericEnumWitnessProof"),
        "{proof_type}"
    );
    assert!(report.contains("verifier: creusot"), "{report}");
    assert!(report.contains("shape: enum"), "{report}");
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
