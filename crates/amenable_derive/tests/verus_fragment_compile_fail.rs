#![cfg(feature = "verus")]

//! A missing harness must fail `cargo build` outright, never reach a
//! caller at runtime -- see `verus_fragment`'s own doc comment.

#[test]
fn missing_harness_fails_to_compile() {
    amenable_core::init_tracing();
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/verus_ensures_fragments_missing_harness.rs");
}
