//! Clap types and dispatch. The `amenable` binary's `main` only parses
//! and calls [`Cli::act`]; every subcommand's own logic and any further
//! nested dispatch live here in the library, never in the binary.

mod commands;
mod run;

use commands::Commands;

use clap::Parser;
use tracing::instrument;

use crate::AmenableResult;

/// Top-level clap parser for the `amenable` binary.
#[derive(Debug, Parser)]
#[command(
    about = "Emit provenance certificates, audit and assess proofs, and run registered verifiers"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    /// Dispatch the selected [`Commands`] variant, or run the default
    /// (no-subcommand) certify behavior.
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub fn act(self) -> AmenableResult<()> {
        match self.command {
            Some(command) => command.act(),
            None => run::run_certify(),
        }
    }
}

/// Install the pretty terminal miette handler (hyperlinks, Unicode
/// box-drawing) before rendering any report. A binary-only nicety, but
/// installing the hook itself defines no clap or error type, so it stays
/// a plain library function `main` calls once, not a reason for a
/// binary-only module.
#[cfg(feature = "cli")]
#[instrument(level = "debug")]
pub fn install_hook() {
    let _ = miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .build(),
        )
    }));
}
