use amenable::{
    ClassifiedWitness, Evidence, MetadataEntry, Provenance, Verifier, Witness, WitnessArtifact,
    WitnessArtifactNode, WitnessModulePath, WitnessSupportKind, WitnessSupportSummary,
    witness_exports,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct LocalVerifier;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LocalVerifierMetadata;

impl Provenance for LocalVerifierMetadata {
    type MetadataIter = std::vec::IntoIter<MetadataEntry>;

    fn metadata(&self) -> Self::MetadataIter {
        vec![MetadataEntry::new("backend", "local-verifier")].into_iter()
    }
}

impl Verifier for LocalVerifier {
    type Metadata = LocalVerifierMetadata;

    fn name() -> &'static str {
        "local-verifier"
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct LocalEvidence;

impl Evidence for LocalEvidence {
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
struct LocalProofArtifact;

impl std::fmt::Display for LocalProofArtifact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "verifier: local-verifier\nshape: unit_struct\nsupport: {}",
            WitnessSupportSummary::checked_leaf()
        )
    }
}

impl WitnessModulePath for LocalProofArtifact {
    const MODULE_PATH: &'static str = "crate::derived_witness::local_evidence_witness";
}

impl WitnessArtifact for LocalProofArtifact {
    fn witness_artifact(&self) -> WitnessArtifactNode {
        WitnessArtifactNode::leaf_with_metadata(
            WitnessSupportKind::Checked,
            WitnessSupportSummary::checked_leaf(),
            "harness: verify_local_evidence_shape",
            [MetadataEntry::new("harness", "verify_local_evidence_shape")],
        )
    }
}

impl Witness<LocalVerifier> for LocalEvidence {
    type SupportingEvidence = Self;
    type ProofArtifact = LocalProofArtifact;

    fn proof() -> Self::ProofArtifact {
        LocalProofArtifact
    }

    fn support() -> WitnessSupportSummary {
        WitnessSupportSummary::checked_leaf()
    }
}

impl ClassifiedWitness<LocalVerifier> for LocalEvidence {}

amenable_core::register_witness_exports!(verifier = LocalVerifier; LocalEvidence);

#[test]
fn witness_exports_include_concrete_local_registrations() -> miette::Result<()> {
    amenable::init_tracing();
    let exports = witness_exports();
    let record = exports
        .iter()
        .find(|record| record.evidence() == std::any::type_name::<LocalEvidence>())
        .ok_or_else(|| miette::miette!("local witness export should be registered"))?;

    assert_eq!(record.verifier(), "local-verifier");
    assert_eq!(
        record.destination_module(),
        "crate::derived_witness::local_evidence_witness"
    );
    assert_eq!(record.support(), WitnessSupportSummary::checked_leaf());
    assert_eq!(record.artifact().shape().as_str(), "leaf");
    assert_eq!(record.artifact().kind(), WitnessSupportKind::Checked);
    assert_eq!(
        record.artifact().detail().as_deref(),
        Some("harness: verify_local_evidence_shape")
    );
    assert_eq!(record.artifact().metadata().len(), 1);
    assert_eq!(record.artifact().metadata()[0].key(), "harness");
    assert_eq!(
        record.artifact().metadata()[0].value(),
        "verify_local_evidence_shape"
    );
    Ok(())
}
