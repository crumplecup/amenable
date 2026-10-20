//! Kani harness discovery, execution, and result-ledger support for the CLI.

use std::{collections::BTreeSet, process::Command};

use crate::{AmenableError, AmenableResult, KaniProof, KaniProofRegistration};
use clap::{Args, ValueEnum};
use serde::Serialize;
use tracing::instrument;

mod ledger;
mod run;
pub use ledger::Ledger;
pub(crate) use ledger::{default_results_path, load_results};
use run::run_proof;

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

/// List or run the selected Kani harnesses.
#[instrument(level = "debug", skip(args))]
pub fn verify(args: VerifyKaniArgs) -> AmenableResult<()> {
    let records = registered_proofs();
    if args.list {
        for record in records {
            crate::write_stdout_line(record.id())?;
        }
        return Ok(());
    }

    let mut ledger = Ledger::load(&args.results)?;
    let selected = select_records(&records, &ledger, &args)?;
    if selected.is_empty() {
        crate::write_stdout_line("No Kani harnesses matched the selection.")?;
        return Ok(());
    }

    crate::write_stdout_line(format!("Kani verification: {} harness(es)", selected.len()))?;
    crate::write_stdout_line(format!("Results: {}", args.results.display()))?;
    crate::write_stdout_line(format!("Per-harness timeout: {}", args.harness_timeout))?;

    let mut unsuccessful = 0;
    for (index, record) in selected.iter().enumerate() {
        crate::write_stdout_line(format!(
            "[{}/{}] Running {}",
            index + 1,
            selected.len(),
            record.id()
        ))?;
        let result = run_proof(record, &args.harness_timeout);
        if let Some(message) = result.message() {
            crate::write_stderr_line(format!("    {message}"))?;
        }

        match result.status() {
            ProofStatus::Passed => crate::write_stdout_line("    passed")?,
            ProofStatus::Failed => {
                crate::write_stdout_line("    failed")?;
                unsuccessful += 1;
            }
            ProofStatus::Timeout => {
                crate::write_stdout_line("    timeout")?;
                unsuccessful += 1;
            }
        }

        ledger.upsert(record.id(), result.status())?;
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

#[instrument(level = "debug")]
fn registered_proofs() -> Vec<KaniProof> {
    let mut records: Vec<_> = inventory::iter::<KaniProofRegistration>()
        .map(|registration| (registration.proof())())
        .collect();
    records.sort_unstable_by(|left, right| left.id().cmp(right.id()));
    records
}

#[instrument(level = "debug", skip(records, ledger, args))]
fn select_records<'a>(
    records: &'a [KaniProof],
    ledger: &Ledger,
    args: &VerifyKaniArgs,
) -> AmenableResult<Vec<&'a KaniProof>> {
    if let Some(proof) = &args.proof {
        return records
            .iter()
            .find(|record| record.id() == proof)
            .map(|record| vec![record])
            .ok_or_else(|| AmenableError::invariant(format!("unknown Kani proof ID: {proof}")));
    }

    if !args.retry_failed && !args.retry_timeout {
        return Ok(records.iter().collect());
    }

    let retry_ids = ledger.ids_with_status(args.retry_failed, args.retry_timeout);
    let registered_ids: BTreeSet<_> = records.iter().map(|record| record.id().as_str()).collect();

    for id in retry_ids.difference(&registered_ids) {
        crate::write_stderr_line(format!(
            "Ledger proof is no longer registered and will be skipped: {id}"
        ))?;
    }

    Ok(records
        .iter()
        .filter(|record| retry_ids.contains(record.id().as_str()))
        .collect())
}

/// Build the `cargo kani` invocation for one registered proof, applying
/// Kani's own native `--harness-timeout` rather than an outer process
/// timeout. `-Z function-contracts`/`-Z stubbing` are always enabled --
/// required for any harness using `kani::requires`/`kani::ensures`/
/// `kani::proof_for_contract`/`kani::stub_verified`, and harmless for a
/// harness that doesn't (this is the one canonical invocation, so it
/// has to handle both without the caller needing to know which kind of
/// harness they're running).
#[instrument(level = "debug", skip(record))]
pub fn kani_command(record: &KaniProof, harness_timeout: &str) -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "kani",
        "-p",
        record.package().as_str(),
        "--lib",
        "--all-features",
        "--exact",
        "--harness",
        record.harness().as_str(),
        "-Z",
        "unstable-options",
        "-Z",
        "function-contracts",
        "-Z",
        "stubbing",
        "--harness-timeout",
        harness_timeout,
    ]);
    command
}

/// Whether `cargo kani`'s combined stdout/stderr names a verification
/// timeout rather than a genuine proof failure.
#[instrument(level = "trace", ret)]
pub fn is_kani_timeout(diagnostics: &str) -> bool {
    let diagnostics = diagnostics.to_ascii_lowercase();
    diagnostics.contains("verification timed out")
        || diagnostics.contains("harness timed out")
        || diagnostics.contains("timed out")
}

/// The first `error`-prefixed line in `cargo kani`'s combined output, or
/// (failing that) the first non-empty line -- preferred over whatever
/// Kani's own startup banner printed first.
#[instrument(level = "debug")]
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
    #[instrument(level = "debug", skip(self))]
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Timeout => "timeout",
        }
    }

    #[instrument(level = "debug")]
    fn parse(value: &str) -> Option<Self> {
        match value {
            "passed" => Some(Self::Passed),
            "failed" => Some(Self::Failed),
            "timeout" => Some(Self::Timeout),
            _ => None,
        }
    }
}
