mod support;

use amenable_core::{Provenance, Witness};
use amenable_derive::{
    Provenance as ProvenanceDerive, Standard as StandardDerive, Witness as WitnessDerive,
};
use support::{
    DeriveFixtureKind, FixtureCase, FixtureVerifier, WitnessLeaf, expected_keys, expected_report,
    expected_values, for_each_fixture_type,
};

fn assert_witness_fixture<F>() -> miette::Result<()>
where
    F: FixtureCase + Witness<FixtureVerifier>,
    <F as Witness<FixtureVerifier>>::ProofArtifact: std::fmt::Display,
{
    for instance in F::instances() {
        assert_eq!(
            Provenance::len(&instance.value),
            instance.expected_entries.len(),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Provenance::keys(&instance.value).collect::<Vec<_>>(),
            expected_keys(&instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Provenance::values(&instance.value).collect::<Vec<_>>(),
            expected_values(&instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Provenance::report(&instance.value).to_string(),
            expected_report(&instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
    }

    let proof = <F as Witness<FixtureVerifier>>::proof().to_string();
    let support = <F as Witness<FixtureVerifier>>::support();
    let proof_type = std::any::type_name::<<F as Witness<FixtureVerifier>>::ProofArtifact>();
    let evidence_nominal_type = std::any::type_name::<F>()
        .split('<')
        .next()
        .ok_or_else(|| miette::miette!("type_name should split into at least one segment"))?
        .rsplit("::")
        .next()
        .ok_or_else(|| miette::miette!("type_name segment should rsplit into at least one part"))?;

    assert!(
        proof.contains(&format!("evidence: {}", std::any::type_name::<F>())),
        "{proof}"
    );
    assert!(
        proof_type.contains(&format!("{evidence_nominal_type}WitnessProof")),
        "{proof_type}"
    );
    assert_eq!(support, F::expected_support());
    assert!(
        proof.contains(&format!("support: {}", F::expected_support())),
        "{proof}"
    );

    for fragment in expected_witness_fragments(F::KIND) {
        assert!(
            proof.contains(fragment),
            "missing `{fragment}` in:\n{proof}"
        );
    }
    Ok(())
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
        DeriveFixtureKind::CheckedPlusTrivialStruct => &[
            "verifier: fixture",
            "shape: named_struct",
            "member authority: leaf:",
            "member marker: verifier: fixture",
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

#[derive(Debug, Clone, PartialEq, Eq, Default, ProvenanceDerive, StandardDerive, WitnessDerive)]
#[provenance(crate = "amenable_core")]
#[standard(basis = "Self", provenance = "self.clone()", provenance_type = "Self")]
#[witness(verus(module = "crate::custom::proofs::shape_override_witness"))]
struct ShapeOverrideStruct {
    alpha: WitnessLeaf,
}

#[test]
fn witness_derive_projects_expected_structure_for_every_fixture() -> miette::Result<()> {
    amenable_core::init_tracing();
    macro_rules! assert_fixture {
        ($fixture:ty) => {
            assert_witness_fixture::<$fixture>()?;
        };
    }

    for_each_fixture_type!(assert_fixture);
    Ok(())
}

#[test]
fn witness_proofs_remember_enclosing_shape_not_just_leaf_multiset() {
    amenable_core::init_tracing();
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

#[test]
fn witness_derive_exposes_default_verus_destination_contract() {
    amenable_core::init_tracing();
    type ShapeMirrorStructProof = <ShapeMirrorStruct as Witness<FixtureVerifier>>::ProofArtifact;
    type ShapeMirrorEnumProof = <ShapeMirrorEnum as Witness<FixtureVerifier>>::ProofArtifact;

    assert_eq!(
        ShapeMirrorStructProof::VERUS_MODULE_PATH,
        "crate::derived_witness::shape_mirror_struct_witness"
    );
    assert_eq!(
        ShapeMirrorEnumProof::VERUS_MODULE_PATH,
        "crate::derived_witness::shape_mirror_enum_witness"
    );
}

#[test]
fn witness_derive_respects_explicit_verus_destination_override() {
    amenable_core::init_tracing();
    type ShapeOverrideStructProof =
        <ShapeOverrideStruct as Witness<FixtureVerifier>>::ProofArtifact;

    let report = <ShapeOverrideStruct as Witness<FixtureVerifier>>::proof().to_string();

    assert_eq!(
        ShapeOverrideStructProof::VERUS_MODULE_PATH,
        "crate::custom::proofs::shape_override_witness"
    );
    assert!(report.contains("shape: named_struct"), "{report}");
}
