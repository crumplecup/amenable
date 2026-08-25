use amenable_core::{Establish, Evidence, EvidenceLink, ProofRecord, Witness};
use amenable_kani::{AddEvidence, AddToken, Credit, Debit, KaniVerifier, Sum, add};

#[test]
fn add_calculation_forms_an_evidence_chain() {
    let evidence = add::<KaniVerifier>(Debit::new(12), Credit::new(5));

    assert_eq!(evidence.audit(), Sum::new(17));
    assert_eq!(
        <AddEvidence as Evidence>::basis(),
        (Debit::new(0), Credit::new(0))
    );

    let _token: AddToken = AddEvidence::establish(evidence);
}

#[test]
fn add_witness_names_its_kani_proof() {
    let proof = <AddEvidence as Witness<KaniVerifier>>::proof();
    assert_eq!(proof.harness, "add_impl_computes_exact_sum");
    assert!(proof.claim.contains("fn add_impl_computes_exact_sum"));
}

#[test]
fn add_evidence_chain_walks_back_through_the_tuple_to_std_i64() {
    let chain = AddEvidence::chain();

    // AddEvidence -> (Debit, Credit) -> (RustStdStandard<i64>, RustStdStandard<i64>);
    // Debit/Credit are thin i64 wrappers, not roots themselves, so the
    // static type-level chain goes one hop further than the tuple.
    assert_eq!(chain.len(), 3);
    assert!(chain[0].ends_with("AddEvidence"));
    assert!(chain[1].contains("Debit"));
    assert!(chain[1].contains("Credit"));
    assert!(chain[2].contains("RustStdStandard<i64>"));
}

#[test]
fn add_evidence_links_fan_out_one_per_argument() {
    let links: Vec<_> = inventory::iter::<EvidenceLink>()
        .filter(|link| link.name().ends_with("AddEvidence"))
        .collect();

    // Two arguments (Debit, Credit) means two separate links sharing the
    // same name, not one link naming a composite tuple basis.
    assert_eq!(links.len(), 2);
    assert!(links.iter().any(|link| link.basis().ends_with("Debit")));
    assert!(links.iter().any(|link| link.basis().ends_with("Credit")));
}

#[test]
fn add_proof_record_is_discoverable_through_inventory() -> miette::Result<()> {
    let record = inventory::iter::<ProofRecord>()
        .into_iter()
        .find(|record| record.evidence().ends_with("AddEvidence") && record.verifier() == "kani")
        .ok_or_else(|| miette::miette!("AddEvidence's kani proof record is registered"))?;

    assert!((record.describe())().contains("add_impl_computes_exact_sum"));
    Ok(())
}
