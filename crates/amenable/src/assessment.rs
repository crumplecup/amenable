//! Structured, append-only assessments of executable proof harnesses.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use amenable::{KaniProof, KaniProofRegistration};
use clap::{Args, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use time::{
    Date, OffsetDateTime,
    format_description::{self, well_known::Rfc3339},
};

const CURRENT_ASSESSMENT_VERSION: &str = "0.1.0";
const LEGACY_SCHEMA_VERSION: u8 = 1;

/// Commands for recording and examining proof assessments.
#[derive(Debug, Args)]
pub(super) struct AssessArgs {
    #[command(subcommand)]
    command: AssessCommand,
}

#[derive(Debug, Subcommand)]
enum AssessCommand {
    /// Append one reviewer assessment for a registered Kani proof.
    Proof(RecordAssessmentArgs),
    /// Summarize assessment counts by recommendation.
    Summary(AssessmentSummaryArgs),
    /// List recorded assessments, optionally filtered by recommendation.
    List(AssessmentListArgs),
    /// Summarize recorded assessments, optionally for one proof.
    Report(AssessmentReportArgs),
    /// List registered proofs that have no assessment yet.
    Queue(AssessmentQueueArgs),
}

/// Arguments that create one assessment record.
#[derive(Debug, Args)]
struct RecordAssessmentArgs {
    /// Exact, fully-qualified registered Kani proof ID.
    #[arg(long)]
    proof: String,
    /// Person or agent responsible for this assessment.
    #[arg(long)]
    reviewer: String,
    /// How well the assertion establishes the intended semantic property (0-4).
    #[arg(long, value_parser = parse_score)]
    claim_alignment: u8,
    /// How justified, representative, and non-vacuous assumptions are (0-4).
    #[arg(long, value_parser = parse_score)]
    assumption_adequacy: u8,
    /// How faithfully the harness exercises production behavior (0-4).
    #[arg(long, value_parser = parse_score)]
    model_fidelity: u8,
    /// How strongly the oracle rules out incorrect outcomes (0-4).
    #[arg(long, value_parser = parse_score)]
    assertion_strength: u8,
    /// How well boundary, error, state, and aliasing cases are covered (0-4).
    #[arg(long, value_parser = parse_score)]
    adversarial_coverage: u8,
    /// How understandable and safe to evolve the proof is (0-4).
    #[arg(long, value_parser = parse_score)]
    clarity: u8,
    /// Recommended next action for this proof.
    #[arg(long, value_enum)]
    recommendation: Recommendation,
    /// Long-form reasoning supporting the scores and recommendation.
    #[arg(
        long,
        required_unless_present = "comment_file",
        conflicts_with = "comment_file"
    )]
    comment: Option<String>,
    /// File containing long-form reasoning supporting the assessment.
    #[arg(long, required_unless_present = "comment", conflicts_with = "comment")]
    comment_file: Option<PathBuf>,
    /// Append to this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    assessments: PathBuf,
}

/// Arguments that render assessment summaries.
#[derive(Debug, Args)]
struct AssessmentReportArgs {
    /// Restrict the report to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    proof: Option<String>,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    assessments: PathBuf,
}

/// Arguments that render recommendation counts.
#[derive(Debug, Args)]
struct AssessmentSummaryArgs {
    /// Restrict the summary to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    proof: Option<String>,
    /// Only count assessments recorded on or after this UTC date (`YYYY-MM-DD`).
    #[arg(long, value_parser = parse_utc_date)]
    since: Option<Date>,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    assessments: PathBuf,
}

/// Arguments that list recorded assessments.
#[derive(Debug, Args)]
struct AssessmentListArgs {
    /// Restrict the list to one exact, fully-qualified registered proof ID.
    #[arg(long)]
    proof: Option<String>,
    /// Restrict the list to one recommendation status.
    #[arg(long, value_enum)]
    recommendation: Option<Recommendation>,
    /// Only list assessments recorded on or after this UTC date (`YYYY-MM-DD`).
    #[arg(long, value_parser = parse_utc_date)]
    since: Option<Date>,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    assessments: PathBuf,
}

/// Arguments that render the unassessed-proof queue.
#[derive(Debug, Args)]
struct AssessmentQueueArgs {
    /// Only count assessments recorded on or after this UTC date (`YYYY-MM-DD`).
    ///
    /// Older assessments do not satisfy the queue when running a fresh sweep.
    #[arg(long, value_parser = parse_utc_date)]
    since: Option<Date>,
    /// Read this JSON Lines assessment artifact.
    #[arg(short, long, default_value_os_t = default_assessment_path())]
    assessments: PathBuf,
}

/// A reviewer's recommended next action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
enum Recommendation {
    /// The proof is persuasive evidence in its current form.
    Accept,
    /// The proof should be strengthened while retaining its current direction.
    Strengthen,
    /// The proof should be replaced with a substantively different argument.
    Replace,
    /// The proof should no longer be relied upon.
    Retire,
}

impl Recommendation {
    fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Strengthen => "strengthen",
            Self::Replace => "replace",
            Self::Retire => "retire",
        }
    }
}

/// Six independently scored dimensions of proof quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct Rubric {
    claim_alignment: u8,
    assumption_adequacy: u8,
    model_fidelity: u8,
    assertion_strength: u8,
    adversarial_coverage: u8,
    clarity: u8,
}

impl Rubric {
    fn validate(self) -> Result<(), String> {
        for (name, score) in [
            ("claim_alignment", self.claim_alignment),
            ("assumption_adequacy", self.assumption_adequacy),
            ("model_fidelity", self.model_fidelity),
            ("assertion_strength", self.assertion_strength),
            ("adversarial_coverage", self.adversarial_coverage),
            ("clarity", self.clarity),
        ] {
            if score > 4 {
                return Err(format!(
                    "invalid {name} score {score}; expected an integer from 0 to 4"
                ));
            }
        }

        Ok(())
    }

    fn values(self) -> [(String, u8); 6] {
        [
            ("claim alignment".to_owned(), self.claim_alignment),
            ("assumption adequacy".to_owned(), self.assumption_adequacy),
            ("model fidelity".to_owned(), self.model_fidelity),
            ("assertion strength".to_owned(), self.assertion_strength),
            ("adversarial coverage".to_owned(), self.adversarial_coverage),
            ("clarity".to_owned(), self.clarity),
        ]
    }
}

/// One immutable assessment event stored in the JSON Lines artifact.
#[derive(Debug)]
struct ProofAssessment {
    version: String,
    proof_id: String,
    reviewer: String,
    timestamp: u64,
    rubric: Rubric,
    recommendation: Recommendation,
    comment: String,
}

impl ProofAssessment {
    fn validate(&self) -> Result<(), String> {
        if self.version != CURRENT_ASSESSMENT_VERSION {
            return Err(format!(
                "unsupported assessment version {}; expected {CURRENT_ASSESSMENT_VERSION}",
                self.version
            ));
        }
        if self.proof_id.trim().is_empty() || self.reviewer.trim().is_empty() {
            return Err("assessment proof ID and reviewer must not be empty".to_owned());
        }
        if self.comment.trim().is_empty() {
            return Err("assessment comment must not be empty".to_owned());
        }

        self.rubric.validate()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredProofAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    schema_version: Option<u8>,
    proof_id: String,
    reviewer: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timestamp_unix_seconds: Option<u64>,
    rubric: Rubric,
    recommendation: Recommendation,
    comment: String,
}

impl StoredProofAssessment {
    fn into_assessment(self) -> Result<ProofAssessment, String> {
        let proof_id = self.proof_id;
        let version = match (self.version, self.schema_version) {
            (Some(version), None) => version,
            (None, Some(LEGACY_SCHEMA_VERSION)) => CURRENT_ASSESSMENT_VERSION.to_owned(),
            (None, Some(schema_version)) => {
                return Err(format!(
                    "unsupported legacy assessment schema version {schema_version}; expected {LEGACY_SCHEMA_VERSION}"
                ));
            }
            (Some(_), Some(_)) => {
                return Err(format!(
                    "assessment record for {proof_id} must not contain both version and schema_version"
                ));
            }
            (None, None) => {
                return Err(format!(
                    "assessment record for {proof_id} is missing version metadata"
                ));
            }
        };

        let timestamp = match (self.timestamp, self.timestamp_unix_seconds) {
            (Some(timestamp), None) => timestamp,
            (None, Some(timestamp)) => timestamp,
            (Some(_), Some(_)) => {
                return Err(format!(
                    "assessment record for {proof_id} must not contain both timestamp and timestamp_unix_seconds"
                ));
            }
            (None, None) => {
                return Err(format!(
                    "assessment record for {proof_id} is missing timestamp metadata"
                ));
            }
        };

        Ok(ProofAssessment {
            version,
            proof_id,
            reviewer: self.reviewer,
            timestamp,
            rubric: self.rubric,
            recommendation: self.recommendation,
            comment: self.comment,
        })
    }
}

impl From<&ProofAssessment> for StoredProofAssessment {
    fn from(assessment: &ProofAssessment) -> Self {
        Self {
            version: Some(assessment.version.clone()),
            schema_version: None,
            proof_id: assessment.proof_id.clone(),
            reviewer: assessment.reviewer.clone(),
            timestamp: Some(assessment.timestamp),
            timestamp_unix_seconds: None,
            rubric: assessment.rubric,
            recommendation: assessment.recommendation,
            comment: assessment.comment.clone(),
        }
    }
}

/// Execute an assessment command.
pub(super) fn run(args: AssessArgs) -> Result<(), String> {
    match args.command {
        AssessCommand::Proof(args) => record(args),
        AssessCommand::Summary(args) => summary(args),
        AssessCommand::List(args) => list(args),
        AssessCommand::Report(args) => report(args),
        AssessCommand::Queue(args) => queue(args),
    }
}

fn default_assessment_path() -> PathBuf {
    super::artifacts_directory().join("proof-assessments.jsonl")
}

fn parse_score(value: &str) -> Result<u8, String> {
    let score: u8 = value
        .parse()
        .map_err(|_| format!("invalid score {value:?}; expected an integer from 0 to 4"))?;
    if score > 4 {
        return Err(format!(
            "invalid score {score}; expected an integer from 0 to 4"
        ));
    }

    Ok(score)
}

fn parse_utc_date(value: &str) -> Result<Date, String> {
    let format = format_description::parse_borrowed::<2>("[year]-[month]-[day]")
        .map_err(|error| format!("internal date-format error: {error}"))?;
    Date::parse(value, &format)
        .map_err(|error| format!("invalid date {value:?}; expected YYYY-MM-DD: {error}"))
}

fn record(args: RecordAssessmentArgs) -> Result<(), String> {
    ensure_registered(&args.proof)?;
    let comment = read_comment(args.comment, args.comment_file)?;
    let assessment = ProofAssessment {
        version: CURRENT_ASSESSMENT_VERSION.to_owned(),
        proof_id: args.proof,
        reviewer: args.reviewer,
        timestamp: timestamp()?,
        rubric: Rubric {
            claim_alignment: args.claim_alignment,
            assumption_adequacy: args.assumption_adequacy,
            model_fidelity: args.model_fidelity,
            assertion_strength: args.assertion_strength,
            adversarial_coverage: args.adversarial_coverage,
            clarity: args.clarity,
        },
        recommendation: args.recommendation,
        comment,
    };
    assessment.validate()?;
    append(&args.assessments, &assessment)?;

    println!(
        "Recorded {} assessment for {} in {}",
        assessment.recommendation.as_str(),
        assessment.proof_id,
        args.assessments.display()
    );
    Ok(())
}

fn summary(args: AssessmentSummaryArgs) -> Result<(), String> {
    if let Some(proof) = &args.proof {
        ensure_registered(proof)?;
    }

    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessments = filtered_assessments(
        load(&args.assessments)?,
        args.proof.as_deref(),
        None,
        since_timestamp,
    );

    let mut counts: BTreeMap<Recommendation, usize> = Recommendation::value_variants()
        .iter()
        .copied()
        .map(|recommendation| (recommendation, 0))
        .collect();
    for assessment in assessments {
        *counts.entry(assessment.recommendation).or_default() += 1;
    }

    let recommendation_width = Recommendation::value_variants()
        .iter()
        .map(|recommendation| recommendation.as_str().len())
        .max()
        .unwrap_or("recommendation".len())
        .max("recommendation".len());
    let count_width = counts
        .values()
        .map(|count| count.to_string().len())
        .max()
        .unwrap_or(1)
        .max("count".len());

    println!(
        "{:<recommendation_width$} {:>count_width$}",
        "recommendation", "count"
    );
    println!(
        "{} {}",
        "-".repeat(recommendation_width),
        "-".repeat(count_width)
    );
    for recommendation in Recommendation::value_variants() {
        println!(
            "{:<recommendation_width$} {:>count_width$}",
            recommendation.as_str(),
            counts[recommendation]
        );
    }

    Ok(())
}

fn read_comment(comment: Option<String>, comment_file: Option<PathBuf>) -> Result<String, String> {
    match (comment, comment_file) {
        (Some(comment), None) => Ok(comment),
        (None, Some(path)) => fs::read_to_string(&path).map_err(|error| {
            format!(
                "could not read assessment comment {}: {error}",
                path.display()
            )
        }),
        (Some(_), Some(_)) => {
            Err("provide either --comment or --comment-file, not both".to_owned())
        }
        (None, None) => Err("provide --comment or --comment-file".to_owned()),
    }
}

fn list(args: AssessmentListArgs) -> Result<(), String> {
    if let Some(proof) = &args.proof {
        ensure_registered(proof)?;
    }

    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessments = filtered_assessments(
        load(&args.assessments)?,
        args.proof.as_deref(),
        args.recommendation,
        since_timestamp,
    );

    if assessments.is_empty() {
        println!("No proof assessments matched the selection.");
        return Ok(());
    }

    for assessment in assessments {
        let recorded_at = format_timestamp(assessment.timestamp)?;
        println!(
            "{recorded_at}\t{}\t{}\t{}",
            assessment.recommendation.as_str(),
            assessment.proof_id,
            assessment.reviewer
        );
    }

    Ok(())
}

fn report(args: AssessmentReportArgs) -> Result<(), String> {
    if let Some(proof) = &args.proof {
        ensure_registered(proof)?;
    }

    let assessments = load(&args.assessments)?;
    let mut by_proof: BTreeMap<String, Vec<ProofAssessment>> = BTreeMap::new();
    for assessment in assessments {
        if args
            .proof
            .as_ref()
            .is_none_or(|proof| assessment.proof_id == *proof)
        {
            by_proof
                .entry(assessment.proof_id.clone())
                .or_default()
                .push(assessment);
        }
    }

    if by_proof.is_empty() {
        println!("No proof assessments matched the selection.");
        return Ok(());
    }

    for (proof_id, entries) in by_proof {
        print_summary(&proof_id, &entries);
    }
    Ok(())
}

fn queue(args: AssessmentQueueArgs) -> Result<(), String> {
    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessments = load(&args.assessments)?;
    let assessed: BTreeSet<_> = assessments
        .iter()
        .filter(|assessment| {
            since_timestamp.is_none_or(|threshold| assessment.timestamp >= threshold)
        })
        .map(|assessment| assessment.proof_id.as_str())
        .collect();
    let unassessed: Vec<_> = registered_proofs()
        .into_iter()
        .filter(|proof| !assessed.contains(proof.id.as_str()))
        .collect();

    if unassessed.is_empty() {
        println!("Every registered Kani proof has at least one assessment.");
        return Ok(());
    }

    println!("Unassessed Kani proofs: {}", unassessed.len());
    for proof in unassessed {
        println!("{}", proof.id);
    }
    Ok(())
}

fn filtered_assessments(
    assessments: Vec<ProofAssessment>,
    proof: Option<&str>,
    recommendation: Option<Recommendation>,
    since_timestamp: Option<u64>,
) -> Vec<ProofAssessment> {
    assessments
        .into_iter()
        .filter(|assessment| proof.is_none_or(|proof_id| assessment.proof_id == proof_id))
        .filter(|assessment| {
            recommendation.is_none_or(|wanted| assessment.recommendation == wanted)
        })
        .filter(|assessment| {
            since_timestamp.is_none_or(|threshold| assessment.timestamp >= threshold)
        })
        .collect()
}

fn print_summary(proof_id: &str, entries: &[ProofAssessment]) {
    println!("{proof_id}: {} assessment(s)", entries.len());

    for (index, (name, _)) in entries[0].rubric.values().into_iter().enumerate() {
        let scores: Vec<_> = entries
            .iter()
            .map(|entry| entry.rubric.values()[index].1)
            .collect();
        let mean =
            scores.iter().map(|score| u32::from(*score)).sum::<u32>() as f64 / scores.len() as f64;
        let distribution = (0..=4)
            .map(|score| {
                let count = scores.iter().filter(|entry| **entry == score).count();
                format!("{score}:{count}")
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("  {name}: mean {mean:.2}; {distribution}");
    }

    let mut recommendations: BTreeMap<Recommendation, usize> = BTreeMap::new();
    for entry in entries {
        *recommendations.entry(entry.recommendation).or_default() += 1;
    }
    let recommendations = recommendations
        .into_iter()
        .map(|(recommendation, count)| format!("{}:{count}", recommendation.as_str()))
        .collect::<Vec<_>>()
        .join(" ");
    println!("  recommendations: {recommendations}");
}

fn registered_proofs() -> Vec<KaniProof> {
    let mut proofs: Vec<_> = inventory::iter::<KaniProofRegistration>()
        .map(|registration| (registration.proof)())
        .collect();
    proofs.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    proofs
}

fn ensure_registered(proof_id: &str) -> Result<(), String> {
    registered_proofs()
        .into_iter()
        .any(|proof| proof.id == proof_id)
        .then_some(())
        .ok_or_else(|| format!("unknown registered Kani proof ID: {proof_id}"))
}

fn timestamp() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("system clock is before the Unix epoch: {error}"))
        .map(|duration| duration.as_secs())
}

fn start_of_utc_date_timestamp(date: Date) -> Result<u64, String> {
    let timestamp = date.midnight().assume_utc().unix_timestamp();
    u64::try_from(timestamp).map_err(|_| {
        format!("date {date} is before the Unix epoch; expected YYYY-MM-DD on or after 1970-01-01")
    })
}

fn format_timestamp(timestamp: u64) -> Result<String, String> {
    let seconds = i64::try_from(timestamp)
        .map_err(|_| format!("assessment timestamp {timestamp} is too large to format"))?;
    let recorded_at = OffsetDateTime::from_unix_timestamp(seconds)
        .map_err(|error| format!("invalid assessment timestamp {timestamp}: {error}"))?;
    recorded_at
        .format(&Rfc3339)
        .map_err(|error| format!("could not format assessment timestamp {timestamp}: {error}"))
}

fn load(path: &Path) -> Result<Vec<ProofAssessment>, String> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path).map_err(|error| {
        format!(
            "could not read assessment artifact {}: {error}",
            path.display()
        )
    })?;
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            let assessment: StoredProofAssessment =
                serde_json::from_str(line).map_err(|error| {
                    format!(
                        "invalid assessment JSON on line {} in {}: {error}",
                        index + 1,
                        path.display()
                    )
                })?;
            let assessment = assessment.into_assessment().map_err(|error| {
                format!(
                    "invalid assessment on line {} in {}: {error}",
                    index + 1,
                    path.display()
                )
            })?;
            assessment.validate().map_err(|error| {
                format!(
                    "invalid assessment on line {} in {}: {error}",
                    index + 1,
                    path.display()
                )
            })?;
            Ok(assessment)
        })
        .collect()
}

fn append(path: &Path, assessment: &ProofAssessment) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "could not create assessment directory {}: {error}",
                parent.display()
            )
        })?;
    }

    let record = serde_json::to_string(&StoredProofAssessment::from(assessment))
        .map_err(|error| format!("could not serialize proof assessment: {error}"))?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| {
            format!(
                "could not open assessment artifact {}: {error}",
                path.display()
            )
        })?;
    file.write_all(record.as_bytes())
        .and_then(|()| file.write_all(b"\n"))
        .map_err(|error| {
            format!(
                "could not append assessment artifact {}: {error}",
                path.display()
            )
        })
}
