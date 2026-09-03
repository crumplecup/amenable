//! Clap types for `amenable creusot <leaf>`.
//!
//! This mirrors the Verus command split, but with the single Creusot leaf the
//! CLI exposes today.

use std::path::PathBuf;

use clap::{Args, Subcommand};
use tracing::instrument;

use crate::AmenableResult;

#[derive(Debug, Args)]
pub(in crate::cli) struct CreusotArgs {
    #[command(subcommand)]
    command: CreusotCommands,
}

impl CreusotArgs {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub(super) fn act(self) -> AmenableResult<()> {
        self.command.act()
    }
}

#[derive(Debug, Subcommand)]
enum CreusotCommands {
    /// Materialize derived Creusot `Exchange`-edge companions from the real
    /// registry.
    #[command(name = "emit-companions")]
    EmitCompanions(EmitCreusotCompanionsArgs),
}

impl CreusotCommands {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    fn act(self) -> AmenableResult<()> {
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
