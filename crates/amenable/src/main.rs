//! `amenable` CLI: emits provenance certificates for manual review, and
//! audits registered proof chains without running any verifier.

#![forbid(unsafe_code)]

mod boundary;

use amenable::{AmenableResult, assessment, gallery, kani};
use clap::{Args, Parser, Subcommand};
use std::{fs, path::PathBuf, process::ExitCode};

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
        Ok(cli) => match boundary::run(cli) {
            Ok(()) => ExitCode::SUCCESS,
            Err(report) => {
                boundary::exit_on_error(&report);
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            let _ = error.print();
            ExitCode::FAILURE
        }
    }
}

/// Dispatch a parsed [`Cli`] to its subcommand, returning a single unified
/// result type so `boundary::run` has one place to convert failures into a
/// presented [`miette::Report`].
fn dispatch(cli: Cli) -> AmenableResult<()> {
    match cli.command {
        Some(Commands::Audit(args)) => run_audit(args),
        Some(Commands::Assess(args)) => assessment::run(args),
        Some(Commands::Gallery(args)) => gallery::run(args),
        Some(Commands::DumpRegistry(args)) => run_dump_registry(args),
        Some(Commands::Verify(VerifyArgs {
            backend: VerifyBackend::Kani(args),
        })) => kani::verify(args),
        None => run_certify(),
    }
}

fn run_certify() -> AmenableResult<()> {
    let directory = amenable::paths::artifacts_directory().join("std-certificates");
    let paths = amenable::write_rust_std_certificate_artifacts(&directory)
        .map_err(|error| amenable::AmenableError::io(&directory, error))?;

    println!(
        "Wrote {} provenance certificate artifact(s) to {}:",
        paths.len(),
        directory.display()
    );

    for path in &paths {
        println!("  {}", path.display());
    }

    Ok(())
}

fn run_audit(args: AuditArgs) -> AmenableResult<()> {
    let verifiers: Vec<&str> = args.verifiers.iter().map(String::as_str).collect();
    let filter = if verifiers.is_empty() {
        None
    } else {
        Some(verifiers.as_slice())
    };

    let report = match amenable::proof_chain_for_verifiers(&args.name, filter) {
        Ok(report) => report,
        Err(error) => {
            // Write the incompleteness report to the requested path too,
            // not just stderr — it's a legitimate audit artifact in its
            // own right ("here's exactly what's missing"), not only a
            // diagnostic to be read once and discarded.
            match fs::write(&args.out, error.to_string()) {
                Ok(()) => eprintln!("Wrote the incompleteness report to {}", args.out.display()),
                Err(write_error) => eprintln!(
                    "Additionally failed to write that error to {}: {write_error}",
                    args.out.display()
                ),
            }

            return Err(amenable::AmenableError::invariant(error.to_string()));
        }
    };

    fs::write(&args.out, report.to_string())
        .map_err(|error| amenable::AmenableError::io(&args.out, error))?;
    println!(
        "Wrote proof chain for {:?} to {}",
        args.name,
        args.out.display()
    );
    Ok(())
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

/// One [`amenable::ContractRecord`], owned for JSON serialization. Unlike
/// [`ProofRecordDump`], this carries the fragment text itself: external
/// tooling comparing real proof-site expressions against registered
/// contracts needs the literal bound, not just a presence/absence flag.
#[derive(serde::Serialize)]
struct ContractRecordDump {
    evidence: String,
    verifier: String,
    kind: String,
    fragment: String,
}

/// The full registry dump written by `dump-registry`.
#[derive(serde::Serialize)]
struct RegistryDump {
    evidence_links: Vec<EvidenceLinkDump>,
    proof_records: Vec<ProofRecordDump>,
    contract_records: Vec<ContractRecordDump>,
    kani_proofs: Vec<KaniProofDump>,
}

/// One [`amenable::KaniProof`], owned for JSON serialization.
#[derive(serde::Serialize)]
struct KaniProofDump {
    id: String,
    harness: String,
    package: String,
}

fn run_dump_registry(args: DumpRegistryArgs) -> AmenableResult<()> {
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
        contract_records: inventory::iter::<amenable::ContractRecord>()
            .map(|record| ContractRecordDump {
                evidence: record.evidence.to_owned(),
                verifier: record.verifier.to_owned(),
                kind: record.kind.to_owned(),
                fragment: (record.fragment)().to_owned(),
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

    let json = serde_json::to_string_pretty(&dump)?;
    fs::write(&args.out, json).map_err(|error| amenable::AmenableError::io(&args.out, error))?;
    println!("Wrote registry dump to {}", args.out.display());
    Ok(())
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
