//! Assessment recording/validation business-rule invariants --
//! `AssessmentErrorKind` nests the leaf sources split across
//! `assessment_sources_{scoring,comment,version,record_identity,
//! record_resolution}.rs` (one file per sub-concern, so that no single
//! sibling file holds too many types), `AssessmentSource` wraps that
//! `Kind` with its own `file`/`line`, and this file's own
//! `impl AmenableError` block builds each one, co-located with the type
//! it builds rather than centralized in `wrapper.rs`.

use tracing::instrument;

use crate::error::kind::AmenableErrorKind;
use crate::AmenableError;

use super::assessment_sources_comment::{
    CommentConflictSource, CommentMustNotBeEmptySource, CommentOrFileRequiredSource,
};
use super::assessment_sources_record_identity::{
    AssessmentIdMustNotBeEmptySource, AssessmentIdRequiredSource, ProofIdAndReviewerRequiredSource,
};
use super::assessment_sources_record_resolution::{
    DuplicateTimestampFieldsSource, MissingTimestampMetadataSource,
    ResolutionPathIncompatibleSource, ResolutionPathRequiredSource,
};
use super::assessment_sources_scoring::{
    InvalidNamedScoreSource, InvalidScoreRangeSource, UnknownProofIdSource,
};
use super::assessment_sources_version::{
    DuplicateVersionAndSchemaVersionSource, MissingVersionMetadataSource,
    UnsupportedAssessmentVersionSource, UnsupportedLegacySchemaVersionSource,
};

/// Umbrella kind for assessment-domain business-rule invariants, nested
/// one level under [`crate::error::kind::AmenableErrorKind`].
#[derive(Debug, derive_more::Display, derive_more::Error)]
enum AssessmentErrorKind {
    #[display("{_0}")]
    UnknownProofId(UnknownProofIdSource),
    #[display("{_0}")]
    InvalidScoreRange(InvalidScoreRangeSource),
    #[display("{_0}")]
    CommentConflict(CommentConflictSource),
    #[display("{_0}")]
    CommentOrFileRequired(CommentOrFileRequiredSource),
    #[display("{_0}")]
    UnsupportedAssessmentVersion(UnsupportedAssessmentVersionSource),
    #[display("{_0}")]
    ProofIdAndReviewerRequired(ProofIdAndReviewerRequiredSource),
    #[display("{_0}")]
    CommentMustNotBeEmpty(CommentMustNotBeEmptySource),
    #[display("{_0}")]
    AssessmentIdRequired(AssessmentIdRequiredSource),
    #[display("{_0}")]
    AssessmentIdMustNotBeEmpty(AssessmentIdMustNotBeEmptySource),
    #[display("{_0}")]
    ResolutionPathRequired(ResolutionPathRequiredSource),
    #[display("{_0}")]
    ResolutionPathIncompatible(ResolutionPathIncompatibleSource),
    #[display("{_0}")]
    UnsupportedLegacySchemaVersion(UnsupportedLegacySchemaVersionSource),
    #[display("{_0}")]
    DuplicateVersionAndSchemaVersion(DuplicateVersionAndSchemaVersionSource),
    #[display("{_0}")]
    MissingVersionMetadata(MissingVersionMetadataSource),
    #[display("{_0}")]
    DuplicateTimestampFields(DuplicateTimestampFieldsSource),
    #[display("{_0}")]
    MissingTimestampMetadata(MissingTimestampMetadataSource),
    #[display("{_0}")]
    InvalidNamedScore(InvalidNamedScoreSource),
}

/// Nests [`AssessmentErrorKind`] under a field named `kind`, not
/// `source` -- every nested-Kind layer in this domain already renders
/// its own full text (down to the real leaf's `file`/`line`) through
/// plain `Display` passthrough, so a field literally named `source`
/// would make `derive_more::Error` auto-chain through it too, printing
/// the identical message a second (and third, and fourth) time in any
/// `std::error::Error::source()`-walking report (e.g. `miette`'s
/// "caused by" chain). `kind` keeps `Display` forwarding intact while
/// `source()` terminates here, at the one real stop this wrapper is.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{kind}")]
pub struct AssessmentSource {
    #[error(ignore)]
    kind: Box<AssessmentErrorKind>,
    line: u32,
    file: String,
}

impl AssessmentSource {
    #[track_caller]
    #[instrument(level = "debug", skip(kind))]
    fn new(kind: AssessmentErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Convenience constructors for `AmenableErrorKind::Assessment`.
impl AmenableError {
    /// `unknown registered proof ID: {proof_id}`.
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn unknown_registered_proof_id(proof_id: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::UnknownProofId(UnknownProofIdSource::new(proof_id)),
        )))
    }

    /// `invalid score {score}; expected an integer from 0 to 4`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn invalid_score_range(score: u8) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::InvalidScoreRange(InvalidScoreRangeSource::new(score)),
        )))
    }

    /// `provide either --comment or --comment-file, not both`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn comment_conflict() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::CommentConflict(CommentConflictSource::new()),
        )))
    }

    /// `provide --comment or --comment-file`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn comment_or_file_required() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::CommentOrFileRequired(CommentOrFileRequiredSource::new()),
        )))
    }

    /// `unsupported assessment version {version}; expected one of legacy-1 or 0.1.0`.
    #[track_caller]
    #[instrument(level = "debug", skip(version))]
    pub fn unsupported_assessment_version(version: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::UnsupportedAssessmentVersion(
                UnsupportedAssessmentVersionSource::new(version),
            ),
        )))
    }

    /// `assessment proof ID and reviewer must not be empty`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn proof_id_and_reviewer_required() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::ProofIdAndReviewerRequired(ProofIdAndReviewerRequiredSource::new()),
        )))
    }

    /// `assessment comment must not be empty`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn comment_must_not_be_empty() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::CommentMustNotBeEmpty(CommentMustNotBeEmptySource::new()),
        )))
    }

    /// `assessment ID is required for version 0.1.0`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn assessment_id_required() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::AssessmentIdRequired(AssessmentIdRequiredSource::new()),
        )))
    }

    /// `assessment ID must not be empty`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn assessment_id_must_not_be_empty() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::AssessmentIdMustNotBeEmpty(AssessmentIdMustNotBeEmptySource::new()),
        )))
    }

    /// `resolution path is required for version 0.1.0`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn resolution_path_required() -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::ResolutionPathRequired(ResolutionPathRequiredSource::new()),
        )))
    }

    /// `resolution path {path} is incompatible with recommendation {recommendation}`.
    #[track_caller]
    #[instrument(level = "debug", skip(path, recommendation))]
    pub fn resolution_path_incompatible(
        path: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::ResolutionPathIncompatible(ResolutionPathIncompatibleSource::new(
                path,
                recommendation,
            )),
        )))
    }

    /// `unsupported legacy assessment schema version {schema_version}; expected 1`.
    #[track_caller]
    #[instrument(level = "debug")]
    pub fn unsupported_legacy_schema_version(schema_version: u8) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::UnsupportedLegacySchemaVersion(
                UnsupportedLegacySchemaVersionSource::new(schema_version),
            ),
        )))
    }

    /// `assessment record for {proof_id} must not contain both version and schema_version`.
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn duplicate_version_and_schema_version(proof_id: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::DuplicateVersionAndSchemaVersion(
                DuplicateVersionAndSchemaVersionSource::new(proof_id),
            ),
        )))
    }

    /// `assessment record for {proof_id} is missing version metadata`.
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn missing_version_metadata(proof_id: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::MissingVersionMetadata(MissingVersionMetadataSource::new(
                proof_id,
            )),
        )))
    }

    /// `assessment record for {proof_id} must not contain both timestamp and timestamp_unix_seconds`.
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn duplicate_timestamp_fields(proof_id: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::DuplicateTimestampFields(DuplicateTimestampFieldsSource::new(
                proof_id,
            )),
        )))
    }

    /// `assessment record for {proof_id} is missing timestamp metadata`.
    #[track_caller]
    #[instrument(level = "debug", skip(proof_id))]
    pub fn missing_timestamp_metadata(proof_id: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::MissingTimestampMetadata(MissingTimestampMetadataSource::new(
                proof_id,
            )),
        )))
    }

    /// `invalid {name} score {score}; expected an integer from 0 to 4`.
    #[track_caller]
    #[instrument(level = "debug", skip(name))]
    pub fn invalid_named_score(name: impl Into<String>, score: u8) -> Self {
        Self::new(AmenableErrorKind::Assessment(AssessmentSource::new(
            AssessmentErrorKind::InvalidNamedScore(InvalidNamedScoreSource::new(name, score)),
        )))
    }
}
