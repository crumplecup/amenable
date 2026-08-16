//! Gallery case: `Evidence`'s real, deliberate "a root is its own basis"
//! idiom (`type Basis = Self`, stated in `Evidence`'s own doc comment)
//! against Verus's static cyclic-self-reference checker.
//!
//! **Disposition: best practice, confirmed.** **Expected/actual
//! outcome: passes, for real, unmasked.**
//!
//! ## The finding
//!
//! Verus's own `vir::recursive_types::check_recursive_types` (confirmed
//! by reading `~/repos/verus/source/vir/src/recursive_types.rs` directly,
//! not assumed) walks a global type graph over every `Datatype`/`Trait`/
//! `TraitImpl` in the crate and unconditionally rejects any `TraitImpl`
//! participating in a cycle -- no per-item escape hatch exists in that
//! code path (`#[verifier::external_body]` doesn't apply to trait impls
//! at all; `#[verifier::external]` compiles but crashes Verus's own AIR
//! backend with an internal panic once one piece of an interlinked trait
//! graph is externalized while the rest isn't -- both confirmed
//! empirically, not assumed). A self-referential root -- `impl Evidence
//! for Green { type Basis = Self; .. }`, with `Evidence`'s own `type
//! Basis: Evidence` bound -- is exactly this shape: `TraitImpl(Evidence
//! for Green)` depends on `Trait(Evidence)` via the `Basis: Evidence`
//! bound, and `Basis = Self = Green` closes the loop back to the same
//! `TraitImpl` node.
//!
//! The real, confirmed root cause is the bound itself (`type Basis:
//! Evidence`), independent of `Evidence::chain()`'s own recursive
//! default method (which also touches `Self::Basis` but turned out not
//! to matter on its own -- see "A real methodological mistake" below).
//! `amenable_core::evidence` now declares `Evidence` twice, `#[cfg(..
//! verus_keep_ghost)]`-exclusive: the real, unchanged shape (`Basis:
//! Evidence` bound plus `chain()`) for every ordinary toolchain, and a
//! second declaration with no bound on `Basis` at all (and no `chain()`,
//! since nothing left needs to walk that bound) compiled only under
//! Verus's own driver, which unconditionally sets `--cfg
//! verus_keep_ghost` (confirmed by reading `rust_verify/src/driver.rs`
//! in the real `verus-lang/verus` source). Exactly one declaration is
//! ever compiled in a given build, so there is no naming conflict, and
//! every ordinary caller (`amenable_kani::tests::calculation_test`,
//! `amenable_derive::tests::standard_fixture_corpus_test`, both real and
//! passing) sees zero change.
//!
//! ## A real methodological mistake, corrected
//!
//! The first fix attempt (cfg-gating only `Evidence::chain()`'s
//! recursive default method, leaving the `Basis: Evidence` bound
//! itself alone) looked sufficient in testing and was reported as
//! resolved. That conclusion was a false positive: Verus's own error
//! reporting stops at the *first* cyclic definition it finds in a
//! compilation, and every one of those "passing" tests still had the
//! real, un-fixed `Evidence` trait present and failing *elsewhere* in
//! the same file -- masking whatever the thing actually under test was
//! doing. `impl_tuple_evidence!`'s tuple impls turned out to be a real,
//! independent second contributor to that same masking (`type Basis =
//! (A::Basis, B::Basis)` maps one 2-tuple back onto another of the
//! identical structural shape, flagged by the same checker regardless
//! of instantiation -- confirmed separately, also load-bearing for
//! ordinary builds via `#[calculation]`'s multi-argument `Basis`, so
//! also cfg-gated rather than removed).
//!
//! Only a fully isolated, single-variable re-test -- nothing else in the
//! file that could be checked first -- caught the mistake and surfaced
//! the real cause. The lesson generalizes: a Verus "this compiles clean"
//! result is only trustworthy when nothing else able to trigger the
//! *same class* of error coexists in the same compilation unit.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

use crate::{Establish, Evidence, ProofToken, Verifier, Witness};

verus! {

/// Local marker, standing in for what would be `crate::VerusVerifier` in
/// a real design (mirroring `amenable_kani::KaniVerifier`/
/// `amenable_creusot::CreusotVerifier`).
pub struct GalleryVerifier;

impl Verifier for GalleryVerifier {
    type Metadata = GalleryVerifierMetadata;

    fn name() -> &'static str {
        "gallery"
    }
}

#[derive(Default)]
pub struct GalleryVerifierMetadata;

// Incidental plumbing (`Verifier::Metadata` needs *some* `Provenance`
// impl to exist, but nothing in this case's own claim calls into it).
#[verifier::external]
impl crate::Provenance for GalleryVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<crate::MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

/// The genuine self-referential root: `Basis = Self`, exactly the real
/// `amenable_kani::stoplight::Green`/`amenable_creusot::stoplight::Green`
/// shape.
pub struct Green;

impl Evidence for Green {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Green
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

impl Witness<GalleryVerifier> for Green {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// A non-root evidence type whose `Basis` points *at* the root -- the
/// other real, load-bearing shape (`Yellow`'s `Basis = Green` in the
/// real `Stoplight` proofs), confirming the fix doesn't just tolerate
/// roots but keeps ordinary chains working too.
pub struct Yellow;

impl Evidence for Yellow {
    type Basis = Green;
    type Audit = ();

    fn basis() -> Self::Basis {
        Green
    }

    fn audit(&self) -> Self::Audit {}
}

impl Witness<GalleryVerifier> for Yellow {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

#[derive(Clone, Copy)]
pub struct GreenToken;

impl ProofToken for GreenToken {
    type Proposition = Green;
}

#[derive(Clone, Copy)]
pub struct YellowToken;

impl ProofToken for YellowToken {
    type Proposition = Yellow;
}

/// `Establish<C, V>: Evidence + Witness<V>` is the same compound bound
/// `Sidecar<V>` needs -- confirmed working here too, for a transition
/// whose target (`Yellow`) is not itself a root, minted from a
/// credential (`GreenToken`) whose proposition (`Green`) *is*.
impl Establish<GreenToken, GalleryVerifier> for Yellow {
    type Token = YellowToken;

    fn establish(_credential: GreenToken) -> Self::Token {
        YellowToken
    }
}

/// The real, load-bearing check: real, unmodified `Evidence`/`Witness`/
/// `ProofToken`/`Establish` (mod-included via `lib.rs`, `Witness` via
/// `witness_accommodation` -- see that module's own doc comment for why
/// it's a hand-trimmed mirror rather than the genuinely real file),
/// exercising a genuine self-referential root end to end.
pub fn establish_yellow_from_green(credential: GreenToken) -> (token: YellowToken)
    ensures
        true,
{
    Yellow::establish(credential)
}

} // verus!
