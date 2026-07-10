//! Verifier-facing proof-emission roles.

use crate::{CreusotVerifier, Evidence, KaniVerifier, Verifier, VerusVerifier};

/// Leaf proof emitter for a verifier backend.
pub trait WitnessSource<V: Verifier> {
    /// Backend-facing proof artifact emitted by this source.
    type ProofArtifact;

    /// Emit the verifier-facing proof artifact for this backend.
    fn proof() -> Self::ProofArtifact;
}

/// Constitutional extraction of verifier-facing proof emission.
///
/// A witness consumes an evidence stack and emits the verifier-facing proof
/// artifact that backend consumes.
pub trait Witness<V: Verifier> {
    /// Evidence stack used to justify this verifier-facing proof surface.
    type SupportingEvidence: Evidence;

    /// Backend-facing proof artifact emitted for this verifier.
    type ProofArtifact;

    /// Emit the verifier-facing proof artifact for this backend.
    fn proof() -> Self::ProofArtifact;

    /// Concise description of the evidence lineage behind this proof.
    fn lineage_summary() -> &'static str {
        <Self::SupportingEvidence as Evidence>::lineage_summary()
    }

    /// Code-level audit surface responsible for upholding this proof.
    fn audit_surface() -> &'static [&'static str] {
        <Self::SupportingEvidence as Evidence>::audit_surface()
    }
}

/// Alias over the builtin verifier trio carried by the constitutional surface.
pub trait Witnessed:
    Witness<KaniVerifier> + Witness<CreusotVerifier> + Witness<VerusVerifier>
{
}

impl<T> Witnessed for T where
    T: Witness<KaniVerifier> + Witness<CreusotVerifier> + Witness<VerusVerifier>
{
}
