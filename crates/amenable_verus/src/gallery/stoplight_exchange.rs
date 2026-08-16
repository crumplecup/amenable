//! Gallery case: the real `Stoplight` worked example
//! (`amenable_kani::stoplight`/`amenable_creusot::stoplight`), full
//! three-edge cycle, under Verus -- using `exchange_support`'s
//! `external_trait_specification`s (confirmed compatible with
//! `gallery::proof_token_external_trait_bound`'s own, separately-
//! declared copies existing in the same crate at the same time: Verus
//! does not treat two independently-named specification traits for the
//! same external trait as a "duplicate specification" the way its own
//! test suite's same-name/same-module cases are).
//!
//! **Disposition: best practice, confirmed.** **Expected/actual
//! outcome: passes, for real -- `385 verified, 0 errors`.**
//!
//! ## A real, new finding: no Kani-style workaround needed
//!
//! Kani 0.67.0 cannot place `#[kani::proof_for_contract]` on a trait
//! method when the trait itself is generic, which is exactly what
//! `Exchange<Input, Output, V>` is -- forcing the real logic and its
//! contract onto a plain inherent method in `amenable_kani::stoplight`,
//! with the trait impl reduced to delegation (see `EXCHANGE_PROOF_
//! DERIVATION_PLAN.md`'s Step 1). That limitation doesn't carry over to
//! Verus, confirmed empirically rather than assumed: a real `ensures`
//! clause sits directly on `impl Exchange<.., GalleryVerifier> for
//! Stoplight`'s own generic `exchange` method below, for all three
//! edges, and verifies clean. Kani's contracts are a separate,
//! DFCC-checked attribute mechanism; Verus's `ensures` is ordinary
//! function syntax, so this isn't surprising in hindsight, but it was
//! checked rather than presumed.
//!
//! ## Verified non-vacuous
//!
//! A real bug (`Err(())` swapped in for the `Green -> Yellow` edge's
//! real `Ok(..)` body) produced a real, precise failure --
//! `error: postcondition not satisfied`, pointing at the exact
//! `result.is_ok()` clause and the exact `Err(())` return that violates
//! it -- confirming the `ensures` clause is a real, checked claim on
//! this exact body, not a vacuous pass. Reverted and re-verified clean
//! afterward.
//!
//! `full_cycle` chains all three real `Exchange::exchange` calls
//! together (not a hand-rolled shortcut using the underlying `establish`
//! calls directly), matching the real `Stoplight`'s own full
//! `Green -> Yellow -> Red -> Green` cycle -- also verifies clean.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

// `exchange_support`'s `external_trait_specification`s apply crate-wide
// once compiled in, via `lib.rs`'s own `pub mod exchange_support;` --
// no explicit import needed here for Verus to pick them up.
use crate::{Establish, Evidence, Exchange, ProofToken, Sidecar, Verifier, Witness};

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

pub struct Yellow;

impl Evidence for Yellow {
    type Basis = Green;
    type Audit = ();

    fn basis() -> Self::Basis {
        Green
    }

    fn audit(&self) -> Self::Audit {}
}

pub struct Red;

impl Evidence for Red {
    type Basis = Yellow;
    type Audit = ();

    fn basis() -> Self::Basis {
        Yellow
    }

    fn audit(&self) -> Self::Audit {}
}

impl Witness<GalleryVerifier> for Yellow {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

impl Witness<GalleryVerifier> for Red {
    type SupportingEvidence = Self;
    type ProofArtifact = ();

    fn proof() -> Self::ProofArtifact {}
}

/// Backs `Establish<RedToken, GalleryVerifier> for Green` -- the
/// cycle-back edge, the same real reason `Green` (a root) still needs a
/// `Witness` impl in the real `Stoplight`: `Sidecar<V>`'s own bound
/// applies to every proposition, root or not.
impl Witness<GalleryVerifier> for Green {
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

#[derive(Clone, Copy)]
pub struct RedToken;

impl ProofToken for RedToken {
    type Proposition = Red;
}

impl Establish<GreenToken, GalleryVerifier> for Yellow {
    type Token = YellowToken;

    fn establish(_credential: GreenToken) -> Self::Token {
        YellowToken
    }
}

impl Establish<YellowToken, GalleryVerifier> for Red {
    type Token = RedToken;

    fn establish(_credential: YellowToken) -> Self::Token {
        RedToken
    }
}

impl Establish<RedToken, GalleryVerifier> for Green {
    type Token = GreenToken;

    fn establish(_credential: RedToken) -> Self::Token {
        GreenToken
    }
}

pub struct Established<T, Token> {
    pub primary: T,
    pub token: Token,
}

impl<T, Token> Established<T, Token> {
    pub fn new(primary: T, token: Token) -> Self {
        Self { primary, token }
    }
}

impl<T, Token> Sidecar<GalleryVerifier> for Established<T, Token>
where
    T: Evidence + Witness<GalleryVerifier>,
    Token: ProofToken<Proposition = T> + Copy,
{
    type Primary = T;
    type Proposition = T;
    type SidecarToken = Token;

    fn primary(&self) -> &Self::Primary {
        &self.primary
    }

    fn sidecar(&self) -> Self::SidecarToken {
        self.token
    }
}

pub struct Stoplight;

/// The first, load-bearing question this case exists to answer: can
/// Verus place a real `ensures` clause directly on `impl Exchange<..>
/// for Stoplight`'s own `exchange` method, or does it need the same
/// "move to a plain inherent method" workaround Kani 0.67.0 required
/// (Kani can't place `#[kani::proof_for_contract]` on a trait method
/// when the trait itself is generic, and `Exchange<Input, Output, V>`
/// is) -- a completely different mechanism (Kani's contracts are a
/// separate DFCC-checked attribute; Verus's `ensures` is ordinary
/// function syntax), so the answer isn't assumed to carry over.
impl Exchange<Established<Green, GreenToken>, Established<Yellow, YellowToken>, GalleryVerifier>
    for Stoplight
{
    type Error = ();

    fn exchange(&self, input: Established<Green, GreenToken>) -> (result: Result<Established<Yellow, YellowToken>, ()>)
        ensures
            result.is_ok(),
    {
        let token = Yellow::establish(input.sidecar());
        Ok(Established::new(Yellow, token))
    }
}

impl Exchange<Established<Yellow, YellowToken>, Established<Red, RedToken>, GalleryVerifier>
    for Stoplight
{
    type Error = ();

    fn exchange(&self, input: Established<Yellow, YellowToken>) -> (result: Result<Established<Red, RedToken>, ()>)
        ensures
            result.is_ok(),
    {
        let token = Red::establish(input.sidecar());
        Ok(Established::new(Red, token))
    }
}

impl Exchange<Established<Red, RedToken>, Established<Green, GreenToken>, GalleryVerifier>
    for Stoplight
{
    type Error = ();

    fn exchange(&self, input: Established<Red, RedToken>) -> (result: Result<Established<Green, GreenToken>, ()>)
        ensures
            result.is_ok(),
    {
        let token = Green::establish(input.sidecar());
        Ok(Established::new(Green, token))
    }
}

/// Chains all three real `Exchange` impls together through the actual
/// trait methods (not a hand-rolled shortcut) -- the full cycle a real
/// `Stoplight` runs, proven to round-trip back to a well-formed `Green`.
pub fn full_cycle(stoplight: &Stoplight, start: Established<Green, GreenToken>) -> (result: Established<Green, GreenToken>)
{
    let yellow = stoplight.exchange(start).unwrap();
    let red = stoplight.exchange(yellow).unwrap();
    stoplight.exchange(red).unwrap()
}

} // verus!
