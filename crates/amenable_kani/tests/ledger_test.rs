//! Exercises `GAAP_LEDGER_PLAN.md`'s Step 1 `Exchange` edge end to end,
//! through the real `Ledger::validate` transition rather than just the
//! Kani harnesses. All three refusal reasons -- unlike every `Stoplight`
//! edge, which can only ever succeed.

use amenable_core::Sidecar;
use amenable_gaap::{
    AccountId, Amount, Committed, CommittedToken, Ledger, Pending, Rejected,
    RejectedFromPendingToken, RejectedFromValidatedToken, Transfer, TransferError, TransferPayload,
    Validated, ValidatedToken,
};
use amenable_kani::KaniVerifier;

#[test]
fn validate_accepts_a_lawful_transfer() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let validated: Transfer<Validated, ValidatedToken> = ledger
        .validate::<KaniVerifier>(input)
        .expect("lawful transfer");
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

    let result: Result<Transfer<Validated, ValidatedToken>, TransferError> =
        ledger.validate::<KaniVerifier>(input);
    let error = result.expect_err("negative amount");
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

    let result: Result<Transfer<Validated, ValidatedToken>, TransferError> =
        ledger.validate::<KaniVerifier>(input);
    let error = result.expect_err("insufficient funds");
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

    let result: Result<Transfer<Validated, ValidatedToken>, TransferError> =
        ledger.validate::<KaniVerifier>(input);
    let error = result.expect_err("same account");
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

    let validated: Transfer<Validated, ValidatedToken> = ledger
        .validate::<KaniVerifier>(input)
        .expect("lawful transfer");
    let committed: Transfer<Committed, CommittedToken> = ledger
        .commit::<KaniVerifier>(validated)
        .expect("commit never fails");
    assert_eq!(committed.primary().amount().value(), 50);
}

#[test]
fn reject_always_succeeds_and_preserves_the_payload() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let rejected: Transfer<Rejected<Pending>, RejectedFromPendingToken> = ledger
        .reject::<KaniVerifier>(input)
        .expect("reject never fails");
    assert_eq!(rejected.primary().amount().value(), 50);
}

#[test]
fn rollback_always_succeeds_and_preserves_the_payload() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let input = Transfer::pending(payload);

    let validated: Transfer<Validated, ValidatedToken> = ledger
        .validate::<KaniVerifier>(input)
        .expect("lawful transfer");
    let rolled_back: Transfer<Rejected<Validated>, RejectedFromValidatedToken> = ledger
        .rollback::<KaniVerifier>(validated)
        .expect("rollback never fails");
    assert_eq!(rolled_back.primary().amount().value(), 50);
}
