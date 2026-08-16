//! Confirms Step 0 of `docs/GAAP_LEDGER_PLAN.md`: the type-level
//! skeleton (payload types, the four typestate markers, and the five
//! contract types) is real and inspectable, even with no proofs behind
//! any of it yet. No `Sidecar`/`Establish`/`Exchange` wiring exists
//! until Step 1.

use amenable_core::{Evidence, Standard};
use amenable_gaap::{
    AccountId, AccountingEquationHolds, AccountsDistinct, Amount, AmountPositive, BalancedEntries,
    Committed, Pending, Rejected, SufficientFunds, TransferPayload, Validated,
};

#[test]
fn transfer_payload_carries_its_two_accounts_and_amount() {
    let payload = TransferPayload::new(
        AccountId::new("Alice"),
        AccountId::new("Bob"),
        Amount::new(50),
    );

    assert_eq!(payload.from().name(), "Alice");
    assert_eq!(payload.to().name(), "Bob");
    assert_eq!(payload.amount().value(), 50);
}

#[test]
fn transfer_payload_is_a_root_evidence_node() {
    assert!(TransferPayload::is_root());
    assert_eq!(
        TransferPayload::chain(),
        vec![std::any::type_name::<TransferPayload>()]
    );
}

#[test]
fn every_transfer_state_is_a_root_standard_claim() {
    assert!(Pending::is_root());
    assert!(Validated::is_root());
    assert!(Committed::is_root());
    assert!(Rejected::is_root());
}

#[test]
fn every_transfer_state_carries_an_asserted_provenance_report() {
    for report in [
        Pending.report().to_string(),
        Validated.report().to_string(),
        Committed.report().to_string(),
        Rejected.report().to_string(),
    ] {
        assert!(report.contains("asserted:"));
    }
}

#[test]
fn every_contract_type_is_a_root_evidence_node_with_no_proof_yet() {
    assert!(AmountPositive::is_root());
    assert!(SufficientFunds::is_root());
    assert!(AccountsDistinct::is_root());
    assert!(BalancedEntries::is_root());
    assert!(AccountingEquationHolds::is_root());
}
