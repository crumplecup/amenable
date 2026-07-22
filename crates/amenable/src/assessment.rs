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

const SCHEMA_VERSION: u8 = 1;

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

/// Arguments that render the unassessed-proof queue.
#[derive(Debug, Args)]
struct AssessmentQueueArgs {
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
#[derive(Debug, Serialize, Deserialize)]
struct ProofAssessment {
    schema_version: u8,
    proof_id: String,
    reviewer: String,
    timestamp_unix_seconds: u64,
    rubric: Rubric,
    recommendation: Recommendation,
    comment: String,
}

impl ProofAssessment {
    fn validate(&self) -> Result<(), String> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "unsupported assessment schema version {}; expected {SCHEMA_VERSION}",
                self.schema_version
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

/// Execute an assessment command.
pub(super) fn run(args: AssessArgs) -> Result<(), String> {
    match args.command {
        AssessCommand::Proof(args) => record(args),
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

fn record(args: RecordAssessmentArgs) -> Result<(), String> {
    ensure_registered(&args.proof)?;
    let comment = read_comment(args.comment, args.comment_file)?;
    let assessment = ProofAssessment {
        schema_version: SCHEMA_VERSION,
        proof_id: args.proof,
        reviewer: args.reviewer,
        timestamp_unix_seconds: timestamp()?,
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
    let assessments = load(&args.assessments)?;
    let assessed: BTreeSet<_> = assessments
        .iter()
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
            let assessment: ProofAssessment = serde_json::from_str(line).map_err(|error| {
                format!(
                    "invalid assessment JSON on line {} in {}: {error}",
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

    let record = serde_json::to_string(assessment)
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
