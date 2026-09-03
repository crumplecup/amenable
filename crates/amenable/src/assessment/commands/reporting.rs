//! Rollup-style assessment commands: summaries and per-proof reports.

use super::super::catalog;
use super::super::cli::{AssessmentReportArgs, AssessmentSummaryArgs};
use super::super::record::{self, ProofAssessment};
use super::super::vocabulary::{Recommendation, ResolutionPath, SummaryDimension};
use super::queries::filtered_assessments;
use super::{print_json, start_of_utc_date_timestamp};
use crate::{AmenableError, AmenableResult, write_stdout_line};
use clap::ValueEnum;
use serde::Serialize;
use std::collections::BTreeMap;
use tracing::instrument;

#[derive(Debug, Serialize)]
struct SummaryOutput {
    by: String,
    proof: Option<String>,
    since: Option<String>,
    counts: BTreeMap<String, usize>,
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

    write_stdout_line(format!(
        "{:<label_width$} {:>count_width$}",
        args.by.as_str(),
        "count"
    ))?;
    write_stdout_line(format!(
        "{} {}",
        "-".repeat(label_width),
        "-".repeat(count_width)
    ))?;
    for label in counts.keys() {
        write_stdout_line(format!(
            "{:<label_width$} {:>count_width$}",
            label, counts[label]
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
        write_stdout_line("No proof assessments matched the selection.")?;
        return Ok(());
    }

    for (proof_id, entries) in by_proof {
        print_summary(&proof_id, &entries)?;
    }
    Ok(())
}

#[instrument(level = "debug", skip(entries))]
fn print_summary(proof_id: &str, entries: &[ProofAssessment]) -> AmenableResult<()> {
    write_stdout_line(format!("{proof_id}: {} assessment(s)", entries.len()))?;

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
        write_stdout_line(format!("  {name}: mean {mean:.2}; {distribution}"))?;
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
    write_stdout_line(format!("  recommendations: {recommendations}"))?;

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
    write_stdout_line(format!("  resolution paths: {resolution_paths}"))?;
    Ok(())
}
