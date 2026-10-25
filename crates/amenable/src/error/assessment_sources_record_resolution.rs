//! Native leaf sources for assessment resolution-path/timestamp
//! validation -- split out of `assessment_sources.rs`, itself split out
//! of `assessment.rs`, once the combined leaf-source file grew past the
//! point cordial's own too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("resolution path is required for version 0.1.0 ({file}:{line})")]
pub(super) struct ResolutionPathRequiredSource {
    line: u32,
    file: String,
}

impl ResolutionPathRequiredSource {
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn new() -> Self {
        let loc = std::panic::Location::caller();
        Self {
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "resolution path {path} is incompatible with recommendation {recommendation} ({file}:{line})"
)]
pub(super) struct ResolutionPathIncompatibleSource {
    #[error(ignore)]
    path: String,
    #[error(ignore)]
    recommendation: String,
    line: u32,
    file: String,
}

impl ResolutionPathIncompatibleSource {
    #[track_caller]
    #[instrument(level = "debug", skip(path, recommendation))]
    pub fn new(path: impl Into<String>, recommendation: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            path: path.into(),
            recommendation: recommendation.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(

    "assessment record for {proof_id} must not contain both timestamp and timestamp_unix_seconds ({file}:{line})"
)]
pub(super) struct DuplicateTimestampFieldsSource {
    #[error(ignore)]
    proof_id: String,
    line: u32,
    file: String,
}

impl DuplicateTimestampFieldsSource {
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn new(proof_id: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            proof_id: proof_id.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("assessment record for {proof_id} is missing timestamp metadata ({file}:{line})")]
pub(super) struct MissingTimestampMetadataSource {
    #[error(ignore)]
    proof_id: String,
    line: u32,
    file: String,
}

impl MissingTimestampMetadataSource {
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn new(proof_id: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            proof_id: proof_id.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
