//! Creusot proof-function content for `GAAP_LEDGER_PLAN.md`'s Step 3 —
//! the ledger worked example's own claims, the first genuinely
//! non-trivial Creusot predicates anywhere in this workspace (every
//! `Stoplight` claim proven so far is `result.is_ok()`; that never
//! exercised anything beyond translator plumbing).
//!
//! **A real structural difference from `stoplight.rs`.** `Stoplight`'s
//! evidence types (`Green`/`Yellow`/`Red`) live in `amenable_kani`, a
//! crate `amenable_creusot` can never depend on (real cycle risk: it
//! would optionally depend back), so `amenable_creusot::stoplight`
//! needs a full accommodation-model mirror. GAAP's own evidence types
//! (`Pending`/`Validated`/`Committed`/`Rejected<T>`) live in
//! `amenable_gaap` instead, which has no dependency back on
//! `amenable_creusot` at all — so `amenable_creusot` can take a real,
//! ordinary Cargo dependency on `amenable_gaap` and implement
//! `Witness<CreusotVerifier>` directly on the *real* types, no mirror
//! needed. Confirmed empirically, not assumed: `cargo creusot -- -p
//! amenable_creusot` and `cargo creusot prove -- -p amenable_creusot`
//! both succeed with `amenable_gaap` as a real dependency (`Proved (114
//! files) ✔`), and a real injected bug in `check_amount_positive`'s
//! body produced a precise, real failure (`Goal Coma.vc_check_amount_
//! positive: ✘`) before being reverted — the earlier assumption that
//! *any* dependency beyond `amenable_core` would risk an ICE the way a
//! *local* item does (per `amenable_std::creusot_witness`'s own doc
//! comment) turned out not to generalize to an ordinary dependency
//! crate's own items, only to items local to the crate `cargo creusot`
//! actually translates.
//!
//! The proof functions' own signatures are still a *sanitized mirror*
//! of the real Kani-side claims (`amenable_kani::ledger`), not a
//! byte-for-byte structural copy — matching `Stoplight`'s own precedent
//! of dropping the real body's `Result` wrapper: `amount`/`balance`
//! stay plain `i64`, and an account identity is mirrored as a plain
//! `u64` rather than `amenable_gaap::AccountId`'s real `Uuid`-backed
//! struct — the real claim (`from != to`) only needs *some* comparable
//! identity type, and Creusot's support for a hand-rolled `Uuid`-backed
//! equality type is untested territory not worth risking here. `Ledger`/
//! `Transfer<S, Token>` themselves still live only in `amenable_kani`
//! (a crate this one still can't depend on), so the checks below stay
//! isolated predicates over primitive arguments, the same way `Ledger::
//! check_amount_positive`/`check_sufficient_funds` are isolated on the
//! Kani side — connecting them to the real `Ledger::validate`/`::commit`
//! bodies is a separate, later question.
//!
//! **`GAAP_LEDGER_PLAN.md`'s Step 5**: `amenable_gaap::contracts::
//! {AmountPositive, SufficientFunds, AccountsDistinct, BalancedEntries}`
//! — real `Evidence` types since Step 0, previously dead code here too
//! (this module's own `_holds` predicates matched their names by
//! convention only, never touched the real types) — now back real
//! `Witness<CreusotVerifier>`/`Ensures<CreusotVerifier>` impls, one per
//! `_holds` predicate, `#[cfg(not(creusot))]`-only (no `#[cfg(creusot)]`
//! counterpart needed: nothing establishes a token *from* any of these
//! four, so real translation never needs to see either impl — unlike
//! `Validated`/`Committed`'s own `Witness<CreusotVerifier>` impls,
//! further down, which do). `Bound = &'static str`, not `bool`: Pearlite
//! predicates have no exec representation at all, so `Ensures::ensures`
//! just exposes the real `_holds` source text for audit purposes — the
//! actual checking is `amount_positive_holds`/etc. themselves, called
//! directly from Pearlite composition (inside `check_amount_positive`'s
//! own `#[ensures(..)]` and `validated_holds`'s own `match`), never
//! through this trait.

#[cfg(creusot)]
use amenable_core::Witness;
#[cfg(creusot)]
use amenable_gaap::{Committed, Validated};
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(creusot)]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use amenable_gaap::{AccountsDistinct, AmountPositive, BalancedEntries, SufficientFunds};

amenable_derive::harness! {
    creusot, AMOUNT_POSITIVE_HOLDS_SRC, {
        /// The `Validated` postcondition's `AmountPositive` conjunct —
        /// real, callable Pearlite content, named and reused rather
        /// than restated wherever it's checked.
        #[logic(open)]
        pub fn amount_positive_holds(amount: i64, outcome: bool) -> bool {
            pearlite! { outcome == (amount@ > 0) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_AMOUNT_POSITIVE_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::check_amount_positive`'s
        /// own claim, isolated the same way that function is isolated on
        /// the Kani side.
        #[requires(true)]
        #[ensures(amount_positive_holds(amount, result))]
        fn check_amount_positive(amount: i64) -> bool {
            amount > 0
        }
    }
}

// Ties the real `amenable_gaap::AmountPositive` contract type to this
// module's own Pearlite content -- matching `amenable_kani::ledger`'s
// own `kani_ensures!` wiring for the identical type (`GAAP_LEDGER_
// PLAN.md`'s Step 5), previously done on Kani only. `Bound = &'static
// str`, not `bool`: unlike Kani, Pearlite predicates have no exec
// representation at all, so `amenable_core::contract`'s own doc comment
// already anticipates this fallback -- a description of the real bound
// for audit purposes, not a checked value (the actual checking is
// `amount_positive_holds` itself, called directly from Pearlite
// composition above and in `validated_holds` below; nothing in the
// translated proof ever calls through this trait). No `#[cfg(creusot)]`
// counterpart needed at all, unlike `Validated`/`Committed`'s own
// `Witness<CreusotVerifier>` impls: nothing establishes a token *from*
// `AmountPositive` (it's not part of an `Establish<C, V>` chain), so
// real Creusot translation never needs to see either impl.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for AmountPositive {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_amount_positive".to_owned(),
                VERIFY_CHECK_AMOUNT_POSITIVE_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for AmountPositive {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        AMOUNT_POSITIVE_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, SUFFICIENT_FUNDS_HOLDS_SRC, {
        /// The `Validated` postcondition's `SufficientFunds` conjunct.
        #[logic(open)]
        pub fn sufficient_funds_holds(balance: i64, amount: i64, outcome: bool) -> bool {
            pearlite! { outcome == (balance@ >= amount@) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_SUFFICIENT_FUNDS_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::check_sufficient_funds`'s
        /// own claim.
        #[requires(true)]
        #[ensures(sufficient_funds_holds(balance, amount, result))]
        fn check_sufficient_funds(balance: i64, amount: i64) -> bool {
            balance >= amount
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for SufficientFunds {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_sufficient_funds".to_owned(),
                VERIFY_CHECK_SUFFICIENT_FUNDS_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for SufficientFunds {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        SUFFICIENT_FUNDS_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, ACCOUNTS_DISTINCT_HOLDS_SRC, {
        /// The `Validated` postcondition's `AccountsDistinct` conjunct.
        /// `from`/`to` are a sanitized `u64` mirror of `amenable_gaap::
        /// AccountId`'s real `Uuid`-backed identity — see this module's
        /// own doc comment for why.
        #[logic(open)]
        pub fn accounts_distinct_holds(from: u64, to: u64, outcome: bool) -> bool {
            pearlite! { outcome == (from != to) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::validate`'s own
        /// `payload.from() != payload.to()` check, isolated the same
        /// way `AmountPositive`/`SufficientFunds` are.
        #[requires(true)]
        #[ensures(accounts_distinct_holds(from, to, result))]
        fn check_accounts_distinct(from: u64, to: u64) -> bool {
            from != to
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for AccountsDistinct {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_accounts_distinct".to_owned(),
                VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for AccountsDistinct {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        ACCOUNTS_DISTINCT_HOLDS_SRC
    }
}

amenable_derive::harness! {
    creusot, TRANSFER_OUTCOME_MIRROR_SRC, {
        /// Sanitized mirror of `amenable_kani::ledger::TransferError`,
        /// for the combined claim below — real Rust enum, destructured
        /// via Pearlite `match`, which needs no `PartialEq`/`Eq` derive
        /// at all (pattern matching, not an `==` comparison) — dropped
        /// after a real translation error: `creusot-rustc` requires a
        /// `DeepModel` impl for any type deriving `PartialEq`, matching
        /// `CreusotVerifierMetadata`'s own precedent in this crate's
        /// `witness.rs`.
        enum TransferOutcome {
            Ok,
            NegativeAmount(i64),
            InsufficientFunds { balance: i64, required: i64 },
            SameAccount,
        }
    }
}

amenable_derive::harness! {
    creusot, VALIDATED_HOLDS_SRC, {
        /// The real, combined `Validated` postcondition — matches
        /// `amenable_kani::ledger::Validated::validate_ensures`'s exact
        /// match-arm shape (including the real claim's own asymmetry:
        /// the `Ok` arm doesn't restate `SufficientFunds`, only
        /// `AmountPositive`/`AccountsDistinct` — `SufficientFunds` gates
        /// whether `Ok` is reached at all, but isn't re-asserted once
        /// it has been), composing the four isolated conjuncts above
        /// rather than restating their content.
        #[logic(open)]
        fn validated_holds(amount: i64, from: u64, to: u64, outcome: TransferOutcome) -> bool {
            pearlite! {
                match outcome {
                    TransferOutcome::Ok => amount_positive_holds(amount, true) && accounts_distinct_holds(from, to, true),
                    TransferOutcome::NegativeAmount(bad) => bad == amount && amount@ <= 0,
                    TransferOutcome::InsufficientFunds { balance, required } => balance < required,
                    TransferOutcome::SameAccount => true,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_VALIDATE_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::validate`'s own real
        /// body -- calls the same three isolated checks in the same
        /// order, short-circuiting the same way.
        #[requires(true)]
        #[ensures(validated_holds(amount, from, to, result))]
        fn validate(balance: i64, amount: i64, from: u64, to: u64) -> TransferOutcome {
            if !check_amount_positive(amount) {
                return TransferOutcome::NegativeAmount(amount);
            }
            if !check_sufficient_funds(balance, amount) {
                return TransferOutcome::InsufficientFunds { balance, required: amount };
            }
            if !check_accounts_distinct(from, to) {
                return TransferOutcome::SameAccount;
            }
            TransferOutcome::Ok
        }
    }
}

/// Proof artifact for a `Validated`/`Committed` claim that currently
/// rests on more than one isolated Creusot check (`Validated` on
/// `AmountPositive`/`SufficientFunds`/`AccountsDistinct`, each proven
/// separately — see this module's own doc comment for why there is no
/// single composed claim yet, matching `Ledger::check_amount_positive`/
/// `::check_sufficient_funds` being proven separately on the Kani side
/// too). Uses `crate::witness::MultiCheckProof` — see that type's own
/// doc comment for why it's `#[cfg(not(creusot))]` at its definition
/// site, not here (a real internal compiler panic, confirmed the hard
/// way, building this very module).
#[cfg(not(creusot))]
fn validated_proof() -> crate::witness::MultiCheckProof {
    crate::witness::MultiCheckProof {
        checks: vec![
            (
                "check_amount_positive".to_owned(),
                VERIFY_CHECK_AMOUNT_POSITIVE_SRC.to_owned(),
            ),
            (
                "check_sufficient_funds".to_owned(),
                VERIFY_CHECK_SUFFICIENT_FUNDS_SRC.to_owned(),
            ),
            (
                "check_accounts_distinct".to_owned(),
                VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC.to_owned(),
            ),
        ],
    }
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Validated {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_gaap::Validated",
        verifier: "creusot",
        describe: || validated_proof().to_string(),
    }
}

amenable_derive::harness! {
    creusot, BALANCED_ENTRIES_HOLDS_SRC, {
        /// The `Committed` postcondition's `BalancedEntries` conjunct
        /// (`debit + credit == 0`) — honestly tautological by
        /// construction here (`debit` is literally `-credit`), matching
        /// the real Kani-side claim's own documented triviality.
        /// `@`-lifted throughout: `-amount` on a fully unconstrained
        /// `i64` overflows at `i64::MIN` in ordinary Rust arithmetic
        /// (the exact real CBMC timeout `GAAP_LEDGER_PLAN.md`'s Step 2
        /// hit on the Kani side), but Pearlite's `Int` is
        /// arbitrary-precision, so the *claim itself* has no such
        /// overflow — `check_commit_balances` below still carries the
        /// same real `amount > 0` precondition Kani's own `Ledger::
        /// commit` needed, since the *function body*'s ordinary `i64`
        /// negation is not overflow-safe merely because the logical
        /// claim about it is.
        #[logic(open)]
        pub fn balanced_entries_holds(debit: i64, credit: i64, outcome: bool) -> bool {
            pearlite! { outcome == (debit@ + credit@ == 0) }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_COMMIT_BALANCES_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::commit`'s own claim
        /// -- see the real edge's own doc comment for why `amount > 0`
        /// is a genuine precondition, not an artifact of what a harness
        /// happens to assume.
        #[requires(amount@ > 0)]
        #[ensures(balanced_entries_holds(-amount, amount, result))]
        fn check_commit_balances(amount: i64) -> bool {
            let debit = -amount;
            let credit = amount;
            debit + credit == 0
        }
    }
}

// See `AmountPositive`'s own impls, above, for the full rationale.
#[cfg(not(creusot))]
impl amenable_core::Witness<CreusotVerifier> for BalancedEntries {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::witness::MultiCheckProof;

    fn proof() -> Self::ProofArtifact {
        crate::witness::MultiCheckProof {
            checks: vec![(
                "check_commit_balances".to_owned(),
                VERIFY_CHECK_COMMIT_BALANCES_SRC.to_owned(),
            )],
        }
    }
}

#[cfg(not(creusot))]
impl amenable_core::Ensures<CreusotVerifier> for BalancedEntries {
    type Input = ();
    type Bound = &'static str;

    fn ensures((): ()) -> Self::Bound {
        BALANCED_ENTRIES_HOLDS_SRC
    }
}

#[cfg(not(creusot))]
fn committed_proof() -> crate::witness::MultiCheckProof {
    crate::witness::MultiCheckProof {
        checks: vec![(
            "check_commit_balances".to_owned(),
            VERIFY_CHECK_COMMIT_BALANCES_SRC.to_owned(),
        )],
    }
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Committed {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_gaap::Committed",
        verifier: "creusot",
        describe: || committed_proof().to_string(),
    }
}
