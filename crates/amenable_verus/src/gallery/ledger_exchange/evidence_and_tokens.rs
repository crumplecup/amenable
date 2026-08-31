use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::{Evidence, Witness};
// `#[cfg(verus_keep_ghost)]`-gated, matching `amenable_core::evidence`'s
// own precedent: `AmountPositive::ensures(..)` (etc., below) resolves
// fine under ordinary `cargo check`/clippy without this import (`Type::
// trait_fn()` path syntax doesn't require the trait in scope the way
// `.method()` calls do), but real `verus`'s own driver -- which
// unconditionally sets `--cfg verus_keep_ghost` -- needs it, confirmed
// against the real toolchain: a real "function or associated item
// `ensures` not found" error without it.
#[cfg(verus_keep_ghost)]
use crate::Ensures;

use super::verifier_and_bounds::GalleryVerifier;

verus! {

/// The transfer is awaiting validation -- a root state claim, asserted
/// rather than derived, matching `amenable_gaap::Pending`'s own real
/// doc comment: every `Transfer` starts here, and starting here is
/// asserted by construction, not proven.
pub struct Pending;

impl Evidence for Pending {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Pending
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

/// Hand-written, matching `amenable_kani::ledger::Pending`'s own
/// `Witness<KaniVerifier>` impl exactly (see its own doc comment):
/// unlike `Validated`/`Committed`, which get their `Witness<
/// GalleryVerifier>` impl "for free" from `verus_exchange!` (each
/// targets one as its edge's own evidence), nothing in this worked
/// example's scope ever targets `Pending` -- a transfer only ever
/// starts there, never returns to it -- so nothing generates one.
/// Honestly trivial: there is no computation to prove about the fact
/// that a new transfer starts `Pending`.
impl Witness<GalleryVerifier> for Pending {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// The transfer has been validated -- see [`Pending`] for why this is a
/// root claim, not a derived one, even though in practice a transfer
/// only reaches `Validated` via a proven `Pending -> Validated`
/// exchange.
pub struct Validated;

impl Evidence for Validated {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Validated
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

/// The transfer has been committed -- see [`Pending`].
pub struct Committed;

impl Evidence for Committed {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Committed
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

/// The transfer was rejected -- validation failed (`Rejected<Pending>`),
/// or a validated transfer was manually rolled back before commit
/// (`Rejected<Validated>`). Matches `amenable_gaap::Rejected<T>`'s own
/// shape: parameterized by the state it was rejected *from*, not flat
/// (`reject()`/`rollback()` each need their own distinct concrete
/// `Witness<GalleryVerifier>` proof for their own real claim -- see the
/// real type's own doc comment in `amenable_gaap::transfer` for the full
/// `E0119` account). `GAAP_LEDGER_PLAN.md`'s Step 7, revisited: neither
/// edge was connected here at first (a real scope call, not a technical
/// wall), closed once `validate`'s/`commit`'s own connection proved the
/// underlying mechanism out. A blanket `impl<T> Evidence for Rejected<
/// T>` (no per-`T` root claim to state, unlike `amenable_gaap`'s own
/// real `#[derive(Standard)]`-generated impl, which is conditional on
/// `Self: Provenance` -- this mirror carries no `Provenance` chain at
/// all, matching `Pending`'s/`Validated`'s/`Committed`'s own mirrors
/// right above, which skip it too): `Rejected<T>` is a root claim
/// exactly like every other evidence type in this file, for every `T`
/// this gallery ever instantiates it with.
pub struct Rejected<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Evidence for Rejected<T> {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Rejected {
            _marker: std::marker::PhantomData,
        }
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

// `PendingToken`/`ValidatedToken`/`CommittedToken` and their `ProofToken`/
// `Establish<_, GalleryVerifier>` impls used to be hand-written here --
// see `GAAP_LEDGER_PLAN.md`'s Step 8: they're now generated, `include!`d
// near the bottom of this file alongside `validate.rs`/`commit.rs` (see
// that `include!`'s own comment for exactly why it has to live *outside*
// this file's own `verus! { .. }` block, not in-line here).


} // verus!
