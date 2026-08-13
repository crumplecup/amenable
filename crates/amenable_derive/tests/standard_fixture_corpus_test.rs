mod support;

use amenable_core::{EvidenceLink, Standard};
use support::{
    FixtureCase, GenericEnumFixture, GenericStructFixture, GenericTupleStructFixture,
    NamedEnumFixture, NamedStructFixture, NestedStructFixture, NestedTupleStructFixture,
    TupleEnumFixture, TupleStructFixture, UnitEnumFixture, UnitStructFixture, WitnessLeaf,
    expected_keys, expected_report, expected_values, generic_enum_variants,
};

fn assert_registered_root_link<F>()
where
    F: FixtureCase,
{
    let type_name = std::any::type_name::<F>();
    let link = inventory::iter::<EvidenceLink>()
        .into_iter()
        .find(|link| link.name == type_name)
        .unwrap_or_else(|| panic!("{type_name} should register an evidence link"));

    assert_eq!(link.basis, type_name);
}

fn assert_standard_fixture<F>()
where
    F: FixtureCase,
{
    let standard = F::sample();

    assert_eq!(standard.provenance(), standard.clone(), "{:?}", F::KIND);
    assert_eq!(standard.audit(), standard.clone(), "{:?}", F::KIND);
    assert_eq!(F::basis(), F::default(), "{:?}", F::KIND);
    assert!(F::is_root(), "{:?}", F::KIND);
    assert_eq!(
        F::chain(),
        vec![std::any::type_name::<F>()],
        "{:?}",
        F::KIND
    );
    assert_eq!(
        Standard::len(&standard),
        F::expected_entries().len(),
        "{:?}",
        F::KIND
    );
    assert_eq!(
        Standard::keys(&standard).collect::<Vec<_>>(),
        expected_keys::<F>()
    );
    assert_eq!(
        Standard::values(&standard).collect::<Vec<_>>(),
        expected_values::<F>()
    );
    assert_eq!(
        Standard::report(&standard).to_string(),
        expected_report::<F>()
    );

    if expects_registered_root_link(F::KIND) {
        assert_registered_root_link::<F>();
    }
}

fn expects_registered_root_link(kind: support::DeriveFixtureKind) -> bool {
    !matches!(
        kind,
        support::DeriveFixtureKind::InstantiatedGenericStruct
            | support::DeriveFixtureKind::InstantiatedGenericTupleStruct
            | support::DeriveFixtureKind::InstantiatedGenericEnum
    )
}

#[test]
fn standard_derive_projects_expected_metadata_for_every_fixture() {
    let _ = generic_enum_variants();

    assert_standard_fixture::<UnitStructFixture>();
    assert_standard_fixture::<NamedStructFixture>();
    assert_standard_fixture::<TupleStructFixture>();
    assert_standard_fixture::<UnitEnumFixture>();
    assert_standard_fixture::<NamedEnumFixture>();
    assert_standard_fixture::<TupleEnumFixture>();
    assert_standard_fixture::<NestedStructFixture>();
    assert_standard_fixture::<NestedTupleStructFixture>();
    assert_standard_fixture::<GenericStructFixture<WitnessLeaf, WitnessLeaf>>();
    assert_standard_fixture::<GenericTupleStructFixture<WitnessLeaf, WitnessLeaf>>();
    assert_standard_fixture::<GenericEnumFixture<WitnessLeaf, WitnessLeaf>>();
}
