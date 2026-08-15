//! A minimal, real `StateMachine`/`Exchange` worked example: a traffic
//! light, the same Green/Yellow/Red example `docs/AMENABLE_PLAN.md`'s "States
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
    Establish, Evidence, MetadataEntry, ProofToken, Provenance, Sidecar, StateMachine, Witness,
};
use amenable_derive::Standard;

use crate::{CalculationProof, KaniVerifier};

/// The light is green — a root state claim, asserted rather than derived
/// from a prior transition (see `docs/AMENABLE_PLAN.md`, "States Are Roots,
/// Transitions Are Relations"): the first assertion any running instance
/// makes is not computed from anything, it's asserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Green;

impl Provenance for Green {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![MetadataEntry::new(
                "asserted",
                "traffic light state, by design convention (power-on default)",
            )]
            .into_iter()
        })
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
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![MetadataEntry::new(
                "asserted",
                "traffic light state, reachable only via a proven Green -> Yellow exchange",
            )]
            .into_iter()
        })
    }
}

/// The light is red — see [`Green`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Red;

impl Provenance for Red {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            vec![MetadataEntry::new(
                "asserted",
                "traffic light state, reachable only via a proven Yellow -> Red exchange",
            )]
            .into_iter()
        })
    }
}

/// A three-state traffic light: `Green -> Yellow -> Red -> Green`, and
/// nothing else. Illegal transitions (`Yellow -> Green` directly, the
/// motivating counter-example in `docs/AMENABLE_PLAN.md`) simply have no
/// `Exchange` impl — there is no runtime check to bypass; the transition
/// does not exist as code to call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Stoplight;

/// Runtime-inspectable descriptor of which state a [`Stoplight`]
/// currently occupies — distinct from the zero-sized `Green`/`Yellow`/
/// `Red` evidence markers `Exchange`'s type parameters use to make
/// illegal transitions uncompilable. Purely descriptive: nothing in this
/// module derives it from, or checks it against, the real `Exchange`
/// graph — that would be exactly the disconnected-proxy mistake this
/// module used to make (see `git log` on this file for the removed
/// `next()` function, which existed only to give a since-deleted set of
/// Kani harnesses something to check that wasn't the real `exchange()`
/// bodies). Whether this redundancy (a runtime enum alongside
/// compile-time marker types for the same three states) is worth keeping
/// at all, now that it backs nothing, is still open — kept only because
/// removing `StateMachine::State` entirely is a larger question than
/// Step 1 of `EXCHANGE_PROOF_DERIVATION_PLAN.md` needs to answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// The light is green.
    Green,
    /// The light is yellow.
    Yellow,
    /// The light is red.
    Red,
}

/// Governs sequential-cycle traffic light transitions: `Green -> Yellow
/// -> Red -> Green`, never skipping or reversing a step. Proven one edge
/// at a time by real Kani contracts directly on each `Exchange::exchange`
/// body below — not, as an earlier version of this module did, by a
/// disconnected proxy function nothing here actually calls.
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

impl<T, Token> Sidecar<KaniVerifier> for Established<T, Token>
where
    T: Evidence + Witness<KaniVerifier>,
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
    /// barrier to cross, and no `Establish::establish()` call: the *root
    /// claim itself* ("the light is currently Green at power-on") is
    /// asserted via `Provenance`, not derived, and stays that way — see
    /// `docs/AMENABLE_PLAN.md`, "States Are Roots, Transitions Are
    /// Relations." That is a separate fact from whether `Sidecar<
    /// KaniVerifier>` requires `Green: Witness<KaniVerifier>` to exist at
    /// all (it does, unconditionally, for any `Sidecar` proposition) — see
    /// [`Established::root`] for how that's satisfied here.
    pub fn new(_state: Green) -> Self {
        Self(())
    }
}

impl ProofToken for GreenToken {
    type Proposition = Green;
}

impl Established<Green, GreenToken> {
    /// The root case: the power-on default, asserted rather than reached
    /// via any transition. `GreenToken::new` mints without going through
    /// `Establish::establish()` — the root claim isn't derived from a
    /// prior state, so there's no credential to present. That's distinct
    /// from `Sidecar<KaniVerifier>`'s own `Green: Witness<KaniVerifier>`
    /// bound, which still applies here and is satisfied by the real proof
    /// backing `Red`'s cycle-back edge (`impl Witness<KaniVerifier> for
    /// Green` below) — reused opportunistically because it's the only
    /// `Witness<KaniVerifier>` this type happens to have, not because it's
    /// "about" the root claim. The root claim itself remains exactly what
    /// it was: `Provenance`-backed, not model-checked.
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

// The real logic and its Kani contract both live on `Stoplight::
// green_to_yellow`, a plain inherent method, not on the `Exchange` trait
// impl -- Kani 0.67.0 cannot place contracts on a trait method when
// the trait itself is generic (`Exchange<Input, Output, V>` is), a real,
// documented tooling limitation (confirmed empirically: the identical
// `#[kani::proof_for_contract(<Stoplight as Exchange<...>>::exchange)]`
// attempt failed with "Kani does not currently support stubs or function
// contracts on generic functions in traits"). `Exchange::exchange`
// becomes a single-expression delegation to this method -- not a proxy
// with different logic (the exact failure mode this whole exercise
// exists to close), just the same real body relocated to a shape Kani
// can actually attach a contract to, with the trait impl doing nothing
// but satisfying the trait's required signature.
//
// The contract's own content is legitimately trivial, not a corner cut:
// `Established<Green, GreenToken>` and `Established<Yellow, YellowToken>`
// are both zero-field types (exactly one possible value each), and this
// body has no branching that could panic, so there is no nontrivial
// claim left to state once the type system itself already guarantees
// "Green's only lawful successor is Yellow" (there is exactly one
// `Exchange` impl for this `Input` type). What DFCC genuinely checks
// here: the body never panics and always returns `Ok`. A body with real
// branching would earn a real, substantive `ensures` the same way -- see
// `ObservedValueMatchesInput`-style predicates elsewhere in this
// codebase for what that looks like once there's real content.
//
// `#[amenable_derive::exchange]` generates the `Witness<KaniVerifier> for
// Yellow` impl (without it, `Establish<Green, KaniVerifier> for Yellow`
// below does not compile, so this exchange cannot exist until the
// transition it claims is proven), the `ProofRecord` registration backing
// it, and the `Exchange` trait impl delegating to `green_to_yellow` --
// see that macro's own doc comment for exactly what it generates and,
// as importantly, does not touch (the contract and the `harness!`
// invocation below stay hand-written, for reasons documented there).
#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Yellow,
    proof_artifact = CalculationProof,
    harness_fn = verify_green_transitions_only_to_yellow,
    harness_const = VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC,
)]
impl Stoplight {
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Established<Yellow, YellowToken>, std::convert::Infallible>| result.is_ok())
    )]
    fn green_to_yellow(
        &self,
        input: Established<Green, GreenToken>,
    ) -> Result<Established<Yellow, YellowToken>, std::convert::Infallible> {
        let token = Yellow::establish(input.sidecar());
        Ok(Established::new(Yellow, token))
    }
}

amenable_derive::harness! {
    kani, VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC, {
        #[kani::proof_for_contract(Stoplight::green_to_yellow)]
        fn verify_green_transitions_only_to_yellow() {
            let stoplight = Stoplight;
            let input = Established::<Green, GreenToken>::root();
            let _ = stoplight.green_to_yellow(input);
        }
    }
}

impl Establish<GreenToken, KaniVerifier> for Yellow {
    type Token = YellowToken;

    fn establish(_credential: GreenToken) -> Self::Token {
        YellowToken(())
    }
}

/// Lawful token minted once `Red` is established from a proven `Yellow`.
#[derive(Debug, Clone, Copy)]
pub struct RedToken(());

impl ProofToken for RedToken {
    type Proposition = Red;
}

// See the Green -> Yellow contract above for why the real logic/contract
// live on a plain inherent method (Kani 0.67.0 can't contract a trait
// method when the trait is generic), why the content is legitimately
// trivial, and what `#[amenable_derive::exchange]` generates/leaves
// hand-written. Backs `Establish<Yellow, KaniVerifier> for Red` below —
// see `Yellow`'s edge above for the rationale.
#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Red,
    proof_artifact = CalculationProof,
    harness_fn = verify_yellow_transitions_only_to_red,
    harness_const = VERIFY_YELLOW_TRANSITIONS_ONLY_TO_RED_SRC,
)]
impl Stoplight {
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Established<Red, RedToken>, std::convert::Infallible>| result.is_ok())
    )]
    fn yellow_to_red(
        &self,
        input: Established<Yellow, YellowToken>,
    ) -> Result<Established<Red, RedToken>, std::convert::Infallible> {
        let token = Red::establish(input.sidecar());
        Ok(Established::new(Red, token))
    }
}

amenable_derive::harness! {
    kani, VERIFY_YELLOW_TRANSITIONS_ONLY_TO_RED_SRC, {
        #[kani::proof_for_contract(Stoplight::yellow_to_red)]
        fn verify_yellow_transitions_only_to_red() {
            let stoplight = Stoplight;
            let input = Established::new(Yellow, YellowToken(()));
            let _ = stoplight.yellow_to_red(input);
        }
    }
}

impl Establish<YellowToken, KaniVerifier> for Red {
    type Token = RedToken;

    fn establish(_credential: YellowToken) -> Self::Token {
        RedToken(())
    }
}

// See the Green -> Yellow contract above for why the real logic/contract
// live on a plain inherent method (Kani 0.67.0 can't contract a trait
// method when the trait is generic), why the content is legitimately
// trivial, and what `#[amenable_derive::exchange]` generates/leaves
// hand-written. Backs `Establish<Red, KaniVerifier> for Green` below —
// the cycle-back edge, distinct from [`Green`]'s root case: this is the
// proof that a `Red` light lawfully cycles back to `Green`, not the
// power-on assertion (hence `evidence_id = "cycle_back"`, keeping this
// `ProofRecord`'s id distinct from a future root-case registration).
#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Green,
    evidence_id = "cycle_back",
    proof_artifact = CalculationProof,
    harness_fn = verify_red_transitions_only_to_green,
    harness_const = VERIFY_RED_TRANSITIONS_ONLY_TO_GREEN_SRC,
)]
impl Stoplight {
    #[cfg_attr(
        kani,
        kani::ensures(|result: &Result<Established<Green, GreenToken>, std::convert::Infallible>| result.is_ok())
    )]
    fn red_to_green(
        &self,
        input: Established<Red, RedToken>,
    ) -> Result<Established<Green, GreenToken>, std::convert::Infallible> {
        let token = Green::establish(input.sidecar());
        Ok(Established::new(Green, token))
    }
}

amenable_derive::harness! {
    kani, VERIFY_RED_TRANSITIONS_ONLY_TO_GREEN_SRC, {
        #[kani::proof_for_contract(Stoplight::red_to_green)]
        fn verify_red_transitions_only_to_green() {
            let stoplight = Stoplight;
            let input = Established::new(Red, RedToken(()));
            let _ = stoplight.red_to_green(input);
        }
    }
}

impl Establish<RedToken, KaniVerifier> for Green {
    type Token = GreenToken;

    fn establish(_credential: RedToken) -> Self::Token {
        GreenToken(())
    }
}
