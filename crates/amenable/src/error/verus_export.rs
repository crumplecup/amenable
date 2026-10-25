//! Generated-Verus-module export pipeline business-rule invariants --
//! `VerusExportErrorKind` nests the leaf sources split across
//! `verus_export_sources_{module_tree,composition,call_shape}.rs` (one
//! file per sub-concern, so that no single sibling file holds too many
//! types), `VerusExportSource` wraps that `Kind` with its own
//! `file`/`line`, and this file's own `impl AmenableError` block builds
//! each one, co-located with the type it builds rather than
//! centralized in `wrapper.rs`.

use tracing::instrument;

use crate::error::kind::AmenableErrorKind;
use crate::AmenableError;

use super::verus_export_sources_call_shape::{
    HarnessRegisteredAsBarePredicateSource, LeafMissingHarnessMetadataSource,
    NoCallShapeForHarnessSource, NoProofTokenMintRecordSource,
};
use super::verus_export_sources_composition::{
    EnumExportHasNoVariantsSource, ImpossibleLeafSupportKindSource,
    NestedEnumWitnessUnsupportedSource, RenderFailuresSummarySource,
};
use super::verus_export_sources_module_tree::{
    DestinationModuleBadSegmentSource, DestinationModuleMissingCratePrefixSource,
    DestinationModuleMissingSegmentsSource, DestinationModuleNotCrateRelativeSource,
};

/// Umbrella kind for the generated-Verus-module export pipeline's own
/// invariants, nested one level under
/// [`crate::error::kind::AmenableErrorKind`].
#[derive(Debug, derive_more::Display, derive_more::Error)]
enum VerusExportErrorKind {
    #[display("{_0}")]
    EnumExportHasNoVariants(EnumExportHasNoVariantsSource),
    #[display("{_0}")]
    RenderFailuresSummary(RenderFailuresSummarySource),
    #[display("{_0}")]
    DestinationModuleNotCrateRelative(DestinationModuleNotCrateRelativeSource),
    #[display("{_0}")]
    DestinationModuleMissingCratePrefix(DestinationModuleMissingCratePrefixSource),
    #[display("{_0}")]
    DestinationModuleBadSegment(DestinationModuleBadSegmentSource),
    #[display("{_0}")]
    DestinationModuleMissingSegments(DestinationModuleMissingSegmentsSource),
    #[display("{_0}")]
    NestedEnumWitnessUnsupported(NestedEnumWitnessUnsupportedSource),
    #[display("{_0}")]
    ImpossibleLeafSupportKind(ImpossibleLeafSupportKindSource),
    #[display("{_0}")]
    LeafMissingHarnessMetadata(LeafMissingHarnessMetadataSource),
    #[display("{_0}")]
    NoCallShapeForHarness(NoCallShapeForHarnessSource),
    #[display("{_0}")]
    HarnessRegisteredAsBarePredicate(HarnessRegisteredAsBarePredicateSource),
    #[display("{_0}")]
    NoProofTokenMintRecord(NoProofTokenMintRecordSource),
}

/// Nests [`VerusExportErrorKind`] under a field named `kind`, not
/// `source` -- see [`super::assessment::AssessmentSource`]'s doc
/// comment for why (identical reasoning, same pattern).
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("{kind}")]
pub struct VerusExportSource {
    #[error(ignore)]
    kind: Box<VerusExportErrorKind>,
    line: u32,
    file: String,
}

impl VerusExportSource {
    #[track_caller]
    #[instrument(level = "debug", skip(kind))]
    fn new(kind: VerusExportErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind: Box::new(kind),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

/// Convenience constructors for `AmenableErrorKind::VerusExport`.
impl AmenableError {
    /// `enum-shaped export {export} has no variants -- nothing to compose`.
    #[track_caller]
    #[instrument(level = "debug", skip(export))]
    pub fn enum_export_has_no_variants(export: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::EnumExportHasNoVariants(EnumExportHasNoVariantsSource::new(
                export,
            )),
        )))
    }

    /// `failed to render {failed} of {total} Verus witness export(s):\n{detail}`.
    #[track_caller]
    #[instrument(level = "debug", skip(detail))]
    pub fn verus_render_failures_summary(
        failed: usize,
        total: usize,
        detail: impl Into<String>,
    ) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::RenderFailuresSummary(RenderFailuresSummarySource::new(
                failed, total, detail,
            )),
        )))
    }

    /// `invalid Verus destination module {module:?}: expected crate-relative path`.
    #[track_caller]
    #[instrument(level = "debug", skip(module))]
    pub fn verus_destination_module_not_crate_relative(module: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::DestinationModuleNotCrateRelative(
                DestinationModuleNotCrateRelativeSource::new(module),
            ),
        )))
    }

    /// `invalid Verus destination module {module:?}: expected path to start with "crate::"`.
    #[track_caller]
    #[instrument(level = "debug", skip(module))]
    pub fn verus_destination_module_missing_crate_prefix(module: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::DestinationModuleMissingCratePrefix(
                DestinationModuleMissingCratePrefixSource::new(module),
            ),
        )))
    }

    /// `invalid Verus destination module {module:?}: bad Rust module segment {segment:?}`.
    #[track_caller]
    #[instrument(level = "debug", skip(module, segment))]
    pub fn verus_destination_module_bad_segment(
        module: impl Into<String>,
        segment: impl Into<String>,
    ) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::DestinationModuleBadSegment(
                DestinationModuleBadSegmentSource::new(module, segment),
            ),
        )))
    }

    /// `invalid Verus destination module {module:?}: missing module segments`.
    #[track_caller]
    #[instrument(level = "debug", skip(module))]
    pub fn verus_destination_module_missing_segments(module: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::DestinationModuleMissingSegments(
                DestinationModuleMissingSegmentsSource::new(module),
            ),
        )))
    }

    /// `Verus composition for a nested enum-shaped witness is not supported yet (route: {route})`.
    #[track_caller]
    #[instrument(level = "debug", skip(route))]
    pub fn verus_nested_enum_witness_unsupported(route: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::NestedEnumWitnessUnsupported(
                NestedEnumWitnessUnsupportedSource::new(route),
            ),
        )))
    }

    /// `leaf at {route} reports an impossible per-leaf support kind {kind}`.
    #[track_caller]
    #[instrument(level = "debug", skip(route, kind))]
    pub fn verus_impossible_leaf_support_kind(
        route: impl Into<String>,
        kind: impl Into<String>,
    ) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::ImpossibleLeafSupportKind(ImpossibleLeafSupportKindSource::new(
                route, kind,
            )),
        )))
    }

    /// `checked leaf at {route} has no "harness" metadata entry`.
    #[track_caller]
    #[instrument(level = "debug", skip(route))]
    pub fn verus_leaf_missing_harness_metadata(route: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::LeafMissingHarnessMetadata(
                LeafMissingHarnessMetadataSource::new(route),
            ),
        )))
    }

    /// `no Verus call shape found for harness`{harness}`(route: {route})`.
    #[track_caller]
    #[instrument(level = "debug", skip(harness, route))]
    pub fn verus_no_call_shape_for_harness(
        harness: impl Into<String>,
        route: impl Into<String>,
    ) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::NoCallShapeForHarness(NoCallShapeForHarnessSource::new(
                harness, route,
            )),
        )))
    }

    /// ``harness `{harness}` (route: {route}) is registered as a bare predicate``.
    #[track_caller]
    #[instrument(level = "debug", skip(harness, route))]
    pub fn verus_harness_registered_as_bare_predicate(
        harness: impl Into<String>,
        route: impl Into<String>,
    ) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::HarnessRegisteredAsBarePredicate(
                HarnessRegisteredAsBarePredicateSource::new(harness, route),
            ),
        )))
    }

    /// `no ProofTokenMintRecord registered for {token}`.
    #[track_caller]
    #[instrument(level = "debug", skip(token))]
    pub fn no_proof_token_mint_record(token: impl Into<String>) -> Self {
        Self::new(AmenableErrorKind::VerusExport(VerusExportSource::new(
            VerusExportErrorKind::NoProofTokenMintRecord(NoProofTokenMintRecordSource::new(token)),
        )))
    }
}
