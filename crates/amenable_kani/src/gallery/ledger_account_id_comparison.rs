//! Isolates `GAAP_LEDGER_PLAN.md`'s Step 1 CBMC timeout:
//! `verify_validate_accepts_a_lawful_transfer` (`ledger.rs`) times out
//! under `just verify-kani-contract` with two free symbolic `i64`s
//! (`amount`, `balance`) and two *concrete* account names
//! ("Alice"/"Bob").
//!
//! **Root cause, fully and precisely isolated (13 real experiments):**
//! a `String` comparison (`payload.from() != payload.to()`,
//! `AccountId`'s own `PartialEq`) evaluated *inside a `#[kani::ensures]`
//! closure* is expensive for CBMC — even though the identical
//! comparison is cheap everywhere else it was tested: as a bare,
//! standalone check
//! ([`account_id_inequality_over_concrete_strings_passes`], 0.5s), and
//! as an ordinary `if` inside `validate`'s own real function body (part
//! of every passing reproduction below). It is not the `Result`
//! pattern, not `String`/heap allocation in general, not the generic
//! `Sidecar`/`Establish`/`Transfer<S, Token>` machinery, not
//! `#[track_caller]`, not the `Exchange` trait's dispatch layer, not
//! multiple error variants, not `#[kani::ensures]`'s mere presence
//! (several intermediate hypotheses here, since disproven — see below),
//! and not the closure's other pieces (`.primary()`'s real `Sidecar`
//! trait dispatch, the `amount > 0` check — both proven cheap by
//! [`ensures_closure_checking_only_amount_passes`]). It is precisely:
//! *this one comparison, evaluated inside this one kind of closure*.
//!
//! The decisive pair, both against a local twin carrying `validate`'s
//! *exact* real body (branching, heap allocation, `&self`/`self.
//! balance`, everything): [`ensures_closure_checking_only_amount_
//! passes`] — `#[kani::ensures]` closure matches `Ok`/`Err` and checks
//! `.primary().amount().value() > 0`, no string comparison anywhere in
//! the closure — **verifies successfully in 1.2s**.
//! [`ensures_closure_with_accounts_distinct_string_comparison`] — the
//! identical closure and body, with *only* `payload.from() !=
//! payload.to()` added to the `Ok` arm — **times out**. That is the
//! entire, isolated difference.
//!
//! [`trivial_ensures_closure_on_the_real_body_fails_fast`] was the
//! pivot that reopened this investigation after `calling_ledger_
//! validate_directly_times_out` seemed to close it on "`#[kani::
//! ensures]`'s mere presence": the identical real branching/allocating
//! body, contracted with the *trivial* closure `result.is_ok()`
//! (matching every `Stoplight` claim's own shape) — **verifies in
//! 1.27s and correctly reports a real, expected `Failed` check** (the
//! trivial claim is genuinely false on the `Err` path) — not a
//! timeout. That result is what redirected the investigation from "the
//! attribute itself" to "the closure's own content."
//!
//! Earlier false trail, kept for the record (gallery discipline: a
//! failed hypothesis is still a real result — every function below
//! whose name ends `_passes` was *predicted* to time out when written,
//! and renamed to match what actually happened): the first eight
//! experiments seemed to isolate the cause to "a symbolic condition
//! selecting an `Ok` arm that heap-allocates."
//! [`bare_result_transfer_payload_passes`],
//! [`symbolic_branch_constructing_real_transfer_validated_passes`],
//! [`returning_real_result_type_from_a_function_passes`],
//! [`symbolic_pending_transfer_extraction_passes`],
//! [`full_combination_inline_without_calling_validate`], and
//! [`three_distinct_error_variants_from_one_function_passes`] each
//! reproduced the *exact* real types, logic, and symbolic branching
//! with no `#[kani::ensures]` anywhere in their call graph — every one
//! of them fast, cleanly falsifying that hypothesis.
//!
//! Real, characterized, precise — not a guess dressed up as one.
//!
//! **What comparisons work inside a `#[kani::ensures]` closure, checked
//! directly rather than assumed:** every real alternative to raw
//! `String` content comparison verifies fast, against `validate`'s
//! exact real body:
//!
//! - [`ensures_closure_with_enum_account_comparison_passes`] — a
//!   fieldless 2-variant enum (`AccountName`), derived `PartialEq`
//!   (discriminant comparison). **0.12s.** Doesn't scale as a general
//!   `AccountId` replacement (a real ledger has many accounts, not a
//!   fixed closed set of two).
//! - [`ensures_closure_with_numeric_id_comparison_passes`] — a bare
//!   `u64` newtype, fully symbolic (`kani::any()`, not a small closed
//!   set). **0.12s.** Scales to an arbitrarily large identity space.
//! - [`ensures_closure_with_uuid_shaped_comparison_passes`] — a `[u8;
//!   16]`-backed newtype modeling `uuid::Uuid`'s real internal
//!   representation (the `uuid` crate isn't a workspace dependency, so
//!   this models the shape directly rather than pulling it in). **0.42s.**
//! - [`ensures_closure_with_id_plus_name_hybrid_passes`] — the
//!   practically useful shape: `{ id: u64, name: AccountId }`, hand-
//!   written `PartialEq` comparing only `.id`. The `String` field is
//!   still constructed (identical allocation cost to today's
//!   `AccountId(String)`) — it's just never touched by the comparison.
//!   **0.83s.** Confirms a real fix doesn't have to sacrifice human-
//!   readable account names, only change what equality *checks*.
//!
//! **Correction, from a real counter-result — read this over the
//! "fixed size, not heap-allocated" framing above:**
//! [`ensures_closure_with_fixed_capacity_string_comparison_times_out`]
//! tested a *fixed-capacity* string (`{ bytes: [u8; 24], len: u8 }`,
//! stack-allocated, `PartialEq` comparing only `bytes[..len]`) —
//! **still times out**, just like `String`. "No heap allocation" was
//! never the real dividing line; it happened to be true of every
//! passing case above by coincidence, because `u64`/`UuidShapedId`/the
//! enum all also share the trait that actually matters: the
//! *comparison itself* has a length fixed at compile time, always the
//! same number of bytes regardless of content. `len` in the fixed-
//! capacity string is itself symbolic, so the comparison is still
//! variable-length even though the *storage* is bounded and stack-
//! allocated — the identical shape this project's own catalogued
//! "symbolic-length memcmp" timeout class already names, just now
//! confirmed to apply *inside a `#[kani::ensures]` closure specifically*,
//! not only to direct symbolic-length iteration.
//!
//! This points to a fix that needs no change to `#[amenable_derive::
//! exchange]`, `kani_ensures!`, or any other shared macro at all — the
//! generated `#[kani::ensures]` wiring was never the problem. The fix
//! lives entirely at the data-model layer: give `amenable_gaap::
//! AccountId` a cheap-to-compare identity whose comparison has a length
//! fixed at compile time (a numeric id, a UUID-shaped byte array — not
//! a capped-but-still-variable-length string, even a stack-allocated
//! one) that `PartialEq` actually uses, alongside whatever human-
//! readable name it already carries. See `GAAP_LEDGER_PLAN.md`'s Step 1
//! for the write-up.

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::account_id_inequality_over_concrete_strings_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::account_id_inequality_over_concrete_strings_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "AccountId inequality over two concrete short strings resolves immediately".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, ACCOUNT_ID_INEQUALITY_OVER_CONCRETE_STRINGS_PASSES_SRC, {
        /// Control: no `#[kani::proof_for_contract]`, no `Sidecar`/
        /// `Establish` generics, no symbolic `i64`s -- just the bare
        /// `String`-backed comparison `Ledger::validate` performs.
        #[kani::proof]
        fn account_id_inequality_over_concrete_strings_passes() {
            let alice = amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice");
            let bob = amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob");
            assert!(alice != bob);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::validate_with_concrete_amounts_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::validate_with_concrete_amounts_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Ledger::exchange with fully concrete amount/balance, isolating symbolic-branching cost from structural Sidecar/Establish cost".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, VALIDATE_WITH_CONCRETE_AMOUNTS_PASSES_SRC, {
        /// Same call graph as `validate_without_dfcc_checking_times_out`,
        /// but `amount`/`balance` are concrete, not `kani::any()`. This
        /// passing is what isolates the bottleneck to symbolic
        /// branching specifically, not the generic `Sidecar`/
        /// `Establish`/`ProofToken` dispatch chain or `Establish::
        /// establish`'s `#[track_caller]` on their own.
        #[kani::proof]
        fn validate_with_concrete_amounts_passes() {
            use amenable_core::Exchange;

            let ledger = amenable_kani::Ledger::new(100);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(50),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.exchange(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::validate_with_one_symbolic_field_times_out".to_owned(),
            harness: "gallery::ledger_account_id_comparison::validate_with_one_symbolic_field_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Ledger::exchange with only `amount` symbolic (balance concrete and always sufficient) -- still times out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, VALIDATE_WITH_ONE_SYMBOLIC_FIELD_TIMES_OUT_SRC, {
        /// Isolates whether a *single* symbolic `i64` determining the
        /// `Ok`(heap-allocating)/`Err`(non-allocating) split is already
        /// enough to trigger the timeout, or whether it takes *two*
        /// interacting symbolic fields
        /// (`validate_without_dfcc_checking_times_out` has both `amount`
        /// and `balance` symbolic). It's already enough on its own.
        #[kani::proof]
        fn validate_with_one_symbolic_field_times_out() {
            use amenable_core::Exchange;

            let amount: i64 = kani::any();
            let ledger = amenable_kani::Ledger::new(1_000_000);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.exchange(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::validate_without_dfcc_checking_times_out".to_owned(),
            harness: "gallery::ledger_account_id_comparison::validate_without_dfcc_checking_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Ledger::validate's own body, called directly with no #[kani::proof_for_contract] checking -- rules out DFCC/stubbing overhead as the cause".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
            use amenable_core::Exchange;

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = amenable_kani::Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.exchange(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::bare_result_transfer_payload_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::bare_result_transfer_payload_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Bare fn(i64) -> Result<TransferPayload, i64>, symbolic branch, no Sidecar/Establish/Transfer<S,Token> generics at all".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
                        amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                        amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::symbolic_branch_with_track_caller_and_no_string_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::symbolic_branch_with_track_caller_and_no_string_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Symbolic branch that calls Establish::establish (its #[track_caller]) in the Ok arm, with no String/heap allocation anywhere".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
            use amenable_core::{Establish, Sidecar};

            // Built unconditionally, outside the symbolic branch below --
            // isolates the `#[track_caller]` call itself, not whether
            // allocation happens conditionally.
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(1),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let credential = input.sidecar();

            let amount: i64 = kani::any();
            if amount > 0 {
                let token = amenable_gaap::Validated::establish(credential);
                let _ = token;
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::symbolic_branch_constructing_real_transfer_validated_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::symbolic_branch_constructing_real_transfer_validated_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Symbolic branch whose Ok arm constructs the real Transfer<Validated, ValidatedToken> wrapper (pulling in the real, #[amenable_derive::exchange]-generated Witness<KaniVerifier> for Validated)".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
            use amenable_core::{Establish, Sidecar};

            let pending_payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(1),
            );
            let pending = amenable_kani::Transfer::pending(pending_payload);
            let credential = pending.sidecar();

            let amount: i64 = kani::any();
            if amount > 0 {
                let payload = amenable_gaap::TransferPayload::new(
                    amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                    amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                    amenable_gaap::Amount::new(amount),
                );
                let token = amenable_gaap::Validated::establish(credential);
                let validated: amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken> =
                    amenable_kani::Transfer::new(payload, token);
                let _ = validated;
            }
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::returning_real_result_type_from_a_function_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::returning_real_result_type_from_a_function_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "fn(i64) -> Result<Transfer<Validated, ValidatedToken>, TransferError> (validate's exact real types), symbolic amount, result discarded by caller".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
            use amenable_core::{Establish, Sidecar};

            fn build(
                credential: amenable_kani::PendingToken,
                amount: i64,
            ) -> Result<
                amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                amenable_kani::TransferError,
            > {
                if amount <= 0 {
                    return Err(amenable_kani::TransferError::NegativeAmount(amount));
                }
                let payload = amenable_gaap::TransferPayload::new(
                    amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                    amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                    amenable_gaap::Amount::new(amount),
                );
                let token = amenable_gaap::Validated::establish(credential);
                Ok(amenable_kani::Transfer::new(payload, token))
            }

            let pending_payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(1),
            );
            let pending = amenable_kani::Transfer::pending(pending_payload);
            let credential = pending.sidecar();

            let amount: i64 = kani::any();
            let _ = build(credential, amount);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::symbolic_pending_transfer_extraction_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::symbolic_pending_transfer_extraction_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Transfer::pending built from a payload with a symbolic amount, then .primary()/.sidecar() called on it -- no branching downstream at all".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = amenable_kani::Transfer::pending(payload);
            let extracted = pending.primary().clone();
            let credential = pending.sidecar();
            let _ = (extracted, credential);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::full_combination_inline_without_calling_validate".to_owned(),
            harness: "gallery::ledger_account_id_comparison::full_combination_inline_without_calling_validate".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Every piece that passed individually (symbolic Transfer::pending extraction, amount-positive check, sufficient-funds check against self.balance, accounts-distinct check, establish, Transfer::new), assembled inline -- not calling Ledger::validate/Exchange::exchange at all".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
            use amenable_core::{Establish, Sidecar};

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();

            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = amenable_kani::Transfer::pending(payload);
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

            let token = amenable_gaap::Validated::establish(pending.sidecar());
            let validated: amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken> =
                amenable_kani::Transfer::new(payload, token);
            let _ = validated;
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::three_distinct_error_variants_from_one_function_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::three_distinct_error_variants_from_one_function_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Same combination as full_combination_inline_without_calling_validate, but each check path actually constructs and returns its real TransferError variant (not a bare early return) wrapped in Result<Transfer<Validated,..>, TransferError>".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
            use amenable_core::{Establish, Sidecar};

            fn check(
                pending: amenable_kani::Transfer<amenable_gaap::Pending, amenable_kani::PendingToken>,
                balance: i64,
            ) -> Result<
                amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                amenable_kani::TransferError,
            > {
                let payload = pending.primary().clone();
                let amount = payload.amount().value();

                if amount <= 0 {
                    return Err(amenable_kani::TransferError::NegativeAmount(amount));
                }
                if balance < amount {
                    return Err(amenable_kani::TransferError::InsufficientFunds {
                        balance,
                        required: amount,
                    });
                }
                if payload.from() == payload.to() {
                    return Err(amenable_kani::TransferError::SameAccount);
                }

                let token = amenable_gaap::Validated::establish(pending.sidecar());
                Ok(amenable_kani::Transfer::new(payload, token))
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let pending = amenable_kani::Transfer::pending(payload);
            let _ = check(pending, balance);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::calling_ledger_validate_directly_times_out".to_owned(),
            harness: "gallery::ledger_account_id_comparison::calling_ledger_validate_directly_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "The real Ledger::validate, called directly (bypassing Exchange::exchange's extra dispatch layer)".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
            let ledger = amenable_kani::Ledger::new(balance);
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::validate_shape_with_uncontracted_helpers_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::validate_shape_with_uncontracted_helpers_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "validate's exact &self/self.balance/?/.map_err() shape, calling helper methods with the identical bodies as check_amount_positive/check_sufficient_funds but with no #[kani::ensures] attribute at all".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
                    input: amenable_kani::Transfer<amenable_gaap::Pending, amenable_kani::PendingToken>,
                ) -> Result<
                    amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                    amenable_kani::TransferError,
                > {
                    use amenable_core::{Establish, Sidecar};

                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    Self::check_amount_positive(amount)
                        .map_err(amenable_kani::TransferError::NegativeAmount)?;
                    self.check_sufficient_funds(amount)
                        .map_err(|(balance, required)| amenable_kani::TransferError::InsufficientFunds {
                            balance,
                            required,
                        })?;
                    if payload.from() == payload.to() {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }

                    let token = amenable_gaap::Validated::establish(input.sidecar());
                    Ok(amenable_kani::Transfer::new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = TwinLedger { balance };
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::trivial_ensures_closure_on_the_real_body_fails_fast".to_owned(),
            harness: "gallery::ledger_account_id_comparison::trivial_ensures_closure_on_the_real_body_fails_fast".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "validate's exact real branching+allocation body, #[kani::ensures] attached, but with a TRIVIAL closure (result.is_ok(), matching Stoplight's own claim shape) instead of the real biconditional".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Failed,
        },
    }
}

amenable_derive::harness! {
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
                        amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                        amenable_kani::TransferError,
                    >| result.is_ok())
                )]
                fn validate(
                    &self,
                    input: amenable_kani::Transfer<amenable_gaap::Pending, amenable_kani::PendingToken>,
                ) -> Result<
                    amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                    amenable_kani::TransferError,
                > {
                    use amenable_core::{Establish, Sidecar};

                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    if amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(amount));
                    }
                    if self.balance < amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: amount,
                        });
                    }
                    if payload.from() == payload.to() {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }

                    let token = amenable_gaap::Validated::establish(input.sidecar());
                    Ok(amenable_kani::Transfer::new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = TrivialClaimLedger { balance };
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_checking_only_amount_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_checking_only_amount_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "validate's exact real body, #[kani::ensures] closure that matches on Ok/Err and calls .primary().amount().value() > 0 in the Ok arm -- no accounts-distinct string comparison, no per-Err-variant destructuring".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
                        amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                        amenable_kani::TransferError,
                    >| match result {
                        Ok(validated) => validated.primary().amount().value() > 0,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    input: amenable_kani::Transfer<amenable_gaap::Pending, amenable_kani::PendingToken>,
                ) -> Result<
                    amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                    amenable_kani::TransferError,
                > {
                    use amenable_core::Establish;

                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    if amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(amount));
                    }
                    if self.balance < amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: amount,
                        });
                    }
                    if payload.from() == payload.to() {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }

                    let token = amenable_gaap::Validated::establish(input.sidecar());
                    Ok(amenable_kani::Transfer::new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = AmountOnlyClaimLedger { balance };
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_accounts_distinct_string_comparison".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_with_accounts_distinct_string_comparison".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "ensures_closure_checking_only_amount_passes, plus the accounts-distinct string comparison (payload.from() != payload.to()) in the Ok arm -- Err arms still bare `_ => true`".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
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
                        amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                        amenable_kani::TransferError,
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
                    input: amenable_kani::Transfer<amenable_gaap::Pending, amenable_kani::PendingToken>,
                ) -> Result<
                    amenable_kani::Transfer<amenable_gaap::Validated, amenable_kani::ValidatedToken>,
                    amenable_kani::TransferError,
                > {
                    use amenable_core::Establish;

                    let payload = input.primary().clone();
                    let amount = payload.amount().value();

                    if amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(amount));
                    }
                    if self.balance < amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: amount,
                        });
                    }
                    if payload.from() == payload.to() {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }

                    let token = amenable_gaap::Validated::establish(input.sidecar());
                    Ok(amenable_kani::Transfer::new(payload, token))
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let ledger = StringCheckClaimLedger { balance };
            let payload = amenable_gaap::TransferPayload::new(
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
                amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
                amenable_gaap::Amount::new(amount),
            );
            let input = amenable_kani::Transfer::pending(payload);
            let _ = ledger.validate(input);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_enum_account_comparison_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_with_enum_account_comparison_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "ensures_closure_with_accounts_distinct_string_comparison's exact shape, but account identity is a 2-variant enum (AccountName) instead of a String -- swaps only the comparison operand's own type".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
                    kani::ensures(|result: &Result<EnumPayload, amenable_kani::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: EnumPayload,
                ) -> Result<EnumPayload, amenable_kani::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_kani::TransferError::SameAccount);
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_numeric_id_comparison_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_with_numeric_id_comparison_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Same shape again, but account identity is a u64 newtype (AccountNumber) instead of a 2-variant enum -- tests whether an arbitrarily-large, non-fixed identity space is also cheap, not just a small closed enum".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
                    kani::ensures(|result: &Result<NumericPayload, amenable_kani::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: NumericPayload,
                ) -> Result<NumericPayload, amenable_kani::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_kani::TransferError::SameAccount);
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
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_id_plus_name_hybrid_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_with_id_plus_name_hybrid_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "AccountId carries BOTH a numeric id (compared) and a String display name (never compared, present purely for realism/allocation cost) -- the practically useful shape, not just a bare u64".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
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
            // Not `kani::Arbitrary` -- `amenable_gaap::AccountId` is
            // `String`-backed and doesn't implement it (see `KANI_FOR_
            // VSMS.md`'s own warning: "do not add `#[cfg_attr(kani,
            // derive(kani::Arbitrary))]` to any struct containing
            // `String` fields"). Not needed here anyway: `from`/`to`
            // are hand-constructed below, not `kani::any()`'d directly.
            #[derive(Debug, Clone)]
            struct HybridAccountId {
                id: u64,
                name: amenable_gaap::AccountId,
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
                    kani::ensures(|result: &Result<HybridPayload, amenable_kani::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: HybridPayload,
                ) -> Result<HybridPayload, amenable_kani::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let from = HybridAccountId {
                id: kani::any(),
                name: amenable_gaap::AccountId::new(uuid::Uuid::from_u128(1), "Alice"),
            };
            let to = HybridAccountId {
                id: kani::any(),
                name: amenable_gaap::AccountId::new(uuid::Uuid::from_u128(2), "Bob"),
            };
            let ledger = HybridIdentityLedger { balance };
            let payload = HybridPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_uuid_shaped_comparison_passes".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_with_uuid_shaped_comparison_passes".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Account identity modeled as a UUID's real internal representation ([u8; 16], what uuid::Uuid actually wraps -- fixed-size, stack-allocated, not heap-allocated like String) -- no `uuid` crate dependency needed to test the shape".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::Hypothesis,
            expected: ::amenable_kani::KaniGalleryExpectation::Passed,
        },
    }
}

amenable_derive::harness! {
    kani, ENSURES_CLOSURE_WITH_UUID_SHAPED_COMPARISON_PASSES_SRC, {
        /// Does a UUID work as account identity? The `uuid` crate isn't
        /// a workspace dependency, so this models `uuid::Uuid`'s real
        /// internal representation directly: `Bytes = [u8; 16]`, a
        /// fixed-size, `Copy`, stack-allocated value -- structurally
        /// nothing like `String` (heap-allocated, variable-length).
        /// The project's own Kani failure-pattern catalog flags
        /// *symbolic-length* memcmp as a real, distinct timeout cause;
        /// a UUID's length is fixed at compile time, so this is really
        /// testing "is a bounded 16-byte comparison as cheap as a
        /// bounded 8-byte one (`u64`)," not re-litigating the `String`
        /// finding.
        #[kani::proof]
        fn ensures_closure_with_uuid_shaped_comparison_passes() {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[cfg_attr(kani, derive(kani::Arbitrary))]
            struct UuidShapedId([u8; 16]);

            struct UuidPayload {
                from: UuidShapedId,
                to: UuidShapedId,
                amount: i64,
            }

            struct UuidIdentityLedger {
                balance: i64,
            }

            impl UuidIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<UuidPayload, amenable_kani::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: UuidPayload,
                ) -> Result<UuidPayload, amenable_kani::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let from: UuidShapedId = kani::any();
            let to: UuidShapedId = kani::any();
            let ledger = UuidIdentityLedger { balance };
            let payload = UuidPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration {
        case: || ::amenable_kani::KaniGalleryCase {
            id: "amenable_kani::gallery::ledger_account_id_comparison::ensures_closure_with_fixed_capacity_string_comparison_times_out".to_owned(),
            harness: "gallery::ledger_account_id_comparison::ensures_closure_with_fixed_capacity_string_comparison_times_out".to_owned(),
            package: "amenable_kani".to_owned(),
            title: "Account identity is a fixed-CAPACITY string (stack-allocated [u8; 24] buffer + a used-length field, PartialEq compares only the first `len` bytes -- a bounded variable-length comparison, not a bare fixed-array one, the shape arrayvec::ArrayString/heapless::String actually have) -- still times out".to_owned(),
            disposition: ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            expected: ::amenable_kani::KaniGalleryExpectation::Timeout,
        },
    }
}

amenable_derive::harness! {
    kani, ENSURES_CLOSURE_WITH_FIXED_CAPACITY_STRING_COMPARISON_TIMES_OUT_SRC, {
        /// `ensures_closure_with_uuid_shaped_comparison_passes` tested a
        /// bare `[u8; 16]` -- a fixed-size array with no notion of
        /// "used length" (the whole 16 bytes are always meaningful,
        /// same as a plain integer). A *string* is different: even
        /// with a fixed capacity, its comparison is naturally "compare
        /// only the first `len` bytes," which is a *bounded* variable-
        /// length operation, not a pure fixed-size one -- closer in
        /// spirit to `String`'s own comparison (which was expensive)
        /// than to `UuidShapedId`'s. This tests that shape directly:
        /// `{ bytes: [u8; 24], len: u8 }`, `PartialEq` comparing only
        /// `bytes[..len]` -- the real representation `arrayvec::
        /// ArrayString`/`heapless::String` use, modeled directly rather
        /// than adding either crate as a dependency. **Times out**,
        /// same as `String`: "bounded capacity, no heap" is *not*
        /// enough on its own. The real dividing line, corrected by
        /// this result: it's not "heap-allocated vs. not," it's
        /// whether the *comparison itself* has a symbolic/variable
        /// length. `self.len`/`other.len` here are themselves symbolic
        /// (`kani::any()`), so CBMC still has to reason about a
        /// variable-length compare -- the identical shape the
        /// project's own catalogued "symbolic-length memcmp" timeout
        /// class describes, just over a stack buffer instead of a heap
        /// one.
        #[kani::proof]
        fn ensures_closure_with_fixed_capacity_string_comparison_times_out() {
            const CAPACITY: usize = 24;

            #[derive(Debug, Clone, Copy)]
            #[cfg_attr(kani, derive(kani::Arbitrary))]
            struct FixedCapacityString {
                bytes: [u8; CAPACITY],
                len: u8,
            }

            impl PartialEq for FixedCapacityString {
                fn eq(&self, other: &Self) -> bool {
                    self.len == other.len
                        && self.bytes[..self.len as usize] == other.bytes[..other.len as usize]
                }
            }
            impl Eq for FixedCapacityString {}

            struct FixedStringPayload {
                from: FixedCapacityString,
                to: FixedCapacityString,
                amount: i64,
            }

            struct FixedStringIdentityLedger {
                balance: i64,
            }

            impl FixedStringIdentityLedger {
                #[cfg_attr(
                    kani,
                    kani::ensures(|result: &Result<FixedStringPayload, amenable_kani::TransferError>| match result {
                        Ok(validated) => validated.amount > 0 && validated.from != validated.to,
                        Err(_) => true,
                    })
                )]
                fn validate(
                    &self,
                    payload: FixedStringPayload,
                ) -> Result<FixedStringPayload, amenable_kani::TransferError> {
                    if payload.amount <= 0 {
                        return Err(amenable_kani::TransferError::NegativeAmount(payload.amount));
                    }
                    if self.balance < payload.amount {
                        return Err(amenable_kani::TransferError::InsufficientFunds {
                            balance: self.balance,
                            required: payload.amount,
                        });
                    }
                    if payload.from == payload.to {
                        return Err(amenable_kani::TransferError::SameAccount);
                    }
                    Ok(payload)
                }
            }

            let amount: i64 = kani::any();
            let balance: i64 = kani::any();
            let mut from: FixedCapacityString = kani::any();
            let mut to: FixedCapacityString = kani::any();
            kani::assume(from.len as usize <= CAPACITY);
            kani::assume(to.len as usize <= CAPACITY);
            from.len = from.len.min(CAPACITY as u8);
            to.len = to.len.min(CAPACITY as u8);
            let ledger = FixedStringIdentityLedger { balance };
            let payload = FixedStringPayload { from, to, amount };
            let _ = ledger.validate(payload);
        }
    }
}
