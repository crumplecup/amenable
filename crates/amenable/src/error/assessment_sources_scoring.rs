//! Native leaf sources for assessment score/proof-ID validation --
//! split out of `assessment_sources.rs`, itself split out of
//! `assessment.rs`, once the combined leaf-source file grew past the
//! point cordial's own too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("unknown registered proof ID: {proof_id} ({file}:{line})")]
pub(super) struct UnknownProofIdSource {
    #[error(ignore)]
    proof_id: String,
    line: u32,
    file: String,
}

impl UnknownProofIdSource {
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
#[display("invalid score {score}; expected an integer from 0 to 4 ({file}:{line})")]
pub(super) struct InvalidScoreRangeSource {
    #[error(ignore)]
    score: u8,
    line: u32,
    file: String,
}

impl InvalidScoreRangeSource {
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn new(score: u8) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            score,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid {name} score {score}; expected an integer from 0 to 4 ({file}:{line})")]
pub(super) struct InvalidNamedScoreSource {
    #[error(ignore)]
    name: String,
    #[error(ignore)]
    score: u8,
    line: u32,
    file: String,
}

impl InvalidNamedScoreSource {
    #[track_caller]
    #[instrument(level = "debug", skip(name))]
    pub fn new(name: impl Into<String>, score: u8) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            name: name.into(),
            score,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
