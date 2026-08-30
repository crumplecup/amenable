//! The persisted assessment record and its JSON Lines I/O.

use crate::assessment::vocabulary::{Recommendation, ResolutionPath, Rubric};
use crate::{AmenableError, AmenableResult};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    path::Path,
};
use tracing::instrument;

pub(super) const CURRENT_ASSESSMENT_VERSION: &str = "0.1.0";
const LEGACY_ASSESSMENT_VERSION: &str = "legacy-1";
const LEGACY_SCHEMA_VERSION: u8 = 1;

/// One immutable assessment event stored in the JSON Lines artifact. Built
/// via [`ProofAssessmentBuilder`] rather than a many-argument constructor
/// -- nine fields is well past a plain `new` clippy would accept.
#[derive(Debug, derive_builder::Builder)]
pub struct ProofAssessment {
    version: String,
    assessment_id: Option<String>,
    proof_id: String,
    reviewer: String,
    timestamp: u64,
    rubric: Rubric,
    recommendation: Recommendation,
    resolution_path: Option<ResolutionPath>,
    comment: String,
}

impl ProofAssessment {
    #[instrument(level = "debug", skip(self))]
    pub(super) fn assessment_id(&self) -> Option<&str> {
        self.assessment_id.as_deref()
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn version(&self) -> &str {
        &self.version
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn proof_id(&self) -> &str {
        &self.proof_id
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn reviewer(&self) -> &str {
        &self.reviewer
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn timestamp(&self) -> u64 {
        self.timestamp
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn rubric(&self) -> Rubric {
        self.rubric
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn recommendation(&self) -> Recommendation {
        self.recommendation
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn resolution_path(&self) -> Option<ResolutionPath> {
        self.resolution_path
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn comment(&self) -> &str {
        &self.comment
    }

    #[instrument(level = "debug", skip(self))]
    pub(super) fn validate(&self) -> AmenableResult<()> {
        if self.version != CURRENT_ASSESSMENT_VERSION && self.version != LEGACY_ASSESSMENT_VERSION {
            return Err(AmenableError::invariant(format!(
                "unsupported assessment version {}; expected one of {LEGACY_ASSESSMENT_VERSION} or {CURRENT_ASSESSMENT_VERSION}",
                self.version
            )));
        }
        if self.proof_id.trim().is_empty() || self.reviewer.trim().is_empty() {
            return Err(AmenableError::invariant(
                "assessment proof ID and reviewer must not be empty",
            ));
        }
        if self.comment.trim().is_empty() {
            return Err(AmenableError::invariant(
                "assessment comment must not be empty",
            ));
        }
        if self.version == CURRENT_ASSESSMENT_VERSION {
            let assessment_id = self.assessment_id.as_ref().ok_or_else(|| {
                AmenableError::invariant("assessment ID is required for version 0.1.0")
            })?;
            if assessment_id.trim().is_empty() {
                return Err(AmenableError::invariant("assessment ID must not be empty"));
            }

            let resolution_path = self.resolution_path.ok_or_else(|| {
                AmenableError::invariant("resolution path is required for version 0.1.0")
            })?;
            validate_resolution_path(self.recommendation, resolution_path)?;
        }

        self.rubric.validate()
    }
}

/// Confirm a recommendation and its resolution path name the same
/// action -- e.g. `Accept` must pair with `KeepCurrentProof`, not with a
/// path that implies the proof needs work.
#[instrument(level = "debug", skip(recommendation, resolution_path))]
fn validate_resolution_path(
    recommendation: Recommendation,
    resolution_path: ResolutionPath,
) -> AmenableResult<()> {
    let valid = match recommendation {
        Recommendation::Accept => resolution_path == ResolutionPath::KeepCurrentProof,
        Recommendation::Strengthen => resolution_path == ResolutionPath::StrengthenCurrentProof,
        Recommendation::Replace => matches!(
            resolution_path,
            ResolutionPath::ReplaceWithProofSpecificModel
                | ResolutionPath::ReplaceWithAccommodationModel
        ),
        Recommendation::Retire => resolution_path == ResolutionPath::RetireClaim,
    };

    valid.then_some(()).ok_or_else(|| {
        AmenableError::invariant(format!(
            "resolution path {} is incompatible with recommendation {}",
            resolution_path.as_str(),
            recommendation.as_str()
        ))
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredProofAssessment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    schema_version: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    assessment_id: Option<String>,
    proof_id: String,
    reviewer: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timestamp: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timestamp_unix_seconds: Option<u64>,
    rubric: Rubric,
    recommendation: Recommendation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    resolution_path: Option<ResolutionPath>,
    comment: String,
}

impl StoredProofAssessment {
    #[instrument(level = "debug", skip(self))]
    fn into_assessment(self) -> AmenableResult<ProofAssessment> {
        let proof_id = self.proof_id;
        let version = match (self.version, self.schema_version) {
            (Some(version), None) => version,
            (None, Some(LEGACY_SCHEMA_VERSION)) => LEGACY_ASSESSMENT_VERSION.to_owned(),
            (None, Some(schema_version)) => {
                return Err(AmenableError::invariant(format!(
                    "unsupported legacy assessment schema version {schema_version}; expected {LEGACY_SCHEMA_VERSION}"
                )));
            }
            (Some(_), Some(_)) => {
                return Err(AmenableError::invariant(format!(
                    "assessment record for {proof_id} must not contain both version and schema_version"
                )));
            }
            (None, None) => {
                return Err(AmenableError::invariant(format!(
                    "assessment record for {proof_id} is missing version metadata"
                )));
            }
        };

        let timestamp = match (self.timestamp, self.timestamp_unix_seconds) {
            (Some(timestamp), None) => timestamp,
            (None, Some(timestamp)) => timestamp,
            (Some(_), Some(_)) => {
                return Err(AmenableError::invariant(format!(
                    "assessment record for {proof_id} must not contain both timestamp and timestamp_unix_seconds"
                )));
            }
            (None, None) => {
                return Err(AmenableError::invariant(format!(
                    "assessment record for {proof_id} is missing timestamp metadata"
                )));
            }
        };

        ProofAssessmentBuilder::default()
            .version(version)
            .assessment_id(self.assessment_id)
            .proof_id(proof_id)
            .reviewer(self.reviewer)
            .timestamp(timestamp)
            .rubric(self.rubric)
            .recommendation(self.recommendation)
            .resolution_path(self.resolution_path)
            .comment(self.comment)
            .build()
            .map_err(|error| AmenableError::invariant(error.to_string()))
    }
}

impl From<&ProofAssessment> for StoredProofAssessment {
    #[instrument(level = "debug", skip(assessment))]
    fn from(assessment: &ProofAssessment) -> Self {
        Self {
            version: Some(assessment.version.clone()),
            schema_version: None,
            assessment_id: assessment.assessment_id.clone(),
            proof_id: assessment.proof_id.clone(),
            reviewer: assessment.reviewer.clone(),
            timestamp: Some(assessment.timestamp),
            timestamp_unix_seconds: None,
            rubric: assessment.rubric,
            recommendation: assessment.recommendation,
            resolution_path: assessment.resolution_path,
            comment: assessment.comment.clone(),
        }
    }
}

/// Load every recorded assessment from a JSON Lines artifact, or an empty
/// list if it doesn't exist yet.
///
/// # Errors
///
/// Returns an [`amenable::AmenableError`] if the artifact can't be read,
/// or if any line is invalid JSON or fails assessment validation.
#[instrument(level = "debug", skip(path))]
pub fn load(path: &Path) -> AmenableResult<Vec<ProofAssessment>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path).map_err(|error| AmenableError::io(path, error))?;
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            let assessment: StoredProofAssessment = serde_json::from_str(line)
                .map_err(|error| AmenableError::json_line(path, index + 1, error))?;
            let assessment = assessment.into_assessment().map_err(|error| {
                AmenableError::invariant(format!(
                    "invalid assessment on line {} in {}: {error}",
                    index + 1,
                    path.display()
                ))
            })?;
            assessment.validate().map_err(|error| {
                AmenableError::invariant(format!(
                    "invalid assessment on line {} in {}: {error}",
                    index + 1,
                    path.display()
                ))
            })?;
            Ok(assessment)
        })
        .collect()
}

#[instrument(level = "debug", skip(path, assessment))]
pub(super) fn append(path: &Path, assessment: &ProofAssessment) -> AmenableResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| AmenableError::io(parent, error))?;
    }

    let record = serde_json::to_string(&StoredProofAssessment::from(assessment))?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| AmenableError::io(path, error))?;
    file.write_all(record.as_bytes())
        .and_then(|()| file.write_all(b"\n"))
        .map_err(|error| AmenableError::io(path, error))
}
