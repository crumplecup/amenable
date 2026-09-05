//! Clap types and execution for `amenable verus <leaf>`.
//!
//! One `#[cfg(feature = "verus")]` gate on the `mod verus;` declaration in
//! the parent replaces every scattered per-item gate: the whole file only
//! compiles with the feature on. Each leaf's `run_*` executor lives here
//! next to its arg struct rather than in `cli::run`, so nothing crosses a
//! module boundary; only `VerusArgs` stays visible (the parent's
//! `Commands::Verus` variant names it), the per-leaf arg structs are
//! private.

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
            Self::Witnesses(args) => run_emit_verus_witnesses(args),
            Self::ExchangeCompanions(args) => run_emit_verus_exchange_companions(args),
            Self::GaapTokens(args) => run_emit_verus_gaap_tokens(args),
        }
    }
}

#[derive(Debug, Args)]
struct EmitVerusWitnessesArgs {
    /// Root `src/` directory of the Verus crate to write into.
    #[arg(long)]
    root: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct EmitVerusExchangeCompanionsArgs {
    /// Directory to write generated companion files into.
    #[arg(long)]
    root: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct EmitVerusGaapTokensArgs {
    /// File to write the generated companion into.
    #[arg(long)]
    path: Option<PathBuf>,
}

#[instrument(level = "info", skip(args))]
fn run_emit_verus_witnesses(args: EmitVerusWitnessesArgs) -> AmenableResult<()> {
    let root = args
        .root
        .unwrap_or_else(crate::paths::verus_source_directory);
    let paths = crate::write_verus_witness_modules(&root)?;

    crate::write_stdout_line(format!(
        "Wrote {} Verus witness module(s) under {}:",
        paths.len(),
        root.display()
    ))?;
    for path in &paths {
        crate::write_stdout_line(format!("  {}", path.display()))?;
    }

    Ok(())
}

#[instrument(level = "info", skip(args))]
fn run_emit_verus_exchange_companions(args: EmitVerusExchangeCompanionsArgs) -> AmenableResult<()> {
    let root = args
        .root
        .unwrap_or_else(crate::paths::verus_exchange_generated_directory);
    let paths = crate::write_verus_exchange_companions(&root)?;

    crate::write_stdout_line(format!(
        "Wrote {} Verus Exchange-edge companion(s) under {}:",
        paths.len(),
        root.display()
    ))?;
    for path in &paths {
        crate::write_stdout_line(format!("  {}", path.display()))?;
    }

    Ok(())
}

#[instrument(level = "info", skip(args))]
fn run_emit_verus_gaap_tokens(args: EmitVerusGaapTokensArgs) -> AmenableResult<()> {
    let path = args
        .path
        .unwrap_or_else(crate::paths::verus_gaap_ledger_tokens_path);
    let written = crate::write_verus_gaap_token_companion(&path)?;

    crate::write_stdout_line(format!(
        "Wrote the Verus ledger proof-token companion to {}",
        written.display()
    ))?;

    Ok(())
}
