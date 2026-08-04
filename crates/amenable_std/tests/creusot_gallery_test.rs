#![cfg(feature = "creusot")]

use amenable_std::{CreusotGalleryExpectation, CreusotGalleryRegistration};

#[test]
fn all_eight_translator_findings_are_registered_and_distinct() {
    let cases: Vec<_> = inventory::iter::<CreusotGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();

    assert_eq!(
        cases.len(),
        8,
        "expected exactly the 8 findings from this session's real pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 8, "gallery case ids must be unique");

    let ice_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::Ice)
        .count();
    assert_eq!(
        ice_count, 1,
        "exactly one finding was a compiler ICE (RPITIT)"
    );

    let translation_error_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::TranslationError)
        .count();
    assert_eq!(
        translation_error_count, 7,
        "the other seven findings were real, diagnosed translation errors"
    );

    for case in &cases {
        assert!(
            !case.claim.trim().is_empty(),
            "{} has an empty claim",
            case.id
        );
        assert!(!case.title.is_empty(), "{} has an empty title", case.id);
    }
}
