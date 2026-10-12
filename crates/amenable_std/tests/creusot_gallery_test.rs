use amenable_std::{CreusotGalleryExpectation, CreusotGalleryRegistration};

#[test]
fn all_twenty_nine_gallery_findings_are_registered_and_distinct() {
    amenable_core::init_tracing();
    let cases: Vec<_> = inventory::iter::<CreusotGalleryRegistration>()
        .map(|registration| (registration.case())())
        .collect();

    assert_eq!(
        cases.len(),
        29,
        "expected exactly the 29 findings from this session's real pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id().as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 29, "gallery case ids must be unique");

    let ice_count = cases
        .iter()
        .filter(|case| case.expected() == CreusotGalleryExpectation::Ice)
        .count();
    assert_eq!(
        ice_count, 2,
        "two findings were compiler ICEs (RPITIT, float literal in Pearlite)"
    );

    let translation_error_count = cases
        .iter()
        .filter(|case| case.expected() == CreusotGalleryExpectation::TranslationError)
        .count();
    assert_eq!(
        translation_error_count, 21,
        "twenty-one findings were real, diagnosed translation errors"
    );

    let unproved_count = cases
        .iter()
        .filter(|case| case.expected() == CreusotGalleryExpectation::Unproved)
        .count();
    assert_eq!(
        unproved_count, 3,
        "three findings (Ordering::reverse, f64 FromStr, empty atomic_sc callbacks) translated clean but failed at the SMT stage"
    );

    let proved_count = cases
        .iter()
        .filter(|case| case.expected() == CreusotGalleryExpectation::Proved)
        .count();
    assert_eq!(
        proved_count, 3,
        "one finding is a dangerous false-trail proof that Creusot reported as proved despite \
         contractless externals; another is the real, confirmed best practice that #[cfg(not(\
         creusot))]-gating inventory::collect!/submit! and Box<dyn Iterator<..>> in place avoids \
         the translator errors entirely; the third is GapInfoHandle's private-field newtype \
         wrapper confirmed to let a public-field foreign struct be used in Creusot-translated \
         code at all"
    );

    for case in &cases {
        assert!(
            !case.claim().trim().is_empty(),
            "{} has an empty claim",
            case.id()
        );
        assert!(!case.title().is_empty(), "{} has an empty title", case.id());
    }
}
