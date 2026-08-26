//! Coverage for `KaniProofRegistration`'s own self-registration health
//! (stable fully-qualified IDs, uniqueness), plus a real incident:
//! proof-gallery cases (written through `amenable_derive::harness!`,
//! same as tracked production proofs) used to also silently self-
//! register into `KaniProofRegistration`, so `amenable verify kani`'s
//! full sweep -- and `just verify-kani` with no harness argument -- ran
//! every gallery case too, many with deliberately expected `timeout`/
//! `failed` outcomes. Gallery cases must go through `amenable_derive::
//! gallery_harness!` instead, which never emits that registration.

use amenable::{KaniProof, KaniProofRegistration};

#[test]
fn kani_harnesses_self_register_with_stable_fully_qualified_ids() {
    let records: Vec<KaniProof> = inventory::iter::<KaniProofRegistration>()
        .map(|registration| (registration.proof())())
        .collect();

    assert!(
        records.len() > 200,
        "all Kani harnesses should be registered"
    );
    assert!(records.iter().any(|record| {
        record.id() == "amenable_kani::calculator::verify_debit_access_preserves_value"
            && record.harness() == "calculator::verify_debit_access_preserves_value"
            && record.package() == "amenable_kani"
    }));
    assert!(records.iter().any(|record| {
        record.id()
            == "amenable_kani::rust_std::array::verify_try_from_slice_rejects_a_length_mismatch"
            && record.harness()
                == "rust_std::array::verify_try_from_slice_rejects_a_length_mismatch"
            && record.package() == "amenable_kani"
    }));

    let mut ids: Vec<_> = records.iter().map(|record| record.id().as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), records.len(), "proof identifiers must be unique");
}

#[test]
fn tracked_proof_registry_never_includes_a_gallery_case() {
    let tracked: Vec<KaniProof> = inventory::iter::<KaniProofRegistration>()
        .map(|registration| (registration.proof())())
        .collect();

    assert!(
        !tracked.is_empty(),
        "the tracked proof registry should not be empty"
    );
    let gallery_leaks: Vec<_> = tracked
        .iter()
        .filter(|proof| proof.id().contains("::gallery::"))
        .map(|proof| proof.id().as_str())
        .collect();
    assert!(
        gallery_leaks.is_empty(),
        "gallery cases must never appear in the tracked proof registry \
         `amenable verify kani`'s full sweep iterates -- found: {gallery_leaks:?}"
    );
}
