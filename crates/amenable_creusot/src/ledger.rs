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

#[cfg(creusot)]
use amenable_core::Witness;
#[cfg(creusot)]
use amenable_gaap::{Committed, Validated};
#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

#[cfg(creusot)]
use crate::CreusotVerifier;

amenable_derive::harness! {
    creusot, AMOUNT_POSITIVE_HOLDS_SRC, {
        /// The `Validated` postcondition's `AmountPositive` conjunct —
        /// real, callable Pearlite content, named and reused rather
        /// than restated wherever it's checked.
        #[logic(open)]
        fn amount_positive_holds(amount: i64, outcome: bool) -> bool {
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

amenable_derive::harness! {
    creusot, SUFFICIENT_FUNDS_HOLDS_SRC, {
        /// The `Validated` postcondition's `SufficientFunds` conjunct.
        #[logic(open)]
        fn sufficient_funds_holds(balance: i64, amount: i64, outcome: bool) -> bool {
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

amenable_derive::harness! {
    creusot, ACCOUNTS_DISTINCT_HOLDS_SRC, {
        /// The `Validated` postcondition's `AccountsDistinct` conjunct.
        /// `from`/`to` are a sanitized `u64` mirror of `amenable_gaap::
        /// AccountId`'s real `Uuid`-backed identity — see this module's
        /// own doc comment for why.
        #[logic(open)]
        fn accounts_distinct_holds(from: u64, to: u64, outcome: bool) -> bool {
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

/// Proof artifact for a `Validated`/`Committed` claim that currently
/// rests on more than one isolated Creusot check (`Validated` on
/// `AmountPositive`/`SufficientFunds`/`AccountsDistinct`, each proven
/// separately — see this module's own doc comment for why there is no
/// single composed claim yet, matching `Ledger::check_amount_positive`/
/// `::check_sufficient_funds` being proven separately on the Kani side
/// too). Owned strings, not `&'static str` — matching `amenable_kani::
/// CalculationProof`'s own precedent, not `amenable_std::CheckedProof`'s.
///
/// `#[cfg(not(creusot))]`, like the `ProofRecord` registrations that
/// build one — real, confirmed the hard way: an earlier version left
/// this ungated so the `#[cfg(creusot)]`-gated `Witness<CreusotVerifier>`
/// impls below could also report it, and `cargo creusot prove` hit a
/// real internal compiler panic. `creusot-rustc`'s translator sweeps
/// every *local* item in the crate it's translating regardless of
/// `#[cfg(creusot)]`'s own condition being satisfied elsewhere — the
/// `Vec<(String, String)>`/`Display` machinery here is exactly the kind
/// of ordinary Rust infrastructure `amenable_std::creusot_witness`'s own
/// doc comment already warns is unsupported there. So the
/// `#[cfg(creusot)]`-gated `Witness` impls stay trivial (`ProofArtifact
/// = ()`, matching `stoplight.rs`'s own Green/Yellow/Red precedent
/// exactly) — their only role during actual translation is satisfying
/// `Establish`'s `Witness<V>` bound, not reporting anything. The real,
/// descriptive artifact only ever needs to exist outside that pass.
#[cfg(not(creusot))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProof {
    /// Each real Creusot contract function backing this claim, and its
    /// own verbatim source.
    pub checks: Vec<(String, String)>,
}

#[cfg(not(creusot))]
impl std::fmt::Display for CheckedProof {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (harness, claim) in &self.checks {
            writeln!(f, "harness: {harness}")?;
            writeln!(f, "claim: {claim}")?;
        }
        Ok(())
    }
}

#[cfg(not(creusot))]
fn validated_proof() -> CheckedProof {
    CheckedProof {
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
        fn balanced_entries_holds(debit: i64, credit: i64, outcome: bool) -> bool {
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

#[cfg(not(creusot))]
fn committed_proof() -> CheckedProof {
    CheckedProof {
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
