//! Registry-inspection clap types that dispatch directly to `cli::run`.

use std::path::PathBuf;

use clap::Args;

/// Arguments for `amenable audit`.
#[derive(Debug, Args)]
pub(in crate::cli) struct AuditArgs {
    /// Evidence name to audit.
    pub(in crate::cli) name: String,
    /// File to receive the proof-chain report.
    #[arg(short, long)]
    pub(in crate::cli) out: PathBuf,
    /// Restrict the report to one verifier; may be repeated.
    #[arg(long)]
    pub(in crate::cli) verifiers: Vec<String>,
}

/// Arguments for `amenable dump-registry`.
#[derive(Debug, Args)]
pub(in crate::cli) struct DumpRegistryArgs {
    /// File to receive the JSON registry dump.
    #[arg(short, long)]
    pub(in crate::cli) out: PathBuf,
}
