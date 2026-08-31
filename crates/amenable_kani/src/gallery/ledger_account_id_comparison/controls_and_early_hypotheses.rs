::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::account_id_inequality_over_concrete_strings_passes".to_owned(),
            "gallery::ledger_account_id_comparison::account_id_inequality_over_concrete_strings_passes".to_owned(),
            "amenable_kani".to_owned(),
            "AccountId inequality over two concrete short strings resolves immediately".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, ACCOUNT_ID_INEQUALITY_OVER_CONCRETE_STRINGS_PASSES_SRC, {
        /// Control: no `#[kani::proof_for_contract]`, no `Sidecar`/
        /// `Establish` generics, no symbolic `i64`s -- just the bare
        /// `String`-backed comparison `Ledger::validate` performs.
        #[kani::proof]
        fn account_id_inequality_over_concrete_strings_passes() {
            let alice = amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice");
            let bob = amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob");
            assert!(alice != bob);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::validate_with_concrete_amounts_passes".to_owned(),
            "gallery::ledger_account_id_comparison::validate_with_concrete_amounts_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Ledger::exchange with fully concrete amount/balance, isolating symbolic-branching cost from structural Sidecar/Establish cost".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, VALIDATE_WITH_CONCRETE_AMOUNTS_PASSES_SRC, {
        /// Same call graph as `validate_without_dfcc_checking_times_out`,
        /// but `amount`/`balance` are concrete, not `kani::any()`. This
        /// passing is what isolates the bottleneck to symbolic
        /// branching specifically, not the generic `Sidecar`/
        /// `Establish`/`ProofToken` dispatch chain or `Establish::
        /// establish`'s `#[track_caller]` on their own.
        #[kani::proof]
        fn validate_with_concrete_amounts_passes() {
            let ledger = amenable_gaap::Ledger::new(100);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(50),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _: Result<
                amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                amenable_gaap::TransferError,
            > = ledger.validate::<amenable_kani::KaniVerifier>(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::validate_with_one_symbolic_field_times_out".to_owned(),
            "gallery::ledger_account_id_comparison::validate_with_one_symbolic_field_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "Ledger::exchange with only `amount` symbolic (balance concrete and always sufficient) -- still times out".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, VALIDATE_WITH_ONE_SYMBOLIC_FIELD_TIMES_OUT_SRC, {
        /// Isolates whether a *single* symbolic `i64` determining the
        /// `Ok`(heap-allocating)/`Err`(non-allocating) split is already
        /// enough to trigger the timeout, or whether it takes *two*
        /// interacting symbolic fields
        /// (`validate_without_dfcc_checking_times_out` has both `amount`
        /// and `balance` symbolic). It's already enough on its own.
        #[kani::proof]
        fn validate_with_one_symbolic_field_times_out() {
            let amount: i64 = kani::any();
            let ledger = amenable_gaap::Ledger::new(1_000_000);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _: Result<
                amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                amenable_gaap::TransferError,
            > = ledger.validate::<amenable_kani::KaniVerifier>(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::validate_without_dfcc_checking_times_out".to_owned(),
            "gallery::ledger_account_id_comparison::validate_without_dfcc_checking_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "Ledger::validate's own body, called directly with no #[kani::proof_for_contract] checking -- rules out DFCC/stubbing overhead as the cause".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, VALIDATE_WITHOUT_DFCC_CHECKING_TIMES_OUT_SRC, {
        /// Isolates whether the real `validate` body (`Sidecar`/
        /// `Establish` generics, `TransferPayload::clone()`, the four
        /// early-return branches) is itself expensive independent of
        /// `#[kani::proof_for_contract]`'s DFCC requires/ensures
        /// checking and `-Z stubbing`/function-contracts machinery
        /// (both absent here -- plain `#[kani::proof]`, direct call).
        /// It is: this alone reproduces the original timeout.
        #[kani::proof]
        fn validate_without_dfcc_checking_times_out() {
            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = amenable_gaap::Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let _: Result<
                amenable_gaap::Transfer<amenable_gaap::Validated, amenable_gaap::ValidatedToken>,
                amenable_gaap::TransferError,
            > = ledger.validate::<amenable_kani::KaniVerifier>(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::bare_result_transfer_payload_passes".to_owned(),
            "gallery::ledger_account_id_comparison::bare_result_transfer_payload_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Bare fn(i64) -> Result<TransferPayload, i64>, symbolic branch, no Sidecar/Establish/Transfer<S,Token> generics at all".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, BARE_RESULT_TRANSFER_PAYLOAD_PASSES_SRC, {
        /// Strips away every piece of `amenable_kani::ledger`'s own
        /// machinery (`Transfer<S, Token>`, `Sidecar`, `Establish`,
        /// `#[amenable_derive::exchange]`'s generated DFCC wiring) down
        /// to a plain function: does a symbolic branch selecting
        /// between an `Ok` arm that constructs a `String`-carrying
        /// `TransferPayload` and a non-allocating `Err` arm still time
        /// out on its own? If so, the generic `Sidecar`/`Establish`
        /// dispatch chain is exonerated -- the cost is inherent to
        /// `Result<StructContainingString, E>` under a symbolic
        /// discriminant, full stop.
        #[kani::proof]
        fn bare_result_transfer_payload_passes() {
            fn check(amount: i64) -> Result<amenable_gaap::TransferPayload, i64> {
                if amount <= 0 {
                    Err(amount)
                } else {
                    Ok(amenable_gaap::TransferPayload::new(
                        amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                        amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                        amenable_gaap::Amount::new(amount),
                    ))
                }
            }

            let amount: i64 = kani::any();
            let _ = check(amount);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::ledger_account_id_comparison::symbolic_branch_with_track_caller_and_no_string_passes".to_owned(),
            "gallery::ledger_account_id_comparison::symbolic_branch_with_track_caller_and_no_string_passes".to_owned(),
            "amenable_kani".to_owned(),
            "Symbolic branch that calls Establish::establish (its #[track_caller]) in the Ok arm, with no String/heap allocation anywhere".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, SYMBOLIC_BRANCH_WITH_TRACK_CALLER_AND_NO_STRING_PASSES_SRC, {
        /// Isolates `Establish::establish`'s `#[track_caller]` (flagged
        /// every run as an unsupported `caller_location` construct)
        /// from `String` allocation: a symbolic branch whose `Ok` arm
        /// calls a real `#[track_caller]` function but allocates
        /// nothing. `stoplight::Established::root()`/`Establish::
        /// establish` already exercise `#[track_caller]` under Kani
        /// successfully, but never under a *symbolic* branch (every
        /// `Stoplight` edge is unconditional `Ok`) -- this closes that
        /// gap directly.
        #[kani::proof]
        fn symbolic_branch_with_track_caller_and_no_string_passes() {
            use amenable_core::Sidecar;

            // Built unconditionally, outside the symbolic branch below --
            // isolates the `#[track_caller]` call itself, not whether
            // allocation happens conditionally.
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::Account::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::Account::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(1),
            );
            let input = amenable_gaap::Transfer::pending(payload);
            let credential = input.sidecar();

            let amount: i64 = kani::any();
            if amount > 0 {
                let token = <amenable_gaap::Validated as amenable_core::Establish<_, amenable_kani::KaniVerifier>>::establish(credential);
                let _ = token;
            }
        }
    }
}
