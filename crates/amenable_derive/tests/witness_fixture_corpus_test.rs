mod support;

use amenable_core::Witness;
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use support::{
    DeriveFixtureKind, FixtureCase, FixtureVerifier, GenericEnumFixture, GenericStructFixture,
    GenericTupleStructFixture, NamedEnumFixture, NamedStructFixture, NestedStructFixture,
    NestedTupleStructFixture, TupleEnumFixture, TupleStructFixture, UnitEnumFixture,
    UnitStructFixture, WitnessLeaf, expected_keys, expected_report, expected_values,
    generic_enum_variants,
};

fn assert_witness_fixture<F>()
where
    F: FixtureCase + Witness<FixtureVerifier>,
    <F as Witness<FixtureVerifier>>::ProofArtifact: std::fmt::Display,
{
    let _ = F::sample();
    let _ = F::expected_entries();
    let _ = expected_keys::<F>();
    let _ = expected_values::<F>();
    let _ = expected_report::<F>();

    let proof = <F as Witness<FixtureVerifier>>::proof().to_string();
    let proof_type = std::any::type_name::<<F as Witness<FixtureVerifier>>::ProofArtifact>();
    let evidence_nominal_type = std::any::type_name::<F>()
        .split('<')
        .next()
        .unwrap()
        .rsplit("::")
        .next()
        .unwrap();

    assert!(
        proof.contains(&format!("evidence: {}", std::any::type_name::<F>())),
        "{proof}"
    );
    assert!(
        proof_type.contains(&format!("{evidence_nominal_type}WitnessProof")),
        "{proof_type}"
    );

    for fragment in expected_witness_fragments(F::KIND) {
        assert!(
            proof.contains(fragment),
            "missing `{fragment}` in:\n{proof}"
        );
    }
}

fn expected_witness_fragments(kind: DeriveFixtureKind) -> &'static [&'static str] {
    match kind {
        DeriveFixtureKind::UnitStruct => &["verifier: fixture", "shape: unit_struct"],
        DeriveFixtureKind::NamedStruct => &[
            "verifier: fixture",
            "shape: named_struct",
            "member authority: leaf:",
            "member decision_id: leaf:",
        ],
        DeriveFixtureKind::TupleStruct => &[
            "verifier: fixture",
            "shape: tuple_struct",
            "member authority: leaf:",
            "member 1: leaf:",
        ],
        DeriveFixtureKind::UnitEnum => &[
            "verifier: fixture",
            "shape: enum",
            "tag: authority_kind",
            "variant InternalOnly: shape: unit_variant",
            "variant ExternalStandard: shape: unit_variant",
        ],
        DeriveFixtureKind::NamedEnum => &[
            "verifier: fixture",
            "shape: enum",
            "tag: authority_kind",
            "variant RustProject: shape: named_variant",
            "member authority: leaf:",
            "member source_url: leaf:",
            "variant local_design: shape: named_variant",
            "member owner: leaf:",
            "variant InternalOnly: shape: unit_variant",
        ],
        DeriveFixtureKind::TupleEnum => &[
            "verifier: fixture",
            "shape: enum",
            "tag: authority_kind",
            "variant RustProject: shape: tuple_variant",
            "member authority: leaf:",
            "member 1: leaf:",
            "variant local_design: shape: tuple_variant",
            "member owner: leaf:",
            "variant InternalOnly: shape: unit_variant",
        ],
        DeriveFixtureKind::NestedStruct => &[
            "verifier: fixture",
            "shape: named_struct",
            "member authority_source: verifier: fixture",
            "variant local_design: shape: named_variant",
            "member semantic_summary: leaf:",
        ],
        DeriveFixtureKind::NestedTupleStruct => &[
            "verifier: fixture",
            "shape: tuple_struct",
            "member authority_source: verifier: fixture",
            "variant local_design: shape: tuple_variant",
            "member semantic_summary: leaf:",
        ],
        DeriveFixtureKind::InstantiatedGenericStruct => &[
            "verifier: fixture",
            "shape: named_struct",
            "member authority: leaf:",
            "member decision_id: leaf:",
        ],
        DeriveFixtureKind::InstantiatedGenericTupleStruct => &[
            "verifier: fixture",
            "shape: tuple_struct",
            "member authority: leaf:",
            "member decision_id: leaf:",
        ],
        DeriveFixtureKind::InstantiatedGenericEnum => &[
            "verifier: fixture",
            "shape: enum",
            "tag: authority_kind",
            "variant RustProject: shape: named_variant",
            "member authority: leaf:",
            "member source_url: leaf:",
            "variant local_design: shape: tuple_variant",
            "member owner: leaf:",
            "variant InternalOnly: shape: unit_variant",
        ],
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
struct ShapeMirrorStruct {
    alpha: WitnessLeaf,
    beta: WitnessLeaf,
    gamma: WitnessLeaf,
}

#[derive(Debug, Clone, PartialEq, Eq, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core", tag = "shape")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
enum ShapeMirrorEnum {
    Alpha(WitnessLeaf),
    Beta(WitnessLeaf),
    Gamma(WitnessLeaf),
}

impl Default for ShapeMirrorEnum {
    fn default() -> Self {
        Self::Gamma(WitnessLeaf::default())
    }
}

#[test]
fn witness_derive_projects_expected_structure_for_every_fixture() {
    let _ = generic_enum_variants();

    assert_witness_fixture::<UnitStructFixture>();
    assert_witness_fixture::<NamedStructFixture>();
    assert_witness_fixture::<TupleStructFixture>();
    assert_witness_fixture::<UnitEnumFixture>();
    assert_witness_fixture::<NamedEnumFixture>();
    assert_witness_fixture::<TupleEnumFixture>();
    assert_witness_fixture::<NestedStructFixture>();
    assert_witness_fixture::<NestedTupleStructFixture>();
    assert_witness_fixture::<GenericStructFixture<WitnessLeaf, WitnessLeaf>>();
    assert_witness_fixture::<GenericTupleStructFixture<WitnessLeaf, WitnessLeaf>>();
    assert_witness_fixture::<GenericEnumFixture<WitnessLeaf, WitnessLeaf>>();
}

#[test]
fn witness_proofs_remember_enclosing_shape_not_just_leaf_multiset() {
    let _ = ShapeMirrorEnum::Alpha(WitnessLeaf::default());
    let _ = ShapeMirrorEnum::Beta(WitnessLeaf::default());

    let struct_type =
        std::any::type_name::<<ShapeMirrorStruct as Witness<FixtureVerifier>>::ProofArtifact>();
    let enum_type =
        std::any::type_name::<<ShapeMirrorEnum as Witness<FixtureVerifier>>::ProofArtifact>();
    let struct_proof = <ShapeMirrorStruct as Witness<FixtureVerifier>>::proof().to_string();
    let enum_proof = <ShapeMirrorEnum as Witness<FixtureVerifier>>::proof().to_string();

    assert_ne!(struct_type, enum_type);
    assert_ne!(struct_proof, enum_proof);
    assert!(
        struct_proof.contains("shape: named_struct"),
        "{struct_proof}"
    );
    assert!(enum_proof.contains("shape: enum"), "{enum_proof}");
    assert!(enum_proof.contains("tag: shape"), "{enum_proof}");
}
