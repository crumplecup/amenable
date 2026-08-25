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
