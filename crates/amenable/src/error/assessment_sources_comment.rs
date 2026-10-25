//! Native leaf sources for `--comment`/`--comment-file` validation --
//! split out of `assessment_sources.rs`, itself split out of
//! `assessment.rs`, once the combined leaf-source file grew past the
//! point cordial's own too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("provide either --comment or --comment-file, not both ({file}:{line})")]
pub(super) struct CommentConflictSource {
    line: u32,
    file: String,
}

impl CommentConflictSource {
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
#[display("provide --comment or --comment-file ({file}:{line})")]
pub(super) struct CommentOrFileRequiredSource {
    line: u32,
    file: String,
}

impl CommentOrFileRequiredSource {
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
#[display("assessment comment must not be empty ({file}:{line})")]
pub(super) struct CommentMustNotBeEmptySource {
    line: u32,
    file: String,
}

impl CommentMustNotBeEmptySource {
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
