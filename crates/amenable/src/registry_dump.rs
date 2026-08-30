//! JSON-serializable shapes for `dump-registry`'s output. Not CLI code —
//! [`crate::cli`]'s `run_dump_registry` is the only caller; this module
//! just shapes the real `inventory`-registered types into an owned,
//! serializable snapshot.

use tracing::instrument;

use crate::{ContractRecord, EvidenceLink, KaniProofRegistration, ProofRecord, witness_exports};

/// One [`crate::EvidenceLink`], owned for JSON serialization.
#[derive(serde::Serialize)]
struct EvidenceLinkDump {
    name: String,
    basis: String,
    index: usize,
}

/// One [`crate::ProofRecord`], owned for JSON serialization. Never
/// invokes `describe()` — external tooling needs presence/absence per
/// `(evidence, verifier)`, not the rendered proof text, and calling every
/// registered `describe()` would be needlessly slow for a coverage check.
#[derive(serde::Serialize)]
struct ProofRecordDump {
    evidence: String,
    verifier: String,
}

/// One [`crate::ContractRecord`], owned for JSON serialization. Unlike
/// [`ProofRecordDump`], this carries the fragment text itself: external
/// tooling comparing real proof-site expressions against registered
/// contracts needs the literal bound, not just a presence/absence flag.
#[derive(serde::Serialize)]
struct ContractRecordDump {
    evidence: String,
    verifier: String,
    kind: String,
    fragment: String,
}

/// One explicit [`crate::WitnessExportRecord`], owned for JSON
/// serialization.
#[derive(serde::Serialize)]
struct WitnessExportRecordDump {
    verifier: String,
    evidence: String,
    destination_module: String,
    support_kind: String,
    trivial: usize,
    checked: usize,
    trusted: usize,
    opaque: usize,
    artifact: WitnessArtifactNodeDump,
}

/// One structured witness artifact node, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactNodeDump {
    shape: String,
    kind: String,
    tag: Option<String>,
    variant: Option<String>,
    detail: Option<String>,
    metadata: Vec<WitnessArtifactMetadataDump>,
    support_kind: String,
    trivial: usize,
    checked: usize,
    trusted: usize,
    opaque: usize,
    members: Vec<WitnessArtifactMemberDump>,
    variants: Vec<WitnessArtifactVariantDump>,
}

/// One named witness artifact member, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactMemberDump {
    label: String,
    artifact: WitnessArtifactNodeDump,
}

/// One named witness artifact variant, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactVariantDump {
    name: String,
    artifact: WitnessArtifactNodeDump,
}

/// One structured witness artifact metadata fact, owned for JSON
/// serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactMetadataDump {
    key: String,
    value: String,
}

/// One [`crate::KaniProof`], owned for JSON serialization.
#[derive(serde::Serialize)]
struct KaniProofDump {
    id: String,
    harness: String,
    package: String,
}

/// The full registry dump written by `dump-registry`.
#[derive(serde::Serialize)]
pub(crate) struct RegistryDump {
    evidence_links: Vec<EvidenceLinkDump>,
    proof_records: Vec<ProofRecordDump>,
    contract_records: Vec<ContractRecordDump>,
    witness_export_records: Vec<WitnessExportRecordDump>,
    kani_proofs: Vec<KaniProofDump>,
}

impl RegistryDump {
    /// Walk every registered `inventory` type and shape it into an owned,
    /// serializable snapshot.
    #[instrument(level = "debug")]
    pub(crate) fn collect() -> Self {
        Self {
            evidence_links: inventory::iter::<EvidenceLink>()
                .map(|link| EvidenceLinkDump {
                    name: link.name().to_owned(),
                    basis: link.basis().to_owned(),
                    index: link.index(),
                })
                .collect(),
            proof_records: inventory::iter::<ProofRecord>()
                .map(|record| ProofRecordDump {
                    evidence: record.evidence().to_owned(),
                    verifier: record.verifier().to_owned(),
                })
                .collect(),
            contract_records: inventory::iter::<ContractRecord>()
                .map(|record| ContractRecordDump {
                    evidence: record.evidence().to_owned(),
                    verifier: record.verifier().to_owned(),
                    kind: record.kind().to_owned(),
                    fragment: (record.fragment())().to_owned(),
                })
                .collect(),
            witness_export_records: witness_exports()
                .into_iter()
                .map(|record| {
                    let (verifier, evidence, destination_module, support, artifact) =
                        record.dissolve();
                    WitnessExportRecordDump {
                        support_kind: support.kind().as_str().to_owned(),
                        trivial: support.trivial(),
                        checked: support.checked(),
                        trusted: support.trusted(),
                        opaque: support.opaque(),
                        artifact: dump_witness_artifact(artifact),
                        verifier,
                        evidence,
                        destination_module,
                    }
                })
                .collect(),
            kani_proofs: inventory::iter::<KaniProofRegistration>()
                .map(|registration| (registration.proof())())
                .map(|record| {
                    let (id, harness, package) = record.dissolve();
                    KaniProofDump {
                        id,
                        harness,
                        package,
                    }
                })
                .collect(),
        }
    }
}

#[instrument(level = "debug", skip(node))]
fn dump_witness_artifact(node: crate::WitnessArtifactNode) -> WitnessArtifactNodeDump {
    let (shape, support, kind, tag, variant, detail, metadata, members, variants) = node.dissolve();

    WitnessArtifactNodeDump {
        shape: shape.as_str().to_owned(),
        kind: kind.as_str().to_owned(),
        tag,
        variant,
        detail,
        metadata: metadata
            .into_iter()
            .map(|entry| WitnessArtifactMetadataDump {
                key: entry.key().to_owned(),
                value: entry.value().to_owned(),
            })
            .collect(),
        support_kind: support.kind().as_str().to_owned(),
        trivial: support.trivial(),
        checked: support.checked(),
        trusted: support.trusted(),
        opaque: support.opaque(),
        members: members
            .into_iter()
            .map(|member| {
                let (label, artifact) = member.dissolve();
                WitnessArtifactMemberDump {
                    label,
                    artifact: dump_witness_artifact(*artifact),
                }
            })
            .collect(),
        variants: variants
            .into_iter()
            .map(|variant| {
                let (name, artifact) = variant.dissolve();
                WitnessArtifactVariantDump {
                    name,
                    artifact: dump_witness_artifact(*artifact),
                }
            })
            .collect(),
    }
}
