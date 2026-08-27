#![cfg(feature = "verus")]

use amenable_std::{VerusGalleryExpectation, VerusGalleryRegistration};

#[test]
fn all_eleven_gallery_findings_are_registered_and_distinct() {
    amenable_core::init_tracing();
    let cases: Vec<_> = inventory::iter::<VerusGalleryRegistration>()
        .map(|registration| (registration.case())())
        .collect();

    assert_eq!(
        cases.len(),
        11,
        "expected exactly the 11 findings from this session's real Verus pipeline work: {cases:#?}"
    );

    let mut ids: Vec<&str> = cases.iter().map(|case| case.id().as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 11, "gallery case ids must be unique");

    let not_supported_count = cases
        .iter()
        .filter(|case| case.expected() == VerusGalleryExpectation::NotSupported)
        .count();
    assert_eq!(
        not_supported_count, 2,
        "two findings (NonZero::new, Saturating's + operator) were rejected outright as unsupported"
    );

    let unproved_count = cases
        .iter()
        .filter(|case| case.expected() == VerusGalleryExpectation::Unproved)
        .count();
    assert_eq!(
        unproved_count, 5,
        "five findings (Wrapping's + operator, Reverse's cmp, the cfg(verus) hypothesis, Layout::new's size/align, Cell's hidden state) were accepted but failed to establish the intended claim"
    );

    let ice_count = cases
        .iter()
        .filter(|case| case.expected() == VerusGalleryExpectation::Ice)
        .count();
    assert_eq!(
        ice_count, 1,
        "one finding (a duplicate assume_specification for a trait method vstd already specifies) crashed verus outright"
    );

    let proved_count = cases
        .iter()
        .filter(|case| case.expected() == VerusGalleryExpectation::Proved)
        .count();
    assert_eq!(
        proved_count, 2,
        "two findings (TryFromSliceError's phantom-lifetime-binder + match-ergonomics lessons; cross-file spec fn reuse) were fully resolved, not just a documented dead end"
    );

    let compile_error_count = cases
        .iter()
        .filter(|case| case.expected() == VerusGalleryExpectation::CompileError)
        .count();
    assert_eq!(
        compile_error_count, 1,
        "one finding (Cow::deref's lifetime elision ambiguity) failed as an ordinary Rust compile error, not a verus-specific one"
    );

    for case in &cases {
        assert!(
            !case.title().is_empty(),
            "gallery case {} must have a title",
            case.id()
        );
        assert!(
            !case.claim().is_empty(),
            "gallery case {} must have a claim",
            case.id()
        );
    }
}
