#![cfg(feature = "creusot")]

use amenable_std::{CreusotGalleryExpectation, CreusotGalleryRegistration};

#[test]
fn all_thirteen_translator_findings_are_registered_and_distinct() {
    let cases: Vec<_> = inventory::iter::<CreusotGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();

    assert_eq!(
        cases.len(),
        13,
        "expected exactly the 13 findings from this session's real pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 13, "gallery case ids must be unique");

    let ice_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::Ice)
        .count();
    assert_eq!(
        ice_count, 2,
        "two findings were compiler ICEs (RPITIT, float literal in Pearlite)"
    );

    let translation_error_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::TranslationError)
        .count();
    assert_eq!(
        translation_error_count, 10,
        "ten findings were real, diagnosed translation errors"
    );

    let unproved_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::Unproved)
        .count();
    assert_eq!(
        unproved_count, 1,
        "one finding (Ordering::reverse) translated clean but failed at the SMT stage"
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
