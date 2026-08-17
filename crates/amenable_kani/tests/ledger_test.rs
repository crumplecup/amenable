//! Exercises `GAAP_LEDGER_PLAN.md`'s Step 1 `Exchange` edge end to end,
//! through the real `Ledger::validate` transition rather than just the
//! Kani harnesses. All three refusal reasons -- unlike every `Stoplight`
//! edge, which can only ever succeed.

use amenable_core::{Exchange, Sidecar};
use amenable_gaap::{AccountId, Amount, TransferPayload};
use amenable_kani::{Ledger, Transfer, TransferError};

#[test]
fn validate_accepts_a_lawful_transfer() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let validated = ledger.exchange(input).expect("lawful transfer");
    assert_eq!(validated.primary().amount().value(), 50);
}

#[test]
fn validate_rejects_a_negative_amount() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(-1),
    );
    let input = Transfer::pending(payload);

    let error = ledger.exchange(input).expect_err("negative amount");
    assert_eq!(error, TransferError::NegativeAmount(-1));
}

#[test]
fn validate_rejects_insufficient_funds() {
    let ledger = Ledger::new(10);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let error = ledger.exchange(input).expect_err("insufficient funds");
    assert_eq!(
        error,
        TransferError::InsufficientFunds {
            balance: 10,
            required: 50
        }
    );
}

#[test]
fn validate_rejects_the_same_account() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let error = ledger.exchange(input).expect_err("same account");
    assert_eq!(error, TransferError::SameAccount);
}

#[test]
fn commit_always_succeeds_and_carries_the_same_amount() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let validated = ledger.exchange(input).expect("lawful transfer");
    let committed = ledger.exchange(validated).expect("commit never fails");
    assert_eq!(committed.primary().amount().value(), 50);
}
