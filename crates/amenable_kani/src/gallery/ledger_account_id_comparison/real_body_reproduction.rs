::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::three_distinct_error_variants_from_one_function_passes".to_owned(),
            "gallery::ledger_account_id_comparison::real_body_reproduction::three_distinct_error_variants_from_one_function_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Same combination as full_combination_inline_without_calling_validate, but each check path actually constructs and returns its real TransferError variant (not a bare early return) wrapped in Result<Transfer<Validated,..>, TransferError>".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, THREE_DISTINCT_ERROR_VARIANTS_FROM_ONE_FUNCTION_PASSES_SRC, {
        /// `full_combination_inline_without_calling_validate` passed,
        /// but its "error paths" were bare `return;` -- no
        /// `TransferError` value was ever constructed or wrapped in a
        /// `Result`. The real `validate` constructs one of *three*
        /// differently-shaped `TransferError` variants
        /// (`NegativeAmount(i64)`, `InsufficientFunds { balance,
        /// required }`, `SameAccount`) and returns each wrapped in
        /// `Result<Transfer<Validated, ValidatedToken>, TransferError>`
        /// from a real function -- this is the one remaining piece.
        #[kani::proof]
        fn three_distinct_error_variants_from_one_function_passes() {
            use amenable_core::Sidecar;

            fn check(
                pending: amenable_gaap::Transfer<amenable_gaap::Pending, amenable_gaap::PendingToken>,
                balance: i64,
            ) -> Result<
                amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                amenable_gaap::TransferError,
            > {
                let payload = pending.primary().clone();
                let amount = payload.amount().value();

                if amount <= 0 {
                    return Err(amenable_gaap::TransferError::NegativeAmount(amount));
                }
                if balance < amount {
                    return Err(amenable_gaap::TransferError::InsufficientFunds {
                        balance,
                        required: amount,
                    });
                }
                if payload.from() == payload.to() {
                    return Err(amenable_gaap::TransferError::SameAccount);
                }

                let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(pending.sidecar());
                Ok(amenable_gaap::Transfer::diagnostic_new(payload, token))
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = amenable_gaap::Transfer::pending(payload);
            let _ = check(pending, balance);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::calling_ledger_validate_directly_times_out".to_owned(),
            "gallery::ledger_account_id_comparison::real_body_reproduction::calling_ledger_validate_directly_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "The real Ledger::validate, called directly (bypassing Exchange::exchange's extra dispatch layer)".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, CALLING_LEDGER_VALIDATE_DIRECTLY_TIMES_OUT_SRC, {
        /// `three_distinct_error_variants_from_one_function_passes`
        /// reproduces `Ledger::validate`'s exact logic/types/error
        /// variants in a free function and passes. This calls the
        /// *actual* `Ledger::validate` (now `pub(crate)` for this
        /// diagnostic) directly -- `&self`/`self.balance` field access,
        /// the real `Self::check_amount_positive`/`self.check_
        /// sufficient_funds` calls -- but bypassing `Exchange::
        /// exchange`'s extra trait-dispatch layer. If this still times
        /// out, the cost is in `validate` itself despite every
        /// individual piece passing (a real, if strange, composition
        /// effect); if it passes, the `Exchange` trait dispatch layer
        /// is the actual remaining culprit.
        #[kani::proof]
        fn calling_ledger_validate_directly_times_out() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = amenable_gaap::Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _ = ledger.validate::<amenable_kani::KaniVerifier>(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::validate_shape_with_uncontracted_helpers_passes".to_owned(),
            "gallery::ledger_account_id_comparison::real_body_reproduction::validate_shape_with_uncontracted_helpers_passes".to_owned(),
            "amenable_kani".to_owned(),
            "validate's exact &self/self.balance/?/.map_err() shape, calling helper methods with the identical bodies as check_amount_positive/check_sufficient_funds but with no #[kani::ensures] attribute at all".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, VALIDATE_SHAPE_WITH_UNCONTRACTED_HELPERS_PASSES_SRC, {
        /// The one remaining structural difference between `Ledger::
        /// validate` (times out) and `three_distinct_error_variants_
        /// from_one_function_passes` (passes, with its own uncontracted
        /// nested `check()` function): `validate` calls `Self::check_
        /// amount_positive`/`self.check_sufficient_funds`, and *those*
        /// carry `#[kani::ensures(..)]` attributes -- even though this
        /// outer harness
        /// never opts into `-Z function-contracts`/`stub_verified` at
        /// all (`just verify-kani`, not `verify-kani-contract`). This
        /// defines a local twin of `Ledger` with the identical `&self`/
        /// `self.balance`/`?`/`.map_err()` call shape, but its helper
        /// methods carry *no* contract attribute whatsoever. If this
        /// passes where the real `validate` doesn't, the mere presence
        /// of `#[kani::ensures]` on a called function -- independent of
        /// whether the calling context ever checks it -- is the cost.
        #[kani::proof]
        fn validate_shape_with_uncontracted_helpers_passes() {
            struct TwinLedger {
                balance: i64,
            }

            impl TwinLedger {
                fn check_amount_positive(amount: i64) -> Result<(), i64> {
                    if amount <= 0 { Err(amount) } else { Ok(()) }
                }

                fn check_sufficient_funds(&self, amount: i64) -> Result<(), (i64, i64)> {
                    if self.balance < amount {
                        Err((self.balance, amount))
                    } else {
                        Ok(())
                    }
                }

                fn validate(
                    &self,
                    input: amenable_gaap::Transfer<amenable_gaap::Pending, amenable_gaap::PendingToken>,
                ) -> Result<
                    amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                    amenable_gaap::TransferError,
                > {
                    use amenable_core::Sidecar;

                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    Self::check_amount_positive(amount)
                        .map_err(amenable_gaap::TransferError::NegativeAmount)?;
                    self.check_sufficient_funds(amount)
                        .map_err(|(balance, required)| amenable_gaap::TransferError::InsufficientFunds {
                            balance,
                            required,
                        })?;
                    if payload.from() == payload.to() {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }

                    let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(input.sidecar());
                    Ok(amenable_gaap::Transfer::diagnostic_new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = TwinLedger { balance };
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}
