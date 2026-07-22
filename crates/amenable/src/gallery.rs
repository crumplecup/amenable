//! CLI support for the Kani proof gallery.

use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use amenable::{KaniGalleryCase, KaniGalleryExpectation, KaniGalleryRegistration};
use clap::{Args, Subcommand};

const LEDGER_HEADER: &str =
    "case_id,timestamp,disposition,expected_status,observed_status,matched_expectation";

/// Commands for inspecting and running non-production Kani gallery cases.
#[derive(Debug, Args)]
pub(super) struct GalleryArgs {
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

fn default_results_path() -> PathBuf {
    super::artifacts_directory().join("kani-gallery-results.csv")
}

/// Execute a proof-gallery command.
pub(super) fn run(args: GalleryArgs) -> Result<(), String> {
    match args.command {
        GalleryCommand::List => list_cases(),
        GalleryCommand::Run(args) => run_cases(args),
    }
}

fn list_cases() -> Result<(), String> {
    let cases = registered_cases();
    if cases.is_empty() {
        return Err("no Kani proof-gallery cases are registered".to_owned());
    }

    for case in cases {
        println!(
            "{} [{} / {}] {}",
            case.id,
            case.disposition.as_str(),
            case.expected.as_str(),
            case.title
        );
    }

    Ok(())
}

fn run_cases(args: RunGalleryArgs) -> Result<(), String> {
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
        println!("[{}/{}] Running {}", index + 1, selected.len(), case.id);
        let run = run_case(case, &args.harness_timeout);
        if let Some(message) = &run.message {
            eprintln!("    {message}");
        }

        if run.observed == case.expected {
            println!("    observed {}", run.observed.as_str());
        } else {
            println!(
                "    observed {} (expected {})",
                run.observed.as_str(),
                case.expected.as_str()
            );
            mismatches += 1;
        }

        ledger.upsert(case, run.observed)?;
        ledger.persist(&args.results)?;
    }

    if mismatches == 0 {
        Ok(())
    } else {
        Err(format!(
            "{mismatches} proof-gallery case(s) did not match their expected outcome"
        ))
    }
}

fn registered_cases() -> Vec<KaniGalleryCase> {
    let mut cases: Vec<_> = inventory::iter::<KaniGalleryRegistration>()
        .map(|registration| (registration.case)())
        .collect();
    cases.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    cases
}

fn select_cases<'a>(
    cases: &'a [KaniGalleryCase],
    id: Option<&str>,
) -> Result<Vec<&'a KaniGalleryCase>, String> {
    match id {
        Some(id) => cases
            .iter()
            .find(|case| case.id == id)
            .map(|case| vec![case])
            .ok_or_else(|| format!("unknown Kani proof-gallery case ID: {id}")),
        None => Ok(cases.iter().collect()),
    }
}

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

fn kani_command(case: &KaniGalleryCase, harness_timeout: &str) -> Command {
    let mut command = Command::new("cargo");
    command.args([
        "kani",
        "-p",
        case.package.as_str(),
        "--lib",
        "--all-features",
        "--exact",
        "--harness",
        case.harness.as_str(),
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
    fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self {
                rows: BTreeMap::new(),
            });
        }

        let contents = fs::read_to_string(path).map_err(|error| {
            format!("could not read gallery ledger {}: {error}", path.display())
        })?;
        let mut lines = contents.lines();
        match lines.next() {
            Some(LEDGER_HEADER) => {}
            _ => {
                return Err(format!(
                    "invalid Kani proof-gallery ledger header in {}",
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
            let case_id = fields.next().unwrap_or_default();
            let timestamp = fields.next().unwrap_or_default();
            let disposition = fields.next().unwrap_or_default();
            let expected = fields.next().and_then(KaniGalleryExpectation::parse);
            let observed = fields.next().and_then(KaniGalleryExpectation::parse);
            let matched = fields.next().and_then(parse_bool);
            if fields.next().is_some()
                || case_id.is_empty()
                || timestamp.is_empty()
                || disposition.is_empty()
                || expected.is_none()
                || observed.is_none()
                || matched.is_none()
            {
                return Err(format!(
                    "invalid Kani proof-gallery ledger row {} in {}",
                    line_number + 2,
                    path.display()
                ));
            }

            rows.insert(
                case_id.to_owned(),
                LedgerRow {
                    timestamp: timestamp.to_owned(),
                    disposition: disposition.to_owned(),
                    expected: expected.expect("validated above"),
                    observed: observed.expect("validated above"),
                    matched: matched.expect("validated above"),
                },
            );
        }

        Ok(Self { rows })
    }

    fn upsert(
        &mut self,
        case: &KaniGalleryCase,
        observed: KaniGalleryExpectation,
    ) -> Result<(), String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock is before the Unix epoch: {error}"))?
            .as_secs()
            .to_string();
        self.rows.insert(
            case.id.clone(),
            LedgerRow {
                timestamp,
                disposition: case.disposition.as_str().to_owned(),
                expected: case.expected,
                observed,
                matched: observed == case.expected,
            },
        );
        Ok(())
    }

    fn persist(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "could not create gallery ledger directory {}: {error}",
                    parent.display()
                )
            })?;
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
        fs::write(&temporary, contents).map_err(|error| {
            format!(
                "could not write temporary Kani proof-gallery ledger {}: {error}",
                temporary.display()
            )
        })?;
        fs::rename(&temporary, path).map_err(|error| {
            format!(
                "could not replace Kani proof-gallery ledger {}: {error}",
                path.display()
            )
        })
    }
}

fn parse_bool(value: &str) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}
