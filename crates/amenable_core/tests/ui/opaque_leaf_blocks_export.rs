//! `OpaqueEvidence` implements `Witness<DemoVerifier>` but never
//! overrides `support()`, so its support surface stays `Opaque` -- and
//! it deliberately never implements `ClassifiedWitness<DemoVerifier>`.
//! Registering it should fail `cargo check` with a real trait-resolution
//! error, not compile, panic at runtime, or silently succeed.

use amenable_core::{
    Evidence, MetadataEntry, Provenance, Verifier, Witness, WitnessArtifact, WitnessArtifactNode,
    WitnessModulePath, WitnessSupportSummary,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct DemoVerifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct DemoVerifierMetadata;

impl Provenance for DemoVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        Vec::new().into_iter()
    }
}

impl Verifier for DemoVerifier {
    type Metadata = DemoVerifierMetadata;

    fn name() -> &'static str {
        "demo"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct OpaqueEvidence;

impl Evidence for OpaqueEvidence {
    type Basis = Self;
    type Audit = ();

    fn basis() -> Self::Basis {
        Self
    }

    fn audit(&self) -> Self::Audit {}

    fn is_root() -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpaqueProofArtifact;

impl std::fmt::Display for OpaqueProofArtifact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "opaque")
    }
}

impl WitnessModulePath for OpaqueProofArtifact {
    const MODULE_PATH: &'static str = "crate::opaque_witness";
}

impl WitnessArtifact for OpaqueProofArtifact {
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf(
            amenable_core::WitnessSupportKind::Opaque,
            WitnessSupportSummary::opaque_leaf(),
            "opaque",
        )
    }
}

impl Witness<DemoVerifier> for OpaqueEvidence {
    type SupportingEvidence = Self;
    type ProofArtifact = OpaqueProofArtifact;

    fn proof() -> Self::ProofArtifact {
        OpaqueProofArtifact
    }

    // Deliberately not overriding `support()` -- stays `Opaque` via
    // `Witness`'s own default, and `ClassifiedWitness<DemoVerifier>` is
    // deliberately never implemented for this type. This is the point
    // of the test: the missing bound must fail `register_witness_exports!`
    // at compile time.
}

amenable_core::register_witness_exports!(verifier = DemoVerifier; OpaqueEvidence);

fn main() {}
