//! Business logic for each [`super::Commands`] variant. Plain functions,
//! never taking a clap `Subcommand` type — [`super::commands::Commands::act`]
//! is the only dispatcher.

use std::fs;

use tracing::instrument;

use super::commands::{AuditArgs, DumpRegistryArgs};
use crate::AmenableResult;

#[instrument(level = "info")]
pub(super) fn run_certify() -> AmenableResult<()> {
    let directory = crate::paths::artifacts_directory().join("std-certificates");
    let paths = crate::write_rust_std_certificate_artifacts(&directory)
        .map_err(crate::AmenableError::std)?;

    crate::write_stdout_line(format!(
        "Wrote {} provenance certificate artifact(s) to {}:",
        paths.len(),
        directory.display()
    ))?;

    for path in &paths {
        crate::write_stdout_line(format!("  {}", path.display()))?;
    }

    Ok(())
}

#[instrument(level = "info", skip(args))]
pub(super) fn run_audit(args: AuditArgs) -> AmenableResult<()> {
    let verifiers: Vec<&str> = args.verifiers.iter().map(String::as_str).collect();
    let filter = if verifiers.is_empty() {
        None
    } else {
        Some(verifiers.as_slice())
    };

    let report = match crate::proof_chain_for_verifiers(&args.name, filter) {
        Ok(report) => report,
        Err(error) => {
            // Write the incompleteness report to the requested path too,
            // not just stderr — it's a legitimate audit artifact in its
            // own right ("here's exactly what's missing"), not only a
            // diagnostic to be read once and discarded.
            match fs::write(&args.out, error.to_string()) {
                Ok(()) => crate::write_stderr_line(format!(
                    "Wrote the incompleteness report to {}",
                    args.out.display()
                ))?,
                Err(write_error) => crate::write_stderr_line(format!(
                    "Additionally failed to write that error to {}: {write_error}",
                    args.out.display()
                ))?,
            }

            return Err(crate::AmenableError::chain(error));
        }
    };

    fs::write(&args.out, report.to_string())
        .map_err(|error| crate::AmenableError::io(&args.out, error))?;
    crate::write_stdout_line(format!(
        "Wrote proof chain for {:?} to {}",
        args.name,
        args.out.display()
    ))?;
    Ok(())
}

#[instrument(level = "info", skip(args))]
pub(super) fn run_dump_registry(args: DumpRegistryArgs) -> AmenableResult<()> {
    let dump = crate::registry_dump::RegistryDump::collect();
    let json = serde_json::to_string_pretty(&dump)?;
    fs::write(&args.out, json).map_err(|error| crate::AmenableError::io(&args.out, error))?;
    crate::write_stdout_line(format!("Wrote registry dump to {}", args.out.display()))?;
    Ok(())
}

/// `emit-verus-*` commands, one `#[cfg(feature = "verus")]` gate on this
/// `mod` instead of scattered per-function ones -- see
/// `amenable_verus::rust_std::misc::mod`'s own doc comment for the full
/// rationale (same pattern, applied to plain functions instead of
/// re-exports).
#[cfg(feature = "verus")]
mod verus_commands {
    use super::super::commands::{
        EmitVerusExchangeCompanionsArgs, EmitVerusGaapTokensArgs, EmitVerusWitnessesArgs,
    };
    use crate::AmenableResult;

    use tracing::instrument;

    #[instrument(level = "info", skip(args))]
    pub(in super::super) fn run_emit_verus_witnesses(
        args: EmitVerusWitnessesArgs,
    ) -> AmenableResult<()> {
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
    pub(in super::super) fn run_emit_verus_exchange_companions(
        args: EmitVerusExchangeCompanionsArgs,
    ) -> AmenableResult<()> {
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
    pub(in super::super) fn run_emit_verus_gaap_tokens(
        args: EmitVerusGaapTokensArgs,
    ) -> AmenableResult<()> {
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
}
#[cfg(feature = "verus")]
pub(super) use verus_commands::{
    run_emit_verus_exchange_companions, run_emit_verus_gaap_tokens, run_emit_verus_witnesses,
};

/// `emit-creusot-companions`, one `#[cfg(feature = "creusot")]` gate on
/// this `mod` instead of scattered per-item ones -- see `verus_commands`'
/// own doc comment for the full rationale.
#[cfg(feature = "creusot")]
mod creusot_commands {
    use super::super::commands::EmitCreusotCompanionsArgs;
    use crate::AmenableResult;

    use tracing::instrument;

    #[instrument(level = "info", skip(args))]
    pub(in super::super) fn run_emit_creusot_companions(
        args: EmitCreusotCompanionsArgs,
    ) -> AmenableResult<()> {
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
}
#[cfg(feature = "creusot")]
pub(super) use creusot_commands::run_emit_creusot_companions;
