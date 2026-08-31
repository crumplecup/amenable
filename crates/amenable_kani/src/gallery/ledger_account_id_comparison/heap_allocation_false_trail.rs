::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::symbolic_branch_constructing_real_transfer_validated_passes".to_owned(),
            "gallery::ledger_account_id_comparison::symbolic_branch_constructing_real_transfer_validated_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Symbolic branch whose Ok arm constructs the real Transfer<Validated, ValidatedToken> wrapper (pulling in the real, #[amenable_derive::exchange]-generated Witness<KaniVerifier> for Validated)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SYMBOLIC_BRANCH_CONSTRUCTING_REAL_TRANSFER_VALIDATED_PASSES_SRC, {
        /// `bare_result_transfer_payload_passes` proved a bare
        /// `Result<TransferPayload, i64>` under a symbolic branch is
        /// cheap -- no `Sidecar`/`Transfer<S, Token>` involved at all.
        /// `symbolic_branch_with_track_caller_and_no_string_passes`
        /// proved `Establish::establish` under a symbolic branch is
        /// cheap too. This combines them into the one piece neither
        /// covered: constructing the actual `Transfer<Validated,
        /// ValidatedToken>` wrapper -- which brings in the *real*,
        /// `#[amenable_derive::exchange]`-generated `Witness<
        /// KaniVerifier> for Validated` impl (`CalculationProof`,
        /// carrying the captured harness source text as a `&'static
        /// str`) via `Sidecar`'s own `Proposition: Witness<V>` bound --
        /// under a symbolic branch. If this alone times out, the real
        /// `Witness<KaniVerifier>`/`CalculationProof` machinery
        /// reachable through `Sidecar`'s bound is the actual culprit,
        /// not `Transfer<S, Token>`'s own shape.
        #[kani::proof]
        fn symbolic_branch_constructing_real_transfer_validated_passes() {
            use amenable_core::Sidecar;

            let pending_payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(1),
            );
            let pending = amenable_gaap::Transfer::pending(pending_payload);
            let credential = pending.sidecar();

            let amount: i64 = kani::any();
            if amount > 0 {
                let payload = amenable_gaap::TransferPayload::new(
                    amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                    amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                    amenable_gaap::Amount::new(amount),
                );
                let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(credential);
                let validated: amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken> =
                    amenable_gaap::Transfer::diagnostic_new(payload, token);
                let _ = validated;
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::returning_real_result_type_from_a_function_passes".to_owned(),
            "gallery::ledger_account_id_comparison::returning_real_result_type_from_a_function_passes".to_owned(),
            "amenable_kani".to_owned(),
            "fn(i64) -> Result<Transfer<Validated, ValidatedToken>, TransferError> (validate's exact real types), symbolic amount, result discarded by caller".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, RETURNING_REAL_RESULT_TYPE_FROM_A_FUNCTION_PASSES_SRC, {
        /// `symbolic_branch_constructing_real_transfer_validated_passes` proved
        /// constructing `Transfer<Validated, ValidatedToken>` inline,
        /// under a symbolic branch, with the value never RETURNED from
        /// a function (just built and dropped in the harness's own
        /// scope), is cheap. This tests the one remaining structural
        /// difference from the real, failing `validate`: does the SAME
        /// construction, but *returned* across a function boundary as
        /// `Result<Transfer<Validated, ValidatedToken>, TransferError>`
        /// (`validate`'s exact real return type, 3-variant error
        /// included), cost something that inline construction doesn't?
        #[kani::proof]
        fn returning_real_result_type_from_a_function_passes() {
            use amenable_core::Sidecar;

            fn build(
                credential: amenable_gaap::PendingToken,
                amount: i64,
            ) -> Result<
                amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                amenable_gaap::TransferError,
            > {
                if amount <= 0 {
                    return Err(amenable_gaap::TransferError::NegativeAmount(amount));
                }
                let payload = amenable_gaap::TransferPayload::new(
                    amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                    amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                    amenable_gaap::Amount::new(amount),
                );
                let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(credential);
                Ok(amenable_gaap::Transfer::diagnostic_new(payload, token))
            }

            let pending_payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(1),
            );
            let pending = amenable_gaap::Transfer::pending(pending_payload);
            let credential = pending.sidecar();

            let amount: i64 = kani::any();
            let _ = build(credential, amount);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::symbolic_pending_transfer_extraction_passes".to_owned(),
            "gallery::ledger_account_id_comparison::symbolic_pending_transfer_extraction_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Transfer::pending built from a payload with a symbolic amount, then .primary()/.sidecar() called on it -- no branching downstream at all".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SYMBOLIC_PENDING_TRANSFER_EXTRACTION_PASSES_SRC, {
        /// Every prior probe passed a *plain* symbolic `i64` directly
        /// into a function or inline block. The real `validate` instead
        /// receives a `Transfer<Pending, PendingToken>` whose own
        /// `TransferPayload` was built with a symbolic `amount` (via
        /// `Transfer::pending`), then calls `.primary()`/`.sidecar()`
        /// on *that* -- the one remaining structural difference. This
        /// isolates just that: build `Transfer::pending` from a
        /// symbolic payload, extract from it, nothing else.
        #[kani::proof]
        fn symbolic_pending_transfer_extraction_passes() {
            use amenable_core::Sidecar;

            let amount: i64 = kani::any();
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = amenable_gaap::Transfer::pending(payload);
            let extracted = pending.primary().clone();
            let credential = pending.sidecar();
            let _ = (extracted, credential);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::full_combination_inline_without_calling_validate".to_owned(),
            "gallery::ledger_account_id_comparison::full_combination_inline_without_calling_validate".to_owned(),
            "amenable_kani".to_owned(),
            "Every piece that passed individually (symbolic Transfer::pending extraction, amount-positive check, sufficient-funds check against self.balance, accounts-distinct check, establish, Transfer::new), assembled inline -- not calling Ledger::validate/Exchange::exchange at all".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, FULL_COMBINATION_INLINE_WITHOUT_CALLING_VALIDATE_SRC, {
        /// Every individual piece of `Ledger::validate` has now passed
        /// in isolation. This assembles all of them -- symbolic
        /// `Transfer::pending` extraction, the amount-positive check,
        /// the sufficient-funds check against `self.balance`, the
        /// accounts-distinct check, `establish`, `Transfer::new` -- into
        /// one function with the *exact* real control flow, but written
        /// directly here rather than calling `Ledger::validate`/
        /// `Exchange::exchange`. If this passes, the timeout is
        /// specific to going through those actual functions (dispatch,
        /// or something `#[amenable_derive::exchange]`'s generated code
        /// does even without DFCC contract-checking active); if it
        /// still times out, it really is the sheer combination of
        /// individually-cheap pieces.
        #[kani::proof]
        fn full_combination_inline_without_calling_validate() {
            use amenable_core::Sidecar;

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();

            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = amenable_gaap::Transfer::pending(payload);
            let payload = pending.primary().clone();
            let amount = payload.amount().value();

            if amount <= 0 {
                return;
            }
            if balance < amount {
                return;
            }
            if payload.from() == payload.to() {
                return;
            }

            let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(pending.sidecar());
            let validated: amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken> =
                amenable_gaap::Transfer::diagnostic_new(payload, token);
            let _ = validated;
        }
    }
}
