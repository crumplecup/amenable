//! Exercises `GAAP_LEDGER_PLAN.md`'s Step 1 `Exchange` edge end to end,
//! through the real `Ledger::validate` transition rather than just the
//! Kani harnesses. All three refusal reasons -- unlike every `Stoplight`
//! edge, which can only ever succeed.

use amenable_core::{ContractRecord, ExchangeEdgeRecord, Requires, Sidecar, StateMachine};
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

/// `Validated`'s real `Requires<KaniVerifier>` impl -- `commit`'s own
/// precondition, sewn to `validate`'s postcondition (`docs/
/// STATE_MACHINE_DERIVATION_PLAN.md`'s Step 3) rather than an
/// independently hand-typed copy. Exercised directly, not just through
/// `commit`'s own DFCC contract (which only Kani itself checks).
#[test]
fn validated_requires_holds_for_a_lawfully_validated_transfer() {
    let ledger = Ledger::new(100);
    let payload = TransferPayload::new(
        AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
        AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
        Amount::new(50),
    );
    let validated: Transfer<Validated, ValidatedToken> = ledger
        .validate::<KaniVerifier>(Transfer::pending(payload))
        .expect("lawful transfer");

    assert!(<Validated as Requires<KaniVerifier>>::requires(validated));
}

/// The real "sewing together" this step is about: `validate`'s
/// postcondition and `commit`'s precondition are two distinct registered
/// `ContractRecord`s (different `kind`, different edge), but both
/// fragments name the identical atomic claim, `AmountPositive` -- not
/// two independently hand-typed restatements of "amount is positive"
/// with nothing enforcing they agree.
#[test]
fn validate_ensures_and_commit_requires_are_sewn_to_the_same_atomic_claim() {
    let records: Vec<&ContractRecord> = inventory::iter::<ContractRecord>()
        .filter(|record| {
            record.verifier == "kani"
                && (record.evidence == "amenable_kani::ledger::Validated::validate_ensures"
                    || record.evidence == "amenable_kani::ledger::Validated::commit_requires")
        })
        .collect();

    assert_eq!(
        records.len(),
        2,
        "expected exactly one ensures and one requires record"
    );

    let ensures = records
        .iter()
        .find(|record| record.kind == "ensures")
        .expect("validate_ensures record missing");
    let requires = records
        .iter()
        .find(|record| record.kind == "requires")
        .expect("commit_requires record missing");

    assert!((ensures.fragment)().contains("AmountPositive"));
    assert!((requires.fragment)().contains("AmountPositive"));
}

/// `Ledger`'s real `impl<V: Verifier> StateMachine<V> for Ledger` --
/// `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s Step 5, unblocked once
/// `Ledger`'s methods gained real, generic-over-`V` `Exchange` impls
/// (Step 5's own follow-up correction). Callable against any real
/// verifier; `KaniVerifier` here is just this crate's own concrete
/// instantiation, not special.
#[test]
fn ledger_states_reports_all_five_declared_states_in_declaration_order() {
    assert_eq!(
        <Ledger as StateMachine<KaniVerifier>>::states(),
        &[
            "Pending",
            "Validated",
            "Committed",
            "Rejected<Pending>",
            "Rejected<Validated>"
        ]
    );
}

#[test]
fn ledger_transitions_reports_all_four_declared_edges_in_declaration_order() {
    let transitions = <Ledger as StateMachine<KaniVerifier>>::transitions();
    let pairs: Vec<(&str, &str)> = transitions
        .iter()
        .map(|transition| (transition.from, transition.to))
        .collect();

    assert_eq!(
        pairs,
        vec![
            ("Pending", "Validated"),
            ("Validated", "Committed"),
            ("Pending", "Rejected<Pending>"),
            ("Validated", "Rejected<Validated>"),
        ]
    );
}

/// The same declared-vs-registered cross-check `stoplight_amenable_test.rs`
/// runs for `Stoplight` -- proof this generalizes past a single-module
/// worked example, since `Ledger`'s real edges are registered from
/// `amenable_gaap::ledger.rs` (`capture_exchange_body`), not the same
/// module `Ledger` itself is declared in.
#[test]
fn ledger_declared_transitions_match_real_exchange_edge_registrations_exactly() {
    // `ExchangeEdgeRecord::evidence` is `stringify!(#evidence)` over a
    // `quote!`-respliced `Path`, which -- unlike `stringify!` on
    // directly-written source -- inserts spaces around a generic path's
    // `<`/`>` (`"Rejected < Pending >"`, not `"Rejected<Pending>"`). Both
    // name the identical real type; whitespace is the only difference,
    // so normalizing it away is a real equality check, not a fuzzy one.
    let normalize = |name: &str| {
        name.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    };

    let mut declared: Vec<String> = <Ledger as StateMachine<KaniVerifier>>::transitions()
        .iter()
        .map(|transition| normalize(transition.to))
        .collect();
    declared.sort_unstable();

    let mut registered: Vec<String> = inventory::iter::<ExchangeEdgeRecord>()
        .filter(|record| record.self_ty == "Ledger")
        .map(|record| normalize(record.evidence))
        .collect();
    registered.sort_unstable();

    assert_eq!(
        declared, registered,
        "declared Ledger edges and real ExchangeEdgeRecord registrations diverged"
    );
}
