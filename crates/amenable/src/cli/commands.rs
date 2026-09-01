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
    /// Materialize derived Verus artifacts (witnesses, Exchange-edge
    /// companions, GAAP tokens) from the real registry.
    #[cfg(feature = "verus")]
    Verus(verus_subcommand::VerusArgs),
    /// Materialize derived Creusot artifacts from the real registry.
    #[cfg(feature = "creusot")]
    Creusot(creusot_subcommand::CreusotArgs),
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
            Self::Verus(args) => args.command.act(),
            #[cfg(feature = "creusot")]
            Self::Creusot(args) => args.command.act(),
            Self::Gallery(args) => args.act(),
            Self::DumpRegistry(args) => super::run::run_dump_registry(args),
            Self::Verify(args) => args.backend.act(),
        }
    }
}

/// `amenable verus <leaf>` -- one `#[cfg(feature = "verus")]` gate on this
/// `mod` instead of scattered per-item ones (three enum variants, three
/// match arms, three arg structs, all individually gated before this
/// nesting) -- see `amenable_verus::rust_std::misc::mod`'s own doc
/// comment for the general rationale. `EmitVerusWitnessesArgs`/
/// `EmitVerusExchangeCompanionsArgs`/`EmitVerusGaapTokensArgs` are
/// `pub(in crate::cli)`, not just `pub(super)`: `super::run`'s
/// `run_emit_verus_*` functions (a sibling branch of `cli`, not a
/// descendant of this module) need to name them in their own
/// signatures.
#[cfg(feature = "verus")]
pub(super) mod verus_subcommand {
    use std::path::PathBuf;

    use clap::{Args, Subcommand};
    use tracing::instrument;

    use crate::AmenableResult;

    #[derive(Debug, Args)]
    pub(in crate::cli) struct VerusArgs {
        #[command(subcommand)]
        pub(super) command: VerusCommands,
    }

    #[derive(Debug, Subcommand)]
    pub(in crate::cli) enum VerusCommands {
        /// Materialize derived Verus witness modules into a Verus source tree.
        #[command(name = "emit-witnesses")]
        Witnesses(EmitVerusWitnessesArgs),
        /// Materialize derived Verus `Exchange`-edge companions from the
        /// real registry.
        #[command(name = "emit-exchange-companions")]
        ExchangeCompanions(EmitVerusExchangeCompanionsArgs),
        /// Materialize the derived Verus ledger proof-token companion from
        /// the real registry.
        #[command(name = "emit-gaap-tokens")]
        GaapTokens(EmitVerusGaapTokensArgs),
    }

    impl VerusCommands {
        #[instrument(level = "debug", skip(self), err(level = "warn"))]
        pub(super) fn act(self) -> AmenableResult<()> {
            match self {
                Self::Witnesses(args) => super::super::run::run_emit_verus_witnesses(args),
                Self::ExchangeCompanions(args) => {
                    super::super::run::run_emit_verus_exchange_companions(args)
                }
                Self::GaapTokens(args) => super::super::run::run_emit_verus_gaap_tokens(args),
            }
        }
    }

    #[derive(Debug, Args)]
    pub(in crate::cli) struct EmitVerusWitnessesArgs {
        /// Root `src/` directory of the Verus crate to write into.
        #[arg(long)]
        pub(in crate::cli) root: Option<PathBuf>,
    }

    #[derive(Debug, Args)]
    pub(in crate::cli) struct EmitVerusExchangeCompanionsArgs {
        /// Directory to write generated companion files into.
        #[arg(long)]
        pub(in crate::cli) root: Option<PathBuf>,
    }

    #[derive(Debug, Args)]
    pub(in crate::cli) struct EmitVerusGaapTokensArgs {
        /// File to write the generated companion into.
        #[arg(long)]
        pub(in crate::cli) path: Option<PathBuf>,
    }
}

/// `amenable creusot <leaf>` -- see `verus_subcommand`'s own doc comment
/// for the full rationale (identical pattern, one leaf today).
#[cfg(feature = "creusot")]
pub(super) mod creusot_subcommand {
    use std::path::PathBuf;

    use clap::{Args, Subcommand};
    use tracing::instrument;

    use crate::AmenableResult;

    #[derive(Debug, Args)]
    pub(in crate::cli) struct CreusotArgs {
        #[command(subcommand)]
        pub(super) command: CreusotCommands,
    }

    #[derive(Debug, Subcommand)]
    pub(in crate::cli) enum CreusotCommands {
        /// Materialize derived Creusot `Exchange`-edge companions from the
        /// real registry.
        #[command(name = "emit-companions")]
        EmitCompanions(EmitCreusotCompanionsArgs),
    }

    impl CreusotCommands {
        #[instrument(level = "debug", skip(self), err(level = "warn"))]
        pub(super) fn act(self) -> AmenableResult<()> {
            match self {
                Self::EmitCompanions(args) => super::super::run::run_emit_creusot_companions(args),
            }
        }
    }

    #[derive(Debug, Args)]
    pub(in crate::cli) struct EmitCreusotCompanionsArgs {
        /// Directory to write generated companion files into.
        #[arg(long)]
        pub(in crate::cli) root: Option<PathBuf>,
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

#[derive(Debug, Args)]
pub(super) struct DumpRegistryArgs {
    /// File to receive the JSON registry dump.
    #[arg(short, long)]
    pub(super) out: PathBuf,
}
