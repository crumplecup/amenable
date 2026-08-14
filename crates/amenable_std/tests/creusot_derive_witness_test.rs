#![cfg(feature = "creusot")]

use amenable_core::{Provenance, Witness, WitnessSupportSummary};
use amenable_creusot::{CreusotVerifier, VERIFY_CHAR_ROUNDTRIP_SRC};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use amenable_std::{CheckedProof, RustStdProvenance, RustStdStandard, RustStdType};

type ConcreteDerivedWitnessEnum = DerivedWitnessGenericEnum<CheckedCreusotLeaf, TrustedCreusotLeaf>;
type ConcreteDerivedCheckedPlusTrivialStruct =
    DerivedWitnessCheckedPlusTrivialStruct<CheckedCreusotLeaf>;

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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct DerivedWitnessCheckedPlusTrivialStruct<TChecked: Provenance + Clone + Default> {
    checked: TChecked,
    marker: TrivialCreusotLeaf,
}

impl<TChecked: Provenance + Clone + Default> DerivedWitnessCheckedPlusTrivialStruct<TChecked> {
    fn new(checked: TChecked) -> Self {
        Self {
            checked,
            marker: TrivialCreusotLeaf,
        }
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

fn concrete_checked_plus_trivial() -> ConcreteDerivedCheckedPlusTrivialStruct {
    ConcreteDerivedCheckedPlusTrivialStruct::new(CheckedCreusotLeaf::new("unicode scalar"))
}

#[test]
fn derive_witness_supports_concrete_generic_enums_for_creusot() {
    let _ = concrete_variants();
    let mixed_support = WitnessSupportSummary::compose(&[
        WitnessSupportSummary::compose(&[
            WitnessSupportSummary::checked_leaf(),
            WitnessSupportSummary::trusted_leaf(),
        ]),
        WitnessSupportSummary::trusted_leaf(),
        WitnessSupportSummary::trivial_leaf(),
    ]);

    let proof = <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::proof();
    let proof_type = std::any::type_name::<
        <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::ProofArtifact,
    >();
    let report = proof.to_string();

    assert!(
        proof_type.contains("DerivedWitnessGenericEnumWitnessProof"),
        "{proof_type}"
    );
    assert!(report.contains("verifier: creusot"), "{report}");
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
        <ConcreteDerivedWitnessEnum as Witness<CreusotVerifier>>::support(),
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
    let expected_support = WitnessSupportSummary::compose(&[
        WitnessSupportSummary::checked_leaf(),
        WitnessSupportSummary::trivial_leaf(),
    ]);

    let proof = <ConcreteDerivedCheckedPlusTrivialStruct as Witness<CreusotVerifier>>::proof();
    let report = proof.to_string();

    assert!(report.contains("verifier: creusot"), "{report}");
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
        report.contains("member marker: verifier: creusot"),
        "{report}"
    );
    assert_eq!(
        <ConcreteDerivedCheckedPlusTrivialStruct as Witness<CreusotVerifier>>::support(),
        expected_support
    );
    assert_eq!(proof.support, expected_support);
    assert_eq!(proof.marker.support, WitnessSupportSummary::trivial_leaf());
}
