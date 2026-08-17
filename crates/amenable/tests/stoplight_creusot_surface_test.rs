//! Confirms `Stoplight::creusot_surface()` returns real data when
//! `amenable_kani` and `amenable_creusot` are both linked into the same
//! binary -- the real point of Step 8 (`EXCHANGE_PROOF_DERIVATION_PLAN.md`):
//! `amenable_kani` has no Cargo dependency on `amenable_creusot` at all
//! (verifier backend crates never depend on each other); `amenable_creusot::
//! stoplight` registers its own `ProofRecord`s, `#[cfg(not(creusot))]`-gated,
//! and `creusot_surface()` queries the shared `amenable_core` registry --
//! `inventory` only needs both crates linked into the same final binary, not
//! a direct edge between them. `amenable`'s own `creusot` feature links both,
//! so this is the one binary in the tree that can assert the non-empty case;
//! `amenable_kani`'s own test binary (`stoplight_amenable_test.rs`) asserts
//! the honestly-empty case, since it never links `amenable_creusot` at all.
//!
//! **A real, non-obvious linking requirement, found the hard way.** Having
//! `amenable_creusot` as a Cargo dependency (even a real, non-optional one)
//! is not enough for its `inventory::submit!` registrations to actually
//! reach `inventory::iter()` here -- `cargo test`'s linker only pulls in
//! the *specific compiled object files* this binary's own call graph
//! actually reaches, not a whole rlib just because it's nominally a
//! dependency. Confirmed directly: with nothing in this file referencing
//! `amenable_creusot` at all (only `amenable_core`/`amenable_kani`), the
//! query below returned an empty `Vec`, and the *same* real, pre-existing
//! gap reproduced even for `amenable_kani`'s own registrations queried
//! from a fresh, isolated test binary that never called any `amenable_
//! kani` function directly. A single real call into *any* part of the
//! target crate (`force_creusot_linking()` below) was enough to pull in
//! its *entire* compiled rlib, registrations included -- not just the one
//! function actually called. This is a property of how `cargo test`
//! links each integration-test binary, unrelated to Creusot specifically;
//! it just happened to surface here first.
#![cfg(feature = "creusot")]

use amenable_core::Amenable;
use amenable_kani::Stoplight;

/// Forces the linker to retain `amenable_creusot`'s compiled object code
/// (registrations included) in this test binary -- see this file's own
/// doc comment for why a Cargo dependency alone doesn't guarantee that.
/// The specific symbol touched doesn't matter; what matters is that
/// *something* from the crate is genuinely referenced.
fn force_creusot_linking() {
    let _ = amenable_creusot::CreusotVerifierMetadata;
}

#[test]
fn creusot_surface_reports_all_three_stoplight_edges_when_both_crates_are_linked() {
    force_creusot_linking();
    let surface = Stoplight::creusot_surface();

    assert_eq!(
        surface,
        vec![
            "amenable_creusot::stoplight::green_to_yellow".to_owned(),
            "amenable_creusot::stoplight::red_to_green".to_owned(),
            "amenable_creusot::stoplight::yellow_to_red".to_owned(),
        ]
    );
}
