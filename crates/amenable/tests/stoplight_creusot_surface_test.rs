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

#![cfg(feature = "creusot")]

use amenable_core::Amenable;
use amenable_kani::Stoplight;

#[test]
fn creusot_surface_reports_all_three_stoplight_edges_when_both_crates_are_linked() {
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
