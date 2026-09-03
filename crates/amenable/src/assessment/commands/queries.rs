//! Query-style assessment commands: listings, queues, and failure views.

use super::super::catalog;
use super::super::cli::{AssessmentListArgs, AssessmentQueueArgs, VerificationFailuresArgs};
use super::super::record::{self, ProofAssessment};
use super::super::vocabulary::{Recommendation, ResolutionPath, Rubric};
use super::{format_timestamp, print_json, start_of_utc_date_timestamp};
use crate::kani::{self, ProofStatus};
use crate::{AmenableResult, write_stderr_line, write_stdout_line};
use serde::Serialize;
use std::{collections::BTreeSet, path::Path};
use tracing::instrument;

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
            write_stderr_line(format!(
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
        write_stdout_line("No Kani verification results matched the selection.")?;
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
        write_stdout_line(format!(
            "{}\t{recorded_at}\t{}",
            result.status().as_str(),
            result.proof_id()
        ))?;
    }

    Ok(())
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
        write_stdout_line("No proof assessments matched the selection.")?;
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
        write_stdout_line(format!(
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
        write_stdout_line("Every registered proof has at least one assessment.")?;
        return Ok(());
    }

    write_stdout_line(format!("Unassessed proofs: {}", unassessed.len()))?;
    for proof in unassessed {
        write_stdout_line(proof.id())?;
    }
    Ok(())
}

#[instrument(level = "debug", skip(assessments, recommendation, resolution_path))]
pub(super) fn filtered_assessments(
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
