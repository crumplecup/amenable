use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

use crate::{Evidence, Verifier, Witness};
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

verus! {

pub struct GalleryVerifier;

impl Verifier for GalleryVerifier {
    type Metadata = GalleryVerifierMetadata;

    fn name() -> &'static str {
        "gallery"
    }
}

#[derive(Default)]
pub struct GalleryVerifierMetadata;

#[verifier::external]
impl crate::Provenance for GalleryVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<crate::MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

/// Local mirrors of `amenable_gaap::contracts::{AmountPositive,
/// SufficientFunds, AccountsDistinct, BalancedEntries}` -- `verus
/// --crate-type=lib` resolves no extern crate at all (this crate's own
/// `lib.rs` doc comment explains why), so, matching every other type in
/// this file, these can't be the real types either. `GAAP_LEDGER_
/// PLAN.md`'s Step 5: these four back real `Ensures<GalleryVerifier>`
/// impls (below, past the `verus! {}` block, next to `Validated`'s/
/// `Committed`'s own `verus_ensures!` calls, which now call through
/// them instead of restating the arithmetic) -- the same real gap
/// closed on Kani and Creusot, previously open here too. `Evidence`/
/// `Witness<GalleryVerifier>` still needed even though nothing
/// establishes a token *from* any of these four: `Ensures<V>: Witness<
/// V>` is a hard supertrait bound, so an `Ensures` impl can't exist
/// without one, matching `Pending`'s own trivial `Witness` impl right
/// below for the identical structural reason.
pub struct AmountPositive;

impl Evidence for AmountPositive {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        AmountPositive
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for AmountPositive {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

pub struct SufficientFunds;

impl Evidence for SufficientFunds {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        SufficientFunds
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for SufficientFunds {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

pub struct AccountsDistinct;

impl Evidence for AccountsDistinct {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        AccountsDistinct
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for AccountsDistinct {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

pub struct BalancedEntries;

impl Evidence for BalancedEntries {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        BalancedEntries
    }

    fn audit(&self) {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for BalancedEntries {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

} // verus!
