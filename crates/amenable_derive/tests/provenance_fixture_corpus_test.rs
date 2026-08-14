mod support;

use amenable_core::Provenance;
use strum::IntoEnumIterator;
use support::{
    DeriveFixtureKind, FixtureCase, expected_keys, expected_report, expected_values,
    for_each_fixture_type,
};

fn fixture_kinds() -> Vec<DeriveFixtureKind> {
    let mut kinds = Vec::new();

    macro_rules! push_kind {
        ($fixture:ty) => {
            kinds.push(<$fixture>::KIND);
        };
    }

    for_each_fixture_type!(push_kind);
    kinds
}

fn assert_provenance_fixture<F>()
where
    F: FixtureCase,
{
    let _ = F::expected_support();

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
            expected_keys(instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Provenance::values(&instance.value).collect::<Vec<_>>(),
            expected_values(instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Provenance::report(&instance.value).to_string(),
            expected_report(instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
    }
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
    macro_rules! assert_fixture {
        ($fixture:ty) => {
            assert_provenance_fixture::<$fixture>();
        };
    }

    for_each_fixture_type!(assert_fixture);
}
