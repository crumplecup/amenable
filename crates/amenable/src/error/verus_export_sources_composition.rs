//! Native leaf sources for Verus witness-composition shape validation --
//! split out of `verus_export_sources.rs`, itself split out of
//! `verus_export.rs`, once the combined leaf-source file grew past the
//! point cordial's own too-many-types-per-file modularity check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("enum-shaped export {export} has no variants -- nothing to compose ({file}:{line})")]
pub(super) struct EnumExportHasNoVariantsSource {
    #[error(ignore)]
    export: String,
    line: u32,
    file: String,
}

impl EnumExportHasNoVariantsSource {
    #[track_caller]
    #[instrument(level = "debug", skip(export))]
    pub fn new(export: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            export: export.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "failed to render {failed} of {total} Verus witness export(s):\n{detail} ({file}:{line})"
)]
pub(super) struct RenderFailuresSummarySource {
    #[error(ignore)]
    failed: usize,
    #[error(ignore)]
    total: usize,
    #[error(ignore)]
    detail: String,
    line: u32,
    file: String,
}

impl RenderFailuresSummarySource {
    #[track_caller]
    #[instrument(level = "debug", skip(detail))]
    pub fn new(failed: usize, total: usize, detail: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            failed,
            total,
            detail: detail.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "Verus composition for a nested enum-shaped witness is not supported yet (route: {route}); \
     match-per-variant composition (see render_enum_module) only handles an export's own root \
     shape -- no real nested-enum type is registered anywhere in this codebase yet, so exclude \
     this type from Verus export until one needs it ({file}:{line})"
)]
pub(super) struct NestedEnumWitnessUnsupportedSource {
    #[error(ignore)]
    route: String,
    line: u32,
    file: String,
}

impl NestedEnumWitnessUnsupportedSource {
    #[track_caller]
    #[instrument(level = "debug", skip(route))]
    pub fn new(route: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            route: route.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "leaf at {route} reports an impossible per-leaf support kind {kind} -- Mixed only arises \
     from composing multiple leaves, and Opaque leaves cannot reach this renderer \
     (amenable_core::ClassifiedWitness blocks them at cargo check time) ({file}:{line})"
)]
pub(super) struct ImpossibleLeafSupportKindSource {
    #[error(ignore)]
    route: String,
    #[error(ignore)]
    kind: String,
    line: u32,
    file: String,
}

impl ImpossibleLeafSupportKindSource {
    #[track_caller]
    #[instrument(level = "debug", skip(route, kind))]
    pub fn new(route: impl Into<String>, kind: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            route: route.into(),
            kind: kind.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
