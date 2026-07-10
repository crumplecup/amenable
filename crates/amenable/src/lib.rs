//! Constitutional trait family for lawful proof-carrying software structure.
//!
//! `amenable` is the top-level facade over the `amenable_*` crate family: it
//! re-exports the core constitutional roles from `amenable_core` alongside
//! sibling crates such as `amenable_std`, so most users depend on this one
//! crate rather than assembling the family themselves. This is the single
//! sanctioned exception to the workspace's "no re-exports between crates"
//! rule — see `CLAUDE.md`'s Workspace Organization section.
//!
//! Crates that are themselves part of the family (`amenable_kani`,
//! `amenable_creusot`, `amenable_verus`, `amenable_code`, and `amenable_std`
//! itself) depend on `amenable_core` directly, never on this facade, to
//! avoid a circular dependency.
//!
//! See `AMENABLE_PLAN.md` and `amenable.md` in the repository root for the
//! full design rationale.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub use amenable_core::{
    Amenable, AsObjective, AsStandard, CreusotVerifier, CreusotVerifierMetadata, Establish,
    Evidence, Exchange, KaniVerifier, KaniVerifierMetadata, MetadataEntry, Objective, ProofToken,
    Provenance, Sidecar, Standard, StateMachine, Verifier, VerusVerifier, VerusVerifierMetadata,
    Witness, WitnessSource, Witnessed,
};
pub use amenable_std::RustStdType;
