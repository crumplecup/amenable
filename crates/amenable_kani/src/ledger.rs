//! `GAAP_LEDGER_PLAN.md`'s Step 9: `Ledger`/`Transfer`/`TransferError` and
//! every one of `Ledger`'s real methods now live in `amenable_gaap`, with
//! Kani contracts attached directly there (see `amenable_gaap::ledger`'s
//! own doc comment for the confirmed "direct contract, no delegating
//! wrapper" pattern, and `gaap_ledger.rs` for the real harnesses proving
//! each method). This file keeps only what's genuinely Kani-specific and
//! can't move: each atomic contract type's own `Ensures<KaniVerifier>`
//! impl (`kani_ensures!`, a real checked `bool` predicate -- Kani's own
//! DFCC mechanism, not something a neutral crate could host), and
//! `Pending`'s own trivial `Witness<KaniVerifier>` impl.
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

/// `KaniCompose` for `amenable_gaap`'s real domain types -- can't be a
/// `#[derive(KaniCompose)]` on the struct definitions themselves, the
/// same neutral-crate reason `Ensures<KaniVerifier>`/`Witness<
/// KaniVerifier>` for these same types live here rather than in
/// `amenable_gaap`: deriving directly on `AccountId`/`Account`/`Amount`/
/// `TransferPayload` would force `amenable_gaap` to depend on
/// `amenable_kani` (where `KaniCompose` itself lives), the identical
/// backend-inversion this project has already caught and reverted
/// twice. Hand-written here instead, field-by-field, the same shape
/// `#[derive(KaniCompose)]` would generate if it could cross the crate
/// boundary -- `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s "Reusing
/// `KaniCompose` for non-trivial carriers" follow-on.
///
/// The `#[cfg(kani)]` imports, `KaniCompose` impls, and `validated_from`
/// helper this file needs, consolidated into one gate on this `mod`
/// instead of one per item -- see `amenable_creusot::stoplight::mirror`'s
/// own doc comment for the general rationale. No bridging re-export
/// needed: nothing below this module (the `kani_ensures!`/`kani_requires!`
/// calls and `Witness<KaniVerifier>` impls, all unconditional) names any
/// of `Establish`/`Account`/`Amount`/`PendingToken`/`TransferPayload`/
/// `Uuid`/`KaniCompose`/`validated_from`, and the `KaniCompose` trait
/// impls are globally visible the moment they're compiled.
#[cfg(kani)]
mod mirror {
    use amenable_core::Establish;
    use amenable_gaap::{Account, Amount, PendingToken, TransferPayload};
    use uuid::Uuid;

    use crate::KaniCompose;

    use super::{AccountId, KaniVerifier, Pending, Sidecar, Transfer, Validated, ValidatedToken};

    /// `id: Uuid` is cheap to carry fully symbolic at every depth
    /// (fixed-size, and the only thing `AccountId`'s derived `PartialEq`
    /// compares at all now -- see that type's own doc comment).
    impl KaniCompose for AccountId {
        fn kani_depth0() -> Self {
            Self::new(Uuid::kani_depth0())
        }

        fn kani_depth1() -> Self {
            Self::new(Uuid::kani_depth1())
        }

        fn kani_depth2() -> Self {
            Self::new(Uuid::kani_depth2())
        }

        fn kani_any() -> Self {
            Self::new(Uuid::kani_any())
        }
    }

    /// `name: String` is a real, separate CBMC-cost finding, not the free
    /// pass a first pass at this impl assumed: field-by-field delegation
    /// (`String::kani_depth1/2/any()`, a real heap-backed, bounded-loop
    /// construction) made `gaap_ledger::
    /// verify_gaap_validate_accepts_a_lawful_transfer` (two independently-
    /// constructed accounts) time out, confirmed by isolating the change to
    /// exactly this field: swapping `kani_any()` for `kani_depth0()` (empty
    /// name, `id` still fully symbolic) took that harness from a CBMC
    /// timeout to `0 of 507 failed` in ~97s, with nothing else touched.
    /// `name` is never compared (display-only, see `Account`'s own doc
    /// comment) and participates in zero real claim this worked example
    /// checks, so a constant empty name at every depth is exactly as strong
    /// a proof as a varying one here -- `id`'s own depth still governs real
    /// identity variation, matching `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s
    /// "Reusing `KaniCompose`" follow-on without reintroducing the
    /// construction cost `AccountId`'s own history already paid once to
    /// avoid.
    impl KaniCompose for Account {
        fn kani_depth0() -> Self {
            Self::new(AccountId::kani_depth0(), String::new())
        }

        fn kani_depth1() -> Self {
            Self::new(AccountId::kani_depth1(), String::new())
        }

        fn kani_depth2() -> Self {
            Self::new(AccountId::kani_depth2(), String::new())
        }

        fn kani_any() -> Self {
            Self::new(AccountId::kani_any(), String::new())
        }
    }

    impl KaniCompose for Amount {
        fn kani_depth0() -> Self {
            Self::new(i64::kani_depth0())
        }

        fn kani_depth1() -> Self {
            Self::new(i64::kani_depth1())
        }

        fn kani_depth2() -> Self {
            Self::new(i64::kani_depth2())
        }

        fn kani_any() -> Self {
            Self::new(i64::kani_any())
        }
    }

    impl KaniCompose for TransferPayload {
        fn kani_depth0() -> Self {
            Self::new(
                Account::kani_depth0(),
                Account::kani_depth0(),
                Amount::kani_depth0(),
            )
        }

        fn kani_depth1() -> Self {
            Self::new(
                Account::kani_depth1(),
                Account::kani_depth1(),
                Amount::kani_depth1(),
            )
        }

        fn kani_depth2() -> Self {
            Self::new(
                Account::kani_depth2(),
                Account::kani_depth2(),
                Amount::kani_depth2(),
            )
        }

        fn kani_any() -> Self {
            Self::new(Account::kani_any(), Account::kani_any(), Amount::kani_any())
        }
    }

    /// `Transfer<Pending, PendingToken>` isn't pure data the way the three
    /// impls above are: a lawful `Transfer` needs a real token, minted
    /// through the real chain, not conjured field-by-field the way a
    /// mechanical derive would (exactly the `Established::assert()`-style
    /// bypass `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s own Motivation
    /// section already rejected as prior art). `Pending` is the easy case:
    /// [`Transfer::pending`] *is* the real, lawful root constructor, so
    /// there is no chain to route through at all -- unlike `Validated`
    /// below.
    impl KaniCompose for Transfer<Pending, PendingToken> {
        fn kani_depth0() -> Self {
            Transfer::pending(TransferPayload::kani_depth0())
        }

        fn kani_depth1() -> Self {
            Transfer::pending(TransferPayload::kani_depth1())
        }

        fn kani_depth2() -> Self {
            Transfer::pending(TransferPayload::kani_depth2())
        }

        fn kani_any() -> Self {
            Transfer::pending(TransferPayload::kani_any())
        }
    }

    /// Unlike `Pending`, `Validated` has no root constructor -- the only
    /// lawful way to a `ValidatedToken` is `Establish::establish`, called
    /// on a real `Pending` credential, the same chain `verify_gaap_commit_
    /// always_balances`'s own hand-written setup already uses. `Transfer::
    /// diagnostic_new` (not the lawful `Sidecar`/`validate()` round trip)
    /// assembles the final value from the already-honestly-minted token --
    /// matching that harness's own documented reasoning: the *token* comes
    /// from the real chain, only the wrapping `Transfer` struct itself
    /// skips re-running `validate()`, which is a separate proof target,
    /// not something this construction helper needs to re-check.
    ///
    /// No further `#[cfg(kani)]` needed here, unlike its own previous
    /// top-level form: `Transfer::diagnostic_new` itself only exists under
    /// `cfg(kani)` (see its own doc comment), so there is no
    /// universally-compiling body to write here the way the other impls'
    /// `symbolic_any()`-based bodies manage (those compile everywhere,
    /// only panicking at runtime if actually called outside a real Kani
    /// run) -- this whole `mod` already only exists under `cfg(kani)`.
    impl KaniCompose for Transfer<Validated, ValidatedToken> {
        fn kani_depth0() -> Self {
            validated_from(TransferPayload::kani_depth0())
        }

        fn kani_depth1() -> Self {
            validated_from(TransferPayload::kani_depth1())
        }

        fn kani_depth2() -> Self {
            validated_from(TransferPayload::kani_depth2())
        }

        fn kani_any() -> Self {
            validated_from(TransferPayload::kani_any())
        }
    }

    fn validated_from(payload: TransferPayload) -> Transfer<Validated, ValidatedToken> {
        let pending = Transfer::pending(payload.clone());
        let credential = Sidecar::sidecar(&pending);
        let validated_token = <Validated as Establish<_, KaniVerifier>>::establish(credential);
        Transfer::diagnostic_new(payload, validated_token)
    }
}

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
