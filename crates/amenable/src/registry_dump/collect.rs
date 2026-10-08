//! The full registry dump written by `dump-registry`, and the walk that
//! collects it from every registered `inventory` type.

use tracing::instrument;

use super::evidence::{EvidenceLinkDump, PremiseDump};
use super::kani_proof::KaniProofDump;
use super::proof::{ContractRecordDump, ProofRecordDump};
use super::witness_artifact::{WitnessExportRecordDump, dump_witness_artifact};
use crate::{ContractRecord, EvidenceLink, KaniProofRegistration, ProofRecord, witness_exports};

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
                    bounds: link.bounds().iter().map(|b| (*b).to_owned()).collect(),
                    premises: link
                        .premises()
                        .iter()
                        .map(|p| PremiseDump {
                            id: p.id().to_owned(),
                            statement: p.statement().to_owned(),
                        })
                        .collect(),
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
