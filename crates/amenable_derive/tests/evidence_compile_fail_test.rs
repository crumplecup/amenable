//! `#[evidence(..)]` with any argument must fail to compile -- see
//! `evidence`'s own doc comment for why (a realistic mistake, given
//! `#[derive(Evidence)]`'s separate helper attribute shares this name
//! with real arguments).

#[test]
fn evidence_with_arguments_fails_to_compile() {
    amenable_core::init_tracing();
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/evidence_rejects_arguments.rs");
}
