//! Local witness trait bridging into `amenable_core::Witness`.

use amenable_core::{Evidence, MetadataEntry, Provenance, Verifier};

/// The Kani verifier, local to this crate: there is only one verifier Kani
/// works with — Kani. Being local here (not imported from `amenable_core`)
/// is what makes the per-type bridges in `rust_std.rs` legal under Rust's
/// orphan rule — a blanket bridge over a bare type parameter is not: the
/// orphan rule requires every uncovered generic parameter to be covered
/// before the first local type, and `Self` in a blanket impl never is.
pub struct KaniVerifier;

/// Provenance surface for the Kani verifier backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct KaniVerifierMetadata;

impl Provenance for KaniVerifierMetadata {
    type MetadataIter = Box<dyn Iterator<Item = MetadataEntry>>;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    fn metadata(&self) -> Self::MetadataIter {
        Box::new({
            const FACTS: &[(&str, &str)] = &[
                ("verifier_family", "kani"),
                ("authority", "Kani Rust Verifier"),
                ("source_url", "https://model-checking.github.io/kani/"),
                ("proof_artifact", "Rust proof harness token stream"),
                (
                    "configuration_channel",
                    "CLI arguments and KANI_* or PROVE_* environment variables",
                ),
                (
                    "configuration_surface",
                    "package selection, flags, timeout, and report output",
                ),
            ];
            FACTS.iter().map(|&(k, v)| MetadataEntry::new(k, v))
        })
    }
}

impl Verifier for KaniVerifier {
    type Metadata = KaniVerifierMetadata;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn name() -> &'static str {
        "kani"
    }
}

/// Kani-specific witness: identifies the Kani proof harness (if any) behind
/// a piece of evidence, without ever running it.
pub trait KaniWitness {
    /// Evidence this witness backs.
    type SupportingEvidence: Evidence;

    /// Descriptor of the Kani proof relevant to this evidence.
    type ProofArtifact;

    /// Identify the Kani proof artifact for this evidence.
    fn proof() -> Self::ProofArtifact;
}
