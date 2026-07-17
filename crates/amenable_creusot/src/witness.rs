//! Local witness trait bridging into `amenable_core::Witness`.

use amenable_core::{Evidence, MetadataEntry, Provenance, Verifier};

/// The Creusot verifier, local to this crate: there is only one verifier
/// Creusot works with — Creusot. Being local here (not imported from
/// `amenable_core`) is what makes the per-type bridges in `rust_std.rs`
/// legal under Rust's orphan rule — a blanket bridge over a bare type
/// parameter is not: the orphan rule requires every uncovered generic
/// parameter to be covered before the first local type, and `Self` in a
/// blanket impl never is.
pub struct CreusotVerifier;

/// Provenance surface for the Creusot verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CreusotVerifierMetadata;

impl Provenance for CreusotVerifierMetadata {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "creusot"),
            ("authority", "Creusot project"),
            ("source_url", "https://creusot-rs.github.io/creusot/"),
            ("proof_artifact", "Why3-oriented proof token stream"),
            (
                "configuration_channel",
                "CLI arguments and CREUSOT_* or PROVE_* environment variables",
            ),
            (
                "configuration_surface",
                "package selection, flags, binary path, timeout, and report output",
            ),
        ];
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
    }
}

impl Verifier for CreusotVerifier {
    type Metadata = CreusotVerifierMetadata;

    fn name() -> &'static str {
        "creusot"
    }
}

/// Creusot-specific witness: identifies the Creusot contract (if any) behind
/// a piece of evidence, without ever running it.
pub trait CreusotWitness {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the Creusot proof relevant to this evidence.
    type ProofArtifact;

    /// Identify the Creusot proof artifact for this evidence.
    fn proof() -> Self::ProofArtifact;
}
