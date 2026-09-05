//! Clap subcommands. `Commands` implements the top-level dispatch, while
//! each command family keeps its own clap types and nested dispatch in a
//! sibling module.

// `creusot`/`verus` are whole feature-gated subcommand families: the
// `#[cfg]` sits once on each private `mod` declaration (the shape the
// cfg-scatter lint recommends), and the only other gated sites are the
// exempt enum variant and its one dispatch arm below. Each family's arg
// types are named through the module path (`creusot::CreusotArgs`) and its
// leaf executors live in that same module, so nothing crosses a boundary
// and there is no gated re-export to scatter the predicate onto.
#[cfg(feature = "creusot")]
mod creusot;
mod inspection;
mod verify;
#[cfg(feature = "verus")]
mod verus;

use clap::Subcommand;
use tracing::instrument;

use crate::AmenableResult;

pub(in crate::cli) use inspection::{AuditArgs, DumpRegistryArgs};
pub(in crate::cli) use verify::VerifyArgs;

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
    Verus(verus::VerusArgs),
    /// Materialize derived Creusot artifacts from the real registry.
    #[cfg(feature = "creusot")]
    Creusot(creusot::CreusotArgs),
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
