//! This workspace's own cross-crate errors.

use tracing::instrument;

/// Preserved `amenable_core::ChainError` source -- a proof-chain lookup
/// failure (`ChainError::NotFound`/`Incomplete`), kept as the real typed
/// value rather than stringified. `main::run_audit` used to discard this
/// via `AmenableError::invariant(error.to_string())`, losing the ability
/// to downcast back to the real `ChainError` the way every other real
/// error in this crate can be.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct ChainSource {
    /// The preserved `amenable_core::ChainError`.
    source: crate::ChainError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ChainSource {
    /// Preserve a proof-chain lookup failure alongside the caller's
    /// location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: crate::ChainError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Preserved `amenable_std::AmenableStdError` source -- `amenable_std`
/// has its own umbrella error, the same as every other crate ("each
/// crate gets its own umbrella error, that's how it works"); `amenable`
/// wraps it here rather than re-deriving a fresh error from its own
/// `std::io::Error` chain.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{source}")]
pub struct StdSource {
    /// The preserved `amenable_std::AmenableStdError`.
    source: amenable_std::AmenableStdError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl StdSource {
    /// Preserve an `amenable_std` error alongside the caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: amenable_std::AmenableStdError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// A `RubricBuilder::build` validation failure, preserving the real
/// `derive_builder`-generated error as `source` rather than stringifying
/// it -- the builder's own error *is* the genuine cause, so wrapping it
/// natively is normalizing, not the stringifying antipattern
/// `AmenableError::invariant(error.to_string())` used to commit.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("could not build a rubric: {source}")]
pub struct RubricBuilderFailedSource {
    /// The preserved `RubricBuilderError`.
    source: crate::assessment::RubricBuilderError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl RubricBuilderFailedSource {
    /// Preserve a `RubricBuilder` failure alongside the caller's
    /// location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: crate::assessment::RubricBuilderError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// A `ProofAssessmentBuilder::build` validation failure, preserving the
/// real builder error as `source`.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("could not build a proof assessment: {source}")]
pub struct ProofAssessmentBuilderFailedSource {
    /// The preserved `ProofAssessmentBuilderError`.
    source: crate::assessment::ProofAssessmentBuilderError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl ProofAssessmentBuilderFailedSource {
    /// Preserve a `ProofAssessmentBuilder` failure alongside the
    /// caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: crate::assessment::ProofAssessmentBuilderError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// An assessment record on one line of a JSON Lines artifact failed
/// `StoredProofAssessment::into_assessment` or `ProofAssessment::validate`,
/// preserving the real nested [`crate::AmenableError`] as `source` --
/// both call sites used to collapse it into a brand-new
/// `AmenableError::invariant(format!("...{error}"))`, restringifying an
/// already-real typed error a second time and losing the ability to
/// downcast back to it.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid assessment on line {artifact_line} in {}: {source}", path.display())]
pub struct InvalidAssessmentLineSource {
    /// The preserved, already-real `AmenableError` from the failed
    /// conversion or validation step.
    source: Box<crate::AmenableError>,
    /// The artifact's own 1-indexed line number, not the Rust call-site
    /// `line` below.
    #[error(ignore)]
    artifact_line: usize,
    /// The JSON Lines artifact path.
    #[error(ignore)]
    path: std::path::PathBuf,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

/// A `WitnessExportRecordDumpBuilder::build` validation failure,
/// preserving the real builder error as `source`.
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("could not build a witness export record dump: {source}")]
pub struct WitnessExportRecordDumpBuilderFailedSource {
    /// The preserved `WitnessExportRecordDumpBuilderError`.
    source: crate::registry_dump::WitnessExportRecordDumpBuilderError,
    /// Source line of the call site that produced this error.
    line: u32,
    /// Source file of the call site that produced this error.
    file: String,
}

impl WitnessExportRecordDumpBuilderFailedSource {
    /// Preserve a `WitnessExportRecordDumpBuilder` failure alongside the
    /// caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source))]
    pub fn new(source: crate::registry_dump::WitnessExportRecordDumpBuilderError) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source,
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

impl InvalidAssessmentLineSource {
    /// Preserve an invalid assessment-record line, capturing the
    /// caller's location.
    #[track_caller]
    #[instrument(level = "debug", skip(source, path))]
    pub fn new(
        source: crate::AmenableError,
        path: impl Into<std::path::PathBuf>,
        artifact_line: usize,
    ) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            source: Box::new(source),
            artifact_line,
            path: path.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
