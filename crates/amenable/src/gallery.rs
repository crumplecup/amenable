//! CLI support for the Kani proof gallery.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    AmenableError, AmenableResult, KaniGalleryCase, KaniGalleryExpectation, KaniGalleryRegistration,
};
use clap::{Args, Subcommand};

use tracing::instrument;
const LEDGER_HEADER: &str =
    "case_id,timestamp,disposition,expected_status,observed_status,matched_expectation";

/// Commands for inspecting and running non-production Kani gallery cases.
#[derive(Debug, Args)]
pub struct GalleryArgs {
    #[command(subcommand)]
    command: GalleryCommand,
}

#[derive(Debug, Subcommand)]
enum GalleryCommand {
    /// List the registered proof-gallery cases without running Kani.
    List,
    /// Run one or all registered proof-gallery cases through Kani.
    Run(RunGalleryArgs),
}

/// Execution options for `amenable gallery run`.
#[derive(Debug, Args)]
struct RunGalleryArgs {
    /// Run one exact, fully-qualified gallery-case ID.
    #[arg(long)]
    case: Option<String>,
    /// CSV ledger path.
    #[arg(short, long, default_value_os_t = default_results_path())]
    results: PathBuf,
    /// Kani-native timeout applied to each harness, for example `3m`.
    #[arg(long, default_value = "3m")]
    harness_timeout: String,
}

#[instrument(level = "debug")]
fn default_results_path() -> PathBuf {
    crate::paths::artifacts_directory().join("kani-gallery-results.csv")
}

/// Execute a proof-gallery command.
#[instrument(level = "info", skip(args))]
pub fn run(args: GalleryArgs) -> AmenableResult<()> {
    match args.command {
        GalleryCommand::List => list_cases(),
        GalleryCommand::Run(args) => run_cases(args),
    }
}

#[instrument(level = "debug")]
fn list_cases() -> AmenableResult<()> {
    let cases = registered_cases();
    if cases.is_empty() {
        return Err(AmenableError::invariant(
            "no Kani proof-gallery cases are registered",
        ));
    }

    for case in cases {
        println!(
            "{} [{} / {}] {}",
            case.id(),
            case.disposition().as_str(),
            case.expected().as_str(),
            case.title()
        );
    }

    Ok(())
}

#[instrument(level = "info", skip(args))]
fn run_cases(args: RunGalleryArgs) -> AmenableResult<()> {
    let cases = registered_cases();
    let selected = select_cases(&cases, args.case.as_deref())?;
    if selected.is_empty() {
        println!("No proof-gallery cases matched the selection.");
        return Ok(());
    }

    let mut ledger = Ledger::load(&args.results)?;
    println!("Kani proof gallery: {} case(s)", selected.len());
    println!("Results: {}", args.results.display());
    println!("Per-harness timeout: {}", args.harness_timeout);

    let mut mismatches = 0;
    for (index, case) in selected.iter().enumerate() {
        println!("[{}/{}] Running {}", index + 1, selected.len(), case.id());
        let run = run_case(case, &args.harness_timeout);
        if let Some(message) = &run.message {
            eprintln!("    {message}");
        }

        if run.observed == case.expected() {
            println!("    observed {}", run.observed.as_str());
        } else {
            println!(
                "    observed {} (expected {})",
                run.observed.as_str(),
                case.expected().as_str()
            );
            mismatches += 1;
        }

        ledger.upsert(case, run.observed)?;
        ledger.persist(&args.results)?;
    }

    if mismatches == 0 {
        Ok(())
    } else {
        Err(AmenableError::invariant(format!(
            "{mismatches} proof-gallery case(s) did not match their expected outcome"
        )))
    }
}

#[instrument(level = "debug")]
fn registered_cases() -> Vec<KaniGalleryCase> {
    let mut cases: Vec<_> = inventory::iter::<KaniGalleryRegistration>()
        .map(|registration| (registration.case())())
        .collect();
    cases.sort_unstable_by(|left, right| left.id().cmp(right.id()));
    cases
}

#[instrument(level = "debug", skip(cases))]
fn select_cases<'a>(
    cases: &'a [KaniGalleryCase],
    id: Option<&str>,
) -> AmenableResult<Vec<&'a KaniGalleryCase>> {
    match id {
        Some(id) => cases
            .iter()
            .find(|case| case.id() == id)
            .map(|case| vec![case])
            .ok_or_else(|| {
                AmenableError::invariant(format!("unknown Kani proof-gallery case ID: {id}"))
            }),
        None => Ok(cases.iter().collect()),
    }
}

#[instrument(level = "info", skip(case))]
fn run_case(case: &KaniGalleryCase, harness_timeout: &str) -> GalleryRun {
    let output = kani_command(case, harness_timeout)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(output) if output.status.success() => GalleryRun {
            observed: KaniGalleryExpectation::Passed,
            message: None,
        },
        Ok(output) => {
            let diagnostics = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let observed = if is_kani_timeout(&diagnostics) {
                KaniGalleryExpectation::Timeout
            } else {
                KaniGalleryExpectation::Failed
            };
            GalleryRun {
                observed,
                message: first_diagnostic_line(&diagnostics),
            }
        }
        Err(error) => GalleryRun {
            observed: KaniGalleryExpectation::Failed,
            message: Some(format!("could not start cargo kani: {error}")),
        },
    }
}

/// `-Z function-contracts`/`-Z stubbing` are always enabled -- required
/// for any case using `kani::requires`/`kani::ensures`/`kani::
/// proof_for_contract`/`kani::stub_verified`, and harmless for a case
/// that doesn't, matching `kani::kani_command`'s own identical
/// canonical-invocation rationale (missing here found the hard way:
/// `commit_contract_no_wrapper`, a real `#[kani::proof_for_contract]`
/// gallery case, failed with "requires activating the unstable
/// `function-contracts` feature" via `gallery run` even though the
/// identical harness shape verifies clean through `verify kani`).
#[instrument(level = "debug", skip(case))]
fn kani_command(case: &KaniGalleryCase, harness_timeout: &str) -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "kani",
        "-p",
        case.package().as_str(),
        "--lib",
        "--all-features",
        "--exact",
        "--harness",
        case.harness().as_str(),
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

#[instrument(level = "trace", ret)]
fn is_kani_timeout(diagnostics: &str) -> bool {
    let diagnostics = diagnostics.to_ascii_lowercase();
    diagnostics.contains("verification timed out")
        || diagnostics.contains("harness timed out")
        || diagnostics.contains("timed out")
}

#[instrument(level = "debug")]
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

struct GalleryRun {
    observed: KaniGalleryExpectation,
    message: Option<String>,
}

struct LedgerRow {
    timestamp: String,
    disposition: String,
    expected: KaniGalleryExpectation,
    observed: KaniGalleryExpectation,
    matched: bool,
}

struct Ledger {
    rows: BTreeMap<String, LedgerRow>,
}

impl Ledger {
    #[instrument(level = "debug", skip(path))]
    fn load(path: &Path) -> AmenableResult<Self> {
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
                    "invalid Kani proof-gallery ledger header in {}",
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
            let case_id = fields.next().unwrap_or_default();
            let timestamp = fields.next().unwrap_or_default();
            let disposition = fields.next().unwrap_or_default();
            let expected = fields.next().and_then(KaniGalleryExpectation::parse);
            let observed = fields.next().and_then(KaniGalleryExpectation::parse);
            let matched = fields.next().and_then(parse_bool);
            let row_is_malformed = fields.next().is_some()
                || case_id.is_empty()
                || timestamp.is_empty()
                || disposition.is_empty();

            let (false, Some(expected), Some(observed), Some(matched)) =
                (row_is_malformed, expected, observed, matched)
            else {
                return Err(AmenableError::invariant(format!(
                    "invalid Kani proof-gallery ledger row {} in {}",
                    line_number + 2,
                    path.display()
                )));
            };

            rows.insert(
                case_id.to_owned(),
                LedgerRow {
                    timestamp: timestamp.to_owned(),
                    disposition: disposition.to_owned(),
                    expected,
                    observed,
                    matched,
                },
            );
        }

        Ok(Self { rows })
    }

    #[instrument(level = "debug", skip(self, case, observed))]
    fn upsert(
        &mut self,
        case: &KaniGalleryCase,
        observed: KaniGalleryExpectation,
    ) -> AmenableResult<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs()
            .to_string();
        self.rows.insert(
            case.id().clone(),
            LedgerRow {
                timestamp,
                disposition: case.disposition().as_str().to_owned(),
                expected: case.expected(),
                observed,
                matched: observed == case.expected(),
            },
        );
        Ok(())
    }

    #[instrument(level = "debug", skip(self, path))]
    fn persist(&self, path: &Path) -> AmenableResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| AmenableError::io(parent, error))?;
        }

        let mut contents = format!("{LEDGER_HEADER}\n");
        for (case_id, row) in &self.rows {
            contents.push_str(case_id);
            contents.push(',');
            contents.push_str(&row.timestamp);
            contents.push(',');
            contents.push_str(&row.disposition);
            contents.push(',');
            contents.push_str(row.expected.as_str());
            contents.push(',');
            contents.push_str(row.observed.as_str());
            contents.push(',');
            contents.push_str(if row.matched { "true" } else { "false" });
            contents.push('\n');
        }

        let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
        fs::write(&temporary, contents).map_err(|error| AmenableError::io(&temporary, error))?;
        fs::rename(&temporary, path).map_err(|error| AmenableError::io(path, error))
    }
}

#[instrument(level = "debug")]
fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
