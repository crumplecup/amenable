use amenable::cli::Cli;
use clap::Parser;
use clap::error::ErrorKind;

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
