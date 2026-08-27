//! Locks in `ClassifiedWitness`'s compile-time enforcement: an `Opaque`
//! leaf (a `Witness` impl that never overrode `support()`) must fail
//! `cargo check` with a real trait-resolution error when composed into
//! `register_witness_exports!`, not merely be caught by a runtime
//! `Result::Err` or a `const`-eval panic. Verified once by hand during
//! this mechanism's original design (see
//! `docs/VERUS_DERIVE_WITNESS_COMPOSITION_PLAN.md`, Design A) but never
//! locked in as a permanent regression test until now.

#[test]
fn opaque_leaf_fails_to_compile_for_export() {
    amenable_core::init_tracing();
    let cases = trybuild::TestCases::new();
    cases.compile_fail("tests/ui/opaque_leaf_blocks_export.rs");
}
