//! Clap subcommands. `Commands` implements the top-level dispatch, while
//! each command family keeps its own clap types and nested dispatch in a
//! sibling module.

#[cfg(feature = "creusot")]
mod creusot;
mod inspection;
mod verify;
#[cfg(feature = "verus")]
mod verus;

use clap::Subcommand;
use tracing::instrument;

use crate::AmenableResult;

#[cfg(feature = "creusot")]
pub(in crate::cli) use creusot::{CreusotArgs, EmitCreusotCompanionsArgs};
pub(in crate::cli) use inspection::{AuditArgs, DumpRegistryArgs};
pub(in crate::cli) use verify::VerifyArgs;
#[cfg(feature = "verus")]
pub(in crate::cli) use verus::{
    EmitVerusExchangeCompanionsArgs, EmitVerusGaapTokensArgs, EmitVerusWitnessesArgs, VerusArgs,
};

/// Top-level `amenable` subcommands.
#[derive(Debug, Subcommand)]
pub(super) enum Commands {
    /// Write the registered proof chain for one evidence name.
    Audit(AuditArgs),
    /// Record and report structured assessments of registered proof harnesses.
    Assess(crate::assessment::AssessArgs),
    /// Materialize derived Verus artifacts (witnesses, Exchange-edge
    /// companions, GAAP tokens) from the real registry.
    #[cfg(feature = "verus")]
    Verus(VerusArgs),
    /// Materialize derived Creusot artifacts from the real registry.
    #[cfg(feature = "creusot")]
    Creusot(CreusotArgs),
    /// Run and inspect non-production Kani proof-gallery experiments.
    Gallery(crate::gallery::GalleryArgs),
    /// Write the full evidence and proof registry as JSON.
    #[command(name = "dump-registry")]
    DumpRegistry(DumpRegistryArgs),
    /// Run registered proof harnesses through a verifier backend.
    Verify(VerifyArgs),
}

impl Commands {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub(super) fn act(self) -> AmenableResult<()> {
        match self {
            Self::Audit(args) => super::run::run_audit(args),
            Self::Assess(args) => args.act(),
            #[cfg(feature = "verus")]
            Self::Verus(args) => args.act(),
            #[cfg(feature = "creusot")]
            Self::Creusot(args) => args.act(),
            Self::Gallery(args) => args.act(),
            Self::DumpRegistry(args) => super::run::run_dump_registry(args),
            Self::Verify(args) => args.act(),
        }
    }
}
