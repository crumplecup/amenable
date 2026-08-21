//! Real, backend-neutral proof tokens for the `Pending -> Validated ->
//! Committed` lineage -- an experiment for `GAAP_LEDGER_PLAN.md`'s Step 7,
//! testing whether the mirror-token cascade (`amenable_creusot::ledger`'s
//! separate `PendingToken`/`ValidatedToken`/`CommittedToken`) can be
//! eliminated at the source rather than accommodated per backend.
//!
//! **Why this wasn't possible before.** `Establish<C, V>::establish`'s
//! body has to construct the token's own private field -- Rust requires
//! an inherent constructor site to share a crate with the type it
//! constructs. As long as these tokens lived only in `amenable_kani`
//! (`GAAP_LEDGER_PLAN.md`'s Step 1), only `amenable_kani` could ever mint
//! a *real* one; `amenable_creusot`/`amenable_verus` each resorted to
//! their own separate mirror token just to have something they were
//! allowed to construct.
//!
//! **The fix**: move the token type here, alongside `Pending`/`Validated`/
//! `Committed` (already real, backend-neutral types), and mint it via
//! `#[amenable_derive::establish(credential = .., proposition = ..)]`'s
//! *verifier-less* form -- one real, backend-generic `impl<V: Verifier>
//! Establish<C, V> for Y where Y: Witness<V>` per edge, gated only by
//! whichever backend has actually registered a real `Witness<V>` proof for
//! `Y` (`amenable_kani`'s/`amenable_creusot`'s own `#[cfg(..)]`-gated impls
//! elsewhere, unchanged). No verifier-specific code lives here at all.
//!
//! `PendingToken::new` is `pub`, not gated behind `Establish` at all --
//! matching `Pending`'s own status as a root claim (asserted, not proven:
//! every transfer starts here, there is no prior credential to check). The
//! "lawful construction" guarantee this whole token family exists for only
//! applies *after* the root; minting the root itself has nothing to gate.

use amenable_derive::ProofToken;

use crate::{Committed, Pending, Rejected, Validated};

/// Lawful token asserting a transfer is [`Pending`] -- the entry state,
/// minted without going through [`amenable_core::Establish::establish`]
/// the same way a root evidence type has no credential to present one
/// from.
#[derive(Debug, Clone, Default, ProofToken)]
#[proof_token(proposition = "Pending")]
pub struct PendingToken(());

impl PendingToken {
    /// Mint the root token. Public, not privacy-gated: every transfer
    /// starts `Pending`, asserted rather than derived, so there is no
    /// lawfulness condition to enforce here -- see this module's own doc
    /// comment. Takes no `Pending` argument: `Pending` is a bare, freely
    /// constructible unit struct (`#[derive(Default)]`, no fields), so a
    /// `_state: Pending` parameter here would gate nothing -- every real
    /// call site just built one fresh at the call, confirmed via a full
    /// grep of every `PendingToken::new` call before removing it.
    #[must_use]
    pub fn new() -> Self {
        Self(())
    }
}

/// Lawful token minted once [`Validated`] is established from a proven
/// [`Pending`], by any backend that has registered a real `Witness<V>`
/// proof for `Validated`.
#[derive(Debug, Clone, ProofToken)]
#[proof_token(proposition = "Validated")]
#[amenable_derive::establish(credential = "PendingToken", proposition = "Validated")]
pub struct ValidatedToken(());

impl ValidatedToken {
    /// Diagnostic-only escape hatch, not part of the lawful establish
    /// chain -- see `amenable_kani::gallery::ledger_commit_contract_
    /// timeout`'s own doc comment: constructing a `Transfer<Validated,
    /// ValidatedToken>` in a `#[kani::proof_for_contract]` harness's own
    /// setup code via the *lawful* `Sidecar::sidecar`/`Establish::
    /// establish` chain is real, structural CBMC cost (~143s even with
    /// fully concrete values), independent of `commit`'s own contract
    /// content. This bypasses that chain for harness setup only -- the
    /// CONTRACT being checked is `commit`'s own, unaffected by how the
    /// harness's *input* was assembled. `pub`, not `pub(crate)`: the
    /// crate calling this (`amenable_kani`'s gallery) is no longer the
    /// crate defining it (`GAAP_LEDGER_PLAN.md`'s Step 7 token
    /// relocation) -- `#[cfg(kani)]` is the real gate instead, relying on
    /// the same global-`--cfg` scoping this whole relocation depends on:
    /// `cargo kani -p amenable_kani` compiles this crate too, with
    /// `cfg(kani)` active, so the escape hatch only exists during a real
    /// Kani run, never in an ordinary build.
    #[cfg(kani)]
    pub fn diagnostic_only() -> Self {
        Self(())
    }
}

/// Lawful token minted once [`Committed`] is established from a proven
/// [`Validated`], by any backend that has registered a real `Witness<V>`
/// proof for `Committed`.
#[derive(Debug, Clone, ProofToken)]
#[proof_token(proposition = "Committed")]
#[amenable_derive::establish(credential = "ValidatedToken", proposition = "Committed")]
pub struct CommittedToken(());

/// Lawful token minted once `Rejected<Pending>` is established from a
/// proven [`Pending`] -- validation was never attempted (e.g. an operator
/// cancelled the request). Distinct from [`RejectedFromValidatedToken`]
/// even though both back the same logical "rejected" *outcome*:
/// `ProofToken::Proposition` is an associated type, so one concrete token
/// type can only ever name one `Proposition` -- a single shared token
/// can't serve both `Rejected<Pending>` and `Rejected<Validated>` at
/// once, the same reason `Rejected<T>` itself had to become generic (see
/// its own doc comment in `crate::transfer`).
#[derive(Debug, Clone, ProofToken)]
#[proof_token(proposition = "Rejected<Pending>")]
#[amenable_derive::establish(credential = "PendingToken", proposition = "Rejected<Pending>")]
pub struct RejectedFromPendingToken(());

/// Lawful token minted once `Rejected<Validated>` is established from a
/// proven [`Validated`] -- a validated transfer was manually rolled back
/// before commit. See [`RejectedFromPendingToken`].
#[derive(Debug, Clone, ProofToken)]
#[proof_token(proposition = "Rejected<Validated>")]
#[amenable_derive::establish(credential = "ValidatedToken", proposition = "Rejected<Validated>")]
pub struct RejectedFromValidatedToken(());
