#[cfg(creusot)]
use amenable_core::{Establish, Evidence, Sidecar, Witness};
#[cfg(creusot)]
use amenable_gaap::{
    Committed, CommittedToken, Pending, PendingToken, Rejected, RejectedFromPendingToken,
    RejectedFromValidatedToken, Validated, ValidatedToken,
};
#[cfg(creusot)]
use creusot_std::macros::{ensures, extern_spec, logic, requires};
#[cfg(creusot)]
use creusot_std::std::ops::FnOnceExt;

use super::contract_bounds::{
    VERIFY_CHECK_ACCOUNTS_DISTINCT_SRC, VERIFY_CHECK_SUFFICIENT_FUNDS_SRC,
};
#[cfg(creusot)]
use super::contract_bounds::{accounts_distinct_holds, sufficient_funds_holds};
use super::ledger_validate::VERIFY_CHECK_AMOUNT_POSITIVE_SRC;
#[cfg(creusot)]
use super::ledger_validate::{Ledger, TransferError, amount_positive_holds};
#[cfg(creusot)]
use super::transfer_and_types::Transfer;
#[cfg(creusot)]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use crate::CreusotVerifier;
#[cfg(not(creusot))]
use amenable_gaap::BalancedEntries;
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
    crate::witness::MultiCheckProof::new(vec![
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
    ])
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Validated {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_gaap::Validated",
        "creusot",
        || validated_proof().to_string(),
    )
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

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::balanced_entries_holds",
        "creusot",
        "ensures",
        || BALANCED_ENTRIES_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, COMMITTED_AMOUNT_HOLDS_SRC, {
        /// Real captured `commit` companion's own postcondition (`GAAP_
        /// LEDGER_PLAN.md`'s Step 6) -- unlike `check_commit_balances`
        /// above, whose own `#[requires(amount@ > 0)]` guards its
        /// *exec* body's real `-amount` negation, the generated
        /// `commit` companion carries no precondition at all (its real
        /// body never computes `-amount`, only `check_commit_balances`'s
        /// own isolated proof does) -- so this needs to be provable for
        /// *every* `i64`, not just positive ones. `@`-lifts `amount`
        /// *before* negating, inside this function's own body, rather
        /// than computing `-amount` at `i64` width at the ensures
        /// clause's own call site the way `balanced_entries_holds(
        /// -amount, amount, result)` does above -- confirmed the hard
        /// way: that shape needs `amount > 0` to avoid a real overflow
        /// obligation at `i64::MIN`, and this call site has no such
        /// precondition to rely on.
        #[logic(open)]
        pub fn committed_amount_holds(amount: i64) -> bool {
            pearlite! { (-amount@) + amount@ == 0 }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_CHECK_COMMIT_BALANCES_SRC, {
        /// Mirrors `amenable_kani::ledger::Ledger::commit`'s own claim
        /// -- see the real edge's own doc comment for why `amount > 0`
        /// is a genuine precondition, not an artifact of what a harness
        /// happens to assume.
        #[requires(amount_positive_holds(amount, true))]
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
        crate::witness::MultiCheckProof::new(vec![(
            "check_commit_balances".to_owned(),
            VERIFY_CHECK_COMMIT_BALANCES_SRC.to_owned(),
        )])
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
    crate::witness::MultiCheckProof::new(vec![(
        "check_commit_balances".to_owned(),
        VERIFY_CHECK_COMMIT_BALANCES_SRC.to_owned(),
    )])
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Committed {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_gaap::Committed",
        "creusot",
        || committed_proof().to_string(),
    )
}

// `reject`'s/`rollback`'s own claims are legitimately trivial (`result.
// is_ok()`, matching every `Stoplight` edge's own shape) -- no isolated
// Pearlite predicate to name here the way `Validated`'s/`Committed`'s
// own combined claims do, matching `Pending`'s/`AccountsDistinct`'s/
// `BalancedEntries`'s own trivial `Witness<CreusotVerifier>` impls
// above. `GAAP_LEDGER_PLAN.md`'s Step 7, revisited: connected on
// Creusot/Verus for the first time, closing the asymmetry `Stoplight`'s
// own equally trivial edges never had.
#[cfg(creusot)]
impl Witness<CreusotVerifier> for Rejected<Pending> {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[cfg(creusot)]
impl Witness<CreusotVerifier> for Rejected<Validated> {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

amenable_derive::harness! {
    creusot, VALIDATED_RESULT_HOLDS_SRC, {
        /// `Ledger::validate`'s real generated `Exchange` companion's own
        /// whole-`Result` postcondition (`amenable_gaap::ledger`'s
        /// `capture_exchange_body(creusot_ensures = ..)` attribute) --
        /// named once here rather than restated inline in the generated
        /// companion's own `#[ensures(..)]` clause, composing
        /// `amount_positive_holds`/`accounts_distinct_holds`/
        /// `sufficient_funds_holds` (each already named) the same way
        /// `validated_holds` composes them for the isolated `validate`
        /// proof function above -- a distinct fn since this one matches
        /// over the real `Result<Transfer<..>, TransferError>` shape the
        /// generated companion actually returns, not `TransferOutcome`.
        /// `pub`, not private: Creusot's proof-transparency check
        /// requires everything an `#[ensures(..)]` clause touches to be
        /// at least as visible as the function carrying it, and the
        /// generated companion's clause sits on the real, `pub`
        /// `Exchange::exchange` trait method (see `Amount::value`'s own
        /// doc comment for the same real constraint hit earlier in this
        /// file).
        #[logic(open)]
        pub fn validated_result_holds(
            outcome: Result<Transfer<Validated, ValidatedToken>, TransferError>,
        ) -> bool {
            pearlite! {
                match outcome {
                    Ok(validated) => {
                        amount_positive_holds(validated.payload.amount.0, true)
                            && accounts_distinct_holds(validated.payload.from, validated.payload.to, true)
                    }
                    Err(TransferError::NegativeAmount(bad)) => amount_positive_holds(bad, false),
                    Err(TransferError::InsufficientFunds { balance, required }) => {
                        sufficient_funds_holds(balance, required, false)
                    }
                    Err(TransferError::SameAccount) => true,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::generated::validate",
        "creusot",
        "ensures",
        || VALIDATED_RESULT_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, COMMITTED_RESULT_HOLDS_SRC, {
        /// `Ledger::commit`'s real generated `Exchange` companion's own
        /// whole-`Result` postcondition -- same reasoning as
        /// `validated_result_holds`, composing `committed_amount_holds`
        /// (already named) with the `Err` arm's own `false` (`commit`
        /// never actually fails, matching `amenable_kani::ledger::
        /// Ledger::commit`'s own claim). `pub`, not private: see
        /// `validated_result_holds`'s own doc comment for why.
        #[logic(open)]
        pub fn committed_result_holds(
            outcome: Result<Transfer<Committed, CommittedToken>, TransferError>,
        ) -> bool {
            pearlite! {
                match outcome {
                    Ok(committed) => committed_amount_holds(committed.payload.amount.0),
                    Err(_) => false,
                }
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ledger::generated::commit",
        "creusot",
        "ensures",
        || COMMITTED_RESULT_HOLDS_SRC,
    )
}

// `Ledger::validate`'s/`::commit`'s/`::reject`'s/`::rollback`'s real
// bodies -- generated by `amenable::creusot_export` from `amenable_
// core::ExchangeEdgeRecord`, not hand-written or hand-copied, matching
// `amenable_creusot::stoplight`'s own three edges exactly (`GAAP_LEDGER_
// PLAN.md`'s Step 6). `include!`, not `mod`: shares this file's own
// scope directly (`Ledger`/`Transfer`/`TransferError`/`Pending`/
// `Validated`/`Committed`/`Rejected`/tokens above, already in scope).
// Regenerate with `just generate-creusot` after changing a real
// Kani-side transition; do not hand-edit the included files.
include!("../generated/validate.rs");
include!("../generated/commit.rs");
include!("../generated/reject.rs");
include!("../generated/rollback.rs");
