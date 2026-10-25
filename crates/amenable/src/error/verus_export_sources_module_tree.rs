//! Native leaf sources for Verus destination-module path validation --
//! split out of `verus_export_sources.rs`, itself split out of
//! `verus_export.rs`, once the combined leaf-source file grew past the
//! point cordial's own too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "invalid Verus destination module {module:?}: expected crate-relative path ({file}:{line})"
)]
pub(super) struct DestinationModuleNotCrateRelativeSource {
    #[error(ignore)]
    module: String,
    line: u32,
    file: String,
}

impl DestinationModuleNotCrateRelativeSource {
    #[track_caller]
    #[instrument(level = "debug", skip(module))]
    pub fn new(module: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            module: module.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "invalid Verus destination module {module:?}: expected path to start with \"crate::\" ({file}:{line})"
)]
pub(super) struct DestinationModuleMissingCratePrefixSource {
    #[error(ignore)]
    module: String,
    line: u32,
    file: String,
}

impl DestinationModuleMissingCratePrefixSource {
    #[track_caller]
    #[instrument(level = "debug", skip(module))]
    pub fn new(module: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            module: module.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "invalid Verus destination module {module:?}: bad Rust module segment {segment:?} ({file}:{line})"
)]
pub(super) struct DestinationModuleBadSegmentSource {
    #[error(ignore)]
    module: String,
    #[error(ignore)]
    segment: String,
    line: u32,
    file: String,
}

impl DestinationModuleBadSegmentSource {
    #[track_caller]
    #[instrument(level = "debug", skip(module, segment))]
    pub fn new(module: impl Into<String>, segment: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            module: module.into(),
            segment: segment.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("invalid Verus destination module {module:?}: missing module segments ({file}:{line})")]
pub(super) struct DestinationModuleMissingSegmentsSource {
    #[error(ignore)]
    module: String,
    line: u32,
    file: String,
}

impl DestinationModuleMissingSegmentsSource {
    #[track_caller]
    #[instrument(level = "debug", skip(module))]
    pub fn new(module: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            module: module.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
