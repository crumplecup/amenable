//! Native leaf sources for Verus checked-leaf call-shape lookup
//! validation -- split out of `verus_export_sources.rs`, itself split
//! out of `verus_export.rs`, once the combined leaf-source file grew
//! past the point cordial's own too-many-types-per-file modularity
//! check allows.

use tracing::instrument;

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("checked leaf at {route} has no \"harness\" metadata entry ({file}:{line})")]
pub(super) struct LeafMissingHarnessMetadataSource {
    #[error(ignore)]
    route: String,
    line: u32,
    file: String,
}

impl LeafMissingHarnessMetadataSource {
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
    "no Verus call shape found for harness `{harness}` (route: {route}) -- it should be a real, \
     public `pub fn` in some crates/amenable_verus/src/**/*.rs carrier file (call shapes derive \
     automatically from real source), or explicitly registered via \
     amenable_std::register_verus_call_shape! if it's a synthetic/test-only shape with no real \
     carrier file behind it ({file}:{line})"
)]
pub(super) struct NoCallShapeForHarnessSource {
    #[error(ignore)]
    harness: String,
    #[error(ignore)]
    route: String,
    line: u32,
    file: String,
}

impl NoCallShapeForHarnessSource {
    #[track_caller]
    #[instrument(level = "debug", skip(harness, route))]
    pub fn new(harness: impl Into<String>, route: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            harness: harness.into(),
            route: route.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "harness `{harness}` (route: {route}) is registered as a bare predicate, not a callable \
     function -- predicate-kind composition isn't wired up yet ({file}:{line})"
)]
pub(super) struct HarnessRegisteredAsBarePredicateSource {
    #[error(ignore)]
    harness: String,
    #[error(ignore)]
    route: String,
    line: u32,
    file: String,
}

impl HarnessRegisteredAsBarePredicateSource {
    #[track_caller]
    #[instrument(level = "debug", skip(harness, route))]
    pub fn new(harness: impl Into<String>, route: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            harness: harness.into(),
            route: route.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display(
    "no ProofTokenMintRecord registered for {token} (named in verus_gaap_tokens_export::TOKENS) ({file}:{line})"
)]
pub(super) struct NoProofTokenMintRecordSource {
    #[error(ignore)]
    token: String,
    line: u32,
    file: String,
}

impl NoProofTokenMintRecordSource {
    #[track_caller]
    #[instrument(level = "debug", skip(token))]
    pub fn new(token: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            token: token.into(),
            line: loc.line(),
            file: loc.file().to_string(),
        }
    }
}
