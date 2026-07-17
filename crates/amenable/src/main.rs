//! `amenable` CLI: emits provenance certificates for manual review, and
//! audits registered proof chains without running any verifier.

#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

fn artifacts_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts")
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("audit") => run_audit(&args[1..]),
        Some("dump-registry") => run_dump_registry(&args[1..]),
        Some(other) => {
            eprintln!("Unrecognized command: {other}");
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
        None => run_certify(),
    }
}

const USAGE: &str = "Usage:\n  amenable                                        Write std-lib provenance certificates\n  amenable audit <name> --out <file> [--verifier <name>]...  Write the registered proof chain for <name>, optionally filtered to one or more verifiers (e.g. --verifier kani)\n  amenable dump-registry --out <file>             Write every registered EvidenceLink/ProofRecord as JSON, for external coverage tooling (e.g. elicit_doc)";

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

fn run_audit(args: &[String]) -> ExitCode {
    let Some(parsed) = parse_audit_args(args) else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };

    let verifiers: Vec<&str> = parsed.verifiers.iter().map(String::as_str).collect();
    let filter = if verifiers.is_empty() {
        None
    } else {
        Some(verifiers.as_slice())
    };

    let report = match amenable::proof_chain_for_verifiers(parsed.name, filter) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("{error}");

            // Write the incompleteness report to the requested path too,
            // not just stderr — it's a legitimate audit artifact in its
            // own right ("here's exactly what's missing"), not only a
            // diagnostic to be read once and discarded.
            if let Err(write_error) = fs::write(&parsed.out, error.to_string()) {
                eprintln!(
                    "Additionally failed to write that error to {}: {write_error}",
                    parsed.out.display()
                );
            } else {
                eprintln!(
                    "Wrote the incompleteness report to {}",
                    parsed.out.display()
                );
            }

            return ExitCode::FAILURE;
        }
    };

    match fs::write(&parsed.out, report.to_string()) {
        Ok(()) => {
            println!(
                "Wrote proof chain for {:?} to {}",
                parsed.name,
                parsed.out.display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "Failed to write proof chain to {}: {error}",
                parsed.out.display()
            );
            ExitCode::FAILURE
        }
    }
}

struct AuditArgs<'a> {
    name: &'a str,
    out: PathBuf,
    verifiers: Vec<String>,
}

fn parse_audit_args(args: &[String]) -> Option<AuditArgs<'_>> {
    let mut name = None;
    let mut out = None;
    let mut verifiers = Vec::new();
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--out" | "-o" => out = Some(PathBuf::from(iter.next()?)),
            "--verifier" => verifiers.push(iter.next()?.clone()),
            _ if name.is_none() => name = Some(arg.as_str()),
            _ => return None,
        }
    }

    Some(AuditArgs {
        name: name?,
        out: out?,
        verifiers,
    })
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
}

fn run_dump_registry(args: &[String]) -> ExitCode {
    let Some(out) = parse_dump_registry_args(args) else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };

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
    };

    let json = match serde_json::to_string_pretty(&dump) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("Failed to serialize registry dump: {error}");
            return ExitCode::FAILURE;
        }
    };

    match fs::write(&out, json) {
        Ok(()) => {
            println!("Wrote registry dump to {}", out.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!(
                "Failed to write registry dump to {}: {error}",
                out.display()
            );
            ExitCode::FAILURE
        }
    }
}

fn parse_dump_registry_args(args: &[String]) -> Option<PathBuf> {
    let mut out = None;
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--out" | "-o" => out = Some(PathBuf::from(iter.next()?)),
            _ => return None,
        }
    }

    out
}
