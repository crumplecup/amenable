//! Confirms Step 0 of `docs/GAAP_LEDGER_PLAN.md`: the type-level
//! skeleton (payload types, the four typestate markers, and the five
//! contract types) is real and inspectable, even with no proofs behind
//! any of it yet. No `Sidecar`/`Establish`/`Exchange` wiring exists
//! until Step 1.

use amenable_core::{Evidence, Standard};
use amenable_gaap::{
    Account, AccountingEquationHolds, AccountsDistinct, Amount, AmountPositive, BalancedEntries,
    Committed, Pending, Rejected, SufficientFunds, TransferPayload, Validated,
};

#[test]
fn transfer_payload_carries_its_two_accounts_and_amount() {
    amenable_core::init_tracing();
    let payload = TransferPayload::new(
        Account::new(uuid::Uuid::from_u128(1), "Alice"),
        Account::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );

    assert_eq!(payload.from().name(), "Alice");
    assert_eq!(payload.to().name(), "Bob");
    assert_eq!(payload.amount().value(), 50);
}

#[test]
fn account_id_equality_compares_the_id_not_the_name() {
    amenable_core::init_tracing();
    let alice_again = Account::new(uuid::Uuid::from_u128(1), "Alice (checking)");
    let alice = Account::new(uuid::Uuid::from_u128(1), "Alice");
    let bob = Account::new(uuid::Uuid::from_u128(2), "Alice");

    assert_eq!(
        alice.id(),
        alice_again.id(),
        "same id, different name: still the same identity"
    );
    assert_ne!(
        alice.id(),
        bob.id(),
        "different id, same name: still distinct identities"
    );
}

#[test]
fn transfer_payload_is_a_root_evidence_node() {
    amenable_core::init_tracing();
    assert!(TransferPayload::is_root());
    assert_eq!(
        TransferPayload::chain(),
        vec![std::any::type_name::<TransferPayload>()]
    );
}

#[test]
fn every_transfer_state_is_a_root_standard_claim() {
    amenable_core::init_tracing();
    assert!(Pending::is_root());
    assert!(Validated::is_root());
    assert!(Committed::is_root());
    assert!(Rejected::<Pending>::is_root());
    assert!(Rejected::<Validated>::is_root());
}

#[test]
fn every_transfer_state_carries_an_asserted_provenance_report() {
    amenable_core::init_tracing();
    for report in [
        Pending.report().to_string(),
        Validated.report().to_string(),
        Committed.report().to_string(),
        Rejected::<Pending>::default().report().to_string(),
        Rejected::<Validated>::default().report().to_string(),
    ] {
        assert!(report.contains("asserted:"));
    }
}

#[test]
fn rejected_from_pending_and_rejected_from_validated_are_distinct_claims() {
    amenable_core::init_tracing();
    assert_ne!(
        Rejected::<Pending>::default().report().to_string(),
        Rejected::<Validated>::default().report().to_string(),
    );
}

#[test]
fn every_contract_type_is_a_root_evidence_node_with_no_proof_yet() {
    amenable_core::init_tracing();
    assert!(AmountPositive::is_root());
    assert!(SufficientFunds::is_root());
    assert!(AccountsDistinct::is_root());
    assert!(BalancedEntries::is_root());
    assert!(AccountingEquationHolds::is_root());
}
