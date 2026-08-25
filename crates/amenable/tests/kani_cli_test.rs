//! Unit-level coverage for `amenable::kani`'s CLI-support internals:
//! command construction, result-ledger persistence, and diagnostics
//! parsing. Deliberately not exercised through the real `amenable verify
//! kani` CLI (unlike `gallery_cli_test.rs`'s ledger) -- that would mean
//! actually running Kani, which this workspace treats as an expensive,
//! serialized operation, not something a fast unit test should trigger.

use amenable::KaniProof;
use amenable::kani::{Ledger, ProofStatus, first_diagnostic_line, is_kani_timeout, kani_command};
use miette::{IntoDiagnostic, WrapErr};

#[test]
fn command_uses_kani_native_timeout_without_an_outer_timeout_program() {
    let record = KaniProof::new(
        "amenable_kani::calculator::verify_debit_access_preserves_value".to_owned(),
        "calculator::verify_debit_access_preserves_value".to_owned(),
        "amenable_kani".to_owned(),
    );

    let command = kani_command(&record, "3m");
    let arguments: Vec<_> = command
        .get_args()
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect();

    assert_eq!(command.get_program(), "cargo");
    assert_eq!(
        arguments,
        [
            "kani",
            "-p",
            "amenable_kani",
            "--lib",
            "--all-features",
            "--exact",
            "--harness",
            "calculator::verify_debit_access_preserves_value",
            "-Z",
            "unstable-options",
            "--harness-timeout",
            "3m",
        ]
    );
    assert!(!arguments.iter().any(|argument| argument == "timeout"));
}

#[test]
fn ledger_replaces_a_previous_result_for_the_same_proof() -> miette::Result<()> {
    let path =
        std::env::temp_dir().join(format!("amenable-kani-ledger-{}.csv", std::process::id()));

    let mut ledger = Ledger::load(&path).into_diagnostic().wrap_err("load")?;
    ledger
        .upsert("proof::one", ProofStatus::Failed)
        .into_diagnostic()
        .wrap_err("upsert failed status")?;
    ledger
        .persist(&path)
        .into_diagnostic()
        .wrap_err("persist")?;

    let mut ledger = Ledger::load(&path).into_diagnostic().wrap_err("reload")?;
    ledger
        .upsert("proof::one", ProofStatus::Passed)
        .into_diagnostic()
        .wrap_err("upsert passed status")?;
    ledger
        .persist(&path)
        .into_diagnostic()
        .wrap_err("persist again")?;

    let reloaded = Ledger::load(&path)
        .into_diagnostic()
        .wrap_err("reload again")?;
    assert_eq!(reloaded.len(), 1);
    assert_eq!(reloaded.status_for("proof::one"), Some(ProofStatus::Passed));

    std::fs::remove_file(path)
        .into_diagnostic()
        .wrap_err("remove fixture")?;
    Ok(())
}

#[test]
fn native_timeout_diagnostics_are_distinct_from_failure() {
    assert!(is_kani_timeout(
        "VERIFICATION:- SUCCESSFUL\nverification timed out"
    ));
    assert!(!is_kani_timeout("error: assertion failed"));
}

#[test]
fn diagnostics_prefer_the_first_error_over_kani_startup_output() {
    let diagnostics =
        "Kani Rust Verifier 0.67\nCompiling amenable_kani\nerror[E0433]: missing type\n";

    assert_eq!(
        first_diagnostic_line(diagnostics).as_deref(),
        Some("error[E0433]: missing type")
    );
}
