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
    Amenable, Establish, Evidence, MetadataEntry, ProofToken, Provenance, Sidecar, StateMachine,
    Witness,
};
use amenable_derive::Standard;

use crate::rust_std::macros::kani_ensures;
use crate::{CalculationProof, KaniProof, KaniProofRegistration, KaniVerifier};

/// The light is green — a root state claim, asserted rather than derived
/// from a prior transition (see `docs/AMENABLE_PLAN.md`, "States Are Roots,
/// Transitions Are Relations"): the first assertion any running instance
/// makes is not computed from anything, it's asserted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Standard)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
#[cfg_attr(kani, derive(kani::Arbitrary))]
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
///
/// `Clone`/`Copy` (bounded on `T`/`Token`, both always `Copy` for every
/// concrete state/token pair in this module): needed so a `#[kani::
/// ensures]` closure's `&Result<Established<..>, StoplightError>` can be
/// dereferenced into the owned `Ensures::Input` the real contract types
/// below take — the postcondition closure calls through the registered
/// contract rather than restating the check inline, so it needs an owned
/// value to hand it, not just a borrow to read.
#[derive(Clone, Copy)]
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

/// Lets Kani reconstruct an `Established<T, Token>` symbolically without
/// running any `Exchange` body — required for `#[kani::stub_verified]`
/// composition (Step 4 of `EXCHANGE_PROOF_DERIVATION_PLAN.md`): a stubbed
/// function's body is replaced by its proven contract, so Kani needs a way
/// to conjure an arbitrary value of its return type standing in for "some
/// value satisfying the postcondition," the same real, documented
/// requirement `~/repos/elicitation`'s `KANI_FOR_VSMS.md` §6.4 found for
/// its own `Established<P>`. Both fields are honest here (unlike
/// elicitation's `PhantomData`-only case): a real `T::any()`/`Token::any()`
/// call each, not a placeholder.
#[cfg(kani)]
impl<T: kani::Arbitrary, Token: kani::Arbitrary> kani::Arbitrary for Established<T, Token> {
    fn any() -> Self {
        Self::new(T::any(), Token::any())
    }
}

/// Every `Stoplight` edge's `Error` type. Not `std::convert::Infallible`:
/// an *uninhabited* `Error` turned out to be incompatible with
/// `#[kani::stub_verified]` composition (Step 4), not merely inconvenient
/// — confirmed empirically, not assumed. Kani's own `Result<T, E>`
/// reconstruction unconditionally needs `E: kani::Arbitrary`'s `any()` to
/// be callable, and CBMC actually executes that call while exploring the
/// stub's symbolic return-value space; the only body a genuinely
/// uninhabited type can offer is `unreachable!()`, which then fails for
/// real, and no safe-Rust `any()` can construct a value of a type with no
/// values. `StoplightError` trades a compile-time "this can never be
/// constructed" guarantee for a runtime one instead: `NotUsed` is real,
/// ordinary, constructible data — Kani can build it honestly — and no
/// edge below ever actually returns it (each edge's own `#[kani::
/// ensures]` contract, proven per-edge in Step 1/3, is what establishes
/// that; `stub_verified` composition then reuses those proofs rather than
/// re-deriving them).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoplightError {
    /// The one variant. Exists so `StoplightError` is an ordinary
    /// constructible type, not so any edge below constructs it.
    NotUsed,
}

#[cfg(kani)]
impl kani::Arbitrary for StoplightError {
    fn any() -> Self {
        Self::NotUsed
    }
}

/// Lawful token minted once a [`Stoplight`] is confirmed `Green`.
#[derive(Debug, Clone, Copy, amenable_derive::ProofToken)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[proof_token(proposition = "Green")]
#[amenable_derive::establish(credential = RedToken, verifier = KaniVerifier, proposition = Green)]
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
#[derive(Debug, Clone, Copy, amenable_derive::ProofToken)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[proof_token(proposition = "Yellow")]
#[amenable_derive::establish(
    credential = GreenToken,
    verifier = KaniVerifier,
    proposition = Yellow,
)]
pub struct YellowToken(());

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
// here: the body never panics and always returns `Ok`.
//
// The claim itself lives in a real, registered `Ensures<KaniVerifier>`
// impl (`kani_ensures!` below) -- the same single-source-of-truth
// pattern `rust_std`'s own proofs already use throughout (e.g.
// `RustStdStandard::<char>::ensures(..)`, called directly from inside a
// plain harness body), just not previously applied to any `Exchange`
// edge. `#[amenable_derive::exchange]` generates the DFCC `#[kani::
// ensures]` attribute on `green_to_yellow` below, calling through
// `Yellow::ensures(..)` rather than restating the boolean inline (see
// that macro's own doc comment) -- the contract type carries the bound,
// and every proof context that needs to check it calls through the one
// definition rather than each independently retyping it.
// `Established<T, Token>` derives `Copy` specifically so the generated
// closure's `&Result<..>` can be dereferenced into the owned `Ensures::
// Input` this needs.
kani_ensures!(
    Yellow,
    "amenable_kani::stoplight::Yellow::green_to_yellow_ensures",
    Result<Established<Yellow, YellowToken>, StoplightError>,
    |result| result.is_ok()
);

// `#[amenable_derive::exchange]` generates the `Witness<KaniVerifier> for
// Yellow` impl (without it, `Establish<Green, KaniVerifier> for Yellow`
// below does not compile, so this exchange cannot exist until the
// transition it claims is proven), the `ProofRecord` registration backing
// it, the `Exchange` trait impl delegating to `green_to_yellow`, and
// (per the `kani_ensures!` registration above) `green_to_yellow`'s own
// DFCC contract -- see that macro's own doc comment for exactly what it
// generates and, as importantly, does not touch (the method body and the
// `harness!` invocation below stay hand-written, for reasons documented
// there).
#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Yellow,
    proof_artifact = CalculationProof,
    harness_fn = verify_green_transitions_only_to_yellow,
    harness_const = VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC,
)]
impl Stoplight {
    fn green_to_yellow(
        &self,
        input: Established<Green, GreenToken>,
    ) -> Result<Established<Yellow, YellowToken>, StoplightError> {
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

/// Lawful token minted once `Red` is established from a proven `Yellow`.
#[derive(Debug, Clone, Copy, amenable_derive::ProofToken)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
#[proof_token(proposition = "Red")]
#[amenable_derive::establish(
    credential = YellowToken,
    verifier = KaniVerifier,
    proposition = Red,
)]
pub struct RedToken(());

// See the Green -> Yellow contract above for why the real logic/contract
// live on a plain inherent method (Kani 0.67.0 can't contract a trait
// method when the trait is generic), why the content is legitimately
// trivial, why the claim itself lives in a real, registered `Ensures<
// KaniVerifier>` impl rather than inline, and what `#[amenable_derive::
// exchange]` generates/leaves hand-written. Backs `Establish<Yellow,
// KaniVerifier> for Red` below — see `Yellow`'s edge above for the
// rationale.
kani_ensures!(
    Red,
    "amenable_kani::stoplight::Red::yellow_to_red_ensures",
    Result<Established<Red, RedToken>, StoplightError>,
    |result| result.is_ok()
);

#[amenable_derive::exchange(
    cfg = kani,
    verifier = KaniVerifier,
    evidence = Red,
    proof_artifact = CalculationProof,
    harness_fn = verify_yellow_transitions_only_to_red,
    harness_const = VERIFY_YELLOW_TRANSITIONS_ONLY_TO_RED_SRC,
)]
impl Stoplight {
    fn yellow_to_red(
        &self,
        input: Established<Yellow, YellowToken>,
    ) -> Result<Established<Red, RedToken>, StoplightError> {
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

// See the Green -> Yellow contract above for why the real logic/contract
// live on a plain inherent method (Kani 0.67.0 can't contract a trait
// method when the trait is generic), why the content is legitimately
// trivial, why the claim itself lives in a real, registered `Ensures<
// KaniVerifier>` impl rather than inline, and what `#[amenable_derive::
// exchange]` generates/leaves hand-written. Backs `Establish<Red,
// KaniVerifier> for Green` below — the cycle-back edge, distinct from
// [`Green`]'s root case: this is the proof that a `Red` light lawfully
// cycles back to `Green`, not the power-on assertion (hence
// `evidence_id = "cycle_back"` below and on the macro invocation,
// keeping both registrations distinct from a future root-case one).
kani_ensures!(
    Green,
    "amenable_kani::stoplight::Green::red_to_green_ensures::cycle_back",
    Result<Established<Green, GreenToken>, StoplightError>,
    |result| result.is_ok()
);

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
    fn red_to_green(
        &self,
        input: Established<Red, RedToken>,
    ) -> Result<Established<Green, GreenToken>, StoplightError> {
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

// Step 4 of `EXCHANGE_PROOF_DERIVATION_PLAN.md`: modular composition. Each
// individual edge above is already `proof_for_contract`-verified in
// isolation; this harness composes all three into the full cycle via
// `#[kani::stub_verified]`, which replaces a stubbed call's body with its
// already-proven contract rather than re-exploring it -- CBMC's task here
// reduces to "does each step's postcondition satisfy the next step's
// precondition," not three full body explorations stacked into one. The
// first real use of compositional/modular Kani verification in this
// codebase; every other `#[kani::proof]` harness here is direct symbolic
// execution with no stubbing. Requires `Established<T, Token>`/
// `StoplightError`: `kani::Arbitrary` above so Kani can reconstruct each
// stubbed call's return value -- the same real requirement `~/repos/
// elicitation`'s `KANI_FOR_VSMS.md` §6.4/§6.5 documents for its own
// composition proofs, plus `StoplightError` existing at all in the first
// place (see its own doc comment: an uninhabited `Error`, `std::convert::
// Infallible`, blocks this specific composition, confirmed empirically).
amenable_derive::harness! {
    kani, VERIFY_FULL_CYCLE_COMPOSES_SRC, {
        #[kani::proof]
        #[kani::stub_verified(Stoplight::green_to_yellow)]
        #[kani::stub_verified(Stoplight::yellow_to_red)]
        #[kani::stub_verified(Stoplight::red_to_green)]
        fn verify_full_cycle_composes() {
            let stoplight = Stoplight;
            let green = Established::<Green, GreenToken>::root();
            let yellow = stoplight.green_to_yellow(green).unwrap();
            let red = stoplight.yellow_to_red(yellow).unwrap();
            let _green_again = stoplight.red_to_green(red).unwrap();
        }
    }
}

// Step 5 of `EXCHANGE_PROOF_DERIVATION_PLAN.md`: the first real occupant
// of `Amenable::kani_surface()`/`creusot_surface()`/`verus_surface()`/
// `audit_surface()` -- grepping the whole tree before this found zero
// `impl Amenable for` anywhere. Every method below either queries real,
// already-registered data or references real, compiler-checked items --
// no hand-typed name that could silently drift from what this module
// actually proved, matching the whole plan's own discipline.
impl Amenable for Stoplight {
    /// A list of proof identifiers -- verifier-agnostic in shape (Kani's
    /// own `KaniProof.id` format, `crate::module::path::harness_name`),
    /// reused rather than inventing a second near-identical struct just
    /// for this trait.
    type ProofSurface = Vec<String>;

    /// Queries the same real `KaniProofRegistration` inventory `harness!`
    /// already populates (the exact mechanism `amenable::kani::
    /// registered_proofs` uses for the CLI's own harness listing),
    /// filtered to this module's own entries via `module_path!()` --
    /// evaluated here, in `stoplight.rs`, so it's the same string
    /// `harness!`'s own `id: concat!(module_path!(), "::", ..)` computed
    /// when it ran, with no hand-typed module name to drift from either
    /// side.
    fn kani_surface() -> Self::ProofSurface {
        let mut ids: Vec<String> = inventory::iter::<KaniProofRegistration>()
            .map(|registration| (registration.proof)())
            .filter(|proof: &KaniProof| proof.id.starts_with(concat!(module_path!(), "::")))
            .map(|proof| proof.id)
            .collect();
        ids.sort_unstable();
        ids
    }

    /// Queries the shared `amenable_core::ProofRecord` registry -- the
    /// same mechanism `kani_surface()` above already uses, not a
    /// cross-crate import. Verifier backend crates never depend on each
    /// other (`amenable_kani` has no Cargo dependency on `amenable_
    /// creusot` at all, and never will): `amenable_creusot::stoplight`
    /// registers its own `ProofRecord`s for these three edges, `#[cfg(
    /// not(creusot))]`-gated in place so `cargo creusot`'s translator
    /// never sees the registration (see that module's own doc comment,
    /// and `amenable_std::creusot_gallery`'s confirmed finding that
    /// precise gating avoids the translator errors that used to justify
    /// routing every registry call through a different crate entirely).
    /// `inventory` only sees registrations from crates actually linked
    /// into the binary querying it, so this returns real data when
    /// something (e.g. `amenable`'s own CLI/tests) links both `amenable_
    /// kani` and `amenable_creusot`, and an honestly empty list from
    /// `amenable_kani`'s own test binaries, which never link `amenable_
    /// creusot` at all -- no `#[cfg(feature = ..)]` branching needed
    /// either way, unlike the cross-crate-import version this replaced.
    fn creusot_surface() -> Self::ProofSurface {
        let mut ids: Vec<String> = inventory::iter::<amenable_core::ProofRecord>()
            .filter(|record| {
                record.verifier == "creusot"
                    && record.evidence.starts_with("amenable_creusot::stoplight::")
            })
            .map(|record| record.evidence.to_owned())
            .collect();
        ids.sort_unstable();
        ids
    }

    /// Honest, not aspirational: a real Verus `Exchange` proof of this
    /// same `Stoplight` cycle exists now (`amenable_verus::gallery::
    /// stoplight_exchange`, see `VERUS_EXCHANGE_PROOF_DERIVATION_PLAN.md`)
    /// via `#[path]` mod-inclusion of this crate's own trait family, not
    /// mere axiomatization -- but it lives in a separate crate with no
    /// `inventory`-backed registry of its own yet (an open question in
    /// that plan doc), so there is nothing real to query from here.
    fn verus_surface() -> Self::ProofSurface {
        Vec::new()
    }

    /// Real captured verbatim source, not identifiers -- every Kani
    /// harness in this module, via the same `harness!`-exported constants
    /// its own contracts already use to keep `Witness::proof()` honest.
    fn audit_surface() -> &'static [&'static str] {
        &[
            VERIFY_GREEN_TRANSITIONS_ONLY_TO_YELLOW_SRC,
            VERIFY_YELLOW_TRANSITIONS_ONLY_TO_RED_SRC,
            VERIFY_RED_TRANSITIONS_ONLY_TO_GREEN_SRC,
            VERIFY_FULL_CYCLE_COMPOSES_SRC,
        ]
    }
}
