//! `amenable` CLI: emits provenance certificates for manual review, and
//! audits registered proof chains without running any verifier.

#![forbid(unsafe_code)]

use amenable::Cli;
use clap::Parser;
use miette::Report;

use tracing::instrument;
#[instrument(level = "info", err(level = "warn"))]
fn main() -> miette::Result<()> {
    amenable::init_tracing();
    amenable::install_hook();
    // `Report::from`, not `.into_diagnostic()`: `AmenableError` already
    // implements `miette::Diagnostic` (real code/help), and `.into_diagnostic()`
    // is the *generic* std-error wrapper that never consults `Diagnostic`
    // at all -- routing through it here would silently discard the code
    // and help text this crate's own `Diagnostic` impl exists to carry.
    Cli::parse().act().map_err(Report::from)
}
