//! Clap types for `amenable verus <leaf>`.
//!
//! One `#[cfg(feature = "verus")]` gate on this file replaces the earlier
//! scattered per-item gates. The leaf arg structs stay `pub(in crate::cli)`
//! because `cli::run` is a sibling branch, not a descendant of this module.

use std::path::PathBuf;

use clap::{Args, Subcommand};
use tracing::instrument;

use crate::AmenableResult;

#[derive(Debug, Args)]
pub(in crate::cli) struct VerusArgs {
    #[command(subcommand)]
    command: VerusCommands,
}

impl VerusArgs {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub(super) fn act(self) -> AmenableResult<()> {
        self.command.act()
    }
}

#[derive(Debug, Subcommand)]
enum VerusCommands {
    /// Materialize derived Verus witness modules into a Verus source tree.
    #[command(name = "emit-witnesses")]
    Witnesses(EmitVerusWitnessesArgs),
    /// Materialize derived Verus `Exchange`-edge companions from the real
    /// registry.
    #[command(name = "emit-exchange-companions")]
    ExchangeCompanions(EmitVerusExchangeCompanionsArgs),
    /// Materialize the derived Verus ledger proof-token companion from the
    /// real registry.
    #[command(name = "emit-gaap-tokens")]
    GaapTokens(EmitVerusGaapTokensArgs),
}

impl VerusCommands {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    fn act(self) -> AmenableResult<()> {
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
