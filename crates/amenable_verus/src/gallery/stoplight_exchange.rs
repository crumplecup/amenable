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
//! outcome: passes, for real -- `418 verified, 0 errors`.**
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
//! clause sits directly on `Exchange<.., GalleryVerifier>`'s own generic
//! `exchange` method for all three edges (via `verus_exchange!`, see
//! below), and verifies clean. Kani's contracts are a separate,
//! DFCC-checked attribute mechanism; Verus's `ensures` is ordinary
//! function syntax, so this isn't surprising in hindsight, but it was
//! checked rather than presumed.
//!
//! ## Every mechanical piece of each edge is macro-generated
//!
//! Each edge needs, beyond its own real transition body: an `Ensures<
//! GalleryVerifier>` impl carrying the postcondition (spec companion +
//! exec body + `#[verifier::when_used_as_spec]` bridge), a `Witness<
//! GalleryVerifier>` impl for the target evidence (without it, the
//! preceding `Establish` impl above does not compile, so the exchange
//! cannot exist until the transition it claims is proven), and the
//! `Exchange<Input, Output, GalleryVerifier>` impl itself, its `ensures`
//! clause calling through the registered `Ensures` impl rather than
//! restating the bound (the same single-source-of-truth pattern
//! `EXCHANGE_PROOF_DERIVATION_PLAN.md`'s Step 6 wires up for Kani).
//! None of that is hand-written here: `exchange_support::verus_ensures!`
//! generates the first, `exchange_support::verus_exchange!` the other
//! two -- the real Verus-side counterparts to Kani's `kani_ensures!` and
//! `#[amenable_derive::exchange(..)]` respectively (`EXCHANGE_PROOF_
//! DERIVATION_PLAN.md`'s Step 6/7). A *proc*-macro from a separate crate
//! like `amenable_derive` is not available here at all -- `verus
//! --crate-type=lib` never resolves any extern crate beyond what the
//! real `verus` binary itself bakes in, regardless of what `Cargo.toml`
//! declares -- so both are `macro_rules!` macros instead, each invoked
//! *outside* the main `verus! {}` block below, not inside it: a `macro_
//! rules!` macro's plain output can't itself contain `spec`/`open`/
//! `ensures` syntax the way a directly-authored `verus! {}` body can, so
//! each wraps its own content in a fresh, nested `verus! {}` invocation
//! instead (confirmed the hard way, including a real macro-hygiene fix
//! for `verus_exchange!`'s generated parameter name, in `gallery::
//! ensures_macro_generated`/`gallery::exchange_macro_generated`).
//!
//! `Bound = bool` (not the weaker `Bound = &'static str` `amenable_
//! core::contract`'s own doc comment originally anticipated for any
//! non-Rust-DSL backend) works for Verus via `#[verifier::
//! when_used_as_spec]`: each `Ensures` impl pairs its real exec
//! `ensures()` body with a private `spec fn` companion of identical
//! logic, and Verus transparently substitutes the spec version wherever
//! the exec version is referenced from spec position -- the exact
//! mechanism `vstd::std_specs::result::is_ok` uses for `Result::is_ok()`
//! itself (see `gallery::ensures_contract_bound` for the isolated case
//! that found and confirmed this).
//!
//! ## Verified non-vacuous
//!
//! A real bug (`Err(())` swapped in for the `Green -> Yellow` edge's
//! real `Ok(..)` body, inside its `verus_exchange!` invocation) produced
//! a real, precise failure -- `error: postcondition not satisfied`,
//! pointing at the exact macro-generated `ensures` clause (inside
//! `exchange_support.rs`'s own `verus_exchange!` definition, correctly
//! attributed back to this file's call site) and the exact `Err(())`
//! return that violates it -- confirming the `ensures` clause is a real,
//! checked claim on this exact body, not a vacuous pass. Reverted and
//! re-verified clean afterward.
//!
//! `full_cycle` chains all three real `Exchange::exchange` calls
//! together (not a hand-rolled shortcut using the underlying `establish`
//! calls directly), matching the real `Stoplight`'s own full
//! `Green -> Yellow -> Red -> Green` cycle -- also verifies clean.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::*;

// `Ensures`/`Witness`/`Exchange` impls for each edge are macro-generated
// (`verus_exchange!`, below) using fully-qualified `<Evidence as
// crate::Ensures<V>>::ensures(..)` syntax internally, so this file itself
// no longer references those trait names directly -- no import needed
// for them here at all (unlike the earlier hand-written version, which
// needed a `#[cfg(verus_keep_ghost)]`-gated `use crate::Ensures;` since
// plain `cargo clippy` erases `ensures(...)` clause content entirely).
use crate::exchange_support::{verus_ensures, verus_exchange};
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

// The contract type carries the bound; `verus_exchange!`'s own generated
// `ensures(...)` clause below calls through it rather than restating it
// -- the same single-source-of-truth pattern `EXCHANGE_PROOF_DERIVATION_
// PLAN.md`'s Step 6 wires up for Kani (`kani_ensures!`), confirmed to
// also work for Verus (not merely `Bound = &'static str` description
// text, contrary to `amenable_core::contract`'s own doc comment at the
// time it was written -- see `gallery::ensures_contract_bound` for where
// that was checked, not assumed). Both calls generated via macro, not
// hand-written -- see this file's own doc comment for why they sit
// outside the main `verus! {}` block above rather than inside it, and
// `gallery::exchange_macro_generated`'s own doc comment for `verus_
// exchange!`'s real hygiene fix (the generated method's parameter name
// has to be a macro argument, not hardcoded, for the body below to
// reference it).
verus_ensures!(
    Yellow,
    GalleryVerifier,
    yellow_ensures_spec,
    Result<Established<Yellow, YellowToken>, ()>,
    |result| result.is_ok()
);

// Generates `Witness<GalleryVerifier> for Yellow` (without it,
// `Establish<GreenToken, GalleryVerifier> for Yellow` above does not
// compile, so this exchange cannot exist until the transition it claims
// is proven -- the identical reason `#[amenable_derive::exchange]`
// generates this impl for Kani) and `Exchange<Established<Green,
// GreenToken>, Established<Yellow, YellowToken>, GalleryVerifier> for
// Stoplight`, wired to the `Ensures<GalleryVerifier>` impl just
// registered above. Only this real transition body is hand-authored.
verus_exchange!(
    Stoplight,
    input: Established<Green, GreenToken>,
    Established<Yellow, YellowToken>,
    (),
    Yellow,
    GalleryVerifier,
    {
        let token = Yellow::establish(input.sidecar());
        Ok(Established::new(Yellow, token))
    }
);

verus_ensures!(
    Red,
    GalleryVerifier,
    red_ensures_spec,
    Result<Established<Red, RedToken>, ()>,
    |result| result.is_ok()
);

verus_exchange!(
    Stoplight,
    input: Established<Yellow, YellowToken>,
    Established<Red, RedToken>,
    (),
    Red,
    GalleryVerifier,
    {
        let token = Red::establish(input.sidecar());
        Ok(Established::new(Red, token))
    }
);

// Backs the cycle-back edge, same as the real `Stoplight`'s own
// `evidence_id = "cycle_back"` on this edge's `#[amenable_derive::
// exchange]` invocation: `Green` (a root) still needs a `Witness` impl
// here, `Sidecar<V>`'s own bound applies to every proposition, root or
// not, and this is the only `Exchange` edge that can generate it.
verus_ensures!(
    Green,
    GalleryVerifier,
    green_ensures_spec,
    Result<Established<Green, GreenToken>, ()>,
    |result| result.is_ok()
);

verus_exchange!(
    Stoplight,
    input: Established<Red, RedToken>,
    Established<Green, GreenToken>,
    (),
    Green,
    GalleryVerifier,
    {
        let token = Green::establish(input.sidecar());
        Ok(Established::new(Green, token))
    }
);
