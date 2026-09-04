//! `KaniCompose` for `amenable_gaap`'s real domain types -- can't be a
//! `#[derive(KaniCompose)]` on the struct definitions themselves, the
//! same neutral-crate reason `Ensures<KaniVerifier>`/`Witness<
//! KaniVerifier>` for these same types live in `super` rather than in
//! `amenable_gaap`: deriving directly on `AccountId`/`Account`/`Amount`/
//! `TransferPayload` would force `amenable_gaap` to depend on
//! `amenable_kani` (where `KaniCompose` itself lives), the identical
//! backend-inversion this project has already caught and reverted
//! twice. Hand-written here instead, field-by-field, the same shape
//! `#[derive(KaniCompose)]` would generate if it could cross the crate
//! boundary -- `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s "Reusing
//! `KaniCompose` for non-trivial carriers" follow-on.
//!
//! No bridging re-export needed: nothing in `super` (the
//! `kani_ensures!`/`kani_requires!` calls and `Witness<KaniVerifier>`
//! impls, all unconditional) names any of `Establish`/`Account`/`Amount`/
//! `PendingToken`/`TransferPayload`/`Uuid`/`KaniCompose`/`validated_from`,
//! and the `KaniCompose` trait impls are globally visible the moment
//! they're compiled.

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
/// run) -- this whole module already only exists under `cfg(kani)`.
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
