//! Kani harness discovery, execution, and result-ledger support for the CLI.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use amenable::{KaniProof, KaniProofRegistration};
use clap::{Args, ValueEnum};
use serde::Serialize;

const LEDGER_HEADER: &str = "proof_id,timestamp,status";

/// Selection and execution options for `amenable verify kani`.
#[derive(Debug, Args)]
pub(super) struct VerifyKaniArgs {
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
    super::artifacts_directory().join("kani-verification-results.csv")
}

pub(super) fn load_results(path: &Path) -> Result<Vec<VerificationResult>, String> {
    Ok(Ledger::load(path)?.into_results())
}

/// List or run the selected Kani harnesses.
pub(super) fn verify(args: VerifyKaniArgs) -> Result<(), String> {
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
        Err(format!("{unsuccessful} Kani proof(s) did not pass"))
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
) -> Result<Vec<&'a KaniProof>, String> {
    if let Some(proof) = &args.proof {
        return records
            .iter()
            .find(|record| record.id == *proof)
            .map(|record| vec![record])
            .ok_or_else(|| format!("unknown Kani proof ID: {proof}"));
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

fn kani_command(record: &KaniProof, harness_timeout: &str) -> Command {
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

fn is_kani_timeout(diagnostics: &str) -> bool {
    let diagnostics = diagnostics.to_ascii_lowercase();
    diagnostics.contains("verification timed out")
        || diagnostics.contains("harness timed out")
        || diagnostics.contains("timed out")
}

fn first_diagnostic_line(diagnostics: &str) -> Option<String> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub(super) enum ProofStatus {
    Passed,
    Failed,
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

struct Ledger {
    rows: BTreeMap<String, LedgerRow>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct VerificationResult {
    pub(super) proof_id: String,
    pub(super) timestamp: u64,
    pub(super) status: ProofStatus,
}

impl Ledger {
    fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self {
                rows: BTreeMap::new(),
            });
        }

        let contents = fs::read_to_string(path)
            .map_err(|error| format!("could not read result ledger {}: {error}", path.display()))?;
        let mut lines = contents.lines();
        match lines.next() {
            Some(LEDGER_HEADER) => {}
            _ => {
                return Err(format!(
                    "invalid Kani result ledger header in {}",
                    path.display()
                ));
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
            if fields.next().is_some()
                || proof_id.is_empty()
                || timestamp.is_none()
                || status.is_none()
            {
                return Err(format!(
                    "invalid Kani result ledger row {} in {}",
                    line_number + 2,
                    path.display()
                ));
            }
            rows.insert(
                proof_id.to_owned(),
                LedgerRow {
                    timestamp: timestamp.expect("validated above"),
                    status: status.expect("validated above"),
                },
            );
        }

        Ok(Self { rows })
    }

    fn upsert(&mut self, proof_id: &str, status: ProofStatus) -> Result<(), String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock is before the Unix epoch: {error}"))?
            .as_secs();
        self.rows
            .insert(proof_id.to_owned(), LedgerRow { timestamp, status });
        Ok(())
    }

    fn persist(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "could not create result ledger directory {}: {error}",
                    parent.display()
                )
            })?;
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
        fs::write(&temporary, contents).map_err(|error| {
            format!(
                "could not write temporary Kani result ledger {}: {error}",
                temporary.display()
            )
        })?;
        fs::rename(&temporary, path).map_err(|error| {
            format!(
                "could not replace Kani result ledger {}: {error}",
                path.display()
            )
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_uses_kani_native_timeout_without_an_outer_timeout_program() {
        let record = KaniProof {
            id: "amenable_kani::calculator::verify_debit_access_preserves_value".to_owned(),
            harness: "calculator::verify_debit_access_preserves_value".to_owned(),
            package: "amenable_kani".to_owned(),
        };

        let command = kani_command(&record, "3m");
        let arguments: Vec<_> = command
            .get_args()
            .map(|argument| argument.to_string_lossy().into_owned())
            .collect();

        assert_eq!(command.get_program(), "cargo");
        assert_eq!(
            arguments,
            [
                "kani",
                "-p",
                "amenable_kani",
                "--lib",
                "--all-features",
                "--exact",
                "--harness",
                "calculator::verify_debit_access_preserves_value",
                "-Z",
                "unstable-options",
                "--harness-timeout",
                "3m",
            ]
        );
        assert!(!arguments.iter().any(|argument| argument == "timeout"));
    }

    #[test]
    fn ledger_replaces_a_previous_result_for_the_same_proof() {
        let path =
            std::env::temp_dir().join(format!("amenable-kani-ledger-{}.csv", std::process::id()));
        let mut ledger = Ledger {
            rows: BTreeMap::new(),
        };
        ledger.upsert("proof::one", ProofStatus::Failed).unwrap();
        ledger.persist(&path).unwrap();

        let mut ledger = Ledger::load(&path).unwrap();
        ledger.upsert("proof::one", ProofStatus::Passed).unwrap();
        ledger.persist(&path).unwrap();

        let reloaded = Ledger::load(&path).unwrap();
        assert_eq!(reloaded.rows.len(), 1);
        assert_eq!(reloaded.rows["proof::one"].status, ProofStatus::Passed);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn native_timeout_diagnostics_are_distinct_from_failure() {
        assert!(is_kani_timeout(
            "VERIFICATION:- SUCCESSFUL\nverification timed out"
        ));
        assert!(!is_kani_timeout("error: assertion failed"));
    }

    #[test]
    fn diagnostics_prefer_the_first_error_over_kani_startup_output() {
        let diagnostics =
            "Kani Rust Verifier 0.67\nCompiling amenable_kani\nerror[E0433]: missing type\n";

        assert_eq!(
            first_diagnostic_line(diagnostics).as_deref(),
            Some("error[E0433]: missing type")
        );
    }
}
