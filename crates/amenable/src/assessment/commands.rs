//! Effects: read/write the assessment artifact, query the proof catalog.

use super::catalog;
use super::cli::{
    AssessmentListArgs, AssessmentQueueArgs, AssessmentReportArgs, AssessmentSummaryArgs,
    RecordAssessmentArgs, VerificationFailuresArgs,
};
use super::record::{self, CURRENT_ASSESSMENT_VERSION, ProofAssessment, ProofAssessmentBuilder};
use super::vocabulary::{Recommendation, ResolutionPath, Rubric, RubricBuilder, SummaryDimension};
use crate::kani::{self, ProofStatus};
use crate::{AmenableError, AmenableResult};
use clap::ValueEnum;
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use time::{Date, OffsetDateTime, format_description::well_known::Rfc3339};
use tracing::instrument;

#[derive(Debug, Serialize)]
struct SummaryOutput {
    by: String,
    proof: Option<String>,
    since: Option<String>,
    counts: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize)]
struct ListedAssessment {
    assessment_id: Option<String>,
    version: String,
    proof_id: String,
    reviewer: String,
    timestamp: u64,
    recorded_at: String,
    rubric: Rubric,
    recommendation: Recommendation,
    resolution_path: Option<ResolutionPath>,
    comment: String,
}

#[derive(Debug, Serialize)]
struct QueueOutput {
    since: Option<String>,
    count: usize,
    proof_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ListedVerificationFailure {
    proof_id: String,
    timestamp: u64,
    recorded_at: String,
    status: ProofStatus,
}

#[instrument(level = "debug", skip(args))]
pub(super) fn record(args: RecordAssessmentArgs) -> AmenableResult<()> {
    catalog::ensure_registered(&args.proof)?;
    let comment = read_comment(args.comment, args.comment_file)?;
    let rubric = RubricBuilder::default()
        .claim_alignment(args.claim_alignment)
        .assumption_adequacy(args.assumption_adequacy)
        .model_fidelity(args.model_fidelity)
        .assertion_strength(args.assertion_strength)
        .adversarial_coverage(args.adversarial_coverage)
        .clarity(args.clarity)
        .build()
        .map_err(|error| AmenableError::invariant(error.to_string()))?;
    let assessment = ProofAssessmentBuilder::default()
        .version(CURRENT_ASSESSMENT_VERSION.to_owned())
        .assessment_id(Some(assessment_id()?))
        .proof_id(args.proof)
        .reviewer(args.reviewer)
        .timestamp(timestamp()?)
        .rubric(rubric)
        .recommendation(args.recommendation)
        .resolution_path(Some(args.resolution_path))
        .comment(comment)
        .build()
        .map_err(|error| AmenableError::invariant(error.to_string()))?;
    assessment.validate()?;
    record::append(&args.assessments, &assessment)?;

    crate::write_stdout_line(format!(
        "Recorded {} assessment for {} in {}",
        assessment.recommendation().as_str(),
        assessment.proof_id(),
        args.assessments.display()
    ))?;
    Ok(())
}

#[instrument(level = "debug", skip(args))]
pub(super) fn summary(args: AssessmentSummaryArgs) -> AmenableResult<()> {
    if let Some(proof) = &args.proof {
        catalog::ensure_registered(proof)?;
    }

    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessments = filtered_assessments(
        record::load(&args.assessments)?,
        args.proof.as_deref(),
        None,
        None,
        since_timestamp,
    );

    let counts = match args.by {
        SummaryDimension::Recommendation => {
            let mut counts: BTreeMap<String, usize> = Recommendation::value_variants()
                .iter()
                .copied()
                .map(|recommendation| (recommendation.as_str().to_owned(), 0))
                .collect();
            for assessment in assessments {
                *counts
                    .entry(assessment.recommendation().as_str().to_owned())
                    .or_default() += 1;
            }
            counts
        }
        SummaryDimension::ResolutionPath => {
            let mut counts: BTreeMap<String, usize> = ResolutionPath::value_variants()
                .iter()
                .copied()
                .map(|resolution_path| (resolution_path.as_str().to_owned(), 0))
                .collect();
            counts.insert("legacy_unspecified".to_owned(), 0);
            for assessment in assessments {
                let key = assessment
                    .resolution_path()
                    .map(ResolutionPath::as_str)
                    .unwrap_or("legacy_unspecified")
                    .to_owned();
                *counts.entry(key).or_default() += 1;
            }
            counts
        }
    };

    if args.json {
        let output = SummaryOutput {
            by: args.by.as_str().to_owned(),
            proof: args.proof,
            since: args.since.map(|date| date.to_string()),
            counts,
        };
        return print_json(&output);
    }

    let label_width = counts
        .keys()
        .map(String::len)
        .max()
        .unwrap_or(args.by.as_str().len())
        .max(args.by.as_str().len());
    let count_width = counts
        .values()
        .map(|count| count.to_string().len())
        .max()
        .unwrap_or(1)
        .max("count".len());

    crate::write_stdout_line(format!(
        "{:<label_width$} {:>count_width$}",
        args.by.as_str(),
        "count"
    ))?;
    crate::write_stdout_line(format!(
        "{} {}",
        "-".repeat(label_width),
        "-".repeat(count_width)
    ))?;
    for label in counts.keys() {
        crate::write_stdout_line(format!(
            "{:<label_width$} {:>count_width$}",
            label, counts[label]
        ))?;
    }

    Ok(())
}

#[instrument(level = "debug", skip(args))]
pub(super) fn failures(args: VerificationFailuresArgs) -> AmenableResult<()> {
    if let Some(proof) = &args.proof {
        catalog::ensure_registered(proof)?;
    }

    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessed = if args.needs_assessment {
        assessed_proof_ids(&args.assessments, since_timestamp)?
    } else {
        BTreeSet::new()
    };

    let registered: BTreeSet<_> = catalog::registered_proofs()
        .into_iter()
        .map(|proof| proof.id().to_owned())
        .collect();
    let mut failures = Vec::new();

    for result in kani::load_results(&args.results)? {
        if !registered.contains(result.proof_id()) {
            crate::write_stderr_line(format!(
                "Verification result is no longer registered and will be skipped: {}",
                result.proof_id()
            ))?;
            continue;
        }

        if args
            .proof
            .as_ref()
            .is_some_and(|proof| *result.proof_id() != *proof)
        {
            continue;
        }

        if !matches_failure_filter(result.status(), args.status) {
            continue;
        }

        if args.needs_assessment && assessed.contains(result.proof_id().as_str()) {
            continue;
        }

        failures.push(result);
    }

    if failures.is_empty() {
        crate::write_stdout_line("No Kani verification results matched the selection.")?;
        return Ok(());
    }

    if args.json {
        let listed = failures
            .into_iter()
            .map(|result| {
                let (proof_id, timestamp, status) = result.dissolve();
                Ok(ListedVerificationFailure {
                    proof_id,
                    timestamp,
                    recorded_at: format_timestamp(timestamp)?,
                    status,
                })
            })
            .collect::<AmenableResult<Vec<_>>>()?;
        return print_json(&listed);
    }

    for result in failures {
        let recorded_at = format_timestamp(result.timestamp())?;
        crate::write_stdout_line(format!(
            "{}\t{recorded_at}\t{}",
            result.status().as_str(),
            result.proof_id()
        ))?;
    }

    Ok(())
}

#[instrument(level = "info")]
fn read_comment(comment: Option<String>, comment_file: Option<PathBuf>) -> AmenableResult<String> {
    match (comment, comment_file) {
        (Some(comment), None) => Ok(comment),
        (None, Some(path)) => {
            fs::read_to_string(&path).map_err(|error| AmenableError::io(path, error))
        }
        (Some(_), Some(_)) => Err(AmenableError::invariant(
            "provide either --comment or --comment-file, not both",
        )),
        (None, None) => Err(AmenableError::invariant(
            "provide --comment or --comment-file",
        )),
    }
}

#[instrument(level = "debug", skip(args))]
pub(super) fn list(args: AssessmentListArgs) -> AmenableResult<()> {
    if let Some(proof) = &args.proof {
        catalog::ensure_registered(proof)?;
    }

    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessments = filtered_assessments(
        record::load(&args.assessments)?,
        args.proof.as_deref(),
        args.recommendation,
        args.resolution_path,
        since_timestamp,
    );

    if assessments.is_empty() {
        crate::write_stdout_line("No proof assessments matched the selection.")?;
        return Ok(());
    }

    if args.json {
        let listed = assessments
            .into_iter()
            .map(|assessment| {
                Ok(ListedAssessment {
                    assessment_id: assessment.assessment_id().map(str::to_owned),
                    version: assessment.version().to_owned(),
                    proof_id: assessment.proof_id().to_owned(),
                    reviewer: assessment.reviewer().to_owned(),
                    timestamp: assessment.timestamp(),
                    recorded_at: format_timestamp(assessment.timestamp())?,
                    rubric: assessment.rubric(),
                    recommendation: assessment.recommendation(),
                    resolution_path: assessment.resolution_path(),
                    comment: assessment.comment().to_owned(),
                })
            })
            .collect::<AmenableResult<Vec<_>>>()?;
        return print_json(&listed);
    }

    for assessment in assessments {
        let recorded_at = format_timestamp(assessment.timestamp())?;
        crate::write_stdout_line(format!(
            "{}\t{recorded_at}\t{}\t{}\t{}\t{}",
            assessment.assessment_id().unwrap_or("legacy-unidentified"),
            assessment.recommendation().as_str(),
            assessment
                .resolution_path()
                .map(ResolutionPath::as_str)
                .unwrap_or("legacy_unspecified"),
            assessment.proof_id(),
            assessment.reviewer()
        ))?;
    }

    Ok(())
}

#[instrument(level = "debug", skip(args))]
pub(super) fn report(args: AssessmentReportArgs) -> AmenableResult<()> {
    if let Some(proof) = &args.proof {
        catalog::ensure_registered(proof)?;
    }

    let assessments = record::load(&args.assessments)?;
    let mut by_proof: BTreeMap<String, Vec<ProofAssessment>> = BTreeMap::new();
    for assessment in assessments {
        if args
            .proof
            .as_ref()
            .is_none_or(|proof| assessment.proof_id() == *proof)
        {
            let proof_id = assessment.proof_id().to_owned();
            by_proof.entry(proof_id).or_default().push(assessment);
        }
    }

    if by_proof.is_empty() {
        crate::write_stdout_line("No proof assessments matched the selection.")?;
        return Ok(());
    }

    for (proof_id, entries) in by_proof {
        print_summary(&proof_id, &entries)?;
    }
    Ok(())
}

#[instrument(level = "debug", skip(args))]
pub(super) fn queue(args: AssessmentQueueArgs) -> AmenableResult<()> {
    let since_timestamp = args.since.map(start_of_utc_date_timestamp).transpose()?;
    let assessed = assessed_proof_ids(&args.assessments, since_timestamp)?;
    let unassessed: Vec<_> = catalog::registered_proofs()
        .into_iter()
        .filter(|proof| !assessed.contains(proof.id()))
        .collect();

    if args.json {
        let output = QueueOutput {
            since: args.since.map(|date| date.to_string()),
            count: unassessed.len(),
            proof_ids: unassessed
                .iter()
                .map(|proof| proof.id().to_owned())
                .collect(),
        };
        return print_json(&output);
    }

    if unassessed.is_empty() {
        crate::write_stdout_line("Every registered proof has at least one assessment.")?;
        return Ok(());
    }

    crate::write_stdout_line(format!("Unassessed proofs: {}", unassessed.len()))?;
    for proof in unassessed {
        crate::write_stdout_line(proof.id())?;
    }
    Ok(())
}

#[instrument(level = "debug", skip(assessments, recommendation, resolution_path))]
fn filtered_assessments(
    assessments: Vec<ProofAssessment>,
    proof: Option<&str>,
    recommendation: Option<Recommendation>,
    resolution_path: Option<ResolutionPath>,
    since_timestamp: Option<u64>,
) -> Vec<ProofAssessment> {
    assessments
        .into_iter()
        .filter(|assessment| proof.is_none_or(|proof_id| assessment.proof_id() == proof_id))
        .filter(|assessment| {
            recommendation.is_none_or(|wanted| assessment.recommendation() == wanted)
        })
        .filter(|assessment| {
            resolution_path.is_none_or(|wanted| assessment.resolution_path() == Some(wanted))
        })
        .filter(|assessment| {
            since_timestamp.is_none_or(|threshold| assessment.timestamp() >= threshold)
        })
        .collect()
}

#[instrument(level = "debug", skip(path))]
fn assessed_proof_ids(
    path: &Path,
    since_timestamp: Option<u64>,
) -> AmenableResult<BTreeSet<String>> {
    Ok(record::load(path)?
        .into_iter()
        .filter(|assessment| {
            since_timestamp.is_none_or(|threshold| assessment.timestamp() >= threshold)
        })
        .map(|assessment| assessment.proof_id().to_owned())
        .collect())
}

#[instrument(level = "debug", skip(status, filter))]
fn matches_failure_filter(status: ProofStatus, filter: Option<ProofStatus>) -> bool {
    match filter {
        Some(wanted) => status == wanted,
        None => status != ProofStatus::Passed,
    }
}

#[instrument(level = "debug", skip(entries))]
fn print_summary(proof_id: &str, entries: &[ProofAssessment]) -> AmenableResult<()> {
    crate::write_stdout_line(format!("{proof_id}: {} assessment(s)", entries.len()))?;

    for (index, (name, _)) in entries[0].rubric().values().into_iter().enumerate() {
        let scores: Vec<_> = entries
            .iter()
            .map(|entry| entry.rubric().values()[index].1)
            .collect();
        let mean = f64::from(scores.iter().map(|score| u32::from(*score)).sum::<u32>())
            / f64::from(
                u32::try_from(scores.len())
                    .map_err(|error| AmenableError::assessment_count(scores.len(), error))?,
            );
        let distribution = (0..=4)
            .map(|score| {
                let count = scores.iter().filter(|entry| **entry == score).count();
                format!("{score}:{count}")
            })
            .collect::<Vec<_>>()
            .join(" ");
        crate::write_stdout_line(format!("  {name}: mean {mean:.2}; {distribution}"))?;
    }

    let mut recommendations: BTreeMap<Recommendation, usize> = BTreeMap::new();
    for entry in entries {
        *recommendations.entry(entry.recommendation()).or_default() += 1;
    }
    let recommendations = recommendations
        .into_iter()
        .map(|(recommendation, count)| format!("{}:{count}", recommendation.as_str()))
        .collect::<Vec<_>>()
        .join(" ");
    crate::write_stdout_line(format!("  recommendations: {recommendations}"))?;

    let mut resolution_paths: BTreeMap<String, usize> = BTreeMap::new();
    for entry in entries {
        let key = entry
            .resolution_path()
            .map(ResolutionPath::as_str)
            .unwrap_or("legacy_unspecified")
            .to_owned();
        *resolution_paths.entry(key).or_default() += 1;
    }
    let resolution_paths = resolution_paths
        .into_iter()
        .map(|(resolution_path, count)| format!("{resolution_path}:{count}"))
        .collect::<Vec<_>>()
        .join(" ");
    crate::write_stdout_line(format!("  resolution paths: {resolution_paths}"))?;
    Ok(())
}

#[instrument(level = "debug")]
fn timestamp() -> AmenableResult<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

#[instrument(level = "debug")]
fn assessment_id() -> AmenableResult<String> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(format!("assessment-{}", duration.as_nanos()))
}

#[instrument(level = "debug", skip(date))]
fn start_of_utc_date_timestamp(date: Date) -> AmenableResult<u64> {
    let timestamp = date.midnight().assume_utc().unix_timestamp();
    u64::try_from(timestamp).map_err(|error| AmenableError::pre_epoch_date(date.to_string(), error))
}

#[instrument(level = "debug")]
fn format_timestamp(timestamp: u64) -> AmenableResult<String> {
    let seconds = i64::try_from(timestamp)
        .map_err(|error| AmenableError::timestamp_too_large(timestamp, error))?;
    let recorded_at = OffsetDateTime::from_unix_timestamp(seconds)?;
    Ok(recorded_at.format(&Rfc3339)?)
}

#[instrument(level = "debug", skip(value))]
fn print_json<T: Serialize>(value: &T) -> AmenableResult<()> {
    let json = serde_json::to_string_pretty(value)?;
    crate::write_stdout_line(json)
}
