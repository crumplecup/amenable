//! `GAAP_LEDGER_PLAN.md`'s Step 9: `Ledger`/`Transfer`/`TransferError` and
//! every one of `Ledger`'s real methods now live in `amenable_gaap`, with
//! Kani contracts attached directly there (see `amenable_gaap::ledger`'s
//! own doc comment for the confirmed "direct contract, no delegating
//! wrapper" pattern, and `gaap_ledger.rs` for the real harnesses proving
//! each method). This module keeps only what's genuinely Kani-specific
//! and can't move: each atomic contract type's own `Ensures<KaniVerifier>`
//! impl (`kani_ensures!`, a real checked `bool` predicate -- Kani's own
//! DFCC mechanism, not something a neutral crate could host), and
//! `Pending`'s own trivial `Witness<KaniVerifier>` impl. The `KaniCompose`
//! impls for these same domain types (another Kani-only concern) live in
//! [`mirror`].
//!
//! `AccountId` is a bare `Uuid` identity (not the `String` it started
//! with in `GAAP_LEDGER_PLAN.md`'s Step 0, and not the combined id+name
//! struct it became after that: see `amenable_gaap::transfer::AccountId`'s
//! own doc comment for why it's now split from `Account`) precisely
//! because of a real CBMC cost the first version of this proof hit:
//! comparing two independently-constructed `String`s for equality
//! *inside a `#[kani::ensures]` closure* is expensive regardless of
//! content or length -- fully root-caused via `amenable_kani::gallery::
//! ledger_account_id_comparison`'s own investigation, which also
//! confirmed a *fixed-capacity* string (bounded buffer + a length
//! field) is exactly as expensive, so bounding the name wouldn't have
//! helped. `Uuid`'s 16-byte, fixed-length comparison is cheap in the
//! identical position.

use amenable_core::{Ensures, Sidecar, Witness};
use amenable_gaap::{
    AccountId, AccountsDistinct, AmountPositive, BalancedEntries, Committed, CommittedToken,
    Pending, Rejected, RejectedFromPendingToken, RejectedFromValidatedToken, SufficientFunds,
    Transfer, TransferError, Validated, ValidatedToken,
};

use crate::gaap_ledger::{
    VERIFY_GAAP_CHECK_AMOUNT_POSITIVE_SRC, VERIFY_GAAP_CHECK_SUFFICIENT_FUNDS_SRC,
};
use crate::rust_std::macros::{kani_ensures, kani_requires};
use crate::{CalculationProof, KaniVerifier};

#[cfg(kani)]
mod mirror;

/// `Pending`'s own trivial witness. Unlike `stoplight::Green`, which
/// gets its `Witness<KaniVerifier>` impl "for free" from the `Red ->
/// Green` cycle-back edge (`Green` is also an edge *target* in that
/// cycle), nothing in this worked example's initial scope ever targets
/// `Pending` — a transfer only ever starts there, never returns to it.
/// So nothing auto-generates one via `#[amenable_derive::exchange]`;
/// this is hand-written, and honestly trivial: there is no computation
/// to prove about the fact that a new transfer starts `Pending`, the
/// same way there's none for `Green`'s own power-on claim.
impl Witness<KaniVerifier> for Pending {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}
}

// The real, canonical claim each contract type in `amenable_gaap::
// contracts` names -- previously dead code workspace-wide (real
// `Evidence` types since `GAAP_LEDGER_PLAN.md`'s Step 0, but nothing
// anywhere ever imported or referenced them; every proof independently
// re-derived the same claims by name-matching convention only). Every
// consumer -- `amenable_gaap::Ledger::check_amount_positive`'s/`::check_
// sufficient_funds`'s own DFCC contracts, and `Validated`'s/`Committed`'s
// combined `kani_ensures!` claims below -- calls through these `Ensures<
// KaniVerifier>` impls now, the same "generation covers the mechanical
// wiring, the registered impl carries the one real claim" discipline
// `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 6 already established one
// level up (`Validated`/`Committed` calling through their own registered
// impls, instead of restating their bound inline). `AccountsDistinct`/
// `BalancedEntries` get a trivial `Witness<KaniVerifier>` (matching
// `Pending`'s own precedent above): neither has its own isolated Kani
// harness the way `AmountPositive`/`SufficientFunds` do (accounts-
// distinct is checked inline in `validate`'s own body; balanced-entries
// inline in `commit`'s), so there's no dedicated proof artifact to
// honestly report.
kani_ensures!(
    AmountPositive,
    "amenable_gaap::AmountPositive::ensures",
    i64,
    |amount| amount > 0
);

impl Witness<KaniVerifier> for AmountPositive {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "gaap_ledger::verify_gaap_check_amount_positive".to_owned(),
            VERIFY_GAAP_CHECK_AMOUNT_POSITIVE_SRC.to_owned(),
        )
    }
}

kani_ensures!(
    SufficientFunds,
    "amenable_gaap::SufficientFunds::ensures",
    (i64, i64),
    |(balance, amount)| balance >= amount
);

impl Witness<KaniVerifier> for SufficientFunds {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "gaap_ledger::verify_gaap_check_sufficient_funds".to_owned(),
            VERIFY_GAAP_CHECK_SUFFICIENT_FUNDS_SRC.to_owned(),
        )
    }
}

kani_ensures!(
    AccountsDistinct,
    "amenable_gaap::AccountsDistinct::ensures",
    (AccountId, AccountId),
    |(from, to)| from != to
);

impl Witness<KaniVerifier> for AccountsDistinct {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}
}

// `i128`-widened, matching `gallery::ledger_exchange`'s own Verus
// predicate: avoids needing any precondition to keep `-amount` from
// overflow-panicking at `i64::MIN`, since `i128`'s range is vastly
// larger than `i64`'s -- genuinely stronger than restating the claim in
// bounded `i64` space the way `Committed`'s own combined claim used to
// (see that `kani_ensures!` call's own comment).
kani_ensures!(
    BalancedEntries,
    "amenable_gaap::BalancedEntries::ensures",
    i64,
    |amount| (-(amount as i128)) + (amount as i128) == 0
);

impl Witness<KaniVerifier> for BalancedEntries {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {}
}

// `validate`'s own combined claim -- a real biconditional, not merely
// `result.is_ok()` -- relating the transfer's own amount/the ledger's
// own balance/the two account identities to which branch fires and, on
// the `Err` path, to the *exact* violated value the error variant
// reports. Calls through `AmountPositive`/`SufficientFunds`/
// `AccountsDistinct`'s own registered `Ensures<KaniVerifier>` impls
// (above) rather than restating their arithmetic inline -- the composite
// claim's own control flow (which `TransferError` variant backs which
// check) stays hand-written: it's genuine, bespoke logic tied to
// `TransferError`'s real shape, not a mechanically-derivable pattern the
// way each atomic contract's own bound is.
kani_ensures!(
    Validated,
    "amenable_kani::ledger::Validated::validate_ensures",
    Result<Transfer<Validated, ValidatedToken>, TransferError>,
    |result| match result {
        Ok(validated) => {
            let payload = validated.primary();
            AmountPositive::ensures(payload.amount().value())
                && AccountsDistinct::ensures((payload.from().id(), payload.to().id()))
        }
        Err(TransferError::NegativeAmount(amount)) => !AmountPositive::ensures(amount),
        Err(TransferError::InsufficientFunds { balance, required }) => {
            !SufficientFunds::ensures((balance, required))
        }
        Err(TransferError::SameAccount) => true,
    }
);

// `commit`'s own precondition, sewn to `validate`'s postcondition rather
// than restated: both ultimately rest on the same real, registered
// `AmountPositive` claim -- `validate`'s `Ensures<KaniVerifier>` impl
// above calls through it to check its *output*; this `Requires<
// KaniVerifier>` impl calls through the identical claim to check
// `commit`'s *input* -- because a `Validated`-carrying `Transfer` is
// exactly the value that flows from one edge's output position into the
// next edge's input position, the same real fact serves both roles, not
// two independently hand-typed copies with nothing enforcing they agree
// (`commit`'s own precondition used to be a hand-typed inline expression,
// `input.primary().amount().value() > 0`, restating what `AmountPositive`
// already states once). See `amenable_gaap::ledger::Ledger::commit`'s own
// `#[amenable_derive::capture_exchange_body(kani_requires_evidence =
// "Validated", ..)]` for where this gets wired in.
kani_requires!(
    Validated,
    "amenable_kani::ledger::Validated::commit_requires",
    Transfer<Validated, ValidatedToken>,
    |input| AmountPositive::ensures(input.primary().amount().value())
);

// `Witness<KaniVerifier>` for `Validated`/`Committed`/`Rejected<Pending>`/
// `Rejected<Validated>` used to come "for free" from `#[amenable_derive::
// exchange(..)]`'s own generated `impl Witness<#verifier> for #evidence`
// (attached to `validate`/`commit`/`reject`/`rollback`'s own impl
// blocks) -- now hand-written here, since those methods' real bodies (and
// the `exchange` attribute that used to sit on them) moved to
// `amenable_gaap` for real (`GAAP_LEDGER_PLAN.md`'s Step 9), leaving
// nothing in this crate to generate them anymore. `Ensures<V>: Witness<
// V>` is a hard supertrait bound, so each `kani_ensures!` call above
// cannot compile without a matching one of these.
impl Witness<KaniVerifier> for Validated {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "gaap_ledger::verify_gaap_validate_accepts_a_lawful_transfer".to_owned(),
            crate::gaap_ledger::VERIFY_GAAP_VALIDATE_ACCEPTS_A_LAWFUL_TRANSFER_SRC.to_owned(),
        )
    }
}

// `commit`'s own claim -- `BalancedEntries`'s real claim (`debit + credit
// == 0`) is honestly trivial by construction here (`debit` is literally
// `-credit`), the same kind of triviality `Stoplight`'s own edges
// document rather than hide (zero-field states, no branching that could
// fail): naming and checking the claim is still real value.
kani_ensures!(
    Committed,
    "amenable_kani::ledger::Committed::commit_ensures",
    Result<Transfer<Committed, CommittedToken>, TransferError>,
    |result| match result {
        Ok(committed) => BalancedEntries::ensures(committed.primary().amount().value()),
        Err(_) => false,
    }
);

impl Witness<KaniVerifier> for Committed {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "gaap_ledger::verify_gaap_commit_always_balances".to_owned(),
            crate::gaap_ledger::VERIFY_GAAP_COMMIT_ALWAYS_BALANCES_SRC.to_owned(),
        )
    }
}

// `reject`/`rollback`: infallible, like every `Stoplight` edge -- the
// claim is legitimately trivial (`result.is_ok()`), the same shape every
// `Stoplight` edge already documents rather than hides -- `validate`/
// `commit` are where this worked example's real, non-trivial claims
// live.
kani_ensures!(
    Rejected<Pending>,
    "amenable_kani::ledger::Rejected::reject_ensures",
    Result<Transfer<Rejected<Pending>, RejectedFromPendingToken>, TransferError>,
    |result| result.is_ok()
);

impl Witness<KaniVerifier> for Rejected<Pending> {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "gaap_ledger::verify_gaap_reject_always_succeeds".to_owned(),
            crate::gaap_ledger::VERIFY_GAAP_REJECT_ALWAYS_SUCCEEDS_SRC.to_owned(),
        )
    }
}

kani_ensures!(
    Rejected<Validated>,
    "amenable_kani::ledger::Rejected::rollback_ensures",
    Result<Transfer<Rejected<Validated>, RejectedFromValidatedToken>, TransferError>,
    |result| result.is_ok()
);

impl Witness<KaniVerifier> for Rejected<Validated> {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CalculationProof::new(
            "gaap_ledger::verify_gaap_rollback_always_succeeds".to_owned(),
            crate::gaap_ledger::VERIFY_GAAP_ROLLBACK_ALWAYS_SUCCEEDS_SRC.to_owned(),
        )
    }
}
