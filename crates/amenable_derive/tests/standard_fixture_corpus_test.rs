mod support;

use amenable_core::{EvidenceLink, Standard};
use support::{
    DeriveFixtureKind, FixtureCase, expected_keys, expected_report, expected_values,
    for_each_fixture_type,
};

fn assert_registered_root_link<F>()
where
    F: FixtureCase,
{
    let type_name = std::any::type_name::<F>();
    let basis = inventory::iter::<EvidenceLink>()
        .into_iter()
        .find(|link| link.name() == type_name)
        .map(|link| link.basis());

    assert_eq!(
        basis,
        Some(type_name),
        "{type_name} should register an evidence link with a matching basis"
    );
}

fn assert_standard_fixture<F>()
where
    F: FixtureCase,
{
    let _ = F::expected_support();

    assert_eq!(F::basis(), F::default(), "{:?}", F::KIND);
    assert!(F::is_root(), "{:?}", F::KIND);
    assert_eq!(
        F::chain(),
        vec![std::any::type_name::<F>()],
        "{:?}",
        F::KIND
    );

    if expects_registered_root_link(F::KIND) {
        assert_registered_root_link::<F>();
    }

    for instance in F::instances() {
        assert_eq!(
            instance.value.provenance(),
            instance.value.clone(),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            instance.value.audit(),
            instance.value.clone(),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Standard::len(&instance.value),
            instance.expected_entries.len(),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Standard::keys(&instance.value).collect::<Vec<_>>(),
            expected_keys(instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Standard::values(&instance.value).collect::<Vec<_>>(),
            expected_values(instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
        assert_eq!(
            Standard::report(&instance.value).to_string(),
            expected_report(instance.expected_entries),
            "{:?}::{}",
            F::KIND,
            instance.label
        );
    }
}

fn expects_registered_root_link(kind: DeriveFixtureKind) -> bool {
    !matches!(
        kind,
        DeriveFixtureKind::InstantiatedGenericStruct
            | DeriveFixtureKind::InstantiatedGenericTupleStruct
            | DeriveFixtureKind::InstantiatedGenericEnum
    )
}

#[test]
fn standard_derive_projects_expected_metadata_for_every_fixture() {
    amenable_core::init_tracing();
    macro_rules! assert_fixture {
        ($fixture:ty) => {
            assert_standard_fixture::<$fixture>();
        };
    }

    for_each_fixture_type!(assert_fixture);
}
