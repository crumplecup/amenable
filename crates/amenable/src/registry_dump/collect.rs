//! The full registry dump written by `dump-registry`, and the walk that
//! collects it from every registered `inventory` type.

use tracing::instrument;

use super::evidence::{EvidenceLinkDump, PremiseDump};
use super::kani_proof::KaniProofDump;
use super::proof::{ContractRecordDump, ProofRecordDump};
use super::witness_artifact::{
    WitnessExportRecordDump, WitnessExportRecordDumpBuilder, dump_witness_artifact,
};
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
                .map(|link| {
                    EvidenceLinkDump::new(
                        link.name().to_owned(),
                        link.basis().to_owned(),
                        link.index(),
                        link.bounds().iter().map(|b| (*b).to_owned()).collect(),
                        link.premises()
                            .iter()
                            .map(|p| PremiseDump::new(p.id().to_owned(), p.statement().to_owned()))
                            .collect(),
                    )
                })
                .collect(),
            proof_records: inventory::iter::<ProofRecord>()
                .map(|record| {
                    ProofRecordDump::new(record.evidence().to_owned(), record.verifier().to_owned())
                })
                .collect(),
            contract_records: inventory::iter::<ContractRecord>()
                .map(|record| {
                    ContractRecordDump::new(
                        record.evidence().to_owned(),
                        record.verifier().to_owned(),
                        record.kind().to_owned(),
                        (record.fragment())().to_owned(),
                    )
                })
                .collect(),
            witness_export_records: witness_exports()
                .into_iter()
                .map(|record| {
                    let (verifier, evidence, destination_module, support, artifact) =
                        record.dissolve();
                    WitnessExportRecordDumpBuilder::default()
                        .verifier(verifier)
                        .evidence(evidence)
                        .destination_module(destination_module)
                        .support_kind(support.kind().as_str().to_owned())
                        .trivial(support.trivial())
                        .checked(support.checked())
                        .trusted(support.trusted())
                        .opaque(support.opaque())
                        .artifact(dump_witness_artifact(artifact))
                        .build()
                        .expect("every WitnessExportRecordDump field is set above")
                })
                .collect(),
            kani_proofs: inventory::iter::<KaniProofRegistration>()
                .map(|registration| (registration.proof())())
                .map(|record| {
                    let (id, harness, package) = record.dissolve();
                    KaniProofDump::new(id, harness, package)
                })
                .collect(),
        }
    }
}
