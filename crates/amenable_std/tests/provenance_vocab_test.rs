use amenable_core::{Entry, Metadata};
use amenable_std::{Authority, AuthorityKind, SourceUrl, VerifierFamily};

/// A verifier-descriptor record composed from the shared vocabulary via
/// `#[entry(flatten)]` — the same six keys `KaniVerifierMetadata` /
/// `CreusotVerifierMetadata` / `VerusVerifierMetadata` now share.
#[derive(Debug, Clone, amenable_derive::Metadata)]
struct Descriptor {
    #[entry(flatten)]
    family: VerifierFamily,
    #[entry(flatten)]
    authority: Authority,
    #[entry(flatten)]
    url: SourceUrl,
    #[entry(flatten)]
    kind: AuthorityKind,
}

#[test]
fn vocabulary_keys_are_canonical_and_values_stay_typed() {
    amenable_core::init_tracing();

    assert_eq!(Authority::KEY, "authority");
    assert_eq!(SourceUrl::KEY, "source_url");
    assert_eq!(AuthorityKind::KEY, "authority_kind");

    let descriptor = Descriptor {
        family: VerifierFamily::new("kani"),
        authority: Authority::new("Kani Rust Verifier"),
        url: SourceUrl::from("https://model-checking.github.io/kani/"),
        kind: AuthorityKind::ExternalStandard,
    };

    // `#[entry(flatten)]` fields land under their vocabulary key, not the field
    // name (`family`, `authority`, `url`, `kind`).
    assert_eq!(
        descriptor.keys(),
        vec![
            "verifier_family",
            "authority",
            "source_url",
            "authority_kind"
        ]
    );

    assert_eq!(
        descriptor.get_as::<Authority>("authority"),
        Some(Authority::new("Kani Rust Verifier"))
    );
    assert_eq!(
        descriptor.get_as::<AuthorityKind>(AuthorityKind::KEY),
        Some(AuthorityKind::ExternalStandard)
    );
    assert_eq!(
        descriptor.report().to_string().lines().next(),
        Some("verifier_family: kani")
    );
    assert_eq!(
        AuthorityKind::ExternalStandard.to_string(),
        "external_standard"
    );
}
