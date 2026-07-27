use amenable_kani::{KaniUtf8, KaniUtf8Buffer, KaniUtf8BufferError};

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

#[test]
fn utf8_buffer_accepts_valid_content_and_reports_length() {
    let buffer = KaniUtf8Buffer::<4>::new([b'h', b'i', 0, 0], 2).expect("ascii is valid utf8");

    assert_eq!(buffer.as_bytes(), b"hi");
    assert_eq!(buffer.len(), 2);
    assert!(!buffer.is_empty());
}

#[test]
fn utf8_buffer_rejects_invalid_content() {
    let bytes = [b'x', 0xFFu8, 0, 0];

    assert_eq!(
        KaniUtf8Buffer::<4>::new(bytes, 2),
        Err(KaniUtf8BufferError::InvalidUtf8)
    );
}

#[test]
fn utf8_buffer_rejects_a_length_over_capacity() {
    assert_eq!(
        KaniUtf8Buffer::<2>::new(*b"ab", 3),
        Err(KaniUtf8BufferError::TooLong)
    );
}

#[test]
fn utf8_buffer_empty_length_is_valid_and_empty() {
    let buffer = KaniUtf8Buffer::<4>::new([0, 0, 0, 0], 0).expect("empty content is valid utf8");

    assert!(buffer.is_empty());
    assert_eq!(buffer.as_bytes(), b"");
}
