use clap::Subcommand;
use tracing::instrument;

use crate::AmenableResult;

use super::{AuditArgs, DumpRegistryArgs, VerifyArgs};

/// Top-level `amenable` subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Write the registered proof chain for one evidence name.
    Audit(AuditArgs),
    /// Record and report structured assessments of registered proof harnesses.
    Assess(crate::assessment::AssessArgs),
    /// Materialize derived Verus artifacts (witnesses, Exchange-edge
    /// companions, GAAP tokens) from the real registry.
    #[cfg(feature = "verus")]
    Verus(super::verus::VerusArgs),
    /// Materialize derived Creusot artifacts from the real registry.
    #[cfg(feature = "creusot")]
    Creusot(super::creusot::CreusotArgs),
    /// Run and inspect non-production Kani proof-gallery experiments.
    Gallery(crate::gallery::GalleryArgs),
    /// Write the full evidence and proof registry as JSON.
    #[command(name = "dump-registry")]
    DumpRegistry(DumpRegistryArgs),
    /// Report which temporal contracts each backend proves.
    #[command(name = "temporal-coverage")]
    TemporalCoverage,
    /// Run registered proof harnesses through a verifier backend.
    Verify(VerifyArgs),
}

impl Commands {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub(crate) fn act(self) -> AmenableResult<()> {
        match self {
            Self::Audit(args) => crate::cli::run::run_audit(args),
            Self::Assess(args) => args.act(),
            #[cfg(feature = "verus")]
            Self::Verus(args) => args.act(),
            #[cfg(feature = "creusot")]
            Self::Creusot(args) => args.act(),
            Self::Gallery(args) => args.act(),
            Self::DumpRegistry(args) => crate::cli::run::run_dump_registry(args),
            Self::TemporalCoverage => crate::cli::run::run_temporal_coverage(),
            Self::Verify(args) => args.act(),
        }
    }
}
