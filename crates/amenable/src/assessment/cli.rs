//! The `clap` argument surface for `amenable assess`.

use crate::assessment::vocabulary::{Recommendation, ResolutionPath, SummaryDimension};
use crate::kani::{self, ProofStatus};
use crate::{AmenableError, AmenableResult};
use clap::{Args, Subcommand};
use std::path::PathBuf;
use time::{Date, format_description};
use tracing::instrument;

/// Commands for recording and examining proof assessments.
#[derive(Debug, Args)]
pub struct AssessArgs {
    #[command(subcommand)]
    command: AssessCommand,
}

impl AssessArgs {
    /// Dispatch to the selected [`AssessCommand`].
    #[instrument(level = "info", skip(self))]
    pub(crate) fn act(self) -> AmenableResult<()> {
        self.command.act()
    }
}

#[derive(Debug, Subcommand)]
enum AssessCommand {
    /// Append one reviewer assessment for a registered proof (Kani, Creusot, or Verus).
    Proof(RecordAssessmentArgs),
    /// List latest Kani verification results that are not currently passing.
    Failures(VerificationFailuresArgs),
    /// Summarize assessment counts by recommendation.
    Summary(AssessmentSummaryArgs),
    /// List recorded assessments, optionally filtered by recommendation.
    List(AssessmentListArgs),
    /// Summarize recorded assessments, optionally for one proof.
    Report(AssessmentReportArgs),
    /// List registered proofs that have no assessment yet.
    Queue(AssessmentQueueArgs),
}

impl AssessCommand {
    #[instrument(level = "debug", skip(self))]
    fn act(self) -> AmenableResult<()> {
        match self {
            Self::Proof(args) => super::commands::record(args),
            Self::Failures(args) => super::commands::failures(args),
            Self::Summary(args) => super::commands::summary(args),
            Self::List(args) => super::commands::list(args),
            Self::Report(args) => super::commands::report(args),
            Self::Queue(args) => super::commands::queue(args),
        }
    }
}

/// Arguments that create one assessment record.
#[derive(Debug, Args)]
pub(super) struct RecordAssessmentArgs {
    /// Exact, fully-qualified registered proof ID (a KaniProof.id, or amenable_{creusot,verus}::{harness} for the other two backends).
    #[arg(long)]
    pub(super) proof: String,
    /// Person or agent responsible for this assessment.
    #[arg(long)]
    pub(super) reviewer: String,
    /// How well the assertion establishes the intended semantic property (0-4).
    #[arg(long, value_parser = parse_score)]
    pub(super) claim_alignment: u8,
    /// How justified, representative, and non-vacuous assumptions are (0-4).
    #[arg(long, value_parser = parse_score)]
    pub(super) assumption_adequacy: u8,
    /// How faithfully the harness exercises production behavior (0-4).
    #[arg(long, value_parser = parse_score)]
    pub(super) model_fidelity: u8,
    /// How strongly the oracle rules out incorrect outcomes (0-4).
    #[arg(long, value_parser = parse_score)]
    pub(super) assertion_strength: u8,
    /// How well boundary, error, state, and aliasing cases are covered (0-4).
    #[arg(long, value_parser = parse_score)]
    pub(super) adversarial_coverage: u8,
    /// How understandable and safe to evolve the proof is (0-4).
    #[arg(long, value_parser = parse_score)]
    pub(super) clarity: u8,
    /// Recommended next action for this proof.
    #[arg(long, value_enum)]
    pub(super) recommendation: Recommendation,
    /// Explicit triage lane for acting on this assessment.
    #[arg(long, value_enum)]
    pub(super) resolution_path: ResolutionPath,
    /// Long-form reasoning supporting the scores and recommendation.
    #[arg(
        long,
        required_unless_present = "comment_file",
        conflicts_with = "comment_file"
    )]
    pub(super) comment: Option<String>,
    /// File containing long-form reasoning supporting the assessment.
    #[arg(long, required_unless_present = "comment", conflicts_with = "comment")]
    pub(super) comment_file: Option<PathBuf>,
    /// Append to this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    pub(super) assessments: PathBuf,
}

/// Arguments that render assessment summaries.
#[derive(Debug, Args)]
pub(super) struct AssessmentReportArgs {
    /// Restrict the report to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    pub(super) proof: Option<String>,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    pub(super) assessments: PathBuf,
}

/// Arguments that render recommendation counts.
#[derive(Debug, Args)]
pub(super) struct AssessmentSummaryArgs {
    /// Restrict the summary to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    pub(super) proof: Option<String>,
    /// Dimension to aggregate when counting matching assessments.
    #[arg(long, value_enum, default_value_t = SummaryDimension::Recommendation)]
    pub(super) by: SummaryDimension,
    /// Only count assessments recorded on or after this UTC date (`YYYY-MM-DD`).
    #[arg(long, value_parser = parse_utc_date)]
    pub(super) since: Option<Date>,
    /// Emit the summary as pretty JSON instead of a text table.
    #[arg(long)]
    pub(super) json: bool,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    pub(super) assessments: PathBuf,
}

/// Arguments that list recorded assessments.
#[derive(Debug, Args)]
pub(super) struct AssessmentListArgs {
    /// Restrict the list to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    pub(super) proof: Option<String>,
    /// Restrict the list to one recommendation status.
    #[arg(long, value_enum)]
    pub(super) recommendation: Option<Recommendation>,
    /// Restrict the list to one explicit triage lane.
    #[arg(long, value_enum)]
    pub(super) resolution_path: Option<ResolutionPath>,
    /// Only list assessments recorded on or after this UTC date (`YYYY-MM-DD`).
    #[arg(long, value_parser = parse_utc_date)]
    pub(super) since: Option<Date>,
    /// Emit matching assessments as pretty JSON instead of tab-separated text.
    #[arg(long)]
    pub(super) json: bool,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    pub(super) assessments: PathBuf,
}

/// Arguments that render the unassessed-proof queue.
#[derive(Debug, Args)]
pub(super) struct AssessmentQueueArgs {
    /// Only count assessments recorded on or after this UTC date (`YYYY-MM-DD`).
    ///
    /// Older assessments do not satisfy the queue when running a fresh sweep.
    #[arg(long, value_parser = parse_utc_date)]
    pub(super) since: Option<Date>,
    /// Emit the queue as pretty JSON instead of plain text.
    #[arg(long)]
    pub(super) json: bool,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    pub(super) assessments: PathBuf,
}

/// Arguments that list non-passing Kani verification results.
#[derive(Debug, Args)]
pub(super) struct VerificationFailuresArgs {
    /// Restrict the list to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    pub(super) proof: Option<String>,
    /// Restrict the list to one latest verification status.
    ///
    /// By default, this lists every proof whose latest result is not `passed`.
    #[arg(long, value_enum)]
    pub(super) status: Option<ProofStatus>,
    /// Only list failing proofs that do not yet have an assessment.
    #[arg(long)]
    pub(super) needs_assessment: bool,
    /// Only count assessments recorded on or after this UTC date when evaluating
    /// `--needs-assessment`.
    #[arg(long, value_parser = parse_utc_date)]
    pub(super) since: Option<Date>,
    /// Emit matching verification results as pretty JSON instead of tab-separated text.
    #[arg(long)]
    pub(super) json: bool,
    /// Read this Kani verification CSV ledger.
    #[arg(short, long, default_value_os_t = kani::default_results_path())]
    pub(super) results: PathBuf,
    /// Read this JSON Lines assessment artifact when evaluating `--needs-assessment`.
    #[arg(short = 'a', long, default_value_os_t = default_assessment_path())]
    pub(super) assessments: PathBuf,
}

#[instrument(level = "debug")]
fn default_assessment_path() -> PathBuf {
    crate::paths::artifacts_directory().join("proof-assessments.jsonl")
}

#[instrument(level = "debug")]
fn parse_score(value: &str) -> AmenableResult<u8> {
    let score: u8 = value
        .parse()
        .map_err(|error| AmenableError::invalid_score(value, error))?;
    if score > 4 {
        return Err(AmenableError::invariant(format!(
            "invalid score {score}; expected an integer from 0 to 4"
        )));
    }

    Ok(score)
}

#[instrument(level = "debug")]
fn parse_utc_date(value: &str) -> AmenableResult<Date> {
    let format = format_description::parse_borrowed::<2>("[year]-[month]-[day]")?;
    Date::parse(value, &format).map_err(|error| AmenableError::invalid_utc_date(value, error))
}
