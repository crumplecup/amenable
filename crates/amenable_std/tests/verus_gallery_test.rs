#![cfg(feature = "verus")]

use amenable_std::{VerusGalleryExpectation, VerusGalleryRegistration};

#[test]
fn all_five_gallery_findings_are_registered_and_distinct() {
    let cases: Vec<_> = inventory::iter::<VerusGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();

    assert_eq!(
        cases.len(),
        5,
        "expected exactly the 5 findings from this session's real Verus pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 5, "gallery case ids must be unique");

    let not_supported_count = cases
        .iter()
        .filter(|case| case.expected == VerusGalleryExpectation::NotSupported)
        .count();
    assert_eq!(
        not_supported_count, 2,
        "two findings (NonZero::new, Saturating's + operator) were rejected outright as unsupported"
    );

    let unproved_count = cases
        .iter()
        .filter(|case| case.expected == VerusGalleryExpectation::Unproved)
        .count();
    assert_eq!(
        unproved_count, 3,
        "three findings (Wrapping's + operator, Reverse's cmp, the cfg(verus) hypothesis) were accepted but failed to establish the intended claim"
    );

    for case in &cases {
        assert!(
            !case.title.is_empty(),
            "gallery case {} must have a title",
            case.id
        );
        assert!(
            !case.claim.is_empty(),
            "gallery case {} must have a claim",
            case.id
        );
    }
}
