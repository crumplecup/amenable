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
    pub to: &'static str,
    /// The real inherent method's own name.
    pub method_name: &'static str,
    /// The real inherent method's own body, verbatim.
    pub body: &'static str,
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
}
