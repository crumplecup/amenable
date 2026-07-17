//! Verifier-facing proof-emission roles.

use crate::{Evidence, Verifier};

/// Constitutional extraction of verifier-facing proof emission.
///
/// A witness names which proof (if any) backs a piece of evidence for a
/// given verifier — a descriptor, discoverable without running anything.
/// Proving is a separate mode from doing: `proof` never executes a
/// verifier, it identifies the harness/contract that a separate tool
/// invocation (`cargo kani`, etc.) would check. Like `Evidence::basis`,
/// this is a static fact about the type, true for every instance.
pub trait Witness<V: Verifier> {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the backend-facing proof for this verifier.
    type ProofArtifact;

    /// Identify the proof artifact relevant to this evidence, for this
    /// verifier.
    fn proof() -> Self::ProofArtifact;

    /// Produce the basis behind this proof's supporting evidence.
    fn basis() -> <Self::SupportingEvidence as Evidence>::Basis {
        <Self::SupportingEvidence as Evidence>::basis()
    }
}
