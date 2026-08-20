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
    /// Materialize derived Verus witness modules into a Verus source tree.
    #[cfg(feature = "verus")]
    #[command(name = "emit-verus-witnesses")]
    EmitVerusWitnesses(EmitVerusWitnessesArgs),
    /// Materialize derived Creusot `Exchange`-edge companions from the
    /// real registry.
    #[cfg(feature = "creusot")]
    #[command(name = "emit-creusot-companions")]
    EmitCreusotCompanions(EmitCreusotCompanionsArgs),
    /// Materialize derived Verus `Exchange`-edge companions from the
    /// real registry.
    #[cfg(feature = "verus")]
    #[command(name = "emit-verus-exchange-companions")]
    EmitVerusExchangeCompanions(EmitVerusExchangeCompanionsArgs),
    /// Materialize the derived Verus ledger proof-token companion from
    /// the real registry.
    #[cfg(feature = "verus")]
    #[command(name = "emit-verus-gaap-tokens")]
    EmitVerusGaapTokens(EmitVerusGaapTokensArgs),
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
        #[cfg(feature = "verus")]
        Some(Commands::EmitVerusWitnesses(args)) => run_emit_verus_witnesses(args),
        #[cfg(feature = "creusot")]
        Some(Commands::EmitCreusotCompanions(args)) => run_emit_creusot_companions(args),
        #[cfg(feature = "verus")]
        Some(Commands::EmitVerusExchangeCompanions(args)) => {
            run_emit_verus_exchange_companions(args)
        }
        #[cfg(feature = "verus")]
        Some(Commands::EmitVerusGaapTokens(args)) => run_emit_verus_gaap_tokens(args),
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

            return Err(amenable::AmenableError::chain(error));
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

#[cfg(feature = "verus")]
fn run_emit_verus_witnesses(args: EmitVerusWitnessesArgs) -> AmenableResult<()> {
    let root = args
        .root
        .unwrap_or_else(amenable::paths::verus_source_directory);
    let paths = amenable::write_verus_witness_modules(&root)?;

    println!(
        "Wrote {} Verus witness module(s) under {}:",
        paths.len(),
        root.display()
    );
    for path in &paths {
        println!("  {}", path.display());
    }

    Ok(())
}

#[cfg(feature = "creusot")]
fn run_emit_creusot_companions(args: EmitCreusotCompanionsArgs) -> AmenableResult<()> {
    let root = args
        .root
        .unwrap_or_else(amenable::paths::creusot_generated_directory);
    let paths = amenable::write_creusot_exchange_companions(&root)?;

    println!(
        "Wrote {} Creusot Exchange-edge companion(s) under {}:",
        paths.len(),
        root.display()
    );
    for path in &paths {
        println!("  {}", path.display());
    }

    Ok(())
}

#[cfg(feature = "creusot")]
#[derive(Debug, Args)]
struct EmitCreusotCompanionsArgs {
    /// Directory to write generated companion files into.
    #[arg(long)]
    root: Option<PathBuf>,
}

#[cfg(feature = "verus")]
fn run_emit_verus_exchange_companions(args: EmitVerusExchangeCompanionsArgs) -> AmenableResult<()> {
    let root = args
        .root
        .unwrap_or_else(amenable::paths::verus_exchange_generated_directory);
    let paths = amenable::write_verus_exchange_companions(&root)?;

    println!(
        "Wrote {} Verus Exchange-edge companion(s) under {}:",
        paths.len(),
        root.display()
    );
    for path in &paths {
        println!("  {}", path.display());
    }

    Ok(())
}

#[cfg(feature = "verus")]
#[derive(Debug, Args)]
struct EmitVerusExchangeCompanionsArgs {
    /// Directory to write generated companion files into.
    #[arg(long)]
    root: Option<PathBuf>,
}

#[cfg(feature = "verus")]
fn run_emit_verus_gaap_tokens(args: EmitVerusGaapTokensArgs) -> AmenableResult<()> {
    let path = args
        .path
        .unwrap_or_else(amenable::paths::verus_gaap_ledger_tokens_path);
    let written = amenable::write_verus_gaap_token_companion(&path)?;

    println!(
        "Wrote the Verus ledger proof-token companion to {}",
        written.display()
    );

    Ok(())
}

#[cfg(feature = "verus")]
#[derive(Debug, Args)]
struct EmitVerusGaapTokensArgs {
    /// File to write the generated companion into.
    #[arg(long)]
    path: Option<PathBuf>,
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

#[cfg(feature = "verus")]
#[derive(Debug, Args)]
struct EmitVerusWitnessesArgs {
    /// Root `src/` directory of the Verus crate to write into.
    #[arg(long)]
    root: Option<PathBuf>,
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

/// One explicit [`amenable::WitnessExportRecord`], owned for JSON
/// serialization.
#[derive(serde::Serialize)]
struct WitnessExportRecordDump {
    verifier: String,
    evidence: String,
    destination_module: String,
    support_kind: String,
    trivial: usize,
    checked: usize,
    trusted: usize,
    opaque: usize,
    artifact: WitnessArtifactNodeDump,
}

/// One structured witness artifact node, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactNodeDump {
    shape: String,
    kind: String,
    tag: Option<String>,
    variant: Option<String>,
    detail: Option<String>,
    metadata: Vec<WitnessArtifactMetadataDump>,
    support_kind: String,
    trivial: usize,
    checked: usize,
    trusted: usize,
    opaque: usize,
    members: Vec<WitnessArtifactMemberDump>,
    variants: Vec<WitnessArtifactVariantDump>,
}

/// One named witness artifact member, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactMemberDump {
    label: String,
    artifact: WitnessArtifactNodeDump,
}

/// One named witness artifact variant, owned for JSON serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactVariantDump {
    name: String,
    artifact: WitnessArtifactNodeDump,
}

/// One structured witness artifact metadata fact, owned for JSON
/// serialization.
#[derive(serde::Serialize)]
struct WitnessArtifactMetadataDump {
    key: String,
    value: String,
}

/// The full registry dump written by `dump-registry`.
#[derive(serde::Serialize)]
struct RegistryDump {
    evidence_links: Vec<EvidenceLinkDump>,
    proof_records: Vec<ProofRecordDump>,
    contract_records: Vec<ContractRecordDump>,
    witness_export_records: Vec<WitnessExportRecordDump>,
    kani_proofs: Vec<KaniProofDump>,
}

/// One [`amenable::KaniProof`], owned for JSON serialization.
#[derive(serde::Serialize)]
struct KaniProofDump {
    id: String,
    harness: String,
    package: String,
}

fn dump_witness_artifact(node: amenable::WitnessArtifactNode) -> WitnessArtifactNodeDump {
    let support = node.support;

    WitnessArtifactNodeDump {
        shape: node.shape.as_str().to_owned(),
        kind: node.kind.as_str().to_owned(),
        tag: node.tag,
        variant: node.variant,
        detail: node.detail,
        metadata: node
            .metadata
            .into_iter()
            .map(|entry| WitnessArtifactMetadataDump {
                key: entry.key().to_owned(),
                value: entry.value().to_owned(),
            })
            .collect(),
        support_kind: support.kind().as_str().to_owned(),
        trivial: support.trivial(),
        checked: support.checked(),
        trusted: support.trusted(),
        opaque: support.opaque(),
        members: node
            .members
            .into_iter()
            .map(|member| WitnessArtifactMemberDump {
                label: member.label,
                artifact: dump_witness_artifact(*member.artifact),
            })
            .collect(),
        variants: node
            .variants
            .into_iter()
            .map(|variant| WitnessArtifactVariantDump {
                name: variant.name,
                artifact: dump_witness_artifact(*variant.artifact),
            })
            .collect(),
    }
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
        witness_export_records: amenable::witness_exports()
            .into_iter()
            .map(|record| WitnessExportRecordDump {
                support_kind: record.support.kind().as_str().to_owned(),
                trivial: record.support.trivial(),
                checked: record.support.checked(),
                trusted: record.support.trusted(),
                opaque: record.support.opaque(),
                artifact: dump_witness_artifact(record.artifact),
                verifier: record.verifier,
                evidence: record.evidence,
                destination_module: record.destination_module,
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
