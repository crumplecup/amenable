//! Kani harness discovery, execution, and result-ledger support for the CLI.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{AmenableError, AmenableResult, KaniProof, KaniProofRegistration};
use clap::{Args, ValueEnum};
use serde::Serialize;

const LEDGER_HEADER: &str = "proof_id,timestamp,status";

/// Selection and execution options for `amenable verify kani`.
#[derive(Debug, Args)]
pub struct VerifyKaniArgs {
    /// Run one exact, fully-qualified registered proof ID.
    #[arg(long, conflicts_with_all = ["retry_failed", "retry_timeout", "list"])]
    proof: Option<String>,
    /// Retry only proofs whose latest ledger result is `failed`.
    #[arg(long = "failed", conflicts_with = "list")]
    retry_failed: bool,
    /// Retry only proofs whose latest ledger result is `timeout`.
    #[arg(long = "timeout", conflicts_with = "list")]
    retry_timeout: bool,
    /// List registered proof IDs without running Kani.
    #[arg(long)]
    list: bool,
    /// CSV ledger path.
    #[arg(short, long, default_value_os_t = default_results_path())]
    results: std::path::PathBuf,
    /// Kani-native timeout applied to each harness, for example `3m`.
    #[arg(long, default_value = "3m")]
    harness_timeout: String,
}

pub(super) fn default_results_path() -> std::path::PathBuf {
    crate::paths::artifacts_directory().join("kani-verification-results.csv")
}

pub(super) fn load_results(path: &Path) -> AmenableResult<Vec<VerificationResult>> {
    Ok(Ledger::load(path)?.into_results())
}

/// List or run the selected Kani harnesses.
pub fn verify(args: VerifyKaniArgs) -> AmenableResult<()> {
    let records = registered_proofs();
    if args.list {
        for record in records {
            println!("{}", record.id);
        }
        return Ok(());
    }

    let mut ledger = Ledger::load(&args.results)?;
    let selected = select_records(&records, &ledger, &args)?;
    if selected.is_empty() {
        println!("No Kani harnesses matched the selection.");
        return Ok(());
    }

    println!("Kani verification: {} harness(es)", selected.len());
    println!("Results: {}", args.results.display());
    println!("Per-harness timeout: {}", args.harness_timeout);

    let mut unsuccessful = 0;
    for (index, record) in selected.iter().enumerate() {
        println!("[{}/{}] Running {}", index + 1, selected.len(), record.id);
        let result = run_proof(record, &args.harness_timeout);
        if let Some(message) = &result.message {
            eprintln!("    {message}");
        }

        match result.status {
            ProofStatus::Passed => println!("    passed"),
            ProofStatus::Failed => {
                println!("    failed");
                unsuccessful += 1;
            }
            ProofStatus::Timeout => {
                println!("    timeout");
                unsuccessful += 1;
            }
        }

        ledger.upsert(&record.id, result.status)?;
        ledger.persist(&args.results)?;
    }

    if unsuccessful == 0 {
        Ok(())
    } else {
        Err(AmenableError::invariant(format!(
            "{unsuccessful} Kani proof(s) did not pass"
        )))
    }
}

fn registered_proofs() -> Vec<KaniProof> {
    let mut records: Vec<_> = inventory::iter::<KaniProofRegistration>()
        .map(|registration| (registration.proof)())
        .collect();
    records.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    records
}

fn select_records<'a>(
    records: &'a [KaniProof],
    ledger: &Ledger,
    args: &VerifyKaniArgs,
) -> AmenableResult<Vec<&'a KaniProof>> {
    if let Some(proof) = &args.proof {
        return records
            .iter()
            .find(|record| record.id == *proof)
            .map(|record| vec![record])
            .ok_or_else(|| AmenableError::invariant(format!("unknown Kani proof ID: {proof}")));
    }

    if !args.retry_failed && !args.retry_timeout {
        return Ok(records.iter().collect());
    }

    let retry_ids: BTreeSet<_> = ledger
        .rows
        .iter()
        .filter(|(_, row)| {
            (args.retry_failed && row.status == ProofStatus::Failed)
                || (args.retry_timeout && row.status == ProofStatus::Timeout)
        })
        .map(|(id, _)| id.as_str())
        .collect();
    let registered_ids: BTreeSet<_> = records.iter().map(|record| record.id.as_str()).collect();

    for id in retry_ids.difference(&registered_ids) {
        eprintln!("Ledger proof is no longer registered and will be skipped: {id}");
    }

    Ok(records
        .iter()
        .filter(|record| retry_ids.contains(record.id.as_str()))
        .collect())
}

fn run_proof(record: &KaniProof, harness_timeout: &str) -> ProofRun {
    let output = kani_command(record, harness_timeout)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) if output.status.success() => ProofRun::passed(),
        Ok(output) => {
            let diagnostics = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let status = if is_kani_timeout(&diagnostics) {
                ProofStatus::Timeout
            } else {
                ProofStatus::Failed
            };
            ProofRun {
                status,
                message: first_diagnostic_line(&diagnostics),
            }
        }
        Err(error) => ProofRun {
            status: ProofStatus::Failed,
            message: Some(format!("could not start cargo kani: {error}")),
        },
    }
}

/// Build the `cargo kani` invocation for one registered proof, applying
/// Kani's own native `--harness-timeout` rather than an outer process
/// timeout.
pub fn kani_command(record: &KaniProof, harness_timeout: &str) -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "kani",
        "-p",
        record.package.as_str(),
        "--lib",
        "--all-features",
        "--exact",
        "--harness",
        record.harness.as_str(),
        "-Z",
        "unstable-options",
        "--harness-timeout",
        harness_timeout,
    ]);
    command
}

/// Whether `cargo kani`'s combined stdout/stderr names a verification
/// timeout rather than a genuine proof failure.
pub fn is_kani_timeout(diagnostics: &str) -> bool {
    let diagnostics = diagnostics.to_ascii_lowercase();
    diagnostics.contains("verification timed out")
        || diagnostics.contains("harness timed out")
        || diagnostics.contains("timed out")
}

/// The first `error`-prefixed line in `cargo kani`'s combined output, or
/// (failing that) the first non-empty line -- preferred over whatever
/// Kani's own startup banner printed first.
pub fn first_diagnostic_line(diagnostics: &str) -> Option<String> {
    diagnostics
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("error") || line.contains("error["))
        .or_else(|| {
            diagnostics
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty())
        })
        .map(ToOwned::to_owned)
}

/// Outcome recorded in the Kani result ledger for one proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum ProofStatus {
    /// The proof verified successfully.
    Passed,
    /// The proof failed (a real assertion violation, not a timeout).
    Failed,
    /// The proof did not finish within its harness timeout.
    Timeout,
}

impl ProofStatus {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "passed" => Some(Self::Passed),
            "failed" => Some(Self::Failed),
            "timeout" => Some(Self::Timeout),
            _ => None,
        }
    }
}

struct ProofRun {
    status: ProofStatus,
    message: Option<String>,
}

impl ProofRun {
    fn passed() -> Self {
        Self {
            status: ProofStatus::Passed,
            message: None,
        }
    }
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
pub(super) struct VerificationResult {
    proof_id: String,
    #[getter(copy)]
    timestamp: u64,
    #[getter(copy)]
    status: ProofStatus,
}

impl Ledger {
    /// Number of proofs with a recorded result.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Whether the ledger has no recorded results.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// The latest recorded status for `proof_id`, if any.
    pub fn status_for(&self, proof_id: &str) -> Option<ProofStatus> {
        self.rows.get(proof_id).map(|row| row.status)
    }

    /// Load the ledger from `path`, or an empty ledger if it doesn't
    /// exist yet.
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
    pub fn upsert(&mut self, proof_id: &str, status: ProofStatus) -> AmenableResult<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.rows
            .insert(proof_id.to_owned(), LedgerRow { timestamp, status });
        Ok(())
    }

    /// Write the ledger to `path` atomically (write, then rename).
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
