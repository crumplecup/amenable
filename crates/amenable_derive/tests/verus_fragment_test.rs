#![cfg(feature = "verus")]

#[test]
fn verus_ensures_fragments_extracts_the_real_char_roundtrip_clauses() {
    amenable_core::init_tracing();
    let ensures: &[&str] = amenable_derive::verus_ensures_fragments!("verify_char_roundtrip");

    assert_eq!(
        ensures,
        [
            "char_roundtrip_preserves_value(result, c)",
            "char_is_valid_unicode_scalar(c)",
        ]
    );
}

#[test]
fn verus_requires_fragments_extracts_the_real_escape_ascii_clause() {
    amenable_core::init_tracing();
    let requires: &[&str] = amenable_derive::verus_requires_fragments!(
        "verify_escape_ascii_model_leaves_printable_bytes_unescaped"
    );

    assert_eq!(
        requires,
        ["escape_ascii_input_is_printable_ascii(printable)"]
    );
}
