//! A minimal, real `StateMachine`/`Exchange` worked example: a traffic
//! light, the same Green/Yellow/Red example `AMENABLE_PLAN.md`'s "States
//! Are Roots, Transitions Are Relations" section already uses to motivate
//! the split between state claims (`Provenance`, asserted) and transition
//! claims (`Witness`, proven).
//!
//! First real `StateMachine`/`Exchange` implementation anywhere in this
//! workspace — built to discover the bounds and methods an operation
//! actually needs, not to match a design sketched in advance.
//!
//! `Exchange` requires both its `Input` and `Output` to implement
//! [`Sidecar`], and every concrete `Sidecar` value here is an
//! [`Established`] carrier: a primary payload bundled with the *specific*
//! proof token that was actually minted for it. `Established::new` is
//! private, so the only ways to get one are `Established::root()` (the
//! power-on `Green` case: no computational barrier to cross, nothing to
//! prove — see `GreenToken::new`'s doc) or `Stoplight::exchange`, which
//! calls `Establish::establish` on the real credential it was handed and
//! stores the resulting token rather than re-minting an independent one
//! later. That's the whole point of holding onto the token instead of
//! synthesizing a fresh one on every `Sidecar::sidecar()` call: a token
//! disconnected from a real `establish()` call proves nothing about *this*
//! transition, only that the token type is constructible in the abstract.
//!
//! The `establish()` calls themselves are gated two ways: by `Witness`
//! (`Establish<C, V>: Evidence + Witness<V>` means `impl
//! Establish<GreenToken, KaniVerifier> for Yellow` cannot exist unless
//! `Yellow: Witness<KaniVerifier>` also does, so each transition earns a
//! real (if small) Kani harness proving the `SequentialCycle` invariant it
//! claims — `Green` only ever transitions to `Yellow`, never skipping to
//! `Red` or looping back to itself, and so on around the cycle), and by
//! `Establish`'s own `C: ProofToken` bound (the credential is the prior
//! state's real token — `GreenToken`, not the bare `Green` marker — so
//! `Stoplight::exchange` must call `input.sidecar()` to obtain a lawful
//! credential; `input.primary()` doesn't type-check).

use amenable_core::{
    Establish, Evidence, Exchange, MetadataEntry, ProofToken, Provenance, Sidecar, StateMachine,
    Witness,
};
use amenable_derive::Standard;

use crate::{CalculationProof, KaniVerifier};

/// The light is green — a root state claim, asserted rather than derived
/// from a prior transition (see `AMENABLE_PLAN.md`, "States Are Roots,
/// Transitions Are Relations"): the first assertion any running instance
/// makes is not computed from anything, it's asserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Green;

impl Provenance for Green {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![MetadataEntry::new(
            "asserted",
            "traffic light state, by design convention (power-on default)",
        )]
        .into_iter()
    }
}

/// The light is yellow — see [`Green`] for why this is a root claim, not
/// a derived one, even though in practice a [`Stoplight`] only ever
/// reaches `Yellow` via a proven `Exchange<Established<Green, GreenToken>,
/// Established<Yellow, YellowToken>>` transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Yellow;

impl Provenance for Yellow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![MetadataEntry::new(
            "asserted",
            "traffic light state, reachable only via a proven Green -> Yellow exchange",
        )]
        .into_iter()
    }
}

/// The light is red — see [`Green`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Red;

impl Provenance for Red {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![MetadataEntry::new(
            "asserted",
            "traffic light state, reachable only via a proven Yellow -> Red exchange",
        )]
        .into_iter()
    }
}

/// A three-state traffic light: `Green -> Yellow -> Red -> Green`, and
/// nothing else. Illegal transitions (`Yellow -> Green` directly, the
/// motivating counter-example in `AMENABLE_PLAN.md`) simply have no
/// `Exchange` impl — there is no runtime check to bypass; the transition
/// does not exist as code to call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stoplight;

/// Runtime-inspectable descriptor of which state a [`Stoplight`]
/// currently occupies — distinct from the zero-sized `Green`/`Yellow`/
/// `Red` evidence markers `Exchange`'s type parameters use to make
/// illegal transitions uncompilable. Whether this redundancy (a runtime
/// enum alongside compile-time marker types for the same three states) is
/// the right shape, or whether `StateMachine::State` should be able to
/// name the marker family directly, is exactly the kind of question this
/// exercise exists to surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// The light is green.
    Green,
    /// The light is yellow.
    Yellow,
    /// The light is red.
    Red,
}

/// The single lawful successor for each color in the cycle — what each
/// transition's Kani harness below actually checks, and the one place the
/// `SequentialCycle` order lives as executable code rather than only as a
/// name. `pub` for the same reason `#[calculation]`-generated `*_impl`
/// functions are: every call site lives inside a `#[cfg(kani)]`-gated
/// harness body (see `amenable_derive::harness!`'s doc), invisible to an
/// ordinary build, so a private `next` would be flagged as dead code.
pub fn next(color: Color) -> Color {
    match color {
        Color::Green => Color::Yellow,
        Color::Yellow => Color::Red,
        Color::Red => Color::Green,
    }
}

/// Governs sequential-cycle traffic light transitions: `Green -> Yellow
/// -> Red -> Green`, never skipping or reversing a step. Proven one edge
/// at a time by the `Witness` impls backing each `Establish` impl below.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SequentialCycle;

impl StateMachine for Stoplight {
    type State = Color;
    type Invariant = SequentialCycle;
}

/// A primary payload bundled with the specific proof token that was
/// actually minted for it — the concrete `Sidecar` shape used throughout
/// this module. `new` is private: the only lawful ways to produce one are
/// [`Established::root`] (the `Green` power-on case) or `Stoplight`'s
/// `Exchange` impls, both of which mint the token via a real
/// `Establish`/`Witness`-gated call rather than handing back an
/// independently re-minted token disconnected from what happened.
pub struct Established<T, Token> {
    primary: T,
    token: Token,
}

impl<T, Token> Established<T, Token> {
    fn new(primary: T, token: Token) -> Self {
        Self { primary, token }
    }
}

impl<T, Token> Sidecar for Established<T, Token>
where
    T: Evidence,
    Token: ProofToken<Proposition = T> + Clone,
{
    type Primary = T;
    type Proposition = T;
    type SidecarToken = Token;

    fn primary(&self) -> &Self::Primary {
        &self.primary
    }

    fn sidecar(&self) -> Self::SidecarToken {
        self.token.clone()
    }
}

/// Lawful token minted once a [`Stoplight`] is confirmed `Green`.
#[derive(Debug, Clone, Copy)]
pub struct GreenToken(());

impl GreenToken {
    /// Mint a token asserting the light is `Green` — no computational
    /// barrier to cross. `Green` carries no data, so there is nothing
    /// about it a model checker could falsify beyond what its own
    /// `Provenance` impl already documents. A root state is "trusted with
    /// provenance," not "trusted because a model checker verified it" —
    /// see `AMENABLE_PLAN.md`, "States Are Roots, Transitions Are
    /// Relations."
    pub fn new(_state: Green) -> Self {
        Self(())
    }
}

impl ProofToken for GreenToken {
    type Proposition = Green;
}

impl Established<Green, GreenToken> {
    /// The root case: the power-on default, asserted rather than reached
    /// via any transition. See [`GreenToken::new`] for why no `Establish`/
    /// `Witness` gate applies here.
    #[must_use]
    pub fn root() -> Self {
        Self::new(Green, GreenToken::new(Green))
    }
}

/// Lawful token minted once `Yellow` is established from a proven `Green`.
#[derive(Debug, Clone, Copy)]
pub struct YellowToken(());

impl ProofToken for YellowToken {
    type Proposition = Yellow;
}

/// Backs `Establish<Green, KaniVerifier> for Yellow`: without this impl,
/// that `Establish` impl does not compile, so `Stoplight`'s `Green ->
/// Yellow` exchange cannot exist until the transition it claims is proven.
impl Witness<KaniVerifier> for Yellow {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    fn proof() -> Self::ProofArtifact {
        CalculationProof {
            harness: "verify_green_transitions_only_to_yellow".to_owned(),
            claim: VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC.to_owned(),
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: concat!(module_path!(), "::", stringify!(Yellow)),
        verifier: "kani",
        describe: || <Yellow as Witness<KaniVerifier>>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC, {
        /// `Green`'s only lawful successor in the `SequentialCycle` is
        /// `Yellow`: it never skips ahead to `Red` and never loops back
        /// to `Green`.
        #[kani::proof]
        fn verify_green_transitions_only_to_yellow() {
            let successor = next(Color::Green);
            assert_eq!(successor, Color::Yellow, "Green transitions to Yellow");
            assert_ne!(successor, Color::Red, "Green must not skip directly to Red");
            assert_ne!(successor, Color::Green, "Green must not stay Green");
        }
    }
}

impl Establish<GreenToken, KaniVerifier> for Yellow {
    type Token = YellowToken;

    fn establish(_credential: GreenToken) -> Self::Token {
        YellowToken(())
    }
}

impl Exchange<Established<Green, GreenToken>, Established<Yellow, YellowToken>> for Stoplight {
    type Error = std::convert::Infallible;

    fn exchange(
        &self,
        input: Established<Green, GreenToken>,
    ) -> Result<Established<Yellow, YellowToken>, Self::Error> {
        let token = Yellow::establish(input.sidecar());
        Ok(Established::new(Yellow, token))
    }
}

/// Lawful token minted once `Red` is established from a proven `Yellow`.
#[derive(Debug, Clone, Copy)]
pub struct RedToken(());

impl ProofToken for RedToken {
    type Proposition = Red;
}

/// Backs `Establish<Yellow, KaniVerifier> for Red` — see `Yellow`'s
/// `Witness` impl above for the rationale.
impl Witness<KaniVerifier> for Red {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    fn proof() -> Self::ProofArtifact {
        CalculationProof {
            harness: "verify_yellow_transitions_only_to_red".to_owned(),
            claim: VERIFY_YELLOW_TRANSITIONS_ONLY_TO_RED_SRC.to_owned(),
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: concat!(module_path!(), "::", stringify!(Red)),
        verifier: "kani",
        describe: || <Red as Witness<KaniVerifier>>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_YELLOW_TRANSITIONS_ONLY_TO_RED_SRC, {
        /// `Yellow`'s only lawful successor is `Red`: it never reverses
        /// back to `Green` and never stays `Yellow`.
        #[kani::proof]
        fn verify_yellow_transitions_only_to_red() {
            let successor = next(Color::Yellow);
            assert_eq!(successor, Color::Red, "Yellow transitions to Red");
            assert_ne!(successor, Color::Green, "Yellow must not reverse back to Green");
            assert_ne!(successor, Color::Yellow, "Yellow must not stay Yellow");
        }
    }
}

impl Establish<YellowToken, KaniVerifier> for Red {
    type Token = RedToken;

    fn establish(_credential: YellowToken) -> Self::Token {
        RedToken(())
    }
}

impl Exchange<Established<Yellow, YellowToken>, Established<Red, RedToken>> for Stoplight {
    type Error = std::convert::Infallible;

    fn exchange(
        &self,
        input: Established<Yellow, YellowToken>,
    ) -> Result<Established<Red, RedToken>, Self::Error> {
        let token = Red::establish(input.sidecar());
        Ok(Established::new(Red, token))
    }
}

/// Backs `Establish<Red, KaniVerifier> for Green` — the cycle-back edge.
/// Distinct from [`Green`]'s root case: this is the proof that a `Red`
/// light lawfully cycles back to `Green`, not the power-on assertion.
impl Witness<KaniVerifier> for Green {
    type SupportingEvidence = Self;
    type ProofArtifact = CalculationProof;

    fn proof() -> Self::ProofArtifact {
        CalculationProof {
            harness: "verify_red_transitions_only_to_green".to_owned(),
            claim: VERIFY_RED_TRANSITIONS_ONLY_TO_GREEN_SRC.to_owned(),
        }
    }
}

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: concat!(module_path!(), "::", stringify!(Green), "::cycle_back"),
        verifier: "kani",
        describe: || <Green as Witness<KaniVerifier>>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RED_TRANSITIONS_ONLY_TO_GREEN_SRC, {
        /// `Red`'s only lawful successor is `Green`: the cycle closes,
        /// never advancing straight to `Yellow` and never staying `Red`.
        #[kani::proof]
        fn verify_red_transitions_only_to_green() {
            let successor = next(Color::Red);
            assert_eq!(successor, Color::Green, "Red cycles back to Green");
            assert_ne!(successor, Color::Yellow, "Red must not advance directly to Yellow");
            assert_ne!(successor, Color::Red, "Red must not stay Red");
        }
    }
}

impl Establish<RedToken, KaniVerifier> for Green {
    type Token = GreenToken;

    fn establish(_credential: RedToken) -> Self::Token {
        GreenToken(())
    }
}

impl Exchange<Established<Red, RedToken>, Established<Green, GreenToken>> for Stoplight {
    type Error = std::convert::Infallible;

    fn exchange(
        &self,
        input: Established<Red, RedToken>,
    ) -> Result<Established<Green, GreenToken>, Self::Error> {
        let token = Green::establish(input.sidecar());
        Ok(Established::new(Green, token))
    }
}
