use clap::Parser;
use tracing::instrument;

use crate::AmenableResult;

use super::commands::Commands;
use super::run;

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
    /// Dispatch the selected `Commands` variant, or run the default
    /// (no-subcommand) certify behavior.
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub fn act(self) -> AmenableResult<()> {
        match self.command {
            Some(command) => command.act(),
            None => run::run_certify(),
        }
    }
}
