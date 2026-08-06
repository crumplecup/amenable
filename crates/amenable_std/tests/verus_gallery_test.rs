#![cfg(feature = "verus")]

use amenable_std::{VerusGalleryExpectation, VerusGalleryRegistration};

#[test]
fn all_nine_gallery_findings_are_registered_and_distinct() {
    let cases: Vec<_> = inventory::iter::<VerusGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();

    assert_eq!(
        cases.len(),
        9,
        "expected exactly the 9 findings from this session's real Verus pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 9, "gallery case ids must be unique");

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
        unproved_count, 5,
        "five findings (Wrapping's + operator, Reverse's cmp, the cfg(verus) hypothesis, Layout::new's size/align, Cell's hidden state) were accepted but failed to establish the intended claim"
    );

    let ice_count = cases
        .iter()
        .filter(|case| case.expected == VerusGalleryExpectation::Ice)
        .count();
    assert_eq!(
        ice_count, 1,
        "one finding (a duplicate assume_specification for a trait method vstd already specifies) crashed verus outright"
    );

    let proved_count = cases
        .iter()
        .filter(|case| case.expected == VerusGalleryExpectation::Proved)
        .count();
    assert_eq!(
        proved_count, 1,
        "one finding (TryFromSliceError's phantom-lifetime-binder + match-ergonomics lessons) was fully resolved, not just a documented dead end"
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
