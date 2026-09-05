//! Clap types and execution for `amenable creusot <leaf>`.
//!
//! Mirrors the Verus command split, with the single Creusot leaf the CLI
//! exposes today. One `#[cfg(feature = "creusot")]` gate on the `mod
//! creusot;` declaration in the parent covers the whole file; the leaf's
//! executor lives here next to its arg struct. Only `CreusotArgs` stays
//! visible (named by the parent's `Commands::Creusot` variant); the leaf
//! arg struct is private.

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
            Self::EmitCompanions(args) => run_emit_creusot_companions(args),
        }
    }
}

#[derive(Debug, Args)]
struct EmitCreusotCompanionsArgs {
    /// Directory to write generated companion files into.
    #[arg(long)]
    root: Option<PathBuf>,
}

#[instrument(level = "info", skip(args))]
fn run_emit_creusot_companions(args: EmitCreusotCompanionsArgs) -> AmenableResult<()> {
    let root = args
        .root
        .unwrap_or_else(crate::paths::creusot_generated_directory);
    let paths = crate::write_creusot_exchange_companions(&root)?;

    crate::write_stdout_line(format!(
        "Wrote {} Creusot Exchange-edge companion(s) under {}:",
        paths.len(),
        root.display()
    ))?;
    for path in &paths {
        crate::write_stdout_line(format!("  {}", path.display()))?;
    }

    Ok(())
}
