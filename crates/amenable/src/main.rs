//! `amenable` CLI: emits provenance certificates for manual review, and
//! audits registered proof chains without running any verifier.

#![forbid(unsafe_code)]

mod assessment;
mod gallery;
mod kani;

use clap::{Args, Parser, Subcommand};
use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

fn artifacts_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts")
}

#[derive(Debug, Parser)]
#[command(
    about = "Emit provenance certificates, audit and assess proofs, and run registered verifiers"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Write the registered proof chain for one evidence name.
    Audit(AuditArgs),
    /// Record and report structured assessments of registered proof harnesses.
    Assess(assessment::AssessArgs),
    /// Run and inspect non-production Kani proof-gallery experiments.
    Gallery(gallery::GalleryArgs),
    /// Write the full evidence and proof registry as JSON.
    #[command(name = "dump-registry")]
    DumpRegistry(DumpRegistryArgs),
    /// Run registered proof harnesses through a verifier backend.
    Verify(VerifyArgs),
}

#[derive(Debug, Args)]
struct VerifyArgs {
    #[command(subcommand)]
    backend: VerifyBackend,
}

#[derive(Debug, Subcommand)]
enum VerifyBackend {
    /// Run self-registered Kani proof harnesses.
    Kani(kani::VerifyKaniArgs),
}

fn main() -> ExitCode {
    match Cli::try_parse() {
        Ok(Cli {
            command: Some(Commands::Audit(args)),
        }) => run_audit(args),
        Ok(Cli {
            command: Some(Commands::Assess(args)),
        }) => match assessment::run(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Proof assessment failed: {error}");
                ExitCode::FAILURE
            }
        },
        Ok(Cli {
            command: Some(Commands::Gallery(args)),
        }) => match gallery::run(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Proof gallery failed: {error}");
                ExitCode::FAILURE
            }
        },
        Ok(Cli {
            command: Some(Commands::DumpRegistry(args)),
        }) => run_dump_registry(args),
        Ok(Cli {
            command:
                Some(Commands::Verify(VerifyArgs {
                    backend: VerifyBackend::Kani(args),
                })),
        }) => match kani::verify(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Kani verification failed: {error}");
                ExitCode::FAILURE
            }
        },
        Ok(Cli { command: None }) => run_certify(),
        Err(error) => {
            let _ = error.print();
            ExitCode::FAILURE
        }
    }
}

fn run_certify() -> ExitCode {
    let directory = artifacts_directory().join("std-certificates");

    match amenable::write_rust_std_certificate_artifacts(&directory) {
        Ok(paths) => {
            println!(
                "Wrote {} provenance certificate artifact(s) to {}:",
                paths.len(),
                directory.display()
            );

            for path in &paths {
                println!("  {}", path.display());
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "Failed to write provenance certificate artifacts to {}: {error}",
                directory.display()
            );

            ExitCode::FAILURE
        }
    }
}

fn run_audit(args: AuditArgs) -> ExitCode {
    let verifiers: Vec<&str> = args.verifiers.iter().map(String::as_str).collect();
    let filter = if verifiers.is_empty() {
        None
    } else {
        Some(verifiers.as_slice())
    };

    let report = match amenable::proof_chain_for_verifiers(&args.name, filter) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");

            // Write the incompleteness report to the requested path too,
            // not just stderr — it's a legitimate audit artifact in its
            // own right ("here's exactly what's missing"), not only a
            // diagnostic to be read once and discarded.
            if let Err(write_error) = fs::write(&args.out, error.to_string()) {
                eprintln!(
                    "Additionally failed to write that error to {}: {write_error}",
                    args.out.display()
                );
            } else {
                eprintln!("Wrote the incompleteness report to {}", args.out.display());
            }

            return ExitCode::FAILURE;
        }
    };

    match fs::write(&args.out, report.to_string()) {
        Ok(()) => {
            println!(
                "Wrote proof chain for {:?} to {}",
                args.name,
                args.out.display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "Failed to write proof chain to {}: {error}",
                args.out.display()
            );
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug, Args)]
struct AuditArgs {
    /// Evidence name to audit.
    name: String,
    /// File to receive the proof-chain report.
    #[arg(short, long)]
    out: PathBuf,
    /// Restrict the report to one verifier; may be repeated.
    #[arg(long)]
    verifiers: Vec<String>,
}

/// One [`amenable::EvidenceLink`], owned for JSON serialization.
#[derive(serde::Serialize)]
struct EvidenceLinkDump {
    name: String,
    basis: String,
    index: usize,
}

/// One [`amenable::ProofRecord`], owned for JSON serialization. Never
/// invokes `describe()` — external tooling needs presence/absence per
/// `(evidence, verifier)`, not the rendered proof text, and calling every
/// registered `describe()` would be needlessly slow for a coverage check.
#[derive(serde::Serialize)]
struct ProofRecordDump {
    evidence: String,
    verifier: String,
}

/// The full registry dump written by `dump-registry`.
#[derive(serde::Serialize)]
struct RegistryDump {
    evidence_links: Vec<EvidenceLinkDump>,
    proof_records: Vec<ProofRecordDump>,
    kani_proofs: Vec<KaniProofDump>,
}

/// One [`amenable::KaniProof`], owned for JSON serialization.
#[derive(serde::Serialize)]
struct KaniProofDump {
    id: String,
    harness: String,
    package: String,
}

fn run_dump_registry(args: DumpRegistryArgs) -> ExitCode {
    let dump = RegistryDump {
        evidence_links: inventory::iter::<amenable::EvidenceLink>()
            .map(|link| EvidenceLinkDump {
                name: link.name.to_owned(),
                basis: link.basis.to_owned(),
                index: link.index,
            })
            .collect(),
        proof_records: inventory::iter::<amenable::ProofRecord>()
            .map(|record| ProofRecordDump {
                evidence: record.evidence.to_owned(),
                verifier: record.verifier.to_owned(),
            })
            .collect(),
        kani_proofs: inventory::iter::<amenable::KaniProofRegistration>()
            .map(|registration| (registration.proof)())
            .map(|record| KaniProofDump {
                id: record.id,
                harness: record.harness,
                package: record.package,
            })
            .collect(),
    };

    let json = match serde_json::to_string_pretty(&dump) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("Failed to serialize registry dump: {error}");
            return ExitCode::FAILURE;
        }
    };

    match fs::write(&args.out, json) {
        Ok(()) => {
            println!("Wrote registry dump to {}", args.out.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "Failed to write registry dump to {}: {error}",
                args.out.display()
            );
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug, Args)]
struct DumpRegistryArgs {
    /// File to receive the JSON registry dump.
    #[arg(short, long)]
    out: PathBuf,
}

#[cfg(test)]
mod tests {
    use clap::error::ErrorKind;

    use super::Cli;
    use clap::Parser;

    #[test]
    fn clap_rejects_a_single_proof_combined_with_a_retry_selector() {
        let error = Cli::try_parse_from([
            "amenable",
            "verify",
            "kani",
            "--proof",
            "amenable_kani::calculator::verify_debit_access_preserves_value",
            "--failed",
        ])
        .expect_err("conflicting selectors must be rejected");

        assert_eq!(error.kind(), ErrorKind::ArgumentConflict);
    }
}
