//! Unit-level coverage for `amenable`'s Kani CLI-support internals:
//! command construction, result-ledger persistence, and diagnostics
//! parsing. Deliberately not exercised through the real `amenable verify
//! kani` CLI (unlike `gallery_cli_test.rs`'s ledger) -- that would mean
//! actually running Kani, which this workspace treats as an expensive,
//! serialized operation, not something a fast unit test should trigger.

use amenable::{
    KaniProgressTracker, KaniProof, KaniStage, Ledger, ProofStatus, first_diagnostic_line,
    is_kani_timeout, kani_command,
};
use miette::{IntoDiagnostic, WrapErr};

#[test]
fn command_uses_kani_native_timeout_without_an_outer_timeout_program() {
    amenable::init_tracing();
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
            "-Z",
            "function-contracts",
            "-Z",
            "stubbing",
            "--harness-timeout",
            "3m",
        ]
    );
    assert!(!arguments.iter().any(|argument| argument == "timeout"));
}

#[test]
fn ledger_replaces_a_previous_result_for_the_same_proof() -> miette::Result<()> {
    amenable::init_tracing();
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
    amenable::init_tracing();
    assert!(is_kani_timeout(
        "VERIFICATION:- SUCCESSFUL\nverification timed out"
    ));
    assert!(!is_kani_timeout("error: assertion failed"));
}

#[test]
fn diagnostics_prefer_the_first_error_over_kani_startup_output() {
    amenable::init_tracing();
    let diagnostics =
        "Kani Rust Verifier 0.67\nCompiling amenable_kani\nerror[E0433]: missing type\n";

    assert_eq!(
        first_diagnostic_line(diagnostics).as_deref(),
        Some("error[E0433]: missing type")
    );
}

#[test]
fn tracks_the_known_total_from_generated_vcc_line() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("Generated 662 VCC(s), 99 remaining after simplification");
    assert_eq!(tracker.stage(), KaniStage::GeneratingVccs { total: 99 });
}

#[test]
fn counts_one_property_per_decision_procedure_line_after_vcc_total_known() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("Generated 662 VCC(s), 99 remaining after simplification");
    tracker.observe_line("Runtime decision procedure: 0.111205s");
    assert_eq!(
        tracker.stage(),
        KaniStage::Solving {
            completed: 1,
            total: Some(99)
        }
    );
    tracker.observe_line("Runtime decision procedure: 0.00734341s");
    assert_eq!(
        tracker.stage(),
        KaniStage::Solving {
            completed: 2,
            total: Some(99)
        }
    );
}

#[test]
fn counts_properties_even_without_a_known_total() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("Runtime decision procedure: 0.1s");
    assert_eq!(
        tracker.stage(),
        KaniStage::Solving {
            completed: 1,
            total: None
        }
    );
}

#[test]
fn tracks_unwinding_iterations() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line(
        "Unwinding loop _RNvNt...fractional_digit_bytes_to_nanos.0 iteration 8 \
         file crates/amenable_ext/src/temporal_fraction.rs line 23 column 5 \
         function amenable_ext::fractional_digit_bytes_to_nanos thread 0",
    );
    assert_eq!(tracker.stage(), KaniStage::Unwinding { iteration: 8 });
}

#[test]
fn recognizes_successful_and_failed_verdicts() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("VERIFICATION:- SUCCESSFUL");
    assert_eq!(tracker.stage(), KaniStage::Done { successful: true });

    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("VERIFICATION:- FAILED");
    assert_eq!(tracker.stage(), KaniStage::Done { successful: false });
}

#[test]
fn unrecognized_lines_leave_the_stage_unchanged() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("Generated 662 VCC(s), 99 remaining after simplification");
    tracker.observe_line("     --> crates/amenable_kani/src/chrono/civil_naive_week.rs:47:28");
    assert_eq!(tracker.stage(), KaniStage::GeneratingVccs { total: 99 });
}

#[test]
fn kani_banner_and_warnings_stream_live_during_compilation() {
    amenable::init_tracing();
    let mut tracker = KaniProgressTracker::new();
    tracker.observe_line("Kani Rust Verifier 0.67.0 (cargo plugin)");
    assert_eq!(tracker.stage(), KaniStage::Compiling { warnings_seen: 0 });

    tracker.observe_line("warning: unused import: `Days`");
    assert_eq!(tracker.stage(), KaniStage::Compiling { warnings_seen: 1 });

    tracker.observe_line("warning: unused imports: `Datelike` and `MAX_YEAR`");
    assert_eq!(tracker.stage(), KaniStage::Compiling { warnings_seen: 2 });
}
