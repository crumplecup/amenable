//! Closed-world, proof-bearing state machine surface — generated, not
//! hand-written, by `#[derive(amenable_derive::StateMachine)]` from
//! explicit `#[state_machine(..)]` declarations. Replaces the original
//! `StateMachine`/`Amenable` trait pair entirely; see
//! `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s own account of why the
//! original design (a `Color` runtime enum and `SequentialCycle` marker,
//! both self-documented as backing nothing real) wasn't a foundation to
//! extend.
//!
//! **A known, temporary narrowing, not silently dropped.** The deleted
//! `Amenable::creusot_surface()`/`verus_surface()` had real content for
//! Creusot once both `amenable_kani` and `amenable_creusot` were linked
//! into the same binary (`ProofRecord`, keyed by a fully-qualified
//! function-path string, not a bare evidence-type name). `audit_surface()`
//! below is Kani-only for now, querying `ExchangeEdgeRecord` (which has
//! no `verifier` field at all today — every registration comes from
//! `amenable_kani`, the only crate whose toolchain can run
//! `#[amenable_derive::exchange(..)]`'s generated code safely). Extending
//! this to Creusot/Verus needs either a real `verifier` field on
//! `ExchangeEdgeRecord` or an equivalent registry for those backends —
//! deliberately deferred to `docs/STATE_MACHINE_DERIVATION_PLAN.md`'s
//! Step 4, not solved here. `crates/amenable/tests/
//! stoplight_creusot_surface_test.rs`, which exercised the deleted
//! `Amenable::creusot_surface()`, is deleted alongside it — real
//! Creusot-backed audit content returns in Step 4, not reintroduced as a
//! stopgap here.

use crate::Verifier;

/// One declared transition between two named states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transition {
    /// The source state's declared name.
    pub from: &'static str,
    /// The target state's declared name.
    pub to: &'static str,
}

/// Real, registry-backed audit content for one transition: the real
/// inherent method's own name and verbatim body — never hand-typed, so
/// it can't drift from the real logic the way a hand-maintained
/// description could.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionAudit {
    /// The target state's declared name — the evidence type this
    /// transition establishes.
    pub to: String,
    /// The real inherent method's own name.
    pub method_name: String,
    /// The real inherent method's own body, verbatim.
    pub body: String,
}

/// A declared state's real, lawful root constructor — the answer to
/// "can this state be entered without a prior credential at all, and
/// if so, how do I call the real function that does it, and what (if
/// anything) does it need me to supply."
///
/// Exists because a state can be *both* root-enterable and the target
/// of a real transition (`amenable_kani::stoplight::Green`: asserted at
/// power-on via `Established::<Green, GreenToken>::root()`, but also
/// the real target of the `Red -> Green` cycle-back edge). Neither fact
/// alone is discoverable from `ProofTokenMintRecord`: `inventory` never
/// overwrites, so a token with both a bare `#[derive(ProofToken)]`
/// registration and an `#[establish(..)]` one carries *two* real
/// records, not one collapsed record — but even reading both only tells
/// you "no `#[establish]` was given for this one," never "and here is
/// the real function that constructs it lawfully with no prior state at
/// all." `constructor` closes that gap directly: a real path, checked
/// at compile time (by the same static-assertion mechanism the derive
/// already uses for edges) to actually be a callable function returning
/// the declared state's own carrier type, not a hand-typed string that
/// could drift or silently stop compiling.
///
/// **`seed` distinguishes a freely-callable root from one that needs
/// real external data**, closing the gap a first version of this type
/// left honestly undeclared rather than force-fit: a root whose real
/// constructor takes an argument (`amenable_gaap::Transfer::pending`,
/// needing a real `TransferPayload`) is just as real and lawful as a
/// zero-argument one (`Established::root()`), but was previously
/// omitted from `root_entries()` entirely rather than misrepresented as
/// freely callable. `seed` is `"()"` for a zero-argument root (checked
/// at compile time as `fn() -> Carrier`); otherwise it names the real
/// argument type the compile-time check instead requires the real
/// constructor to accept (`fn(Seed) -> Carrier`) — a single type, the
/// same way a constructor genuinely needing more than one real value
/// bundles them into one real struct elsewhere in this codebase
/// (`TransferPayload` itself bundling `from`/`to`/`amount`) rather than
/// this mechanism supporting several raw arguments directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootEntry {
    /// The state's declared name.
    pub state: &'static str,
    /// The real, compiler-checked path to the function returning that
    /// state's own declared carrier type.
    pub constructor: &'static str,
    /// `"()"` for a root callable with no arguments; otherwise the
    /// real type `constructor` requires as its one real argument.
    pub seed: &'static str,
}

/// A closed, proof-bearing state machine. Every method here is backed by
/// data the `#[derive(amenable_derive::StateMachine)]` macro either
/// echoes directly from its own `#[state_machine(..)]` declarations
/// (`states`/`transitions`) or reads from a real, compiler-populated
/// registry (`audit_surface`) — never a hand-typed string a human could
/// let drift from reality.
pub trait StateMachine<V: Verifier> {
    /// Every declared state's name.
    fn states() -> &'static [&'static str];

    /// Every declared transition between two states.
    fn transitions() -> &'static [Transition];

    /// Real, registry-backed audit content for this system's real
    /// `Exchange` edges. Not necessarily one entry per declared
    /// transition in either direction — a declared edge with no matching
    /// registration, or a real registration nobody declared, is exactly
    /// what the separate declared-vs-registered cross-check (a
    /// runtime/test-time check, not this method) exists to catch.
    fn audit_surface() -> Vec<TransitionAudit>;

    /// Every declared state that can be entered lawfully with no prior
    /// state, the real function that does it, and (via each entry's own
    /// `seed`) whether that function needs real external data to call.
    /// Honestly empty for a state machine with no declared root worth
    /// this way at all, not for one whose only root needs an argument —
    /// see `RootEntry::seed`'s own doc comment for why a data-needing
    /// root (`amenable_gaap::Ledger`'s `Pending`) is a real entry here
    /// too, not a gap.
    fn root_entries() -> &'static [RootEntry] {
        &[]
    }
}
