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
//! One thing that discovery surfaced directly: minting a token for a
//! *state* is not the same problem as proving a *transition*. A
//! transition has real shape — a model checker can decide whether Yellow
//! ever reaches Green directly — so `Exchange` earns real `Witness`-backed
//! proof machinery. A root state carrying no data (`Green`) has nothing
//! of that shape to falsify; its trust rests entirely on `Provenance`
//! (asserted, documented, cited), and gating its token behind a Kani
//! harness would have been proving something that isn't there to prove.

use amenable_core::{Exchange, MetadataEntry, ProofToken, Provenance, StateMachine};
use amenable_derive::Standard;

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
/// reaches `Yellow` via a proven `Exchange<Green, Yellow>` transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[standard(basis = "Self")]
pub struct Yellow;

impl Provenance for Yellow {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        vec![MetadataEntry::new(
            "asserted",
            "traffic light state, reachable only via Exchange<Green, Yellow>",
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
            "traffic light state, reachable only via Exchange<Yellow, Red>",
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

/// Governs sequential-cycle traffic light transitions: `Green -> Yellow
/// -> Red -> Green`, never skipping or reversing a step. Not yet used by
/// any `StateMachine` method directly — naming it here is what the
/// `Exchange` impls below collectively prove, one legal transition at a
/// time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SequentialCycle;

impl StateMachine for Stoplight {
    type State = Color;
    type Invariant = SequentialCycle;
}

/// Lawful token minted once a [`Stoplight`] is confirmed `Green`.
pub struct GreenToken(());

impl GreenToken {
    /// Mint a token asserting the light is `Green` — no computational
    /// barrier to cross. `Green` carries no data, so there is nothing
    /// about it a model checker could falsify beyond what its own
    /// `Provenance` impl already documents. `Exchange`'s `Witness`-backed
    /// proof machinery is for transitions, which have real shape a model
    /// checker has purchase on; a root state is "trusted with
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

/// Lawful token minted once `Exchange<Green, Yellow>` completes.
pub struct YellowToken(());

impl ProofToken for YellowToken {
    type Proposition = Yellow;
}

impl Exchange<Green, Yellow> for Stoplight {
    type Precondition = Green;
    type Postcondition = Yellow;
    type PreconditionToken = GreenToken;
    type PostconditionToken = YellowToken;
    type Error = std::convert::Infallible;

    fn exchange(
        &self,
        _input: Green,
        _proof: GreenToken,
    ) -> Result<(Yellow, YellowToken), Self::Error> {
        Ok((Yellow, YellowToken(())))
    }
}

/// Lawful token minted once `Exchange<Yellow, Red>` completes.
pub struct RedToken(());

impl ProofToken for RedToken {
    type Proposition = Red;
}

impl Exchange<Yellow, Red> for Stoplight {
    type Precondition = Yellow;
    type Postcondition = Red;
    type PreconditionToken = YellowToken;
    type PostconditionToken = RedToken;
    type Error = std::convert::Infallible;

    fn exchange(
        &self,
        _input: Yellow,
        _proof: YellowToken,
    ) -> Result<(Red, RedToken), Self::Error> {
        Ok((Red, RedToken(())))
    }
}

// Reaching Green again by cycling through uses the same GreenToken a
// fresh power-on would — there is no meaningful difference between "Green
// because this is the start" and "Green because the cycle came back
// around" for a type that carries no data to distinguish them by.
impl Exchange<Red, Green> for Stoplight {
    type Precondition = Red;
    type Postcondition = Green;
    type PreconditionToken = RedToken;
    type PostconditionToken = GreenToken;
    type Error = std::convert::Infallible;

    fn exchange(&self, _input: Red, _proof: RedToken) -> Result<(Green, GreenToken), Self::Error> {
        Ok((Green, GreenToken(())))
    }
}
