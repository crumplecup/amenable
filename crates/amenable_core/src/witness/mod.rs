//! Verifier-facing proof-emission roles.
//!
//! Split by role: [`support`] classifies how much of a witness is
//! actually proven ([`WitnessSupportKind`]/[`WitnessSupportSummary`]);
//! [`tree`] is the structural shape of a witness artifact
//! ([`WitnessArtifactNode`] and its members/variants); [`registry`] is
//! the `inventory`-backed export registry
//! ([`WitnessExportRecord`]/[`WitnessExportSnapshot`]); [`core_trait`] is
//! the constitutional [`Witness`]/[`ClassifiedWitness`] trait pair and
//! the `register_witness_exports!` macro that populates the registry.
//! Every item stays re-exported flatly at the crate root, exactly as
//! before the split.

mod core_trait;
mod registry;
mod support;
mod tree;

pub use core_trait::{ClassifiedWitness, Witness, WitnessModulePath};
pub use registry::{WitnessExportRecord, WitnessExportSnapshot, witness_exports};
pub use support::{WitnessSupportKind, WitnessSupportSummary};
pub use tree::{
    WitnessArtifact, WitnessArtifactMember, WitnessArtifactNode, WitnessArtifactShape,
    WitnessArtifactVariant,
};
