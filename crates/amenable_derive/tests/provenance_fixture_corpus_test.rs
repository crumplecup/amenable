mod support;

use amenable_core::Provenance;
use strum::IntoEnumIterator;
use support::{
    DeriveFixtureKind, FixtureCase, GenericEnumFixture, GenericStructFixture,
    GenericTupleStructFixture, NamedEnumFixture, NamedStructFixture, NestedStructFixture,
    NestedTupleStructFixture, TupleEnumFixture, TupleStructFixture, UnitEnumFixture,
    UnitStructFixture, WitnessLeaf, expected_keys, expected_report, expected_values,
    generic_enum_variants,
};

fn fixture_kinds() -> Vec<DeriveFixtureKind> {
    vec![
        UnitStructFixture::KIND,
        NamedStructFixture::KIND,
        TupleStructFixture::KIND,
        UnitEnumFixture::KIND,
        NamedEnumFixture::KIND,
        TupleEnumFixture::KIND,
        NestedStructFixture::KIND,
        NestedTupleStructFixture::KIND,
        GenericStructFixture::<WitnessLeaf, WitnessLeaf>::KIND,
        GenericTupleStructFixture::<WitnessLeaf, WitnessLeaf>::KIND,
        GenericEnumFixture::<WitnessLeaf, WitnessLeaf>::KIND,
    ]
}

fn assert_provenance_fixture<F>()
where
    F: FixtureCase,
{
    let fixture = F::sample();

    assert_eq!(
        Provenance::len(&fixture),
        F::expected_entries().len(),
        "{:?}",
        F::KIND
    );
    assert_eq!(
        Provenance::keys(&fixture).collect::<Vec<_>>(),
        expected_keys::<F>()
    );
    assert_eq!(
        Provenance::values(&fixture).collect::<Vec<_>>(),
        expected_values::<F>()
    );
    assert_eq!(
        Provenance::report(&fixture).to_string(),
        expected_report::<F>()
    );
}

#[test]
fn fixture_inventory_matches_the_registered_fixture_types() {
    assert_eq!(
        DeriveFixtureKind::iter().collect::<Vec<_>>(),
        fixture_kinds()
    );
}

#[test]
fn provenance_derive_projects_expected_metadata_for_every_fixture() {
    let _ = generic_enum_variants();

    assert_provenance_fixture::<UnitStructFixture>();
    assert_provenance_fixture::<NamedStructFixture>();
    assert_provenance_fixture::<TupleStructFixture>();
    assert_provenance_fixture::<UnitEnumFixture>();
    assert_provenance_fixture::<NamedEnumFixture>();
    assert_provenance_fixture::<TupleEnumFixture>();
    assert_provenance_fixture::<NestedStructFixture>();
    assert_provenance_fixture::<NestedTupleStructFixture>();
    assert_provenance_fixture::<GenericStructFixture<WitnessLeaf, WitnessLeaf>>();
    assert_provenance_fixture::<GenericTupleStructFixture<WitnessLeaf, WitnessLeaf>>();
    assert_provenance_fixture::<GenericEnumFixture<WitnessLeaf, WitnessLeaf>>();
}

#[test]
fn generic_enum_fixture_projects_expected_metadata_for_split_variant_usage() {
    let (_, fixture) = generic_enum_variants();

    assert_eq!(
        Provenance::report(&fixture).to_string(),
        "authority_kind: local_design\nowner: UI Working Group"
    );
}
