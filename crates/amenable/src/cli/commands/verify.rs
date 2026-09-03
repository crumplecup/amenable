//! Clap types for `amenable verify`.

use clap::{Args, Subcommand};
use tracing::instrument;

use crate::AmenableResult;

#[derive(Debug, Args)]
pub(in crate::cli) struct VerifyArgs {
    #[command(subcommand)]
    backend: VerifyBackend,
}

impl VerifyArgs {
    #[instrument(level = "debug", skip(self), err(level = "warn"))]
    pub(super) fn act(self) -> AmenableResult<()> {
        self.backend.act()
    }
}

#[derive(Debug, Subcommand)]
enum VerifyBackend {
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
