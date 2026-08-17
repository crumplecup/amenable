use amenable_std::{CreusotGalleryExpectation, CreusotGalleryRegistration};

#[test]
fn all_twenty_eight_gallery_findings_are_registered_and_distinct() {
    let cases: Vec<_> = inventory::iter::<CreusotGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();

    assert_eq!(
        cases.len(),
        28,
        "expected exactly the 28 findings from this session's real pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 28, "gallery case ids must be unique");

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
        translation_error_count, 21,
        "twenty-one findings were real, diagnosed translation errors"
    );

    let unproved_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::Unproved)
        .count();
    assert_eq!(
        unproved_count, 3,
        "three findings (Ordering::reverse, f64 FromStr, empty atomic_sc callbacks) translated clean but failed at the SMT stage"
    );

    let proved_count = cases
        .iter()
        .filter(|case| case.expected == CreusotGalleryExpectation::Proved)
        .count();
    assert_eq!(
        proved_count, 2,
        "one finding is a dangerous false-trail proof that Creusot reported as proved despite \
         contractless externals; the other is the real, confirmed best practice that #[cfg(not(\
         creusot))]-gating inventory::collect!/submit! and Box<dyn Iterator<..>> in place avoids \
         the translator errors entirely"
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
