//! Isolates `GAAP_LEDGER_PLAN.md`'s Step 2 CBMC timeout:
//! `verify_commit_always_balances` (`ledger.rs`) times out under `just
//! verify-kani-contract`, even though `Ledger::commit` is infallible
//! (no branching at all -- always `Ok`), its postcondition doesn't
//! touch `AccountId`/`Uuid` comparison at all (only `payload.amount()`),
//! and it has no `#[kani::stub_verified]` composition (unlike
//! `validate`, `commit` calls no other contracted helper). None of
//! Step 1's already-diagnosed causes apply on their face, so this is a
//! genuinely new question, not a repeat.
//!
//! The one real structural difference from `validate`'s own (working)
//! harness: `commit`'s input (`Transfer<Validated, ValidatedToken>`)
//! isn't the harness's own direct argument type the way `validate`'s
//! `Transfer<Pending, PendingToken>` is built once and passed straight
//! in -- it's assembled through a real `Sidecar::sidecar`/`Establish::
//! establish`/`Transfer::new` chain inside the harness's own setup code.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_commit_contract_timeout::commit_contract_with_concrete_amounts".to_owned(),
            "gallery::ledger_commit_contract_timeout::commit_contract_with_concrete_amounts".to_owned(),
            "amenable_kani".to_owned(),
            "verify_commit_always_balances's exact real shape (Sidecar/Establish/Transfer::new construction chain in the harness setup), but with concrete amount/balance instead of kani::any()".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_WITH_CONCRETE_AMOUNTS_SRC, {
        /// Isolates whether the symbolic `amount`/`balance` is what's
        /// expensive here, or whether the cost is structural to the
        /// `#[kani::proof_for_contract(Ledger::commit)]` setup chain
        /// itself, independent of what's symbolic.
        #[kani::proof_for_contract(amenable_gaap::Ledger::commit)]
        fn commit_contract_with_concrete_amounts() {
            use amenable_core::{Establish, Sidecar};

            let ledger = amenable_gaap::Ledger::new(100);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(50),
            );
            let pending = amenable_gaap::Transfer::pending(payload.clone());
            let credential = pending.sidecar();
            let validated_token = <amenable_gaap::Validated as Establish<_, amenable_kani::KaniVerifier>>::establish(credential);
            let validated: amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken> =
                amenable_gaap::Transfer::diagnostic_new(payload, validated_token);
            let _ = ledger.commit::<amenable_kani::KaniVerifier>(validated);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_commit_contract_timeout::commit_contract_bypassing_establish_chain".to_owned(),
            "gallery::ledger_commit_contract_timeout::commit_contract_bypassing_establish_chain".to_owned(),
            "amenable_kani".to_owned(),
            "verify_commit_always_balances's real symbolic amount/balance, but the harness setup constructs ValidatedToken directly (ValidatedToken::diagnostic_only) instead of through the real Sidecar::sidecar/Establish::establish chain".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_BYPASSING_ESTABLISH_CHAIN_SRC, {
        /// `commit_contract_with_concrete_amounts` proved the setup
        /// chain (`Sidecar::sidecar`/`Establish::establish`/`Transfer::
        /// new`) is expensive (~143s) even fully concrete -- symbolic
        /// content isn't the driver. This tests whether the chain
        /// *itself* is the cost: same symbolic `amount`/`balance` as
        /// the real, timing-out harness, but `ValidatedToken` is
        /// constructed directly (`diagnostic_only`, bypassing `Sidecar`/
        /// `Establish` entirely) rather than through the lawful chain.
        /// `commit`'s own contract is unaffected either way -- DFCC
        /// only checks `commit`'s body/postcondition, not how the
        /// harness's own input was assembled.
        #[kani::proof_for_contract(amenable_gaap::Ledger::commit)]
        fn commit_contract_bypassing_establish_chain() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = amenable_gaap::Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let validated_token = amenable_gaap::ValidatedToken::diagnostic_only();
            let validated: amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken> =
                amenable_gaap::Transfer::diagnostic_new(payload, validated_token);
            let _ = ledger.commit::<amenable_kani::KaniVerifier>(validated);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_commit_contract_timeout::commit_contract_with_amount_assumed_positive".to_owned(),
            "gallery::ledger_commit_contract_timeout::commit_contract_with_amount_assumed_positive".to_owned(),
            "amenable_kani".to_owned(),
            "Same as commit_contract_bypassing_establish_chain, but kani::assume(amount > 0) -- commit's postcondition negates amount (debit = -amount), which overflows at i64::MIN; a real Transfer<Validated,..> could never carry a non-positive amount in the first place (validate already checked AmountPositive)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, COMMIT_CONTRACT_WITH_AMOUNT_ASSUMED_POSITIVE_SRC, {
        /// Isolates whether unconstrained `amount` (including
        /// `i64::MIN`, where `commit`'s own postcondition's `-amount`
        /// overflows) is the real driver, independent of the
        /// `Establish`/`Sidecar` construction chain (already bypassed
        /// here, same as `commit_contract_bypassing_establish_chain`).
        #[kani::proof_for_contract(amenable_gaap::Ledger::commit)]
        fn commit_contract_with_amount_assumed_positive() {
            let amount: i64 = kani::any();
            kani::assume(amount > 0);
            let balance: i64 = kani::any();
            let ledger = amenable_gaap::Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let validated_token = amenable_gaap::ValidatedToken::diagnostic_only();
            let validated: amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken> =
                amenable_gaap::Transfer::diagnostic_new(payload, validated_token);
            let _ = ledger.commit::<amenable_kani::KaniVerifier>(validated);
        }
    }
}
