//! Clap subcommands. `Commands` and `VerifyBackend` each implement `act`
//! and hand off to nested clap types; leaf argument structs stay plain
//! data read by [`super::run`]'s free functions.

use std::path::PathBuf;

use clap::{Args, Subcommand};
use tracing::instrument;

use crate::AmenableResult;

/// Top-level `amenable` subcommands.
#[derive(Debug, Subcommand)]
pub(super) enum Commands {
    /// Write the registered proof chain for one evidence name.
    Audit(AuditArgs),
    /// Record and report structured assessments of registered proof harnesses.
    Assess(crate::assessment::AssessArgs),
    /// Materialize derived Verus witness modules into a Verus source tree.
    #[cfg(feature = "verus")]
    #[command(name = "emit-verus-witnesses")]
    EmitVerusWitnesses(EmitVerusWitnessesArgs),
    /// Materialize derived Creusot `Exchange`-edge companions from the
    /// real registry.
    #[cfg(feature = "creusot")]
    #[command(name = "emit-creusot-companions")]
    EmitCreusotCompanions(EmitCreusotCompanionsArgs),
    /// Materialize derived Verus `Exchange`-edge companions from the
    /// real registry.
    #[cfg(feature = "verus")]
    #[command(name = "emit-verus-exchange-companions")]
    EmitVerusExchangeCompanions(EmitVerusExchangeCompanionsArgs),
    /// Materialize the derived Verus ledger proof-token companion from
    /// the real registry.
    #[cfg(feature = "verus")]
    #[command(name = "emit-verus-gaap-tokens")]
    EmitVerusGaapTokens(EmitVerusGaapTokensArgs),
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
            Self::EmitVerusWitnesses(args) => super::run::run_emit_verus_witnesses(args),
            #[cfg(feature = "creusot")]
            Self::EmitCreusotCompanions(args) => super::run::run_emit_creusot_companions(args),
            #[cfg(feature = "verus")]
            Self::EmitVerusExchangeCompanions(args) => {
                super::run::run_emit_verus_exchange_companions(args)
            }
            #[cfg(feature = "verus")]
            Self::EmitVerusGaapTokens(args) => super::run::run_emit_verus_gaap_tokens(args),
            Self::Gallery(args) => args.act(),
            Self::DumpRegistry(args) => super::run::run_dump_registry(args),
            Self::Verify(args) => args.backend.act(),
        }
    }
}

#[derive(Debug, Args)]
pub(super) struct VerifyArgs {
    #[command(subcommand)]
    backend: VerifyBackend,
}

#[derive(Debug, Subcommand)]
pub(super) enum VerifyBackend {
    /// Run self-registered Kani proof harnesses.
    Kani(crate::kani::VerifyKaniArgs),
}

impl VerifyBackend {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    fn act(self) -> AmenableResult<()> {
        match self {
            Self::Kani(args) => crate::kani::verify(args),
        }
    }
}

#[derive(Debug, Args)]
pub(super) struct AuditArgs {
    /// Evidence name to audit.
    pub(super) name: String,
    /// File to receive the proof-chain report.
    #[arg(short, long)]
    pub(super) out: PathBuf,
    /// Restrict the report to one verifier; may be repeated.
    #[arg(long)]
    pub(super) verifiers: Vec<String>,
}

#[cfg(feature = "verus")]
#[derive(Debug, Args)]
pub(super) struct EmitVerusWitnessesArgs {
    /// Root `src/` directory of the Verus crate to write into.
    #[arg(long)]
    pub(super) root: Option<PathBuf>,
}

#[cfg(feature = "creusot")]
#[derive(Debug, Args)]
pub(super) struct EmitCreusotCompanionsArgs {
    /// Directory to write generated companion files into.
    #[arg(long)]
    pub(super) root: Option<PathBuf>,
}

#[cfg(feature = "verus")]
#[derive(Debug, Args)]
pub(super) struct EmitVerusExchangeCompanionsArgs {
    /// Directory to write generated companion files into.
    #[arg(long)]
    pub(super) root: Option<PathBuf>,
}

#[cfg(feature = "verus")]
#[derive(Debug, Args)]
pub(super) struct EmitVerusGaapTokensArgs {
    /// File to write the generated companion into.
    #[arg(long)]
    pub(super) path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(super) struct DumpRegistryArgs {
    /// File to receive the JSON registry dump.
    #[arg(short, long)]
    pub(super) out: PathBuf,
}
