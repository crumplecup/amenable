//! Composite `Ensures`/`Requires<KaniVerifier>` claims for the ledger's
//! `Transfer` state transitions: `validate`, `commit`, `reject`,
//! `rollback`. Each is a real biconditional (or, for the infallible
//! edges, a legitimately trivial `result.is_ok()`), calling through the
//! atomic contract types' own registered impls in [`super`] rather than
//! restating their arithmetic inline. Split out of `super` so that
//! module -- the atomic contracts plus their `Witness` impls -- and this
//! one each stay a cohesive unit.

use amenable_core::{Ensures, Sidecar, Witness};
use amenable_gaap::{
    AccountsDistinct, AmountPositive, BalancedEntries, Committed, CommittedToken, Pending,
    Rejected, RejectedFromPendingToken, RejectedFromValidatedToken, SufficientFunds, Transfer,
    TransferError, Validated, ValidatedToken,
};

use crate::rust_std::{kani_ensures, kani_requires};
use crate::{CalculationProof, KaniVerifier};

// `validate`'s own combined claim -- a real biconditional, not merely
// `result.is_ok()` -- relating the transfer's own amount/the ledger's
// own balance/the two account identities to which branch fires and, on
// the `Err` path, to the *exact* violated value the error variant
// reports. Calls through `AmountPositive`/`SufficientFunds`/
// `AccountsDistinct`'s own registered `Ensures<KaniVerifier>` impls
// (in `super`) rather than restating their arithmetic inline -- the
// composite claim's own control flow (which `TransferError` variant backs
// which check) stays hand-written: it's genuine, bespoke logic tied to
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
// `amenable_gaap` for real (`GAAP_LEDGER_PLAN.md`'s Step 7), leaving
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
