//! Local witness trait bridging into `amenable_core::Witness`.

use amenable_core::{Evidence, MetadataEntry, Provenance, Verifier};

/// The Verus verifier, local to this crate: there is only one verifier
/// Verus works with — Verus. Being local here (not imported from
/// `amenable_core`) is what makes the per-type bridges in `rust_std.rs`
/// legal under Rust's orphan rule — a blanket bridge over a bare type
/// parameter is not: the orphan rule requires every uncovered generic
/// parameter to be covered before the first local type, and `Self` in a
/// blanket impl never is.
pub struct VerusVerifier;

/// Provenance surface for the Verus verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VerusVerifierMetadata;

impl Provenance for VerusVerifierMetadata {
    fn metadata(&self) -> impl Iterator<Item = MetadataEntry> {
        const FACTS: &[(&str, &str)] = &[
            ("verifier_family", "verus"),
            ("authority", "Verus project"),
            ("source_url", "https://verus-lang.github.io/verus/"),
            ("proof_artifact", "Verus proof module token stream"),
            (
                "configuration_channel",
                "CLI arguments and VERUS_* environment variables",
            ),
            (
                "configuration_surface",
                "binary path, source selection, flags, timeout, and report output",
            ),
        ];
        FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
    }
}

impl Verifier for VerusVerifier {
    type Metadata = VerusVerifierMetadata;

    fn name() -> &'static str {
        "verus"
    }
}

/// Verus-specific witness: identifies the Verus spec (if any) behind a
/// piece of evidence, without ever running it.
pub trait VerusWitness {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the Verus proof relevant to this evidence.
    type ProofArtifact;

    /// Identify the Verus proof artifact for this evidence.
    fn proof() -> Self::ProofArtifact;
}
