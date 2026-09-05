//! Real Kani harnesses for `amenable_gaap::Ledger`'s own methods --
//! `GAAP_LEDGER_PLAN.md`'s Step 7. `amenable_kani::ledger`'s own
//! duplicate `Ledger`/`Transfer`/`TransferError` are retired (see that
//! module's own doc comment): these are the real, final harnesses for
//! the one real `Ledger` now, not a transitional candidate. See
//! `amenable_gaap::ledger`'s own doc comment and
//! `gallery::ledger_gaap_free_function_contract` for the confirmed
//! "direct contract, no delegating wrapper" pattern every harness below
//! relies on.
//!
//! Inputs route through `crate::ledger`'s `KaniCompose` impls
//! (`docs/STATE_MACHINE_DERIVATION_PLAN.md`'s "Reusing `KaniCompose`
//! for non-trivial carriers" follow-on) rather than the two fixed
//! `Account`s ("Alice"/"Bob") and ad hoc `kani::any()` calls this file
//! used to hand-construct: every `Account`/`Amount`/`TransferPayload`
//! below is now genuinely bounded-symbolic, exploring real varying
//! identities instead of only ever exercising the same two literal
//! accounts. Each harness still controls exactly what it needs to
//! control -- `commit`'s own `amount > 0` assumption stays explicit
//! (real, structural CBMC cost reasons, see its own comment below), and
//! `validate`'s two harnesses still assume/force the specific accounts-
//! distinct/accounts-same relationship each one is named for -- routing
//! through `KaniCompose` changes *what* gets constructed, not *which*
//! properties each harness deliberately holds fixed.

#[cfg(kani)]
use amenable_gaap::{Account, Amount, Ledger, Transfer, TransferPayload, Validated};

#[cfg(kani)]
use crate::{KaniCompose, KaniVerifier};

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
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let from = Account::kani_any();
            let to = Account::kani_any();
            kani::assume(from.id() != to.id());
            let payload = TransferPayload::new(from, to, Amount::kani_any());
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
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let account = Account::kani_any();
            let payload =
                TransferPayload::new(account.clone(), account, Amount::kani_any());
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
                Account::kani_any(),
                Account::kani_any(),
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
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let pending = Transfer::<amenable_gaap::Pending, amenable_gaap::PendingToken>::kani_any();
            let _ = ledger.reject::<KaniVerifier>(pending);
        }
    }
}

amenable_derive::harness! {
    kani, VERIFY_GAAP_ROLLBACK_ALWAYS_SUCCEEDS_SRC, {
        #[kani::proof_for_contract(amenable_gaap::Ledger::rollback)]
        fn verify_gaap_rollback_always_succeeds() {
            let balance: i64 = kani::any();
            let ledger = Ledger::new(balance);
            let validated = Transfer::<Validated, amenable_gaap::ValidatedToken>::kani_any();
            let _ = ledger.rollback::<KaniVerifier>(validated);
        }
    }
}
