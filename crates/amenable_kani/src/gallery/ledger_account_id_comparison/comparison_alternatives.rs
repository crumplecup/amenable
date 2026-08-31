::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_enum_account_comparison_passes".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_with_enum_account_comparison_passes".to_owned(),
            "amenable_kani".to_owned(),
            "ensures_closure_with_accounts_distinct_string_comparison's exact shape, but account identity is a 2-variant enum (AccountName) instead of a String -- swaps only the comparison operand's own type".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_WITH_ENUM_ACCOUNT_COMPARISON_PASSES_SRC, {
        /// `ensures_closure_with_accounts_distinct_string_comparison`
        /// isolated the cost to exactly one comparison: two
        /// independently-constructed `String`s compared for equality
        /// *inside* a `#[kani::ensures]` closure -- elicitation's own
        /// real `kani_invariant_fn` usage never exercises that specific
        /// operation (`archive_nav_consistent` only ever checks a
        /// *single* string's `.is_empty()`, never `a == b` across two).
        /// This tests the user's own first idea directly: does the
        /// *identical* claim, expressed over a 2-variant enum instead
        /// of a `String`, work fine in the same closure position? A
        /// derived `PartialEq` on a fieldless enum compares discriminants
        /// (a single integer tag), not heap-allocated byte content.
        #[kani::proof]
        fn ensures_closure_with_enum_account_comparison_passes() {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[cfg_attr(kani, derive(kani::Arbitrary))]
            enum AccountName {
                Alice,
                Bob,
            }

            struct EnumPayload {
                from: AccountName,
                to: AccountName,
                amount: i64,
            }

            struct EnumIdentityLedger {
                balance: i64,
            }

            impl EnumIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<EnumPayload, amenable_gaap::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: EnumPayload,
                ) -> Result<EnumPayload, amenable_gaap::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let from: AccountName = kani::any();
            let to: AccountName = kani::any();
            let ledger = EnumIdentityLedger { balance };
            let payload = EnumPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_numeric_id_comparison_passes".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_with_numeric_id_comparison_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Same shape again, but account identity is a u64 newtype (AccountNumber) instead of a 2-variant enum -- tests whether an arbitrarily-large, non-fixed identity space is also cheap, not just a small closed enum".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_WITH_NUMERIC_ID_COMPARISON_PASSES_SRC, {
        /// `ensures_closure_with_enum_account_comparison_passes` proved
        /// a *fixed, 2-variant* enum works. Real ledgers have many
        /// accounts, not two named ones -- a fixed enum doesn't scale
        /// as a general `AccountId` replacement. This tests the more
        /// broadly useful alternative: a `u64`-backed numeric account
        /// number (`kani::any::<u64>()`, an arbitrarily large identity
        /// space, not a small closed set), compared for equality inside
        /// the same `#[kani::ensures]` position.
        #[kani::proof]
        fn ensures_closure_with_numeric_id_comparison_passes() {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[cfg_attr(kani, derive(kani::Arbitrary))]
            struct AccountNumber(u64);

            struct NumericPayload {
                from: AccountNumber,
                to: AccountNumber,
                amount: i64,
            }

            struct NumericIdentityLedger {
                balance: i64,
            }

            impl NumericIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<NumericPayload, amenable_gaap::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: NumericPayload,
                ) -> Result<NumericPayload, amenable_gaap::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let from: AccountNumber = kani::any();
            let to: AccountNumber = kani::any();
            let ledger = NumericIdentityLedger { balance };
            let payload = NumericPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_id_plus_name_hybrid_passes".to_owned(),
            "gallery::ledger_account_id_comparison::ensures_closure_with_id_plus_name_hybrid_passes".to_owned(),
            "amenable_kani".to_owned(),
            "AccountId carries BOTH a numeric id (compared) and a String display name (never compared, present purely for realism/allocation cost) -- the practically useful shape, not just a bare u64".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ENSURES_CLOSURE_WITH_ID_PLUS_NAME_HYBRID_PASSES_SRC, {
        /// A real `AccountId` likely wants both a cheap-to-compare
        /// identity *and* a human-readable name (real bookkeeping UIs
        /// show "Checking Account", not a bare integer). This tests the
        /// practical shape: `AccountId { id: u64, name: String }`, with
        /// a hand-written `PartialEq` comparing only `.id` -- the
        /// `String` field is still constructed (same heap-allocation
        /// cost as `AccountId(String)` had), just never touched by the
        /// comparison the `#[kani::ensures]` closure performs.
        #[kani::proof]
        fn ensures_closure_with_id_plus_name_hybrid_passes() {
            // Not `kani::Arbitrary` -- `amenable_gaap::Account` is
            // `String`-backed and doesn't implement it (see `KANI_FOR_
            // VSMS.md`'s own warning: "do not add `#[cfg_attr(kani,
            // derive(kani::Arbitrary))]` to any struct containing
            // `String` fields"). Not needed here anyway: `from`/`to`
            // are hand-constructed below, not `kani::any()`'d directly.
            #[derive(Debug, Clone)]
            struct HybridAccountId {
                id: u64,
                name: amenable_gaap::Account,
            }

            impl PartialEq for HybridAccountId {
                fn eq(&self, other: &Self) -> bool {
                    self.id == other.id
                }
            }
            impl Eq for HybridAccountId {}

            struct HybridPayload {
                from: HybridAccountId,
                to: HybridAccountId,
                amount: i64,
            }

            struct HybridIdentityLedger {
                balance: i64,
            }

            impl HybridIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<HybridPayload, amenable_gaap::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: HybridPayload,
                ) -> Result<HybridPayload, amenable_gaap::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_gaap::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_gaap::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_gaap::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let from = HybridAccountId {
                id: kani::any(),
                name: amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
            };
            let to = HybridAccountId {
                id: kani::any(),
                name: amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
            };
            let ledger = HybridIdentityLedger { balance };
            let payload = HybridPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}
