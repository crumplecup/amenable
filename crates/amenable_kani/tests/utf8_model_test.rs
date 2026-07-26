use amenable_kani::KaniUtf8;

#[test]
fn classify_owned_accepts_small_valid_utf8() {
    let valid = KaniUtf8::classify_owned("hi".as_bytes().to_vec()).expect("ascii is valid utf8");

    assert_eq!(valid.as_bytes(), b"hi");
    assert_eq!(valid.as_str(), "hi");
    assert_eq!(valid.len(), 2);
    assert!(!valid.is_empty());
}

#[test]
fn classify_owned_rejects_invalid_utf8_and_recovers_exact_bytes() {
    let bytes = vec![b'x', 0xFFu8];
    let err = KaniUtf8::classify_owned(bytes.clone()).unwrap_err();

    assert_eq!(err.as_bytes(), &bytes[..]);
    assert_eq!(err.into_bytes(), bytes);
}

#[test]
fn compose_representatives_stay_on_the_expected_sides_of_the_boundary() {
    let valid = KaniUtf8::classify_owned(b"xy".to_vec()).expect("ascii is valid utf8");
    let invalid = vec![b'x', b'y', 0xFFu8];

    assert!(KaniUtf8::is_valid(valid.as_bytes()));
    assert!(!KaniUtf8::is_valid(&invalid));
}
