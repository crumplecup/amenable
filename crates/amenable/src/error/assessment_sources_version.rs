//! Native leaf sources for assessment version/schema_version validation
//! -- split out of `assessment_sources.rs`, itself split out of
//! `assessment.rs`, once the combined leaf-source file grew past the
//! point cordial's own too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "unsupported assessment version {version}; expected one of legacy-1 or 0.1.0 ({file}:{line})"
)]
pub(super) struct UnsupportedAssessmentVersionSource {
    #[error(ignore)]
    version: String,
    line: u32,
    file: String,
}

impl UnsupportedAssessmentVersionSource {
    #[track_caller]
    #[instrument(level = "debug", skip(version))]
    pub fn new(version: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            version: version.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "unsupported legacy assessment schema version {schema_version}; expected 1 ({file}:{line})"
)]
pub(super) struct UnsupportedLegacySchemaVersionSource {
    #[error(ignore)]
    schema_version: u8,
    line: u32,
    file: String,
}

impl UnsupportedLegacySchemaVersionSource {
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn new(schema_version: u8) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            schema_version,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(

    "assessment record for {proof_id} must not contain both version and schema_version ({file}:{line})"
)]
pub(super) struct DuplicateVersionAndSchemaVersionSource {
    #[error(ignore)]
    proof_id: String,
    line: u32,
    file: String,
}

impl DuplicateVersionAndSchemaVersionSource {
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
#[display("assessment record for {proof_id} is missing version metadata ({file}:{line})")]
pub(super) struct MissingVersionMetadataSource {
    #[error(ignore)]
    proof_id: String,
    line: u32,
    file: String,
}

impl MissingVersionMetadataSource {
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
