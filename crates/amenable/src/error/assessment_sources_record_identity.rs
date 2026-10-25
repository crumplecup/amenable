//! Native leaf sources for assessment record identity validation
//! (proof ID/reviewer/assessment ID) -- split out of
//! `assessment_sources.rs`, itself split out of `assessment.rs`, once
//! the combined leaf-source file grew past the point cordial's own
//! too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("assessment proof ID and reviewer must not be empty ({file}:{line})")]
pub(super) struct ProofIdAndReviewerRequiredSource {
    line: u32,
    file: String,
}

impl ProofIdAndReviewerRequiredSource {
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
#[display("assessment ID is required for version 0.1.0 ({file}:{line})")]
pub(super) struct AssessmentIdRequiredSource {
    line: u32,
    file: String,
}

impl AssessmentIdRequiredSource {
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
#[display("assessment ID must not be empty ({file}:{line})")]
pub(super) struct AssessmentIdMustNotBeEmptySource {
    line: u32,
    file: String,
}

impl AssessmentIdMustNotBeEmptySource {
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
