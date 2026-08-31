::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::trivial_ensures_closure_on_the_real_body_fails_fast".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_isolation::trivial_ensures_closure_on_the_real_body_fails_fast".to_owned(),
            "amenable_kani".to_owned(),
            "validate's exact real branching+allocation body, #[kani::ensures] attached, but with a TRIVIAL closure (result.is_ok(), matching Stoplight's own claim shape) instead of the real biconditional".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Failed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, TRIVIAL_ENSURES_CLOSURE_ON_THE_REAL_BODY_FAILS_FAST_SRC, {
        /// The one remaining untested variable: is the cost driven by
        /// `validate`'s own *body* (branching + heap allocation) once
        /// `#[kani::ensures]` is attached at all, or specifically by the
        /// *closure's own complexity* (our real claim does its own
        /// `match`, field access, and a string comparison on `result`)?
        /// This is `validate`'s exact real body, `#[kani::ensures]`
        /// attached, but with the *trivial* closure `result.is_ok()` --
        /// the same shape every `Stoplight` edge's claim already uses
        /// successfully. If this still times out, the closure's own
        /// complexity is exonerated: the attribute's mere presence on a
        /// branching/allocating body is enough, confirmed a second way.
        /// If it passes, the fix is a cheaper *claim*, not giving up on
        /// DFCC.
        #[kani::proof]
        fn trivial_ensures_closure_on_the_real_body_fails_fast() {
            struct TrivialClaimLedger {
                balance: i64,
            }

            impl TrivialClaimLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<
                        amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                        amenable_gaap::TransferError,
                    >| result.is_ok())
                )]
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

                    if amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(amount));
                    }
                    if self.balance < amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: amount,
                        });
                    }
                    if payload.from() == payload.to() {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }

                    let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(input.sidecar());
                    Ok(amenable_gaap::Transfer::diagnostic_new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = TrivialClaimLedger { balance };
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_checking_only_amount_passes".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_isolation::ensures_closure_checking_only_amount_passes".to_owned(),
            "amenable_kani".to_owned(),
            "validate's exact real body, #[kani::ensures] closure that matches on Ok/Err and calls .primary().amount().value() > 0 in the Ok arm -- no accounts-distinct string comparison, no per-Err-variant destructuring".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_CHECKING_ONLY_AMOUNT_PASSES_SRC, {
        /// `trivial_ensures_closure_on_the_real_body_fails_fast` proved
        /// `result.is_ok()` is cheap. This is the next step up in
        /// closure richness: `match`-ing on `Ok`/`Err` (not a bare
        /// bool), and in the `Ok` arm calling `.primary()` (real
        /// `Sidecar<KaniVerifier>` trait dispatch inside the
        /// postcondition-checking context) plus `.amount().value() >
        /// 0` -- but *no* accounts-distinct string comparison, and
        /// `Err` arms stay a bare `true` (no per-variant destructuring
        /// of `TransferError`'s own fields). Isolates whether `.primary
        /// ()`/field-access alone inside the closure costs something,
        /// separate from the string comparison and Err-variant
        /// destructuring the real closure also does.
        #[kani::proof]
        fn ensures_closure_checking_only_amount_passes() {
            use amenable_core::Sidecar;

            struct AmountOnlyClaimLedger {
                balance: i64,
            }

            impl AmountOnlyClaimLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<
                        amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                        amenable_gaap::TransferError,
                    >| match result {
                        Ok(validated) => validated.primary().amount().value() > 0,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    input: amenable_gaap::Transfer<amenable_gaap::Pending, amenable_gaap::PendingToken>,
                ) -> Result<
                    amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                    amenable_gaap::TransferError,
                > {
                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    if amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(amount));
                    }
                    if self.balance < amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: amount,
                        });
                    }
                    if payload.from() == payload.to() {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }

                    let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(input.sidecar());
                    Ok(amenable_gaap::Transfer::diagnostic_new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = AmountOnlyClaimLedger { balance };
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

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_accounts_distinct_string_comparison".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_isolation::ensures_closure_with_accounts_distinct_string_comparison".to_owned(),
            "amenable_kani".to_owned(),
            "ensures_closure_checking_only_amount_passes, plus the accounts-distinct string comparison (payload.from() != payload.to()) in the Ok arm -- Err arms still bare `_ => true`".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_WITH_ACCOUNTS_DISTINCT_STRING_COMPARISON_SRC, {
        /// `ensures_closure_checking_only_amount_passes` proved
        /// `.primary()`/amount-checking inside the closure is cheap.
        /// This adds *just* the accounts-distinct string comparison
        /// (`payload.from() != payload.to()`) to the `Ok` arm -- still
        /// no per-`TransferError`-variant destructuring in the `Err`
        /// arms. Isolates the string comparison specifically, inside an
        /// ensures-closure context this time (a bare string comparison
        /// *outside* any closure was already shown cheap by
        /// `account_id_inequality_over_concrete_strings_passes`).
        #[kani::proof]
        fn ensures_closure_with_accounts_distinct_string_comparison() {
            use amenable_core::Sidecar;

            struct StringCheckClaimLedger {
                balance: i64,
            }

            impl StringCheckClaimLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<
                        amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                        amenable_gaap::TransferError,
                    >| match result {
                        Ok(validated) => {
                            let payload = validated.primary();
                            payload.amount().value() > 0 && payload.from() != payload.to()
                        }
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    input: amenable_gaap::Transfer<amenable_gaap::Pending, amenable_gaap::PendingToken>,
                ) -> Result<
                    amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                    amenable_gaap::TransferError,
                > {
                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    if amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(amount));
                    }
                    if self.balance < amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: amount,
                        });
                    }
                    if payload.from() == payload.to() {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }

                    let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(input.sidecar());
                    Ok(amenable_gaap::Transfer::diagnostic_new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = StringCheckClaimLedger { balance };
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
