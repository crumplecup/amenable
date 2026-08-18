//! Real Kani harnesses for `amenable_gaap::Ledger`'s own methods --
//! `GAAP_LEDGER_PLAN.md`'s Step 9. Not `amenable_kani::ledger`'s own
//! (still-separate, still-production) `Ledger`: this verifies the real
//! `amenable_gaap::Ledger` candidate that will eventually replace it,
//! once every method has moved and every harness/caller here has been
//! repointed. See `amenable_gaap::ledger`'s own doc comment and
//! `gallery::ledger_gaap_free_function_contract` for the confirmed
//! "direct contract, no delegating wrapper" pattern every harness below
//! relies on.

#[cfg(kani)]
use amenable_gaap::{AccountId, Amount, Ledger, Transfer, TransferPayload, Validated};

#[cfg(kani)]
use crate::KaniVerifier;

amenable_derive::harness! {
    kani, VERIFY_GAAP_CHECK_AMOUNT_POSITIVE_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::check_amount_positive)]
        fn verify_gaap_check_amount_positive() {
            let amount: i64 = kani::any();
            let _ = Ledger::check_amount_positive::<KaniVerifier>(amount);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_CHECK_SUFFICIENT_FUNDS_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::check_sufficient_funds)]
        fn verify_gaap_check_sufficient_funds() {
            let balance: i64 = kani::any();
            let amount: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let _ = ledger.check_sufficient_funds::<KaniVerifier>(amount);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_VALIDATE_ACCEPTS_A_LAWFUL_TRANSFER_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::validate)]
        #[kani::stub_verified(amenable_gaap::Ledger::check_amount_positive)]
        #[kani::stub_verified(amenable_gaap::Ledger::check_sufficient_funds)]
        fn verify_gaap_validate_accepts_a_lawful_transfer() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                Amount::new(amount),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _ = ledger.validate::<KaniVerifier>(input);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_VALIDATE_REJECTS_THE_SAME_ACCOUNT_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::validate)]
        #[kani::stub_verified(amenable_gaap::Ledger::check_amount_positive)]
        #[kani::stub_verified(amenable_gaap::Ledger::check_sufficient_funds)]
        fn verify_gaap_validate_rejects_the_same_account() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                Amount::new(amount),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _ = ledger.validate::<KaniVerifier>(input);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_COMMIT_ALWAYS_BALANCES_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::commit)]
        fn verify_gaap_commit_always_balances() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token =
                <Validated as amenable_core::Establish<_, KaniVerifier>>::establish(credential);
            let validated: Transfer<Validated, amenable_gaap::ValidatedToken> =
                Transfer::diagnostic_new(payload, validated_token);
            let _ = ledger.commit::<KaniVerifier>(validated);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_REJECT_ALWAYS_SUCCEEDS_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::reject)]
        fn verify_gaap_reject_always_succeeds() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                Amount::new(amount),
            );
            let pending = Transfer::pending(payload);
            let _ = ledger.reject::<KaniVerifier>(pending);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_ROLLBACK_ALWAYS_SUCCEEDS_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::rollback)]
        fn verify_gaap_rollback_always_succeeds() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let payload = TransferPayload::new(
                AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                Amount::new(amount),
            );
            let pending = Transfer::pending(payload.clone());
            let credential = amenable_core::Sidecar::sidecar(&pending);
            let validated_token =
                <Validated as amenable_core::Establish<_, KaniVerifier>>::establish(credential);
            let validated: Transfer<Validated, amenable_gaap::ValidatedToken> =
                Transfer::diagnostic_new(payload, validated_token);
            let _ = ledger.rollback::<KaniVerifier>(validated);
        }
    }
}
