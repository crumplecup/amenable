//! Write one assessment record to the assessment artifact.

use super::super::catalog;
use super::super::cli::RecordAssessmentArgs;
use super::super::record::{self, CURRENT_ASSESSMENT_VERSION, ProofAssessmentBuilder};
use super::super::vocabulary::RubricBuilder;
use crate::{AmenableError, AmenableResult};
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::instrument;

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

#[instrument(level = "debug")]
fn timestamp() -> AmenableResult<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

#[instrument(level = "debug")]
fn assessment_id() -> AmenableResult<String> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(format!("assessment-{}", duration.as_nanos()))
}
