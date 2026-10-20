//! Persisted CSV record of the latest Kani verification result per
//! proof ID. Split out of `kani.rs` as its own named layer, alongside
//! `kani::run`, once that file grew past the point a human reader
//! could take it in as one unit -- CLI argument parsing and proof
//! selection on one side, result persistence on the other.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;
use tracing::instrument;

use super::ProofStatus;
use crate::{AmenableError, AmenableResult};

const LEDGER_HEADER: &str = "proof_id,timestamp,status";

#[instrument(level = "debug")]
pub(crate) fn default_results_path() -> std::path::PathBuf {
    crate::paths::artifacts_directory().join("kani-verification-results.csv")
}

#[instrument(level = "info", skip(path))]
pub(crate) fn load_results(path: &Path) -> AmenableResult<Vec<VerificationResult>> {
    Ok(Ledger::load(path)?.into_results())
}

struct LedgerRow {
    timestamp: u64,
    status: ProofStatus,
}

/// Persisted CSV record of the latest verification result per proof ID.
pub struct Ledger {
    rows: BTreeMap<String, LedgerRow>,
}

#[derive(Debug, Clone, Serialize, derive_getters::Getters, derive_getters::Dissolve)]
pub(crate) struct VerificationResult {
    proof_id: String,
    #[getter(copy)]
    timestamp: u64,
    #[getter(copy)]
    status: ProofStatus,
}

impl Ledger {
    /// Number of proofs with a recorded result.
    #[instrument(level = "trace", skip(self))]
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether the ledger has no recorded results.
    #[instrument(level = "trace", skip(self))]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// The latest recorded status for `proof_id`, if any.
    #[instrument(level = "trace", skip(self))]
    pub fn status_for(&self, proof_id: &str) -> Option<ProofStatus> {
        self.rows.get(proof_id).map(|row| row.status)
    }

    /// Proof IDs whose latest recorded status is `Failed` (if
    /// `include_failed`) or `Timeout` (if `include_timeout`).
    #[instrument(level = "trace", skip(self))]
    pub(crate) fn ids_with_status(
        &self,
        include_failed: bool,
        include_timeout: bool,
    ) -> BTreeSet<&str> {
        self.rows
            .iter()
            .filter(|(_, row)| {
                (include_failed && row.status == ProofStatus::Failed)
                    || (include_timeout && row.status == ProofStatus::Timeout)
            })
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// Load the ledger from `path`, or an empty ledger if it doesn't
    /// exist yet.
    #[instrument(level = "debug", skip(path))]
    pub fn load(path: &Path) -> AmenableResult<Self> {
        if !path.exists() {
            return Ok(Self {
                rows: BTreeMap::new(),
            });
        }

        let contents = fs::read_to_string(path).map_err(|error| AmenableError::io(path, error))?;
        let mut lines = contents.lines();
        match lines.next() {
            Some(LEDGER_HEADER) => {}
            _ => {
                return Err(AmenableError::invariant(format!(
                    "invalid Kani result ledger header in {}",
                    path.display()
                )));
            }
        }

        let mut rows = BTreeMap::new();
        for (line_number, line) in lines.enumerate() {
            if line.is_empty() {
                continue;
            }
            let mut fields = line.split(',');
            let proof_id = fields.next().unwrap_or_default();
            let timestamp = fields.next().unwrap_or_default().parse::<u64>().ok();
            let status = fields.next().and_then(ProofStatus::parse);
            let row_is_malformed = fields.next().is_some() || proof_id.is_empty();

            let (false, Some(timestamp), Some(status)) = (row_is_malformed, timestamp, status)
            else {
                return Err(AmenableError::invariant(format!(
                    "invalid Kani result ledger row {} in {}",
                    line_number + 2,
                    path.display()
                )));
            };
            rows.insert(proof_id.to_owned(), LedgerRow { timestamp, status });
        }

        Ok(Self { rows })
    }

    /// Record `status` as the latest result for `proof_id`, timestamped
    /// now, replacing any previous entry.
    #[instrument(level = "debug", skip(self, status))]
    pub fn upsert(&mut self, proof_id: &str, status: ProofStatus) -> AmenableResult<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.rows
            .insert(proof_id.to_owned(), LedgerRow { timestamp, status });
        Ok(())
    }

    /// Write the ledger to `path` atomically (write, then rename).
    #[instrument(level = "debug", skip(self, path))]
    pub fn persist(&self, path: &Path) -> AmenableResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| AmenableError::io(parent, error))?;
        }

        let mut contents = format!("{LEDGER_HEADER}\n");
        for (proof_id, row) in &self.rows {
            contents.push_str(proof_id);
            contents.push(',');
            contents.push_str(&row.timestamp.to_string());
            contents.push(',');
            contents.push_str(row.status.as_str());
            contents.push('\n');
        }

        let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
        fs::write(&temporary, contents).map_err(|error| AmenableError::io(&temporary, error))?;
        fs::rename(&temporary, path).map_err(|error| AmenableError::io(path, error))
    }

    #[instrument(level = "debug", skip(self))]
    fn into_results(self) -> Vec<VerificationResult> {
        self.rows
            .into_iter()
            .map(|(proof_id, row)| VerificationResult {
                proof_id,
                timestamp: row.timestamp,
                status: row.status,
            })
            .collect()
    }
}
