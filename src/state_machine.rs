//! Closed-world proof-bearing state machine surface.

/// Finite-state-machine surface for a closed proof-bearing system.
pub trait StateMachine {
    /// State carrier for the machine.
    type State;

    /// Invariant preserved across lawful transitions.
    type Invariant;
}

/// A closed world of lawful proof exchanges.
pub trait Amenable: StateMachine {
    /// Verifier-facing proof surface type for this closed proof system.
    type ProofSurface;

    /// Human-readable identifier for the governing invariant type.
    fn invariant_name() -> &'static str {
        std::any::type_name::<Self::Invariant>()
    }

    /// Emit the Kani proof surface for the closed proof system.
    fn kani_surface() -> Self::ProofSurface;

    /// Emit the Creusot proof surface for the closed proof system.
    fn creusot_surface() -> Self::ProofSurface;

    /// Emit the Verus proof surface for the closed proof system.
    fn verus_surface() -> Self::ProofSurface;

    /// Code-level audit surface for the closed proof system.
    fn audit_surface() -> &'static [&'static str];
}
